struct mutate_type {
    // Changes all kinds of values
    Strong,
    // Only cange values
    OnlyValues,
    
}

impl Gene {
    pub fn mutate(&self) -> Self {
        let mut new_values = self.clone();
        if !self.validate() {
            panic!("Genes not compatible. Can't breed.");
        };
        // Mutate nodes.
        for (node_line_index, node_line) in self.node_dna.iter().enumerate() {
            for (node_index, node) in node_line.iter().enumerate() {
                new_values.node_dna[node_line_index][node_index] =
                    node.node_mutate();
            }
        }
        // Mutate lines.
        for (block_index, block) in self.line_dna.iter().enumerate() {
            for (node_point_index, node_point) in block.iter().enumerate() {
                for (line_index, line) in node_point.iter().enumerate() {
                    new_values.line_dna[block_index][node_point_index][line_index] = line
                        .line_mutate()
                }
            }
        }
        new_values.clone()}
}
