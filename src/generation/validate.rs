use {super::Generation, rayon::prelude::*};

//TODO make sure ordering is correct for score
//TODO return error type
impl Generation {
    pub fn validate(&self) -> bool {
        (self.intended_size as usize == self.genes.len()) && self
            .genes
            .par_iter()
            .all(|gene_store| gene_store.gene.validate())
    }
}
