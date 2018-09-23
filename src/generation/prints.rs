use super::Generation;

impl Generation {
    pub fn print_diverse_debug(&self) -> () {
        println!(
            "score {:>width$} {:?} {:>width$}",
            self.genes[self.genes.len() - 1].score,
            self.genes[self.genes.len() - 1].made_from,
            self.generations_before,
            width = 6
        );
        //Code used to check genetic diversity. It failed.
        println!("data {:?}", self.genes[self.genes.len() - 1].key,);
        println!("data {:?}", self.genes[self.genes.len() - 2].key,);
    }
}

//Code used to check genetic diversity. It failed.
// println!(
//     "data {:#?} {:#?}",
//     generation.genes[generation.genes.len() - 1],
//     generation.genes[generation.genes.len() - 2],
// );
// println!("Random end generation {:#?}", generation);
// println!("Random end len {:#?}", generation.genes.len());
