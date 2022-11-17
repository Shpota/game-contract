use anchor_lang::prelude::*;
use crate::account::Game;


#[derive(Accounts)]
pub struct StartGame<'info> {
    // Space: 8 (anchor internal state) + 9 (array of 9 i8 values) + 4 * 32 (4 public keys)
    #[account(init, payer = user, space = 145)]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn start_game(ctx: Context<StartGame>) -> Result<()> {
    let game = &mut ctx.accounts.game;
    game.state = [-1; 9];
    game.first_player = *ctx.accounts.user.key;
    Ok(())
}
