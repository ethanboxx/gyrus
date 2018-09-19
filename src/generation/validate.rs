use {
    crate::{generation::Generation, Gene},
    rayon::prelude::*,
};

impl Generation {
    pub fn validate(&self) -> bool {
        self.intended_size as usize == self.genes.len()
            && self.genes.par_iter().all(|gene| gene.validate())
    }
}
