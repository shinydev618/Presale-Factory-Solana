use crate::constants::*;
use crate::error::ErrorCode;
use crate::state::Launchpad;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct StartPresale<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [LAUNCHPAD_SEED.as_bytes(), signer.key().as_ref()],
        bump,
        constraint = launchpad.owner == signer.key() @ ErrorCode::InvalidLaunchpadOwner
    )]
    pub launchpad: Account<'info, Launchpad>,

    #[account(
        init,
        payer = signer,
        seeds = [LAUNCHPAD_PRESALE_SEED.as_bytes(), launchpad.key().as_ref()],
        bump,
        token::mint = mint,
        token::authority = presale_token_account
    )]
    pub presale_token_account: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = signer,
        seeds = [LAUNCHPAD_RESERVE_SEED.as_bytes(), launchpad.key().as_ref()],
        bump,
        token::mint = mint,
        token::authority = reserve_token_account
    )]
    pub reserve_token_account: Account<'info, TokenAccount>,

    #[account(
        mut, 
        token::mint = mint,
        token::authority = signer
    )]
    pub source_token_account: Account<'info, TokenAccount>,

    #[account(
        mint::authority = signer
    )]
    pub mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn start_presale(ctx: Context<StartPresale>) -> Result<()> {
    let launchpad = &mut ctx.accounts.launchpad;  
    launchpad.start_presale()?;

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(), 
            Transfer {
                authority: ctx.accounts.signer.to_account_info(),
                from: ctx.accounts.source_token_account.to_account_info(),
                to: ctx.accounts.presale_token_account.to_account_info(),
            },
        ),
        LAUNCHPAD_PRESALE,
    )?;

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(), 
            Transfer {
                authority: ctx.accounts.signer.to_account_info(),
                from: ctx.accounts.source_token_account.to_account_info(),
                to: ctx.accounts.reserve_token_account.to_account_info(),
            },
        ),
        LAUNCHPAD_RESERVE,
    )?;

    Ok(())
}