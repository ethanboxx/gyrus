use super::{Gene, MutationNode};

impl Gene {
    /// `find_key` can be used to keep genetic diversity.
    pub fn find_key(&self) -> Vec<i8> {
        let mut key = Vec::new();
        for node in &self.node_dna[0] {
            key.push(match node {
                MutationNode::Multiply => 0,
                MutationNode::Divide => 1,
                MutationNode::Add => 2,
            });
        }
        key
    }
}
