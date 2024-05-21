use crate::error::ErrorCode;
use crate::state::Launchpad;
use crate::{constants::*, state::Launcher};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_lang::solana_program::system_instruction;
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
      seeds = [LAUNCHER_SEED.as_bytes()], 
      bump,
      constraint = launcher.fee_pool == protool_fee_pool.key() @ ErrorCode::InvalidFeePool
    )]
    pub launcher: Account<'info, Launcher>,

    /// CHECK: The protool fee pool account is checked by constraint
    #[account(mut)]
    pub protool_fee_pool: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [LAUNCHPAD_SEED.as_bytes(), signer.key().as_ref()],
        bump,
        constraint = launchpad.owner == signer.key() @ ErrorCode::InvalidLaunchpadOwner
    )]
    pub launchpad: Account<'info, Launchpad>,

    #[account(
        mut ,
        seeds = [LAUNCHPAD_PRESALE_SEED.as_bytes(), launchpad.key().as_ref()],
        bump,
        token::mint = mint,
        token::authority = presale_token_account
    )]
    pub presale_token_account: Account<'info, TokenAccount>,

    #[account(
        mint::authority = signer
    )]
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
        seeds = [LAUNCHPAD_PRESALE_TREASURY_SEED.as_bytes(), launchpad.key().as_ref()],
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

    let launchpad_key = launchpad.key();

    let seeds = &[LAUNCHPAD_PRESALE_TREASURY_SEED.as_bytes(), launchpad_key.as_ref()];

    let transfer_instruction = system_instruction::transfer(
      &ctx.accounts.presale_treasury.key(), 
        ctx.accounts.signer.key, 
        args.amount * PRESALE_PRICE * 99 / 100
    );
    invoke_signed(
        &transfer_instruction,
        &[
            ctx.accounts.presale_treasury.to_account_info(),
            ctx.accounts.signer.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
        &[seeds],
    )?;

    let transfer_instruction = system_instruction::transfer(
      &ctx.accounts.presale_treasury.key(), 
        ctx.accounts.protool_fee_pool.key, 
        args.amount * PRESALE_PRICE / 100
    );
    invoke_signed(
        &transfer_instruction,
        &[
            ctx.accounts.presale_treasury.to_account_info(),
            ctx.accounts.protool_fee_pool.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
        &[seeds],
    )?;

    Ok(())
}
