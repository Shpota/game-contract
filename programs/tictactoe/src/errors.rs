use anchor_lang::prelude::*;


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
