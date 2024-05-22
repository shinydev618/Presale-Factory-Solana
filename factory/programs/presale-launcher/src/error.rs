use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid Admin")]
    InvalidAdmin,
    #[msg("Invalid Token Owner")]
    InvalidOwner,
    #[msg("Invalid Launchpad Owner")]
    InvalidLaunchpadOwner,
    #[msg("Invalid Fee Pool")]
    InvalidFeePool,
    #[msg("Presale Not In Progress")]
    LaunchpadNotInProgress,
    #[msg("Not Enough Balance For Presale")]
    NotEnoughBalance,
    #[msg("Not Enough Sol For Pool Creation")]
    NotEnoughSOL,
    #[msg("Max 1 Sol Limit Exceeded")]
    MaxBuyExceeded,
}
