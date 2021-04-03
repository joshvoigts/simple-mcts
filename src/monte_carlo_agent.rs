use rand::seq::SliceRandom;
use std::time::Instant;

use crate::game::*;
use crate::arena::*;

use crate::p;

pub struct MonteCarloAgent {
}

impl MonteCarloAgent {
    pub fn monte_carlo_search(&self, game: &Game) -> usize {
        let mut arena = Arena::new(game);
        let root_id = 0 as NodeId;
        let now = Instant::now();
        while now.elapsed().as_millis() < 10 {
            let mut node_id = self.selection(root_id, &arena, &game);
            let node = arena.get(node_id);
            let mut winner = game.get_winner(&node.state);
//             p!(winner);
            p!(self.is_leaf(&node.child_ids));
            if !self.is_leaf(&node.child_ids) && winner.is_none() {
                node_id = self.expansion(node_id, &mut arena, &game);
                winner = self.simulate(node_id, &arena);
            }
            self.back_prop(node_id, &mut arena, winner);
        }
        arena.pprint();
        0
//         self.best_deed(0 as NodeId, &arena)
    }

    pub fn pretty_print_board(&self, state: GameState) {
        let state: Vec<&str> = state.iter().map(|&i| {
            match i {
                1 => "x",
                2 => "o",
                _ => " ",
            }
        }).collect();
        p!(&state[0..3]);
        p!(&state[3..6]);
        p!(&state[6..9]);
    }

    fn best_deed(&self, node_id: NodeId, arena: &Arena) -> usize {
        let mut most_plays = f64::NEG_INFINITY;
        let mut best_wins = f64::NEG_INFINITY;
        let mut best_deeds = Vec::new();
        let node = arena.get(node_id);
        for child_id in node.child_ids.iter() {
            let child = arena.get(*child_id);
            if child.plays > most_plays {
                most_plays = child.plays;
                best_deeds = vec![child.deed];
                best_wins = child.wins;
            } else if child.plays == most_plays {
                if child.wins > best_wins {
                    best_wins = child.wins;
                    best_deeds = vec![child.deed];
                } else if child.wins == best_wins {
                    best_deeds.push(child.deed);
                }
            }
        }
//         p!(best_deeds);
        // ok to unwrap since `best_deeds` should not be `None`
        best_deeds.choose(&mut rand::thread_rng()).unwrap().unwrap()
    }

    fn back_prop(&self, mut node_id: NodeId, arena: &mut Arena,
                 winner: Option<usize>) {
        loop {
            let node = arena.get_mut(node_id);
            node.update(&winner);
            match node.parent_id {
                Some(parent_id) => {
                    node_id = parent_id;
                },
                None => break,
            }
        }
    }

    fn simulate(&self, node_id: NodeId, arena: &Arena) -> Option<usize> {
        let node = arena.get(node_id);
        let mut game = Game::new(node.state, node.player_id);
        while game.get_winner(&game.state).is_none() &&
                !game.legal_deeds(&game.state).is_empty() {
            game.player_id = game.next_player_id(game.player_id);
            let player = &game.players[game.player_id];
            match player.get_deed(&game) {
                Some(deed) => {
                    game.state = game.next_state(game.state, deed);
                },
                None => {
                    break;
                }
            }
        }
        game.get_winner(&game.state)
    }

    fn expansion(&self, node_id: usize, arena: &mut Arena,
                 game: &Game) -> NodeId {
        let node = arena.get_mut(node_id);
        let deeds = &mut node.unexpanded_deeds;
        let deed = deeds.pop();
        let new_state = game.next_state(node.state, deed.unwrap());
        let new_player_id = game.next_player_id(node.player_id);
        let new_unexpanded_deeds = game.legal_deeds(&new_state);
        let new_node = Node::new(new_state, new_player_id,
                                 new_unexpanded_deeds);
        arena.add_node(new_node, Some(node_id))
    }

    fn selection(&self, mut node_id: NodeId, arena: &Arena,
                 game: &Game) -> NodeId {
        let mut node = arena.get(node_id);
        let mut child_ids = &node.child_ids;
        while self.is_fully_expanded(&node, &child_ids) {
            let c_val = 1.0; // Exploration value
            let mut max = (child_ids[0], 0.0); // (NodeId, uct)
            for child_id in child_ids.iter() {
                let child = arena.get(*child_id);
                let uct = (child.wins / child.plays) + c_val *
                    f64::sqrt(2.0 * f64::ln(node.plays) / child.plays);
                if uct > max.1 {
                    max = (*child_id, uct);
                }
            }
            node_id = max.0;
            node = arena.get(node_id);
            child_ids = &node.child_ids;
        }
        node_id
    }

    // Select until EITHER not fully expanded OR leaf node
    fn is_fully_expanded(&self, node: &Node, child_ids: &Vec<NodeId>
                         ) -> bool {
        node.unexpanded_deeds.is_empty() && !self.is_leaf(child_ids)
    }

    fn is_leaf(&self, child_ids: &Vec<NodeId>) -> bool {
        node.unexpanded_deeds.is_empty() && child_ids.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::game::*;
    use crate::arena::*;

    use super::*;

    #[test]
    fn test_selection_shallow() {
        let agent = MonteCarloAgent {};
        let game = Game::new([0, 0, 0, 0, 0, 0, 0, 0, 0], 1);
        let mut arena = Arena::new(&game);
        let mut root = arena.get_mut(0);
        root.unexpanded_deeds = Vec::new();
        root.wins = 1.0;
        root.plays = 9.0;
        let mut best_uct = 999;
        for i in 0..9 {
            let mut state = [0, 0, 0, 0, 0, 0, 0, 0, 0];
            state[i] = 1;
            let mut node = Node::new(state, 1, Vec::new());
            node.plays = 1.0;
            if i == 3 {
                node.wins = 1.0;
                best_uct = arena.add_node(node, Some(0));
            } else {
                node.wins = 0.0;
                arena.add_node(node, Some(0));
            }
        }
        arena.pprint();
        assert_eq!(best_uct, agent.selection(0, &arena, &game));
    }

    #[test]
    fn test_selection_deep() {
        let agent = MonteCarloAgent {};
        let game = Game::new([0, 0, 0, 0, 0, 0, 0, 0, 0], 0);
        let mut arena = Arena::new(&game);
        let mut root = arena.get_mut(0);
        root.unexpanded_deeds = Vec::new();
        root.wins = 5.0;
        root.plays = 10.0;
        //       arena,      p_id, plays, wins
        add_node(&mut arena, 0,    2.0,   5.0);
        add_node(&mut arena, 0,    3.0,   3.0);
        add_node(&mut arena, 0,    0.0,   1.0);
        add_node(&mut arena, 1,    0.0,   1.0);
        add_node(&mut arena, 1,    1.0,   3.0);
        add_node(&mut arena, 2,    0.0,   1.0);
        add_node(&mut arena, 2,    1.0,   1.0);
        add_node(&mut arena, 5,    0.0,   1.0);
        add_node(&mut arena, 5,    1.0,   1.0);
        arena.pprint_mode = 1;
        arena.pprint();
        let best_uct = 7;
        let node_id = agent.selection(0, &arena, &game);
        assert_eq!(best_uct, node_id);
    }

    fn add_node(arena: &mut Arena, parent_id: usize, wins: f64,
                plays: f64) -> NodeId {
        // Pretend values for all these
        let mut node = Node::new([0, 0, 0, 0, 0, 0, 0, 0, 0], 0,
                                 Vec::new());
        node.plays = plays;
        node.wins = wins;
        arena.add_node(node, Some(parent_id))
    }
}
