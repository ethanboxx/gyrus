use {crate::Gene, chrono::prelude::Utc};
struct Generation {
    genes: Vec<Gene>,

    date_created: Option<Utc>;
}
