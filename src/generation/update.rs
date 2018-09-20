use super::{Gene, GeneScore, Generation};
use std::cmp::Ordering::Equal;

//TODO create new genes to fill space after kill
impl Generation {
    pub fn update(&mut self) -> () {
        self.kill()
    }
}
