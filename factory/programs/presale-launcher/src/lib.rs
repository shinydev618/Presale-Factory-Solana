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
}
