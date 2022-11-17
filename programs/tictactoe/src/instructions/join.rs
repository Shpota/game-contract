use anchor_lang::prelude::*;
use crate::account::Game;
use crate::errors::TicTacToeError;


pub fn join_game(ctx: Context<JoinGame>) -> Result<()> {
    let game = &mut ctx.accounts.game;
    if ctx.accounts.user.key.to_bytes() == game.first_player.to_bytes() {
        return err!(TicTacToeError::SecondPlayerSameAsFirst);
    }
    if game.second_player.to_bytes() != Pubkey::default().to_bytes() {
        return err!(TicTacToeError::SecondPlayerAlreadyJoined);
    }
    game.second_player = *ctx.accounts.user.key;
    Ok(())
}

#[derive(Accounts)]
pub struct JoinGame<'info> {
    #[account(mut)]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub user: Signer<'info>,
}
