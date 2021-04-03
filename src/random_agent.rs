use rand::seq::SliceRandom;
use crate::game::*;

pub struct RandomAgent {
}

impl RandomAgent {
    pub fn get_deed(&self, game: &Game) -> Option<usize> {
        let deeds = game.legal_deeds(&game.state);
        let chosen = deeds.choose(&mut rand::thread_rng());
//         let chosen = deeds.choose(&mut StdRng::from_entropy());
        return chosen.copied();
    }
}
