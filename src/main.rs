#![allow(dead_code)]

use crate::game::Game;
use crate::monte_carlo_agent::MonteCarloAgent;

mod monte_carlo_agent;
mod arena;
mod random_agent;
mod game;

mod p;

fn main() {
    let agent = MonteCarloAgent {};
    let game = Game::new([0, 1, 0, 2, 0, 0, 2, 0, 0], 1);
    let best_deed = agent.monte_carlo_search(&game);
//     p!(best_deed);
//     p!(game.player_id);
//     p!(agent.pretty_print_board(game.game_state));
}
