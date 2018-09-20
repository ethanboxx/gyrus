mod new;
mod validate;

use {
    super::gene::Gene,
    chrono::{DateTime, Utc},
};

#[derive(Debug)]
struct GeneScore {
    gene: Gene,
    score: f64,
}

#[derive(Debug)]
pub struct Generation {
    genes: Vec<GeneScore>,
    date_created: Option<DateTime<Utc>>,
    intended_size: u16,
    generations_before: u64,
}
