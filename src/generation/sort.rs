use {
    super::{Creature, Generation},
    std::cmp::Ordering::Equal,
};
impl Generation {
    //TODO panic if all values are zero
    pub fn sort(&self) -> Vec<Creature> {
        let mut x = self.clone().unwrap();
        x.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap_or(Equal));
        x
        // Code once used to check the sort. The sort worked and was valid
        // let mut value = -0.1;
        // for (index, gene) in self.genes.iter().enumerate() {
        //     if value <= self.genes[index].score {
        //         value = self.genes[index].score;
        //     } else {
        //         panic!("Not sorted");
        //     }
        // }
    }
}
