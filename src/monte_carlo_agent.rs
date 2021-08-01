use rand::seq::SliceRandom;
use std::time::Instant;

use crate::game::*;
use crate::tree::*;

use crate::p;

pub struct MonteCarloAgent {
}

impl MonteCarloAgent {

    pub fn choose_action(&self, root_state: &impl GameState) -> Option<usize> {
        let mut tree = NodeTree::new(root_state);
        let root_id = 0 as NodeId;
        let now = Instant::now();
        let mut i = 0;
        while now.elapsed().as_millis() < 500 {
            let node_id = self.selection(root_id, &tree);
            let state = tree.state(node_id);
            if !tree.is_leaf(node_id) && state.reward().is_none() {
                let (node_id, state) = self.expansion(node_id, &mut tree, state);
                let final_state = state.simulate();
//                 p!(final_state.pretty_print());
//                 p!(final_state.reward());
            }
//             tree.pprint();
//             let mut winner = &tree.state(node_id, game).winner();
// //             p!(winner);
// //             p!(tree.state(node_id, &game));
//             if !tree.is_leaf(node_id) && winner.is_none() {
//                 node_id = self.expansion(node_id, &mut tree, game);
// //                 winner = self.simulate(node_id, &tree, game);
// //                 println!("Inner winner: {:?}", winner);
//             }
// //             self.back_prop(node_id, &mut tree, winner);
// //             println!("Outer winner: {:?}", winner);
// //             tree.pprint();
// //             p!(self.best_action(0 as NodeId, &tree));
// //             p!("---");
            if i > 14 {
                break;
            }
            i += 1;
        }
        tree.pretty_print();
        self.best_action(0 as NodeId, &tree)
    }

    fn best_action<S: GameState>(&self, node_id: NodeId, tree: &NodeTree<S>) -> Option<usize> {
        let mut most_plays = f64::NEG_INFINITY;
        let mut best_wins = f64::NEG_INFINITY;
        let mut best_actions = Vec::new();
        for child_id in tree.children(node_id) {
            let child = tree.get(child_id);
            if child.plays > most_plays {
                most_plays = child.plays;
                best_actions = vec![child.action];
                best_wins = child.wins;
            } else if child.plays == most_plays {
                if child.wins > best_wins {
                    best_wins = child.wins;
                    best_actions = vec![child.action];
                } else if child.wins == best_wins {
                    best_actions.push(child.action);
                }
            }
        }
        best_actions.choose(&mut rand::thread_rng()).copied()
    }

//     fn back_prop(&self, mut node_id: NodeId, tree: &mut NodeTree,
//                  winner: Option<usize>) {
//         let node = tree.get_mut(node_id);
//         let mut winner_id = node.player_id;
//         match winner {
//             Some(player_id) => {
//                 if player_id == node.player_id {
//                     winner_id = player_id;
//                 }
//             },
//             None => {
//                 winner_id = 0; // Draw
//             },
//         }
//         loop {
//             let node = tree.get_mut(node_id);
//             if (winner_id == 0) {
//                 node.wins += 0.5;
//                 node.plays += 0.5;
//             } else {
//                 if node.player_id == winner_id {
//                     node.wins += 1.0;
//                 }
//                 node.plays += 1.0;
//             }
//             match node.parent {
//                 Some(id) => {
//                     node_id = id;
//                 },
//                 None => break,
//             }
//         }
//     }

    fn expansion<S: GameState>(&self, mut node_id: NodeId, tree: &mut NodeTree<S>,
                 mut state: impl GameState) -> (NodeId, impl GameState) {
        if tree.is_expanded(node_id) {
            return (node_id, state);
        }
        let actions = tree.unexpanded_actions(node_id, &state);
        let action = *actions.choose(&mut rand::thread_rng()).unwrap();
        let node = tree.get_mut(node_id);
//         if actions.len() == 1 {
//             node.status = NodeStatus::Expanded;
//         }
        state = state.next_state(action);
        let child_id = tree.add_node(action, node_id);
        (child_id, state)
    }

