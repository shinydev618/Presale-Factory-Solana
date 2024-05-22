pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

use instructions::*;

declare_id!("DmTYykBGheDKxJN29jSPPieDrQaw2ZKxjgfGpWiJ6Q5Z");

#[program]
pub mod presale_launcher {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, args: InitArgs) -> Result<()> {
        instructions::initialize(ctx, args)
    }

    pub fn set_fee_pool(ctx: Context<SetFeePool>, args: SetFeePoolArgs) -> Result<()> {
        instructions::set_fee_pool(ctx, args)
    }

    pub fn set_admin(ctx: Context<SetAdmin>, args: SetAdminArgs) -> Result<()> {
        instructions::set_admin(ctx, args)
    }

    pub fn initialize_launchpad(
        ctx: Context<InitializeLaunchpad>,
        args: InitLaunchpadArgs,
    ) -> Result<()> {
        instructions::initialize_launchpad(ctx, args)
    }

    pub fn start_presale(ctx: Context<StartPresale>) -> Result<()> {
        instructions::start_presale(ctx)
    }

    pub fn purchase(ctx: Context<Purchase>, args: PurchaseArgs) -> Result<()> {
        instructions::purchase(ctx, args)
    }

    pub fn sell(ctx: Context<Sell>, args: SellArgs) -> Result<()> {
        instructions::sell(ctx, args)
    }

    pub fn initialize_pool(ctx: Context<PoolInitialize>, nonce: u8, open_time: u64) -> Result<()> {
        instructions::initialize_pool(ctx, nonce, open_time)
    }

    pub fn withdraw(ctx: Context<PoolWithdraw>, amount: u64) -> Result<()> {
        instructions::withdraw(ctx, amount)
    }
}
