use anchor_lang::prelude::*;

declare_id!("4Y5g1y7JXb2hpHDWsGQTAhHKbSJtR95W6QGzoXgESN8U");

#[program]
pub mod tic_tac_toe {
    use super::*;

    pub fn start_game(ctx: Context<StartGame>) -> Result<()> {
        let game = &mut ctx.accounts.game;
        game.state = [-1; 9];
        game.first_player = *ctx.accounts.user.key;
        Ok(())
    }

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
}

#[derive(Accounts)]
pub struct StartGame<'info> {
    // Space: 8 (anchor internal state) + 9 (array of 9 i8 values) + 4 * 32 (4 public keys)
    #[account(init, payer = user, space = 145)]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct JoinGame<'info> {
    #[account(mut)]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetValue<'info> {
    #[account(mut)]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[account]
pub struct Game {
    pub state: [i8; 9],
    pub first_player: Pubkey,
    pub second_player: Pubkey,
    pub x_player: Pubkey,
    pub winner: Pubkey,
}

impl Game {
    fn is_first_move(self: &Game) -> bool {
        for i in 0..self.state.len() {
            if self.state[usize::from(i)] != -1 {
                return false;
            }
        }
        true
    }

    fn can_move(self: &Game, player: &Pubkey) -> bool {
        let count = self.state.into_iter().filter(|&x| x != -1).count();
        if count % 2 == 0 {
            self.x_player.to_bytes() == player.to_bytes()
        } else {
            self.x_player.to_bytes() != player.to_bytes()
        }
    }

    fn get_winner(self: &Game) -> i8 {
        let state = self.state;
        if state[0] != -1 && state[0] == state[1] && state[2] == state[1] {
            return state[0];
        }
        if state[3] != -1 && state[3] == state[4] && state[5] == state[3] {
            return state[3];
        }
        if state[6] != -1 && state[6] == state[7] && state[8] == state[6] {
            return state[6];
        }
        if state[0] != -1 && state[0] == state[3] && state[6] == state[0] {
            return state[0];
        }
        if state[1] != -1 && state[1] == state[4] && state[7] == state[1] {
            return state[1];
        }
        if state[2] != -1 && state[2] == state[5] && state[8] == state[2] {
            return state[2];
        }
        if state[0] != -1 && state[0] == state[4] && state[8] == state[0] {
            return state[0];
        }
        if state[2] != -1 && state[2] == state[4] && state[6] == state[2] {
            return state[2];
        }
        -1
    }
}

#[error_code]
pub enum TicTacToeError {
    #[msg("Second Player must be different from the first player")]
    SecondPlayerSameAsFirst,
    #[msg("Cannot join when there is already a second player")]
    SecondPlayerAlreadyJoined,
    #[msg("Only users taking part in the game can make a move")]
    WrongPlayer,
    #[msg("Second player has not joined the game")]
    SecondPlayerNotJoined,
    #[msg("Cannot Override a previously set value")]
    CannotOverrideValue,
    #[msg("Another player is supposed to set value")]
    WrongOrder,
    #[msg("Game is over")]
    GameIsOver,
}
