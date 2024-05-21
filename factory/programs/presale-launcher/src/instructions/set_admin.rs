use crate::constants::*;
use crate::state::Launcher;
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct SetAdminArgs {
    pub admin: Pubkey,
}

#[derive(Accounts)]
pub struct SetAdmin<'info> {
    #[account(mut, seeds = [LAUNCHER_SEED.as_bytes()], bump)]
    pub launcher: Account<'info, Launcher>,
    pub authority: Signer<'info>,
}

pub fn set_admin(ctx: Context<SetAdmin>, args: SetAdminArgs) -> Result<()> {
    let launcher = &mut ctx.accounts.launcher;
    let admin = &ctx.accounts.authority.key();
    launcher.check_admin(*admin)?;
    launcher.set_admin(args.admin)
}
