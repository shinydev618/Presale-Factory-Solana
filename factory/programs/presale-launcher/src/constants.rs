use anchor_lang::prelude::*;

#[constant]
pub const LAUNCHER_SEED: &[u8] = b"launcher";
pub const LAUNCHPAD_SEED: &[u8] = b"launchpad";
pub const LAUNCHPAD_PRESALE_SEED: &[u8] = b"presale";
pub const LAUNCHPAD_RESERVE_SEED: &[u8] = b"reserve";
pub const LAUNCHPAD_PRESALE_TREASURY_SEED: &[u8] = b"presale_treasury";

pub const LAUNCHPAD_PRESALE: u64 = 500_000_000_000_000_000;
pub const LAUNCHPAD_RESERVE: u64 = 500_000_000_000_000_000;

pub const PRESALE_PRICE: u64 = 220; // lamports per 1_000_000_000 token
