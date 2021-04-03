use crate::game::*;

use crate::p;

#[derive(Debug)]
pub struct Arena {
    nodes: Vec<Node>,
    pub pprint_mode: usize,
}

impl Arena {
    pub fn new(game: &Game) -> Self {
        let unexpanded_deeds = game.legal_deeds(&game.state);
        Self {
            nodes: vec![Node::new(game.state, game.player_id,
                                  unexpanded_deeds)],
            pprint_mode: 0,
        }
    }

    pub fn add_node(&mut self, mut node: Node, parent_id: Option<NodeId>
                    ) -> NodeId {
        let node_id = self.nodes.len() as NodeId;
        node.parent_id = parent_id;
        self.nodes.push(node);
        if let Some(id) = parent_id {
            self.get_mut(id).child_ids.push(node_id);
        }
        node_id
    }

    pub fn get(&self, node_id: NodeId) -> &Node {
        &self.nodes[node_id]
    }

    pub fn get_mut(&mut self, node_id: NodeId) -> &mut Node {
        &mut self.nodes[node_id]
    }

    pub fn pprint(&self) {
        fn pprint(arena: &Arena, node_id: NodeId, prefix: String,
                  last: bool) {
            let node = arena.get(node_id);
            let prefix_current = if last { "`- " } else { "|- " };

            if arena.pprint_mode == 1 {
                println!("{}{}wins:{} plays:{}", prefix,
                         prefix_current, node.wins, node.plays);
            } else {
                println!("{}{}p:{} id:{} {:?} wins:{} plays:{}",
                         prefix, prefix_current, node.player_id, node_id,
                         node.state, node.wins, node.plays);
            }

            let prefix_child = if last { "   " } else { "|  " };
            let prefix = prefix + prefix_child;

            let child_ids = &node.child_ids;
            if !child_ids.is_empty() {
                let last_child_id = child_ids.len() - 1;

                for (i, child_id) in child_ids.iter().enumerate() {
                    pprint(&arena, *child_id, prefix.to_string(),
                        i == last_child_id);
                }
            }
        }
        pprint(self, 0, "".to_string(), true)
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub player_id: usize,
    pub state: GameState,
    pub parent_id: Option<NodeId>,
    pub child_ids: Vec<NodeId>,
    pub unexpanded_deeds: Vec<usize>,
    pub deed: Option<usize>,
    pub wins: f64,
    pub plays: f64,
}

impl Node {
    pub fn new(state: GameState, player_id: usize,
               unexpanded_deeds: Vec<usize>) -> Self {
        Self {
            player_id: player_id,
            state: state,
            parent_id: None,
            child_ids: Vec::new(),
            unexpanded_deeds: unexpanded_deeds,
            deed: None,
            wins: 0.0,
            plays: 0.0,
        }
    }

    pub fn update(&mut self, winner: &Option<usize>) {
        match winner {
            Some(player_id) => {
                if *player_id == self.player_id {
                    self.wins += 1.0;
                }
            },
            None => (),
        }
        self.plays += 1.0;
    }
}

pub type NodeId = usize;
