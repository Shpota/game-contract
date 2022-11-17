use anchor_lang::prelude::*;

mod account;
mod instructions;
mod errors;

use instructions::join::*;
use instructions::set::*;
use instructions::start::*;
use instructions::*;

declare_id!("4Y5g1y7JXb2hpHDWsGQTAhHKbSJtR95W6QGzoXgESN8U");

#[program]
pub mod tic_tac_toe {
    use super::*;

    pub fn start_game(ctx: Context<StartGame>) -> Result<()> {
        start::start_game(ctx)
    }

    pub fn join_game(ctx: Context<JoinGame>) -> Result<()> {
        join::join_game(ctx)
    }

    pub fn set_value(ctx: Context<SetValue>, index: u8) -> Result<()> {
        set::set_value(ctx, index)
    }
}
