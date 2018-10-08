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
    pub fn new(
        number_of_inputs: usize,
        number_of_output: usize,
        height: usize,
        width: usize,
        shuffle: bool,
        size: u16,
    ) -> Self {
        Self {
            species: vec![Species {
                creatures: {
                    let mut rand_vec = Vec::new();
                    for _ in 0..size {
                        rand_vec.push({
                            Creature {
                                gene: {
                                    let mut rng = Gene::new(
                                        number_of_inputs,
                                        number_of_output,
                                        height,
                                        width,
                                    );
                                    if shuffle {
                                        rng.shuffle();
                                    }
                                    rng
                                },
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
