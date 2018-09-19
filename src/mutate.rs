use crate::Gene;
use crate::MutationLine;
use crate::MutationNode;
use crate::NODE_TYPES;
use rand::Rng;
pub enum Type {
    // Changes all kinds of values
    Strong,
    // Only cange values
    OnlyValues,
}

impl Gene {
    pub fn mutate(&self, mutation_type: &Type, mutation_level: i8) -> Self {
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
    fn node_mutate(self, mutation_type: &Type, mutation_level: i8) -> Self {
        match mutation_type {
            Type::Strong => {
                let invert_chance: i8 =
                    rand::thread_rng().gen_range(i8::min_value(), i8::max_value());
                if mutation_level > invert_chance {
                    let mut rng = rand::thread_rng();
                    *rng.choose(&NODE_TYPES).unwrap()
                } else {
                    self
                }
            }
            Type::OnlyValues => self,
        }
    }
}

// TODO Need more tests5
impl MutationLine {
    fn line_mutate(self, mutation_type: &Type, mutation_level: i8) -> Self {
        match mutation_type {
            Type::Strong => {
                let invert_chance: i8 =
                    rand::thread_rng().gen_range(i8::min_value(), i8::max_value());
                if mutation_level > invert_chance {
                    let invert_chance_level_two: i8 =
                        rand::thread_rng().gen_range(i8::min_value(), i8::max_value());
                    if mutation_level > invert_chance_level_two {
                        let mut rng = rand::thread_rng();
                        let line_types = [
                            MutationLine::Pass,
                            MutationLine::Reset,
                            MutationLine::Multiply(
                                rand::thread_rng().gen_range(i8::min_value(), i8::max_value()),
                            ),
                            MutationLine::Divide(
                                rand::thread_rng().gen_range(i8::min_value(), i8::max_value()),
                            ),
                            MutationLine::Add(
                                rand::thread_rng().gen_range(i8::min_value(), i8::max_value()),
                            ),
                        ];
                        *rng.choose(&line_types).unwrap()
                    } else {
                        self.line_only_values_mutate(mutation_level)
                    }
                } else {
                    self
                }
            }
            Type::OnlyValues => {
                let invert_chance: i8 =
                    rand::thread_rng().gen_range(i8::min_value(), i8::max_value());
                if mutation_level > invert_chance {
                    self.line_only_values_mutate(mutation_level)
                } else {
                    self
                }
            }
        }
    }
    fn line_only_values_mutate(self, mutation_level: i8) -> Self {
        match self {
            MutationLine::Pass => MutationLine::Pass,
            MutationLine::Reset => MutationLine::Reset,
            MutationLine::Multiply(value) => {
                MutationLine::Multiply(rand_plus_or_minus(value, mutation_level))
            }
            MutationLine::Divide(value) => {
                MutationLine::Divide(rand_plus_or_minus(value, mutation_level))
            }
            MutationLine::Add(value) => {
                MutationLine::Add(rand_plus_or_minus(value, mutation_level))
            }
        }
    }
}

// catch panics
fn rand_plus_or_minus(value_one: i8, value_two: i8) -> i8 {
    if rand::thread_rng().gen_bool(0.5) {
        match value_one.checked_add(value_two) {
            Some(v) => v,
            None => value_one,
        }
    } else {
        match value_one.checked_sub(value_two) {
            Some(v) => v,
            None => value_one,
        }
    }
}
