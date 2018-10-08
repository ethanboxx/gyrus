use {
    super::{Gene, MutationLine, MutationNode, NODE_TYPES},
    rand::Rng,
};

impl Gene {
    pub fn new(
        number_of_inputs: usize,
        number_of_output: usize,
        height: usize,
        width: usize,
    ) -> Self {
        if width < 2 {
            panic!("Genes can not be made with a width lower than 2");
        }
        Self {
            line_dna: {
                let mut v = Vec::new();
                v.push(vec![
                    vec![MutationLine::Multiply(5); height];
                    number_of_output
                ]);
                for _ in 0..(width - 2) {
                    v.push(vec![vec![MutationLine::Multiply(5); height]; height])
                }
                v.push(vec![vec![MutationLine::Multiply(5); number_of_inputs]; 16]);
                v
            },
            node_dna: {
                let mut v = Vec::new();
                for _ in 0..(width - 1) {
                    v.push(vec![MutationNode::Add; height]);
                }
                v.push(vec![MutationNode::Add; number_of_output]);
                v
            },
        }
    }
    // Test new shape of gene
    pub fn new_gene_shape_test() -> Self {
        Self::new(4, 4, 16, 3)
    }
    pub fn shuffle(&mut self) -> () {
        self.node_dna = self
            .node_dna
            .iter_mut()
            .map(|x| x.iter_mut().map(|_| MutationNode::random()).collect())
            .collect();
        self.line_dna = self
            .line_dna
            .iter_mut()
            .map(|x| {
                x.iter_mut()
                    .map(|x| x.iter_mut().map(|_| MutationLine::random()).collect())
                    .collect()
            })
            .collect();
    }
}

trait Random
where
    Self: Sized,
{
    fn random() -> Self;
}

impl Random for MutationNode {
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        *rng.choose(&NODE_TYPES).unwrap()
    }
}

impl Random for MutationLine {
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        let line_types = [
            MutationLine::Pass,
            MutationLine::Reset,
            MutationLine::Multiply(rand_i8()),
            MutationLine::Divide(rand_i8()),
            MutationLine::Add(rand_i8()),
        ];
        *rng.choose(&line_types).unwrap()
    }
}

fn rand_i8() -> i8 {
    rand::thread_rng().gen_range(i8::min_value(), i8::max_value())
}
