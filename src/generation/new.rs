use {
    super::{Gene, GeneScore, Generation},
    chrono::Utc,
};

impl Generation {
    pub fn new_rand(size: u16) -> Self {
        Self {
            genes: {
                let mut rand_vec = Vec::new();
                for _x in 0..size {
                    rand_vec.push(GeneScore {
                        gene: Gene::new_random_gene(),
                        score: 0.0,
                    });
                }
                rand_vec
            },
            date_created: Some(Utc::now()),
            intended_size: size,
            generations_before: 0,
        }
    }
}
