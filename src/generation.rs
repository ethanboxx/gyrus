use {chrono::prelude::Utc, crate::Gene};
struct Generation {
    genes: Vec<Gene>,

    date_created: Option<Utc>,
}
