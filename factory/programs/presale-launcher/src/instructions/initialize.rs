use crate::constants::*;
use crate::state::Launcher;
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct InitArgs {
    pub admin: Pubkey,
    pub fee_pool: Pubkey,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = Launcher::LEN + 8,
        seeds = [LAUNCHER_SEED],
        bump
    )]
    pub launcher: Box<Account<'info, Launcher>>,
    pub system_program: Program<'info, System>,
}

pub fn initialize(ctx: Context<Initialize>, args: InitArgs) -> Result<()> {
    let launcher = &mut ctx.accounts.launcher;
    launcher.initialize(args.fee_pool, args.admin)
}
