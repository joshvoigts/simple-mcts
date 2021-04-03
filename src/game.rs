use crate::random_agent::*;

pub type GameDeed = (usize, usize); // index, value
pub type GameState = [usize; 9];
#[derive(Debug)]
pub enum GameResult {
    Win(usize),
    Lose,
    Draw,
}

pub struct Game {
    pub players: [RandomAgent; 3],
    pub player_id: usize,
    pub state: GameState,
}

impl Game {
    pub fn new(state: GameState, player_id: usize) -> Self {
        Self {
            players: [RandomAgent{}, RandomAgent{}, RandomAgent{}],
            player_id: player_id,
            state: state,
        }
    }

    pub fn next_player_id(&self, player_id: usize) -> usize {
        if player_id == self.players.len()-1 {
            return 1;
        } else {
            return player_id + 1;
        }
    }

    pub fn next_state(&self, state: GameState, deed: usize
                      ) -> GameState {
        let mut state = state.clone();
        state[deed] = self.player_id;
        state
    }

    pub fn get_winner(&self, state: &GameState) -> Option<usize> {
        for row_i in 0..3 {
            let mut row = [0, 0, 0];
            for col_i in 0..3 {
                let i = (3*row_i) + col_i;
                row[col_i] = state[i];
            }
            if row.iter().all(|&cell| cell == row[0] && cell > 0) {
                return Some(row[0]);
            }
        }
        for col_i in 0..3 {
            let mut col = [0, 0, 0];
            for row_i in 0..3 {
                let i = col_i + (3*row_i);
                col[row_i] = state[i];
            }
            if col.iter().all(|&cell| cell == col[0] && cell > 0) {
                return Some(col[0]);
            }
        }
        let rdiag = vec![state[0], state[4], state[8]];
        if rdiag.iter().all(|&cell| cell == rdiag[0] && cell > 0) {
            return Some(rdiag[0]);
        }
        let ldiag = vec![state[2], state[4], state[6]];
        if ldiag.iter().all(|&cell| cell == ldiag[0] && cell > 0) {
            return Some(ldiag[0]);
        }
        return None;
    }

    pub fn legal_deeds(&self, state: &GameState) -> Vec<usize> {
        let mut deeds = Vec::new();
        for i in 0..state.len() {
            if state[i] == 0 {
                deeds.push(i as usize);
            }
        }
        deeds
    }
}
