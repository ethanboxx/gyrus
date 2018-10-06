use super::{Creature, Generation, Species};

impl Generation {
    pub fn resort_species(&self) -> Self {
        let all_creatures = self.unwrap();

        self.wrap(all_creatures)
    }
    pub fn unwrap(&self) -> Vec<Creature> {
        let mut all_creatures = Vec::new();
        for species in &self.species {
            all_creatures.append(&mut species.creatures.clone());
        }
        all_creatures
    }
    pub fn wrap(&self, all_creatures: Vec<Creature>) -> Self {
        let mut new_generation = Self {
            species: Vec::new(),
            date_created: None,
            intended_size: self.intended_size,
            generations_before: self.generations_before,
        };
        for creature in all_creatures {
            let index_of_key = new_generation.index_of_key(&creature);
            match index_of_key {
                Some(value) => {
                    new_generation.species[value].creatures.push(creature);
                }
                None => new_generation.species.push(Species {
                    creatures: vec![creature.clone()],
                    key: Some(creature.gene.clone().find_key()),
                }),
            }
        }
        new_generation
    }
    pub fn index_of_key(&self, creature: &Creature) -> Option<usize> {
        let y = self
            .species
            .iter()
            .enumerate()
            .find(|(_index, value)| value.key == Some(creature.gene.find_key()));
        match y {
            Some(x) => Some(x.0),
            None => None,
        }
    }
}
