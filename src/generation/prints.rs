use super::Generation;

impl Generation {
    pub fn print_diverse_debug(&self) -> () {
        let sorted = self.clone().sort();

        println!(
            "score {:>width$} {:?} {:>width$}",
            sorted[sorted.len() - 1].score,
            sorted[sorted.len() - 1].made_from,
            self.generations_before,
            width = 6
        );
        println!("Number of species {}", self.species.len());
        // for species in self.species.iter() {
        //     println!("Species len {}", species.creatures.len())
        // }
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
