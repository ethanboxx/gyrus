use {
    chrono::{DateTime, Utc},
    crate::Gene,
};
struct Generation {
    genes: Vec<Gene>,
    date_created: Option<DateTime<Utc>>,
    intended_size: u16,
    generations_before: u64,
}

impl Generation {
    pub fn new_rand(size: u16) -> Self {
        Self {
            genes: {
                let mut rand_vec = Vec::new();
                for _x in 0..size {
                    rand_vec.push(Gene::new_random_gene());
                }
                rand_vec
            },
            date_created: Some(Utc::now()),
            intended_size: size,
            generations_before: 0,
        }
    }
}
