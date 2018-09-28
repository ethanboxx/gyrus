use super::Generation;
use super::Species;

impl Generation {
    pub fn resort_species(&self) -> Self {
        let mut all_creatures = Vec::new();
        let mut new_generation = Generation {
            species: Vec::new(),
            date_created: None,
            intended_size: self.intended_size,
            generations_before: self.generations_before,
        };
        for species in self.species.iter() {
            all_creatures.append(&mut species.creatures.clone());
        }
        for creature in all_creatures {
            let index_of_key = new_generation
                .species
                .iter()
                .enumerate()
                .find(|(_index, value)| value.key == Some(creature.gene.find_key()))
                .clone();
            match index_of_key {
                Some(value) => new_generation.species[value.0]
                    .clone()
                    .creatures
                    .push(creature),
                None => new_generation.species.push(Species {
                    creatures: vec![creature.clone()],
                    key: Some(creature.gene.clone().find_key()),
                }),
            }
        }
        new_generation
    }
}
