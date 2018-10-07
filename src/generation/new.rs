use {
    super::{
        species::{
            creature::{Creature, MadeFrom},
            Species,
        },
        Gene, Generation,
    },
    chrono::Utc,
};

//Maybe use repeat() to make an iter

//Macro looks like a good idea hear for speed then suffle for variation

impl Generation {
    pub fn new_rand(size: u16, depth: u8, hight: u8) -> Self {
        Self {
            species: vec![Species {
                creatures: {
                    let mut rand_vec = Vec::new();
                    for _ in 0..size {
                        rand_vec.push({
                            let rng = Gene::new_random_basic_gene(depth, hight);
                            Creature {
                                gene: rng.clone(),
                                score: 0.0,
                                made_from: MadeFrom::Start,
                            }
                        });
                    }
                    rand_vec
                },
                key: None,
            }],
            date_created: Some(Utc::now()),
            intended_size: size,
            generations_before: 0,
        }
        .resort_species()
    }
    pub fn new_rand_simple_custom(size: u16) -> Self {
        Self {
            species: vec![Species {
                creatures: {
                    let mut rand_vec = Vec::new();
                    for _ in 0..size {
                        rand_vec.push({
                            let rng = Gene::new_gene_shape_test_shuffle();
                            Creature {
                                gene: rng.clone(),
                                score: 0.0,
                                made_from: MadeFrom::Start,
                            }
                        });
                    }
                    rand_vec
                },
                key: None,
            }],
            date_created: Some(Utc::now()),
            intended_size: size,
            generations_before: 0,
        }
        .resort_species()
    }
}
