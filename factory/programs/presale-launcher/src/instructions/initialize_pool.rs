use crate::constants::*;
use crate::error::ErrorCode;
use crate::state::{Launcher, Launchpad};
use amm_anchor::Initialize2;
use anchor_lang::{prelude::*, system_program};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

#[derive(Accounts, Clone)]
pub struct PoolInitialize<'info> {
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
    /// CHECK: Safe
    pub amm_program: UncheckedAccount<'info>,
    /// CHECK: Safe. The new amm Account to be create, a PDA create with seed = [program_id, openbook_market_id, b"amm_associated_seed"]
    #[account(mut)]
    pub amm: UncheckedAccount<'info>,
    /// CHECK: Safe. Amm authority, a PDA create with seed = [b"amm authority"]
    #[account()]
    pub amm_authority: UncheckedAccount<'info>,
    /// CHECK: Safe. Amm open_orders Account, a PDA create with seed = [program_id, openbook_market_id, b"open_order_associated_seed"]
    #[account(mut)]
    pub amm_open_orders: UncheckedAccount<'info>,
    /// CHECK: Safe. Pool lp mint account. Must be empty, owned by $authority.
    #[account(mut)]
    pub amm_lp_mint: UncheckedAccount<'info>,
    /// CHECK: Safe. Coin mint account
    #[account(
        owner = token_program.key()
    )]
    pub amm_coin_mint: UncheckedAccount<'info>,
    /// CHECK: Safe. Pc mint account
    #[account(
        owner = token_program.key()
    )]
    pub amm_pc_mint: UncheckedAccount<'info>,
    /// CHECK: Safe. amm_coin_vault Account. Must be non zero, owned by $authority
    #[account(mut)]
    pub amm_coin_vault: UncheckedAccount<'info>,
    /// CHECK: Safe. amm_pc_vault Account. Must be non zero, owned by $authority.
    #[account(mut)]
    pub amm_pc_vault: UncheckedAccount<'info>,
    /// CHECK: Safe. amm_target_orders Account. Must be non zero, owned by $authority.
    #[account(mut)]
    pub amm_target_orders: UncheckedAccount<'info>,
    /// CHECK: Safe. Amm Config.
    #[account()]
    pub amm_config: UncheckedAccount<'info>,
    /// CHECK: Safe. Amm create_fee_destination.
    #[account(mut)]
    pub create_fee_destination: UncheckedAccount<'info>,
    /// CHECK: Safe. OpenBook program.
    #[account(
        address = amm_anchor::openbook_program_id::id(),
    )]
    pub market_program: UncheckedAccount<'info>,
    /// CHECK: Safe. OpenBook market. OpenBook program is the owner.
    #[account(
        owner = market_program.key(),
    )]
    pub market: UncheckedAccount<'info>,
    /// CHECK: Safe. The user wallet create the pool
    #[account(mut)]
    pub user_wallet: Signer<'info>,
    /// CHECK: Safe. The user coin token
    #[account(
        mut,
        owner = token_program.key(),
    )]
    pub user_token_coin: UncheckedAccount<'info>,
    /// CHECK: Safe. The user pc token
    #[account(
        mut,
        owner = token_program.key(),
    )]
    pub user_token_pc: UncheckedAccount<'info>,
    /// CHECK: Safe. The user lp token
    #[account(mut)]
    pub user_token_lp: UncheckedAccount<'info>,
    /// CHECK: This is safe as it's pda
    #[account(
        mut,
        seeds = [LAUNCHPAD_PRESALE_TREASURY_SEED, launchpad.key().as_ref()],
        bump
    )]
    pub presale_treasury: AccountInfo<'info>,
    #[account(
        mut ,
        seeds = [LAUNCHPAD_RESERVE_SEED, launchpad.key().as_ref()],
        bump,
        token::mint = amm_coin_mint,
        token::authority = reserve_token_account
    )]
    pub reserve_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [LAUNCHPAD_SEED, user_wallet.key().as_ref()],
        bump,
        constraint = launchpad.owner == user_wallet.key() @ ErrorCode::InvalidLaunchpadOwner
    )]
    pub launchpad: Account<'info, Launchpad>,
    /// CHECK: Safe. The spl token program
    pub token_program: Program<'info, Token>,
    /// CHECK: Safe. The associated token program
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// CHECK: Safe. System program
    pub system_program: Program<'info, System>,
    /// CHECK: Safe. Rent program
    pub sysvar_rent: Sysvar<'info, Rent>,
}

