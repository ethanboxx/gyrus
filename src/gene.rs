mod breed;
mod key;
pub mod mutate;
mod new;
mod output;
mod validate;

const NODE_TYPES: [MutationNode; 3] = [
    MutationNode::Multiply,
    MutationNode::Divide,
    MutationNode::Add,
];

#[derive(Clone, Copy, Debug)]
enum MutationLine {
    Pass,
    Reset,
    Multiply(i8),
    Divide(i8),
    Add(i8),
}
#[derive(Clone, Copy, Debug)]
enum MutationNode {
    Multiply,
    Divide,
    Add,
}

#[derive(Clone, Debug)]
/// Gene stores the "graph". Different graphs will form different output.
pub struct Gene {
    line_dna: Vec<Vec<Vec<MutationLine>>>,
    node_dna: Vec<Vec<MutationNode>>,
}
