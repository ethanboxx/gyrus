use {super::Generation, rayon::prelude::*};

//TODO make sure ordering is correct for score
//TODO return error type
impl Generation {
    pub fn validate(&self) -> bool {
        (self.intended_size as usize == {
            let mut number = 0;
            for x in &self.species {
                number += x.creatures.len();
            }
            number
        }) && self.species.par_iter().all(|specie| {
            specie
                .creatures
                .par_iter()
                .all(|creature| creature.gene.validate())
        })
    }
}
