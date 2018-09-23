use {
    super::{Creature, Generation},
    rayon::prelude::*,
};

impl Generation {
    pub fn score_update<F>(self, f: F) -> Self
    where
        F: Fn(&Creature) -> f64,
        F: std::marker::Sync,
    {
        let mut generation = self;

        generation = Self {
            genes: generation
                .genes
                .par_iter()
                .map(|current| {
                    Creature {
                        score: f(current),
                        ..current.clone()
                    }.clone()
                }).collect(),
            generations_before: generation.generations_before + 1,
            ..generation
        };

        generation
    }
}
