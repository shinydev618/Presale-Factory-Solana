use anchor_lang::prelude::*;

#[constant]
pub const LAUNCHER_SEED: &str = "launcher";
pub const LAUNCHPAD_SEED: &str = "launchpad";
pub const LAUNCHPAD_PRESALE_SEED: &str = "presale";
pub const LAUNCHPAD_RESERVE_SEED: &str = "reserve";
pub const LAUNCHPAD_PRESALE_TREASURY_SEED: &str = "presale_treasury";

pub const LAUNCHPAD_PRESALE: u64 = 500_000_000;
pub const LAUNCHPAD_RESERVE: u64 = 500_000_000;

pub const PRESALE_PRICE: u64 = 220;
