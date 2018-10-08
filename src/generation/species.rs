pub mod creature;

use self::creature::Creature;

#[derive(Clone, Debug)]
pub struct Species {
    pub(crate) creatures: Vec<Creature>,
    pub(crate) key: Option<Vec<i8>>,
}
