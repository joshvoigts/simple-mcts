use rand::seq::SliceRandom;
use std::time::Instant;

use crate::game::*;
use crate::tree::*;
use crate::tree_node::*;

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
            let mut node_id = root_id;
            let mut state = root_state.clone();
            state.pretty_print();
            node_id = self.selection(root_id, &tree, &state);
            state = state.next_state(tree.get(node_id).action);
            state.pretty_print();
            if !state.legal_actions().is_empty() && state.reward().is_none() {
                node_id = self.expansion(node_id, &mut tree, &state);
                state = state.next_state(tree.get(node_id).action);
                state = self.simulate(state);
            }
            state.pretty_print();
            p!(state.reward());
            p!("-------");
            self.back_prop(node_id, &mut tree, &state);
            if i > 14 {
                break;
            }
            i += 1;
        }
        tree.pretty_print();
        self.best_action(0 as NodeId, &tree)
    }

    fn best_action<S: GameState>(&self, node_id: NodeId,
                                 tree: &NodeTree<S>
                                 ) -> Option<usize> {
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

    fn back_prop<S: GameState>(&self, mut node_id: NodeId,
                               tree: &mut NodeTree<S>,
                               state: &S) {
        // Collect ancestor node_ids
        let mut node_ids = vec![];
        while let Some(par_id) = tree.get(node_id).parent {
            node_ids.push(node_id);
            node_id = par_id;
        }
        // Then replay them
        let mut state = tree.root_state.clone();
        for node_id in node_ids.iter().rev() {
            let node = tree.get_mut(*node_id);
            p!(state.reward());
            if let Some(score) = state.reward() {
                node.wins += score;
            }
            node.plays += 1.0;
            state = state.next_state(node.action);
        }
    }

    fn simulate<S: GameState>(&self, mut state: S) -> S {
        let mut actions = state.legal_actions();
        while !actions.is_empty() {
            let action = *actions.choose(&mut rand::thread_rng()).unwrap();
            state = state.next_state(action);
            actions = state.legal_actions();
        }
        state
    }

    fn expansion<S: GameState>(&self, mut node_id: NodeId,
                               tree: &mut NodeTree<S>,
                               state: &S
                               ) -> NodeId {
        if tree.is_expanded(node_id) {
            return node_id;
        }
        let actions = tree.unexpanded_actions(node_id, state);
        let action = *actions.choose(&mut rand::thread_rng()).unwrap();
        let node = tree.get_mut(node_id);
//         if actions.len() == 1 {
//             node.status = NodeStatus::Expanded;
//         }
        let child_id = tree.add_node(action, node_id);
        child_id
    }

    pub fn selection<S: GameState>(&self, mut node_id: NodeId,
                                   tree: &NodeTree<S>,
                                   state: &S) -> NodeId {
        let mut node = tree.get(node_id);
        while tree.is_expanded(node_id) && !state.legal_actions().is_empty() {
            let child_ids = tree.children(node_id).collect::<Vec<NodeId>>();
            let mut max = (child_ids[0], f64::NEG_INFINITY); // (NodeId, uct)
            for child_id in child_ids {
                let child = tree.get(child_id);
                // UCT is NaN when plays is zero but that's ok as long as
                // something is chosen.
                let uct = (child.wins / child.plays) +
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
