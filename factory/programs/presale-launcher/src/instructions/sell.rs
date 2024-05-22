use crate::error::ErrorCode;
use crate::state::Launchpad;
use crate::{constants::*, state::Launcher};
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct SellArgs {
    pub amount: u64,
}

#[derive(Accounts)]
pub struct Sell<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
      mut, 
      seeds = [LAUNCHER_SEED], 
      bump,
      constraint = launcher.fee_pool == protool_fee_pool.key() @ ErrorCode::InvalidFeePool
    )]
    pub launcher: Account<'info, Launcher>,

    /// CHECK: The protool fee pool account is checked by constraint
    #[account(mut)]
    pub protool_fee_pool: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [LAUNCHPAD_SEED, signer.key().as_ref()],
        bump,
        constraint = launchpad.owner == signer.key() @ ErrorCode::InvalidLaunchpadOwner
    )]
    pub launchpad: Account<'info, Launchpad>,

    #[account(
        mut ,
        seeds = [LAUNCHPAD_PRESALE_SEED, launchpad.key().as_ref()],
        bump,
        token::mint = mint,
        token::authority = presale_token_account
    )]
    pub presale_token_account: Account<'info, TokenAccount>,

    #[account()]
    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = signer
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    /// CHECK: This is safe as it's pda
    #[account(
        mut,
        seeds = [LAUNCHPAD_PRESALE_TREASURY_SEED, launchpad.key().as_ref()],
        bump
    )]
    pub presale_treasury: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn sell(ctx: Context<Sell>, args: SellArgs) -> Result<()> {
    let launchpad = &ctx.accounts.launchpad;
    launchpad.check_launchpad_started()?;

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(), 
            Transfer {
                authority: ctx.accounts.signer.to_account_info(),
                from: ctx.accounts.user_token_account.to_account_info(),
                to: ctx.accounts.presale_token_account.to_account_info(),
            },
        ),
        args.amount,
    )?;

    **ctx.accounts.presale_treasury.try_borrow_mut_lamports()? -= args.amount / 1_000_000_000 * PRESALE_PRICE;
    **ctx.accounts.signer.try_borrow_mut_lamports()? += args.amount / 1_000_000_000 * PRESALE_PRICE * 99 / 100;

    **ctx.accounts.protool_fee_pool.try_borrow_mut_lamports()? += args.amount / 1_000_000_000 * PRESALE_PRICE / 100;

    Ok(())
}
