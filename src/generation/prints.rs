use super::Generation;

impl Generation {
    pub fn print_diverse_debug(&self) -> () {
        let mut sorted = self.clone();
        sorted.sort();
        println!(
            "score {:>width$} {:?} {:>width$}",
            sorted.genes[sorted.genes.len() - 1].score,
            sorted.genes[sorted.genes.len() - 1].made_from,
            sorted.generations_before,
            width = 6
        );
        //Code used to check genetic diversity. It failed.
        println!("data {:?}", sorted.genes[sorted.genes.len() - 1].key,);
        println!("data {:?}", sorted.genes[sorted.genes.len() - 2].key,);
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
