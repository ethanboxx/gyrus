use {super::Generation, rand::Rng};

impl Generation {
    #[allow(clippy::cast_precision_loss)]
    pub fn kill(&mut self) -> () {
        self.sort();
        // let copy = self.genes[self.genes.len() - 1].gene.clone();
        let mut indexes = Vec::new();
        for (index, _gene) in self.genes.clone().iter().enumerate() {
            if !rand::thread_rng().gen_bool(index as f64 / (self.genes.len() - 1) as f64) {
                indexes.push(index);
            }
        }
        for index in indexes.iter().rev() {
            self.genes.remove(*index);
        }
        //This commented out code was used to debug an issue that has been solved ISSUE This code shows that sometimes the best gene is getting killed MUST BE SOLVED #[derive(PartialEq)]
        // if copy != self.genes[self.genes.len() - 1].gene.clone() {
        //     panic!("woah");
        // }
    }
}
