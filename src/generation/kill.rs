use {
    super::{Gene, GeneScore, Generation},
    rand::Rng,
    rayon::prelude::*,
};

impl Generation {
    pub fn kill(&mut self) -> () {
        self.sort();
        for (index, _gene) in self.genes.clone().iter().enumerate() {
            if rand::thread_rng().gen_bool(0.5) {
                self.genes.remove(index);
            }
        }
    }
}
