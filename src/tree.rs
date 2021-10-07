use crate::game::*;
use crate::tree_node::*;

use crate::p;

#[derive(Debug)]
pub struct NodeTree<S: GameState> {
    nodes: Vec<Node>,
    pub pprint_mode: usize,
    pub root_state: S,
}

impl<S: GameState> NodeTree<S> {

    pub fn new(root_state: &S) -> Self {
        Self {
            nodes: vec![Node::new(0, None)],
            pprint_mode: 0,
            root_state: root_state.clone(),
        }
    }

    pub fn add_node(&mut self, action: NodeAction, parent_id: NodeId) -> NodeId {
        let node_id = self.nodes.len() as NodeId;
        let parent = self.get_mut(parent_id);
        if parent.first_child.is_none() {
            parent.first_child = Some(node_id);
        } else {
            // Only call this if parent has children
            let last_child_id = self.get_last_child_id(parent_id);
            let last_child = self.get_mut(last_child_id);
            last_child.next_sibling = Some(node_id);
        }
        self.nodes.push(Node::new(action, Some(parent_id)));
        node_id
    }

    pub fn get_last_child_id(&self, node_id: NodeId) -> NodeId {
        let parent = self.get(node_id);
        let mut child_id = parent.first_child.unwrap();
        loop {
            let child = self.get(child_id);
            match child.next_sibling {
                Some(id) => child_id = id,
                None => break,
            }
        }
        child_id
    }

    pub fn get(&self, node_id: NodeId) -> &Node {
        &self.nodes[node_id]
    }

    pub fn get_mut(&mut self, node_id: NodeId) -> &mut Node {
        &mut self.nodes[node_id]
    }

    pub fn pretty_print(&self) {
        self.pprint_inner(0, "".to_string(), true)
    }
    fn pprint_inner(&self, node_id: NodeId, prefix: String, last: bool) {
        let node = self.get(node_id);
        let prefix_current = if last { "`- " } else { "|- " };

        if self.pprint_mode == 1 {
            println!("{}{}wins:{} plays:{}", prefix,
                     prefix_current, node.wins, node.plays);
        } else if self.pprint_mode == 2 {
            println!("{}{}id:{} next_sibling:{:?}", prefix,
                     prefix_current, node_id, node.next_sibling);
        } else if self.pprint_mode == 3 {
            println!("{}{}id:{} parent:{:?}", prefix,
                     prefix_current, node_id, node.parent);
        } else {
            println!("{}{}id:{} action:{} wins:{} plays:{}",
                     prefix, prefix_current,
                     node_id, node.action, node.wins, node.plays);
        }

        let prefix_child = if last { "   " } else { "|  " };
        let prefix = prefix + prefix_child;

        let mut opt_child_id = node.first_child;
        while let Some(child_id) = opt_child_id {
            self.pprint_inner(child_id, prefix.to_string(),
                opt_child_id.is_none());
            opt_child_id = self.get(child_id).next_sibling;
        }
    }

    pub fn children(&self, node_id: NodeId) -> NodeChildIterator<S> {
        NodeChildIterator::new(&self, node_id)
    }

    pub fn state(&self, node_id: NodeId) -> S {
        let mut state = self.root_state.clone();
        let mut actions = Vec::new();
        // Collect actions
        let mut node = self.get(node_id);
        while let Some(par_id) = node.parent {
            actions.push(node.action);
            node = self.get(par_id);
        }
        // Then replay them
        for action in actions.iter().rev() {
            state = state.next_state(*action);
        }
        state
    }

    pub fn is_leaf(&self, node_id: NodeId) -> bool {
//         self.children(node_id).next().is_none()
        let state = self.state(node_id);
        let actions = state.legal_actions();
        actions.is_empty()
// //         node.status == NodeStatus::Leaf
    }

    pub fn is_expanded(&self, node_id: NodeId) -> bool {
        let state = self.state(node_id);
        self.unexpanded_actions(node_id, &state).is_empty()
//         node.status == NodeStatus::Expanded
    }

    pub fn unexpanded_actions(&self, node_id: NodeId,
                            state: &impl GameState) -> Vec<usize> {
        let mut actions = state.legal_actions();
        for child_id in self.children(node_id) {
            let child = self.get(child_id);
            if let Some(i) = actions.iter().position(|&d| d == child.action) {
                actions.remove(i);
            }
        }
        actions
    }
}

pub struct NodeChildIterator<'a, S: GameState> {
    tree: &'a NodeTree<S>,
    curr: NodeId,
    first: bool,
}

