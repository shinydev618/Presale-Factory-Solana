use crate::error::ErrorCode;
use anchor_lang::prelude::*;

#[account]
pub struct Launchpad {
    pub owner: Pubkey,               // 32
    pub mint: Pubkey,                // 32
    pub presale_state: PresaleState, // 1
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum PresaleState {
    Open,
    InProgress,
    Completed,
}

impl Launchpad {
    pub const LEN: usize = 32 + 32 + 1;

    pub fn check_launchpad_owner(&self, owner: Pubkey) -> Result<()> {
        require_keys_neq!(self.owner, owner, ErrorCode::InvalidLaunchpadOwner);
        Ok(())
    }

    pub fn check_launchpad_started(&self) -> Result<()> {
        require!(
            self.presale_state == PresaleState::InProgress,
            ErrorCode::LaunchpadNotInProgress
        );
        Ok(())
    }

    pub fn initialize_launchpad(&mut self, owner: Pubkey, mint: Pubkey) -> Result<()> {
        self.owner = owner;
        self.mint = mint;
        self.presale_state = PresaleState::Open;
        Ok(())
    }

    pub fn start_presale(&mut self) -> Result<()> {
        self.presale_state = PresaleState::InProgress;
        Ok(())
    }

    pub fn complete_presale(&mut self) -> Result<()> {
        self.presale_state = PresaleState::Completed;
        Ok(())
    }
}
