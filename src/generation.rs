mod kill;
mod new;
mod prints;
pub mod resort_species;
mod score;
mod sort;
pub mod species;
mod update;
mod validate;

use {
    self::species::creature::gene::Gene,
    self::species::creature::Creature,
    self::species::Species,
    chrono::{DateTime, Utc},
};

//TODO make more functions to stop so much pub
#[derive(Clone, Debug)]
pub struct Generation {
    pub species: Vec<Species>,
    pub date_created: Option<DateTime<Utc>>,
    pub intended_size: u16,
    pub generations_before: u64,
}
