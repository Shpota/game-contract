use anchor_lang::prelude::*;
use crate::account::Game;
use crate::errors::TicTacToeError;
    
// -1 represents an unset value, 0 represents "o", and 1 represents "x"
pub fn set_value(ctx: Context<SetValue>, index: u8) -> Result<()> {
    let game = &ctx.accounts.game;
    let user = ctx.accounts.user.key.to_bytes();
    if game.second_player.to_bytes() == Pubkey::default().to_bytes() {
        return err!(TicTacToeError::SecondPlayerNotJoined);
    }
    if game.first_player.to_bytes() != user && game.second_player.to_bytes() != user {
        return err!(TicTacToeError::WrongPlayer);
    }
    if game.winner.to_bytes() != Pubkey::default().to_bytes() {
        return err!(TicTacToeError::GameIsOver);
    }
    if ctx.accounts.game.is_first_move() {
        let game = &mut ctx.accounts.game;
        game.x_player = *ctx.accounts.user.key;
        game.state[usize::from(index)] = 1
    } else if ctx.accounts.game.can_move(ctx.accounts.user.key) {
        let game = &mut ctx.accounts.game;
        let mut value: i8 = 0;
        if game.x_player.to_bytes() == user {
            value = 1;
        }
        if game.state[usize::from(index)] == -1 {
            game.state[usize::from(index)] = value
        } else {
            return err!(TicTacToeError::CannotOverrideValue);
        }
    } else {
        return err!(TicTacToeError::WrongOrder);
    }
    let game = &mut ctx.accounts.game;
    let winning_symbol = game.get_winner();
    if winning_symbol != -1 {
        if winning_symbol == 1 {
            game.winner = game.x_player
        } else if game.x_player.to_bytes() == game.first_player.to_bytes() {
            game.winner = game.second_player
        } else {
            game.winner = game.first_player
        }
    }
    Ok(())
}

#[derive(Accounts)]
pub struct SetValue<'info> {
    #[account(mut)]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub user: Signer<'info>,
}