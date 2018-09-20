mod new;
mod validate;

use {
    super::gene::Gene,
    chrono::{DateTime, Utc},
};

//TODO update score func so can make private
#[derive(Clone, Debug)]
pub struct GeneScore {
    pub gene: Gene,
    pub score: f64,
}

//TODO make more functions to stop so much pub
#[derive(Clone, Debug)]
pub struct Generation {
    pub genes: Vec<GeneScore>,
    pub date_created: Option<DateTime<Utc>>,
    pub intended_size: u16,
    pub generations_before: u64,
}
