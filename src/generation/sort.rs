use super::{Gene, GeneScore, Generation};
use std::cmp::Ordering::Equal;
impl Generation {
    pub fn sort(&mut self) -> () {
        self.genes
            .sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap_or(Equal))
    }
}
