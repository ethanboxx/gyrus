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
            println!("prob {:#?}", index as f64 / self.genes.len() as f64);
            if !rand::thread_rng().gen_bool(index as f64 / self.genes.len() as f64) {
                indexes.push(index);
            }
        }
        println!("indexes {:#?}", indexes);
        println!("indexes {:#?}", self.genes[0]);
    }
}
