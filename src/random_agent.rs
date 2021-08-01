use rand::seq::SliceRandom;
use crate::game::*;

#[derive(Debug, Clone)]
pub struct RandomAgent {
}

impl RandomAgent {

    pub fn choose_action(&self, state: &impl GameState) -> Option<usize> {
        let actions = state.legal_actions();
        let chosen = actions.choose(&mut rand::thread_rng());
//         let chosen = actions.choose(&mut StdRng::from_entropy());
        return chosen.copied();
    }

}