impl<'a, 'b, 'c, 'info> From<&mut PoolInitialize<'info>>
    for CpiContext<'a, 'b, 'c, 'info, Initialize2<'info>>
{
    fn from(
        accounts: &mut PoolInitialize<'info>,
    ) -> CpiContext<'a, 'b, 'c, 'info, Initialize2<'info>> {
        let cpi_accounts = Initialize2 {
            amm: accounts.amm.clone(),
            amm_authority: accounts.amm_authority.clone(),
            amm_open_orders: accounts.amm_open_orders.clone(),
            amm_lp_mint: accounts.amm_lp_mint.clone(),
            amm_coin_mint: accounts.amm_coin_mint.clone(),
            amm_pc_mint: accounts.amm_pc_mint.clone(),
            amm_coin_vault: accounts.amm_coin_vault.clone(),
            amm_pc_vault: accounts.amm_pc_vault.clone(),
            amm_target_orders: accounts.amm_target_orders.clone(),
            amm_config: accounts.amm_config.clone(),
            create_fee_destination: accounts.create_fee_destination.clone(),
            market_program: accounts.market_program.clone(),
            market: accounts.market.clone(),
            user_wallet: accounts.user_wallet.clone(),
            user_token_coin: accounts.user_token_coin.clone(),
            user_token_pc: accounts.user_token_pc.clone(),
            user_token_lp: accounts.user_token_lp.clone(),
            token_program: accounts.token_program.clone(),
            associated_token_program: accounts.associated_token_program.clone(),
            system_program: accounts.system_program.clone(),
            sysvar_rent: accounts.sysvar_rent.clone(),
        };
        let cpi_program = accounts.amm_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

/// Initiazlize a swap pool
pub fn initialize_pool(ctx: Context<PoolInitialize>, nonce: u8, open_time: u64) -> Result<()> {
    require!(
        ctx.accounts.presale_treasury.lamports() >= 100_000_000_000,
        ErrorCode::NotEnoughSOL
    );
    let launchpad = &mut ctx.accounts.launchpad;
    launchpad.check_launchpad_started()?;

    let launchpad_key = launchpad.key();

    let seeds = &[
        LAUNCHPAD_RESERVE_SEED,
        launchpad_key.as_ref(),
        &[ctx.bumps.reserve_token_account],
    ];

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.reserve_token_account.to_account_info(),
                to: ctx.accounts.user_token_coin.to_account_info(),
                authority: ctx.accounts.reserve_token_account.to_account_info(),
            },
        )
        .with_signer(&[&seeds[..]]),
        LAUNCHPAD_RESERVE,
    )?;

    let seeds = &[
        LAUNCHPAD_PRESALE_TREASURY_SEED,
        launchpad_key.as_ref(),
        &[ctx.bumps.presale_treasury],
    ];

    system_program::transfer(CpiContext::new(
            ctx.accounts.system_program.to_account_info(), 
        system_program::Transfer { 
                from: ctx.accounts.presale_treasury.to_account_info(), 
                to: ctx.accounts.user_token_pc.to_account_info() 
            },
        )
        .with_signer(&[&seeds[..]]), 
    100_000_000_000)?;

    token::sync_native(CpiContext::new(
            ctx.accounts.token_program.to_account_info(), 
            token::SyncNative {
                account: ctx.accounts.user_token_pc.to_account_info(),
            },
        ),
    )?;

    launchpad.complete_presale()?;

    amm_anchor::initialize(
        ctx.accounts.into(),
        nonce,
        open_time,
        100_000_000_000,
        LAUNCHPAD_RESERVE,
    )
}
