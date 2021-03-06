pub mod gene;
use super::super::Gene;

#[derive(Clone, Debug)]
pub enum MadeFrom {
    Mutate,
    Breed,
    Start,
}

//IDEA store key as a from of cache
//TODO update score func so can make private
#[derive(Clone, Debug)]
pub struct Creature {
    pub gene: Gene,
    pub score: f64,
    pub made_from: MadeFrom,
}
