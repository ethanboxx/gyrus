use {
    super::{
        species::creature::Creature, species::creature::MadeFrom, species::Species, Gene,
        Generation,
    },
    chrono::Utc,
};

impl Generation {
    pub fn new_rand(size: u16, depth: u8, hight: u8) -> Self {
        Self {
            species: vec![Species {
                creatures: {
                    let mut rand_vec = Vec::new();
                    for _x in 0..size {
                        rand_vec.push({
                            let rng = Gene::new_random_basic_gene(depth, hight);
                            Creature {
                                gene: rng.clone(),
                                score: 0.0,
                                made_from: MadeFrom::Start,
                                key: rng.find_key(),
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
    }
    pub fn new_rand_simple_custom(size: u16) -> Self {
        Self {
            species: vec![Species {
                creatures: {
                    let mut rand_vec = Vec::new();
                    for _x in 0..size {
                        rand_vec.push({
                            let rng = Gene::new_gene_shape_test_shuffle();
                            Creature {
                                gene: rng.clone(),
                                score: 0.0,
                                made_from: MadeFrom::Start,
                                key: rng.find_key(),
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
    }
}
