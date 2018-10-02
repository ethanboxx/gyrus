use {super::Generation, rand::Rng};

impl Generation {
    #[allow(clippy::cast_precision_loss)]
    pub fn kill(&self) -> Self {
        let mut all_creatures = self.sort();
        let mut indexes = Vec::new();
        for (index, _gene) in all_creatures.clone().iter().enumerate() {
            if !rand::thread_rng().gen_bool(index as f64 / (all_creatures.len() - 1) as f64) {
                indexes.push(index);
            }
        }
        for index in indexes.iter().rev() {
            all_creatures.remove(*index);
        }
        self.wrap(all_creatures)
    }
}
