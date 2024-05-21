use crate::error::ErrorCode;
use anchor_lang::prelude::*;

#[account]
pub struct Launcher {
    pub fee_pool: Pubkey, // 32
    pub admin: Pubkey,    // 32
}

impl Launcher {
    pub const LEN: usize = 32 + 32;

    pub fn initialize(&mut self, fee_pool: Pubkey, admin: Pubkey) -> Result<()> {
        self.fee_pool = fee_pool;
        self.admin = admin;
        Ok(())
    }

    pub fn check_admin(&self, admin: Pubkey) -> Result<()> {
        require_keys_eq!(self.admin, admin, ErrorCode::InvalidAdmin);
        Ok(())
    }

    pub fn set_admin(&mut self, admin: Pubkey) -> Result<()> {
        self.admin = admin;
        Ok(())
    }

    pub fn set_fee_pool(&mut self, fee_pool: Pubkey) -> Result<()> {
        self.fee_pool = fee_pool;
        Ok(())
    }
}