    pub fn selection<S: GameState>(&self, mut node_id: NodeId,
                                   tree: &NodeTree<S>
                                  ) -> NodeId {
        let mut node = tree.get(node_id);
        // TODO this is looping forever, why?
        p!(node_id);
        p!(tree.is_expanded(node_id));
        p!(tree.is_leaf(node_id));
        tree.state(node_id).pretty_print();
        tree.pretty_print();
        while tree.is_expanded(node_id) && !tree.is_leaf(node_id) {
            let c_val = 1.4; // Exploration value
            let mut max = (node_id, f64::NEG_INFINITY); // (NodeId, uct)
            for child_id in tree.children(node_id) {
                let child = tree.get(child_id);
                let uct = (child.wins / child.plays) + c_val *
                    f64::sqrt(2.0 * f64::ln(node.plays) / child.plays);
                if uct > max.1 {
                    max = (child_id, uct);
                }
            }
            node_id = max.0;
            node = tree.get(node_id);
        }
        node_id
    }
}

#[cfg(test)]
mod tests {
    use crate::tic_tac_toe::*;
    use crate::tree::*;

    use super::*;

//     #[test]
//     fn test() {
//         let agent = MonteCarloAgent {};
//         let mut state = TicTacToeState::new(); // state doesn't matter
//         let mut tree = NodeTree::new(&state);
//         tree.add_node(1, 0);
//         tree.add_node(2, 0);
//         tree.add_node(3, 0);
//         tree.add_node(4, 0);
//         tree.add_node(5, 0);
//         tree.add_node(6, 0);
//         tree.add_node(7, 0);
//         tree.add_node(8, 0);
//         tree.add_node(9, 0);
//         p!(tree.)
//         tree.pprint();
//         let (node_id, state) = agent.selection(0, &tree);
//         p!(node_id);
//     }

//     #[test]
//     fn test_root() {
//         let agent = MonteCarloAgent {};
//         let mut state = TicTacToeState::new(); // state doesn't matter
//         let mut tree = NodeTree::new(&state);
//         let (node_id, state) = agent.selection(0, &tree);
//         assert_eq!(0, node_id);
//     }

//     #[test]
//     fn test_selection() {
//         let agent = MonteCarloAgent {};
//         let mut state = TicTacToeState::new();
//         state.board = [0, 0, 0, 0, 0, 0, 0, 0, 0];
//         state.player = 1;
//         let mut tree = NodeTree::new();
//         let mut root = tree.get_mut(0);
//         root.wins = 11.0;
//         root.plays = 21.0;
//         root.status = NodeStatus::Expanded;
//         //            tree, pid, wins, plays
//         add_node(&mut tree, 0,   7.0,  10.0, NodeStatus::Expanded);
//         add_node(&mut tree, 0,   0.0,  3.0, NodeStatus::Expanded);
//         add_node(&mut tree, 0,   3.0,  8.0, NodeStatus::Expanded);
//         add_node(&mut tree, 1,   2.0,  4.0, NodeStatus::Expandable);
//         add_node(&mut tree, 1,   1.0,  6.0, NodeStatus::Expanded);
//         add_node(&mut tree, 3,   1.0,  2.0, NodeStatus::Expanded);
//         add_node(&mut tree, 3,   2.0,  3.0, NodeStatus::Expanded);
//         add_node(&mut tree, 3,   2.0,  3.0, NodeStatus::Expanded);
//         add_node(&mut tree, 5,   2.0,  3.0, NodeStatus::Expandable);
//         add_node(&mut tree, 5,   3.0,  3.0, NodeStatus::Expandable);
//         tree.pprint_mode = 1;
//         tree.pprint();
//         let (node_id, state) = agent.selection(0, &tree, state);
//         assert_eq!(10, node_id);
//     }

//     #[test]
//     fn test_selection_shallow() {
//         let agent = MonteCarloAgent {};
//         let mut state = TicTacToeState::new();
//         state.board = [0, 0, 0, 0, 0, 0, 0, 0, 0];
//         state.player = 1;
//         let mut tree = NodeTree::new();
//         let mut root = tree.get_mut(0);
//         root.wins = 5.0;
//         root.plays = 10.0;
//         //            tree, p_id, plays, wins
//         add_node(&mut tree, 0,    2.0,   5.0);
//         add_node(&mut tree, 0,    3.0,   3.0);
//         add_node(&mut tree, 0,    0.0,   1.0);
//         add_node(&mut tree, 1,    0.0,   1.0);
//         add_node(&mut tree, 1,    1.0,   3.0);
//         add_node(&mut tree, 2,    0.0,   1.0);
//         add_node(&mut tree, 2,    1.0,   1.0);
//         add_node(&mut tree, 5,    0.0,   1.0);
//         add_node(&mut tree, 5,    1.0,   1.0);
//         tree.pprint_mode = 1;
//         tree.pprint();
//         let best_uct = 7;
//         let (node_id, state) = agent.selection(0, &tree, state);
//         assert_eq!(best_uct, node_id);
//     }

//     #[test]
//     fn test_selection_shallow() {
//         let agent = MonteCarloAgent {};
//         let state = TicTacToeState {};
//         let game = Game::new([0, 0, 0, 0, 0, 0, 0, 0, 0], 1);
//         let mut tree = NodeTree::new(&game);
//         let mut root = tree.get_mut(0);
//         root.unexpanded_actions = Vec::new();
//         root.wins = 1.0;
//         root.plays = 9.0;
//         let mut best_uct = 999;
//         for i in 0..9 {
//             let mut state = [0, 0, 0, 0, 0, 0, 0, 0, 0];
//             state[i] = 1;
//             let mut node = Node::new(state, 1, Vec::new());
//             node.plays = 1.0;
//             if i == 3 {
//                 node.wins = 1.0;
//                 best_uct = tree.add_node(node, Some(0));
//             } else {
//                 node.wins = 0.0;
//                 tree.add_node(node, Some(0));
//             }
//         }
//         tree.pprint();
//         assert_eq!(best_uct, agent.selection(0, &tree, &game));
//     }

//     #[test]
//     fn test_selection_deep() {
//         let agent = MonteCarloAgent {};
//         let game = Game::new([0, 0, 0, 0, 0, 0, 0, 0, 0], 0);
//         let mut tree = NodeTree::new(&game);
//         let mut root = tree.get_mut(0);
//         root.unexpanded_actions = Vec::new();
//         root.wins = 5.0;
//         root.plays = 10.0;
//         //       tree,      p_id, plays, wins
//         add_node(&mut tree, 0,    2.0,   5.0);
//         add_node(&mut tree, 0,    3.0,   3.0);
//         add_node(&mut tree, 0,    0.0,   1.0);
//         add_node(&mut tree, 1,    0.0,   1.0);
//         add_node(&mut tree, 1,    1.0,   3.0);
//         add_node(&mut tree, 2,    0.0,   1.0);
//         add_node(&mut tree, 2,    1.0,   1.0);
//         add_node(&mut tree, 5,    0.0,   1.0);
//         add_node(&mut tree, 5,    1.0,   1.0);
//         tree.pprint_mode = 1;
//         tree.pprint();
//         let best_uct = 7;
//         let node_id = agent.selection(0, &tree, &game);
//         assert_eq!(best_uct, node_id);
//     }

//     fn add_node(tree: &mut NodeTree, parent_id: usize, wins: f64, plays: f64,
//                 status: NodeStatus) {
//         // Pretend values for all these
//         let node_id = tree.add_node(0, Some(parent_id));
//         let node = tree.get_mut(node_id);
//         node.plays = plays;
//         node.wins = wins;
//         node.status = status;
//     }
}