impl<'a, S: GameState> NodeChildIterator<'a, S> {
    fn new(tree: &'a NodeTree<S>, node_id: NodeId) -> Self {
        Self {
            tree: tree,
            curr: node_id,
            first: true,
        }
    }
}

impl<'a, S: GameState> Iterator for NodeChildIterator<'a, S> {
    type Item = NodeId;
    fn next(&mut self) -> Option<Self::Item> {
        let node = self.tree.get(self.curr);
        let mut next_id = node.next_sibling;
        if self.first {
            self.first = false;
            next_id = node.first_child;
        }
        match next_id {
            Some(id) => {
                self.curr = id;
                Some(id)
            },
            None => None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tic_tac_toe::*;
    use crate::monte_carlo_agent::*;

    use super::*;

    #[test]
    fn test_state() {
        let mut tree = setup_tree();
        tree.add_node(1, 0); // 1 (node id)
        tree.add_node(2, 0); // 2
        tree.add_node(3, 0); // 3
        tree.add_node(4, 0); // 4
        tree.add_node(0, 3); // 5
        tree.add_node(1, 5); // 6
        tree.add_node(2, 5); // 7
        tree.add_node(2, 6); // 8
        let state = tree.state(8);
        assert_eq!(state.board, [2, 1, 2, 1, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_unexpanded_actions() {
        let mut tree = setup_tree();
        let unexpanded = tree.unexpanded_actions(0, &tree.state(0));
        assert_eq!(unexpanded, vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
        tree.add_node(0, 0); // 1 (node id)
        tree.add_node(1, 0); // 2
        tree.add_node(2, 0); // 3
        tree.add_node(3, 0); // 4
        tree.add_node(4, 0); // 5
        tree.add_node(5, 0); // 6
        tree.add_node(6, 0); // 7
        let unexpanded = tree.unexpanded_actions(0, &tree.state(0));
        assert_eq!(unexpanded, vec![7, 8]);
        tree.add_node(7, 0); // 8
        tree.add_node(8, 0); // 9
        let unexpanded = tree.unexpanded_actions(0, &tree.state(0));
        assert_eq!(unexpanded, vec![]);
        tree.add_node(0, 3); // 10
        tree.add_node(1, 3); // 11
        tree.add_node(3, 3); // 12
        tree.add_node(4, 3); // 13
        let unexpanded = tree.unexpanded_actions(3, &tree.state(3));
        assert_eq!(unexpanded, vec![5, 6, 7, 8]);
    }

    #[test]
    fn test_is_leaf() {
        let tree = setup_status_tree();
        assert_eq!(tree.is_leaf(0), false);
        assert_eq!(tree.is_leaf(9), false);
        assert_eq!(tree.is_leaf(13), false);
        assert_eq!(tree.is_leaf(17), true);
    }

    #[test]
    fn test_is_expanded() {
        let tree = setup_status_tree();
        assert_eq!(tree.is_expanded(0), true);
        assert_eq!(tree.is_expanded(9), false);
        assert_eq!(tree.is_expanded(13), false);
        assert_eq!(tree.is_expanded(17), true);
    }

    #[test]
    fn test_is_expandable() {
        let tree = setup_status_tree();
        assert_eq!(tree.is_expandable(0), false);
        assert_eq!(tree.is_expandable(9), true);
        assert_eq!(tree.is_expandable(13), true);
        assert_eq!(tree.is_expandable(17), false);
    }

    fn setup_tree() -> NodeTree<TicTacToeState> {
        let agent = MonteCarloAgent {};
        let mut state = TicTacToeState::new(); // state doesn't matter
        let mut tree = NodeTree::new(&state);
        tree
    }

    fn setup_status_tree() -> NodeTree<TicTacToeState> {
        let mut tree = setup_tree();
        tree.add_node(0, 0); // 1 (node id)
        tree.add_node(1, 0); // 2
        tree.add_node(2, 0); // 3
        tree.add_node(3, 0); // 4
        tree.add_node(4, 0); // 5
        tree.add_node(5, 0); // 6
        tree.add_node(6, 0); // 7
        tree.add_node(7, 0); // 8
        tree.add_node(8, 0); // 9
        tree.add_node(1, 1); // 10
        tree.add_node(2, 10); // 11
        tree.add_node(3, 11); // 12
        tree.add_node(4, 12); // 13
        tree.add_node(5, 13); // 14
        tree.add_node(6, 14); // 15
        tree.add_node(7, 15); // 16
        tree.add_node(8, 16); // 17
        tree
    }

}
