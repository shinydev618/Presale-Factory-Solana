use crate::constants::*;
use crate::error::ErrorCode;
use crate::state::Launchpad;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct InitLaunchpadArgs {
    pub owner: Pubkey,
    pub mint: Pubkey,
}

#[derive(Accounts)]
#[instruction(args: InitLaunchpadArgs)]
pub struct InitializeLaunchpad<'info> {
    #[account(
        mut, 
        constraint = signer.key() == args.owner @ ErrorCode::InvalidOwner
    )]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = Launchpad::LEN + 8,
        seeds = [LAUNCHPAD_SEED, signer.key().as_ref()],
        bump
    )]
    pub launchpad: Account<'info, Launchpad>,

    #[account()]
    pub mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
}

pub fn initialize_launchpad(
    ctx: Context<InitializeLaunchpad>,
    args: InitLaunchpadArgs,
) -> Result<()> {
    let launchpad = &mut ctx.accounts.launchpad;
    launchpad.initialize_launchpad(args.owner, args.mint)
}
