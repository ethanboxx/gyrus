pub mod gene;
use super::Gene;

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
