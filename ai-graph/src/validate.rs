use crate::Gene;

//TODO Not all checks are finnished.
impl Gene {
    pub fn validate(&self) -> bool {
        self.sum_lines_per_row_equal() && self.all_output_avalible()
    }
    pub fn validate_two(&self, second_gene: &Self) -> bool {
        self.validate() && second_gene.validate() && self.equal_size(second_gene)
    }

    fn sum_lines_per_row_equal(&self) -> bool {
        for line_block in &self.line_dna {
            let number = line_block[0].len();
            for line_set in line_block {
                if !line_set.len() == number {
                    return false;
                }
            }
        }
        true
    }
    fn all_output_avalible(&self) -> bool {
        true
    }
    fn equal_size(&self, _second_gene: &Self) -> bool {
        true
    }
}
