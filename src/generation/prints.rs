use super::Generation;

impl Generation {
    pub fn print_diverse_debug(&self) -> () {
        let sorted = self.clone().sort();

        println!(
            "score {:>width$} {:?} {:>width$} {:>width$}",
            sorted[sorted.len() - 1].score,
            sorted[sorted.len() - 1].made_from,
            self.generations_before,
            sorted.len(),
            width = 6
        );
        // println!("Number of species {}", self.resort_species().species.len());
        // println!(
        //     "Len of first species {:#?}",
        //     self.resort_species().species[0].creatures.len()
        // );
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
