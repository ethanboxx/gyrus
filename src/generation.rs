mod new;
mod validate;

use {
    super::gene::Gene,
    chrono::{DateTime, Utc},
};

struct GeneScore {
    gene: Gene,
    score: f64,
}
pub struct Generation {
    genes: Vec<GeneScore>,
    date_created: Option<DateTime<Utc>>,
    intended_size: u16,
    generations_before: u64,
}
