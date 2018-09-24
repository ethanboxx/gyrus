pub mod creature;
use self::creature::Creature;

#[derive(Clone, Debug)]
pub struct Species {
    pub creatures: Vec<Creature>,
}
