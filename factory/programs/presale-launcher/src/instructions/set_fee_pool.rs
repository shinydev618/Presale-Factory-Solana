use crate::constants::*;
use crate::state::Launcher;
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct SetFeePoolArgs {
    pub fee_pool: Pubkey,
}

#[derive(Accounts)]
pub struct SetFeePool<'info> {
    #[account(mut, seeds = [LAUNCHER_SEED], bump)]
    pub launcher: Account<'info, Launcher>,
    pub authority: Signer<'info>,
}

pub fn set_fee_pool(ctx: Context<SetFeePool>, args: SetFeePoolArgs) -> Result<()> {
    let launcher = &mut ctx.accounts.launcher;
    let admin = &ctx.accounts.authority.key();
    launcher.check_admin(*admin)?;
    launcher.set_fee_pool(args.fee_pool)
}
