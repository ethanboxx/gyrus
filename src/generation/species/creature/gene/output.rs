use {
    super::{Gene, MutationLine, MutationNode},
    crate::DEBUG,
};

// Temp way of storing values and type of nodes while outputs are being calculated. Gene.node_dna only stores type MutationNodeStorage stores type and any data it contains.
#[derive(Debug, Clone, Copy)]
struct MutationNodeStorage {
    node_type: MutationNode,
    stored_data: Option<f64>,
}
impl Gene {
    pub fn largest_output(&self, input: &[i32]) -> usize {
        index_of_largest(&self.output(input))
    }
    pub fn score_with_index(&self, input: &[i32], intended_index: usize) -> f64 {
        if index_of_largest(&self.output(input)) == intended_index {
            1.0
        } else {
            0.0
        }
    }
    /// This function calculats an output using a set of inputs and a gene.
    /// If this function takes hard coded values it can be heavily optimised.
    pub fn output(&self, input: &[i32]) -> Vec<f64> {
        let mut output = Vec::new();
        let mut node_values = node_value_calc(&self.node_dna);
        if !self.validate() && DEBUG.check {
            panic!("Gene is not valid")
        };
        // Process inputs into first set of node values.
        for (node_index, node_tree) in self.line_dna[0].iter().enumerate() {
            for (line_index, line) in node_tree.iter().enumerate() {
                node_values[0][line_index].stored_data = Some(
                    node_values[0][line_index]
                        .calc_pass_value(line.calc_pass_value(input[node_index].into())),
                );
            }
        }

        // Process tree.
        for (block_index, block) in self.line_dna.iter().enumerate() {
            // Maybe use `.skip(1)` other than this if
            if block_index == 0 {
                continue;
            }
            for (node_index, node_tree) in block.iter().enumerate() {
                for (line_index, line) in node_tree.iter().enumerate() {
                    node_values[block_index][line_index].stored_data = Some(
                        node_values[block_index][line_index].calc_pass_value(line.calc_pass_value(
                            match node_values[block_index - 1][node_index].stored_data {
                                Some(x) => x,
                                None => panic!("Error 1"),
                            },
                        )),
                    );
                }
            }
        }
        // Fetch output
        for node_values in node_values[node_values.len() - 1].clone() {
            output.push(match node_values.stored_data {
                Some(x) => x,
                None => panic!("Error 2"),
            })
        }
        output
    }
}

fn node_value_calc(node_dna: &[Vec<MutationNode>]) -> Vec<Vec<MutationNodeStorage>> {
    let mut output = Vec::new();
    for row in node_dna {
        output.push(node_value_calc_row(&row));
    }
    output
}
fn node_value_calc_row(row: &[MutationNode]) -> Vec<MutationNodeStorage> {
    let mut output = Vec::new();
    for node in row {
        output.push(convert_mut_node_to_mut_node_store(*node));
    }
    output
}

fn convert_mut_node_to_mut_node_store(node: MutationNode) -> MutationNodeStorage {
    MutationNodeStorage {
        node_type: node,
        stored_data: None,
    }
}

impl MutationLine {
    fn calc_pass_value(self, input_value: f64) -> f64 {
        match self {
            MutationLine::Multiply(x) => input_value * f64::from(x),
            MutationLine::Add(x) => input_value + f64::from(x),
            MutationLine::Divide(x) => input_value / if x == 0 { 1.0 } else { f64::from(x) },
            MutationLine::Pass => input_value,
            MutationLine::Reset => 0.0,
        }
    }
}

impl MutationNodeStorage {
    fn calc_pass_value(&self, input_value: f64) -> f64 {
        match self.stored_data {
            Some(data) => match self.node_type {
                MutationNode::Add => data + input_value,
                MutationNode::Divide => {
                    if data == 0.0 {
                        input_value
                    } else if input_value == 0.0 {
                        data
                    } else {
                        data / input_value
                    }
                }
                MutationNode::Multiply => {
                    if data == 0.0 {
                        input_value
                    } else if input_value == 0.0 {
                        data
                    } else {
                        data * input_value
                    }
                }
            },
            None => input_value,
        }
    }
}

fn index_of_largest(arr: &[f64]) -> usize {
    let x = arr.iter().enumerate().find(|(_index, bigger_value)| {
        arr.iter()
            .all(|smaller_value| bigger_value >= &smaller_value)
    });
    if let Some(y) = x {
        y.0
    } else {
        panic!("Error 6712: should be impossible");
    }
}
