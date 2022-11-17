use anchor_lang::prelude::*;

#[account]
pub struct Game {
    pub state: [i8; 9],
    pub first_player: Pubkey,
    pub second_player: Pubkey,
    pub x_player: Pubkey,
    pub winner: Pubkey,
}

impl Game {
    pub fn is_first_move(self: &Game) -> bool {
        for i in 0..self.state.len() {
            if self.state[usize::from(i)] != -1 {
                return false;
            }
        }
        true
    }

    pub fn can_move(self: &Game, player: &Pubkey) -> bool {
        let count = self.state.into_iter().filter(|&x| x != -1).count();
        if count % 2 == 0 {
            self.x_player.to_bytes() == player.to_bytes()
        } else {
            self.x_player.to_bytes() != player.to_bytes()
        }
    }

    pub fn get_winner(self: &Game) -> i8 {
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
