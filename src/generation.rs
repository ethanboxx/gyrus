mod kill;
mod new;
mod sort;
mod update;
mod validate;

use {
    super::gene::Gene,
    chrono::{DateTime, Utc},
};

#[derive(Clone, Debug)]
pub enum MadeFrom {
    Mutate,
    Breed,
    Start,
}

//TODO update score func so can make private
#[derive(Clone, Debug)]
pub struct Creature {
    pub gene: Gene,
    pub score: f64,
    pub made_from: MadeFrom,
    pub key: Vec<i8>,
}

//TODO make more functions to stop so much pub
#[derive(Clone, Debug)]
pub struct Generation {
    pub genes: Vec<Creature>,
    pub date_created: Option<DateTime<Utc>>,
    pub intended_size: u16,
    pub generations_before: u64,
}
