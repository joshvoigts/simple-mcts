#[derive(Debug, Clone, PartialEq)]
pub enum NodeStatus {
    Expandable,
    Expanded,
    Leaf,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub parent: Option<NodeId>,
    pub first_child: Option<NodeId>,
    pub next_sibling: Option<NodeId>,
//     pub status: NodeStatus,
    pub action: NodeAction,
    pub wins: f64,
    pub plays: f64,
}

impl Node {
    pub fn new(action: NodeAction, parent: Option<NodeId>) -> Self {
        Self {
            parent: parent,
            first_child: None,
            next_sibling: None,
//             status: NodeStatus::Expandable,
            action: action,
            wins: 0.0,
            plays: 0.0,
        }
    }
}

pub type NodeId = usize;
pub type NodeAction = usize;
