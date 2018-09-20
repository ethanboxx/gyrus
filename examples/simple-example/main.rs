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
use ai_graph::generation::Generation;

fn main() {
    //TODO Need to change size of ai
    let generation = Generation::new_rand(20);
    println!("Random start generation {:#?}", generation);
}
