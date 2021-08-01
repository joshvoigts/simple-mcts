#![allow(dead_code)]
#![allow(warnings)]

// TODO remove the above

use std::io;
use std::fmt;

use crate::monte_carlo_agent::MonteCarloAgent;
use crate::tic_tac_toe::*;
use crate::game::*;
use crate::random_agent::*;

mod monte_carlo_agent;
mod tree;
mod random_agent;
mod game;
mod tic_tac_toe;

mod p;

fn main() {
    let agent = MonteCarloAgent {};
    let mut state = TicTacToeState::new();
    state.board = [1, 1, 0, 0, 0, 0, 2, 2, 0];
//     let state = TicTacToeState {
// //         board: [0, 0, 0, 0, 0, 0, 0, 0, 0],
//         board: [1, 2, 2, 1, 2, 2, 1, 1, 0],
//         player: 1,
//         players: vec![RandomAgent{}, RandomAgent{}, RandomAgent{}],
//     };
    let best_action = agent.choose_action(&state);
// //     human_play();
//     let agent = MonteCarloAgent {};
// //     let game = Game::new([1, 1, 0, 0, 0, 0, 2, 2, 0], 1);
//     let game = Game::new([0, 1, 0, 1, 2, 0, 0, 0, 0], 2);
//     let best_action = agent.monte_carlo_search(&game);
//     p!(best_action);
//     p!(game.player_id);
//     p!(game.pretty_print(&game.state));
}

// fn human_play() {
//     let agent = MonteCarloAgent {};
//     let mut game = Game::new([0, 0, 0, 0, 0, 0, 0, 0, 0], 1);
//     game.pretty_print(&game.state);
//     loop {
//         p!("Pick location:");
//         match read_index() {
//             Ok(index) => {
//                 game.state = game.next_state(&game.state, index);
//                 game.pretty_print(&game.state);
//                 let best_action = agent.monte_carlo_search(&game);
//                 game.player_id = game.next_player_id(game.player_id);
//                 match best_action {
//                     Some(action) => {
//                         game.state = game.next_state(&game.state, action);
//                         game.player_id = game.next_player_id(game.player_id);
//                         game.pretty_print(&game.state);
//                     },
//                     None => {
//                         p!("Winner!");
//                         break;
//                     }
//                 }
//             },
//             Err(error) => continue,
//         }
//     }
// }
// 
// fn read_index() -> Result<usize, std::num::ParseIntError> {
//     let mut result = String::new();
//     io::stdin().read_line(&mut result).expect(
//         "Failed to read from stdin");
//     result.pop(); // remove line ending
//     result.parse::<usize>()
// }
