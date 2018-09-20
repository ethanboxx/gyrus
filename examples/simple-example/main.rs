//To run example `cargo run --example simple-example --release`
// Let user pick number of gen to process every time.

// Take 3 inputs
// Each one has 1 or 0 going through.

// in   out
// 0  |
// 1  |  1
// 0  |

// 1  |
// 0  |
// 0  |  1

// 0  |  1
// 0  |
// 1  |

extern crate ai_graph;
use ai_graph::generation::GeneScore;
use ai_graph::generation::Generation;
use rayon::prelude::*;
//TODO lets get some scores to se how easy this is

fn main() {
    let mut generation = Generation::new_rand(20, 2, 4);
    println!("Random start generation {:#?}", generation);
    generation = Generation {
        genes: generation
            .genes
            .par_iter()
            .map(|current| {
                GeneScore {
                    score: 10.0, //TODO Calc score form gene
                    ..current.clone()
                }.clone()
            }).collect(),
        generations_before: generation.generations_before + 1,
        ..generation
    };
    println!("Random start generation {:#?}", generation);
}
