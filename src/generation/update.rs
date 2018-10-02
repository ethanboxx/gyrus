use {
    super::{species::creature::MadeFrom, Creature, Gene, Generation},
    rand::Rng,
};

//TODO Make genetic diversity
impl Generation {
    pub fn update(&mut self) -> () {
        *self = self.kill();
        let mut number_of_genes_to_add = u64::from(self.intended_size) - self.unwrap().len() as u64;
        loop {
            // mut random gene
            let to_push = {
                let rng = &self.species[0].creatures
                    [rand::thread_rng().gen_range(0, self.species[0].creatures.len())]
                .gene
                .mutate(
                    {
                        let mut rng = rand::thread_rng();
                        let types = &[
                            super::species::creature::gene::mutate::Type::Strong,
                            super::species::creature::gene::mutate::Type::OnlyValues,
                        ];
                        rng.choose(types).unwrap()
                    },
                    rand::thread_rng().gen_range(i8::min_value(), i8::max_value()),
                );
                Creature {
                    gene: rng.clone(),
                    score: 0.0,
                    made_from: MadeFrom::Mutate,
                    key: rng.find_key(),
                }
            };
            self.species[0].creatures.push(to_push);
            number_of_genes_to_add -= 1;
            if number_of_genes_to_add == 0 {
                break;
            }
            // breed 2 gene
            let to_push = {
                let rng = Gene::breed(
                    &self.species[0].creatures
                        [rand::thread_rng().gen_range(0, self.species[0].creatures.len())]
                    .gene,
                    &self.species[0].creatures
                        [rand::thread_rng().gen_range(0, self.species[0].creatures.len())]
                    .gene,
                );
                Creature {
                    gene: rng.clone(),
                    score: 0.0,
                    made_from: MadeFrom::Breed,
                    key: rng.find_key(),
                }
            };
            self.species[0].creatures.push(to_push);
            number_of_genes_to_add -= 1;
            if number_of_genes_to_add == 0 {
                break;
            }
        }
        *self = self.resort_species();
    }
}
