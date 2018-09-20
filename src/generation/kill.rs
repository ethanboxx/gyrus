use {
    super::{Gene, GeneScore, Generation},
    rand::Rng,
    rayon::prelude::*,
};

impl Generation {
    pub fn kill(&mut self) -> () {
        self.sort();
        let mut indexes = Vec::new();
        for (index, _gene) in self.genes.clone().iter().enumerate() {
            if !rand::thread_rng().gen_bool(index as f64 / self.genes.len() as f64) {
                indexes.push(index);
            }
        }
        for index in indexes.iter().rev() {
            self.genes.remove(*index);
        }
    }
}
