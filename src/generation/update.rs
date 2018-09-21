use {
    super::{Gene, GeneScore, Generation},
    rand::Rng,
    std::cmp::Ordering::Equal,
};

//TODO create new genes to fill space after kill
impl Generation {
    pub fn update(&mut self) -> () {
        self.kill();
        let mut number_of_genes_to_add =
            (self.intended_size as u16 - self.genes.len() as u16) as i32;
        loop {
            // mut random gene
            self.genes.push(GeneScore {
                gene: self.genes[rand::thread_rng().gen_range(0, self.genes.len())]
                    .gene
                    .mutate(
                        {
                            let mut rng = rand::thread_rng();
                            let types = &[
                                crate::gene::mutate::Type::Strong,
                                crate::gene::mutate::Type::OnlyValues,
                            ];
                            let rng = rng.choose(types).unwrap();
                            rng
                        },
                        rand::thread_rng().gen_range(i8::min_value(), i8::max_value()),
                    ),
                score: 0.0,
            });
            number_of_genes_to_add = number_of_genes_to_add - 1;
            if number_of_genes_to_add == 0 {
                break;
            }
            // breed 2 gene
            self.genes.push(GeneScore {
                gene: Gene::breed(
                    &self.genes[rand::thread_rng().gen_range(0, self.genes.len())].gene,
                    &self.genes[rand::thread_rng().gen_range(0, self.genes.len())].gene,
                ),
                score: 0.0,
            });
            number_of_genes_to_add = number_of_genes_to_add - 1;
            if number_of_genes_to_add == 0 {
                break;
            }
        }
    }
}
