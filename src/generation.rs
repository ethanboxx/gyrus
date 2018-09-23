pub mod creature;
mod kill;
mod new;
mod sort;
mod update;
mod validate;

use {
    self::creature::gene::Gene,
    self::creature::Creature,
    chrono::{DateTime, Utc},
};

//TODO make more functions to stop so much pub
#[derive(Clone, Debug)]
pub struct Generation {
    pub genes: Vec<Creature>,
    pub date_created: Option<DateTime<Utc>>,
    pub intended_size: u16,
    pub generations_before: u64,
}
