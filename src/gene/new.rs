use {
    super::{Gene, MutationLine, MutationNode, NODE_TYPES},
    rand::Rng,
};

impl Gene {
    pub fn new_gene() -> Self {
        Self {
            line_dna: vec![vec![vec![MutationLine::Reset; 9]; 9]; 2],
            node_dna: vec![vec![MutationNode::Add; 9]; 2],
        }
    }
    // Test new shape of gene
    pub fn new_gene_shape_test() -> Self {
        Self {
            line_dna: vec![
                vec![vec![MutationLine::Multiply(5); 16]; 4],
                vec![vec![MutationLine::Multiply(5); 16]; 16],
                vec![vec![MutationLine::Multiply(5); 4]; 16],
            ],
            node_dna: vec![
                vec![MutationNode::Add; 16],
                vec![MutationNode::Add; 16],
                vec![MutationNode::Add; 4],
            ],
        }
    }
    pub fn new_random_gene() -> Self {
        Self::new_random_basic_gene(2, 9)
    }
    pub fn new_random_basic_gene(depth: u8, hight: u8) -> Self {
        Self {
            line_dna: MutationLine::rand_vec3(depth, hight, hight),
            node_dna: MutationNode::rand_vec2(depth, hight),
        }
    }
}

trait RandVec
where
    Self: Sized,
{
    fn rand_mut() -> Self;
    fn rand_vec3(num: u8, num2: u8, num3: u8) -> Vec<Vec<Vec<Self>>> {
        let mut rand_vec = Vec::new();
        for _x in 0..num {
            rand_vec.push(Self::rand_vec2(num2, num3));
        }
        rand_vec
    }
    fn rand_vec2(num: u8, num2: u8) -> Vec<Vec<Self>> {
        let mut rand_vec = Vec::new();
        for _x in 0..num {
            rand_vec.push(Self::rand_vec(num2));
        }
        rand_vec
    }
    fn rand_vec(num: u8) -> Vec<Self> {
        let mut rand_vec = Vec::new();
        for _x in 0..num {
            rand_vec.push(Self::rand_mut());
        }
        rand_vec
    }
}

impl RandVec for MutationNode {
    fn rand_mut() -> Self {
        let mut rng = rand::thread_rng();
        *rng.choose(&NODE_TYPES).unwrap()
    }
}

impl RandVec for MutationLine {
    fn rand_mut() -> Self {
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
