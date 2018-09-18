use crate::node_types;
use crate::Gene;
use crate::MutationLine;
use crate::MutationNode;
use rand::Rng;
pub enum Type {
    // Changes all kinds of values
    Strong,
    // Only cange values
    OnlyValues,
}

impl Gene {
    pub fn mutate(&self, mutation_type: &Type, mutation_level: &i8) -> Self {
        let mut new_values = self.clone();
        if !self.validate() {
            panic!("Genes not valid. Can't mutate.");
        };
        // Mutate nodes.
        for (node_line_index, node_line) in self.node_dna.iter().enumerate() {
            for (node_index, node) in node_line.iter().enumerate() {
                new_values.node_dna[node_line_index][node_index] =
                    node.node_mutate(mutation_type, mutation_level);
            }
        }
        // Mutate lines.
        for (block_index, block) in self.line_dna.iter().enumerate() {
            for (node_point_index, node_point) in block.iter().enumerate() {
                for (line_index, line) in node_point.iter().enumerate() {
                    new_values.line_dna[block_index][node_point_index][line_index] =
                        line.line_mutate(mutation_type, mutation_level)
                }
            }
        }
        new_values.clone()
    }
}

impl MutationNode {
    fn node_mutate(self, mutation_type: &Type, mutation_level: &i8) -> Self {
        match mutation_type {
            Type::Strong => {
                let invert_chance: i8 =
                    rand::thread_rng().gen_range(i8::min_value(), i8::max_value());
                if mutation_level > &invert_chance {
                    let mut rng = rand::thread_rng();
                    *rng.choose(&node_types).unwrap()
                } else {
                    self
                }
            }
            Type::OnlyValues => self,
        }
    }
}

impl MutationLine {
    fn line_mutate(&self, _mutation_type: &Type, _mutation_level: &i8) -> Self {
        MutationLine::Multiply(10)
    }
}
