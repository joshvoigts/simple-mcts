use crate::game::*;
use crate::random_agent::*;

use crate::p;

#[derive(Debug, Clone)]
pub struct TicTacToeState {
    pub board: [usize; 9],
    pub player: usize,
    pub players: Vec<RandomAgent>,
}

impl GameState for TicTacToeState {

    fn new() -> Self {
        Self {
            board: [0, 0, 0, 0, 0, 0, 0, 0, 0],
            player: 1,
            players: vec![RandomAgent{}, RandomAgent{}, RandomAgent{}],
        }
    }

    fn reward(&self) -> Option<f64> {
        fn is_winner(cells: &[usize]) -> bool {
            cells.iter().all(|&cell| cell == cells[0] && cell > 0)
        }
        let board = self.board;
        let win_positions = [
            [board[0], board[1], board[2]], // rows
            [board[3], board[4], board[5]],
            [board[6], board[7], board[8]],
            [board[0], board[3], board[6]], // cols
            [board[1], board[4], board[7]],
            [board[2], board[5], board[8]],
            [board[0], board[4], board[8]], // diags
            [board[2], board[4], board[6]],
        ];
        for positions in win_positions.iter() {
            if is_winner(positions) {
                if positions[0] == self.player {
                    return Some(1.0);
                } else {
                    return Some(0.0);
                }
            }
        }
        if board.iter().all(|&cell| cell > 0) {
            return Some(0.5);
        } else {
            return None;
        }
    }

    fn legal_actions(&self) -> Vec<usize> {
        let mut actions = Vec::new();
        for i in 0..self.board.len() {
            if self.board[i] == 0 {
                actions.push(i as usize);
            }
        }
        actions
    }

    fn pretty_print(&self) {
        let board: Vec<String> = self.board.iter().map(|&i| {
            match i {
                1 => 1.to_string(),
                2 => 2.to_string(),
                _ => " ".to_string(),
            }
        }).collect();
        println!("{:?} Player: {}", &board[0..3], self.player);
        p!(&board[3..6]);
        p!(&board[6..9]);
    }

    fn next_state(&self, action: usize) -> TicTacToeState {
        let mut new_state = self.clone();
        new_state.board[action] = self.player;
        if self.player == self.players.len()-1 {
            new_state.player = 1;
        } else {
            new_state.player += 1;
        }
        new_state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reward() {
        let mut state = TicTacToeState::new();
        state.player = 1;
        state.board = [1, 1, 1,
                       0, 0, 0,
                       2, 2, 0];
        assert_eq!(state.reward(), Some(1.0));
        state.board = [1, 2, 0,
                       0, 1, 0,
                       2, 0, 1];
        assert_eq!(state.reward(), Some(1.0));
        state.board = [2, 1, 0,
                       0, 1, 2,
                       0, 1, 0];
        assert_eq!(state.reward(), Some(1.0));
        state.board = [0, 2, 1,
                       0, 1, 2,
                       1, 0, 0];
        assert_eq!(state.reward(), Some(1.0));
        state.board = [2, 1, 2,
                       1, 1, 2,
                       2, 2, 1];
        assert_eq!(state.reward(), Some(0.5));
        state.board = [2, 2, 2,
                       1, 1, 2,
                       2, 1, 1];
        assert_eq!(state.reward(), Some(0.0));
        state.board = [0, 0, 0,
                       0, 0, 0,
                       0, 0, 0];
        assert_eq!(state.reward(), None);
        state.board = [0, 2, 2,
                       1, 1, 2,
                       2, 1, 1];
        assert_eq!(state.reward(), None);
        state.player = 2;
        state.board = [2, 2, 2,
                       1, 1, 2,
                       2, 1, 1];
        assert_eq!(state.reward(), Some(1.0));
    }

    #[test]
    fn test_simulate() {
        let mut state = TicTacToeState::new();
        state.player = 1;
        state.board = [0, 2, 2,
                       1, 1, 2,
                       2, 1, 1];
        let mut comp_state = state.clone();
        comp_state.board[0] = 1;
        assert_eq!(state.simulate().board, comp_state.board);
    }
}
