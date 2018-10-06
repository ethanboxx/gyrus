#![feature(tool_lints)]
#![warn(clippy::pedantic)]
//To run example `cargo run --example less-simple-example --release`
// Let user pick number of gen to process every time.

//TODO Print out alot more data
//TODO fix genetic diversity.
//TODO study if lines have the same diversity problem as nodes // They do

// Take 4 inputs
// Each one has 1 or 0 going through.

// in   out
// 0  |  1
// 0  |
// 0  |
// 0  |

// 1  |
// 0  |
// 0  |
// 0  |  1

// 0  |
// 1  |  1
// 0  |
// 0  |

// 0  |
// 0  |
// 1  |  1
// 0  |

// 0  |  1
// 0  |
// 0  |
// 1  |

// 1  |
// 1  |
// 0  |
// 0  |  1

// 0  |
// 1  |
// 1  |  1
// 0  |

// 0  |  1
// 0  |
// 1  |
// 1  |

// 1  |
// 0  |  1
// 1  |
// 0  |

// 0  |
// 1  |
// 0  |  1
// 1  |

// 1  |
// 0  |
// 0  |
// 1  |  1

// 1  |
// 1  |  1
// 1  |
// 0  |

// 0  |
// 1  |
// 1  |  1
// 1  |

// 1  |
// 1  |
// 0  |  1
// 1  |

// 1  |
// 0  |  1
// 1  |
// 1  |

// 1  |  1
// 1  |
// 1  |
// 1  |

extern crate ai_graph;
use ai_graph::generation::Generation;

//TODO lets get some scores to se how easy this is

fn main() {
    let mut generation = Generation::new_rand_simple_custom(100);
    loop {
        generation.print_generation_info();
        generation = generation.score_update(|current_creature| {
            let mut score = 0.0;
            if current_creature.gene.largest_output(&[0, 0, 0, 0]) == 0 {
                score += 1.0;
            }
            if current_creature.gene.largest_output(&[1, 0, 0, 0]) == 3 {
                score += 1.0;
            }
            if current_creature.gene.largest_output(&[0, 1, 0, 0]) == 1 {
                score += 1.0;
            }
            if current_creature.gene.largest_output(&[0, 0, 1, 0]) == 2 {
                score += 1.0;
            }
            if current_creature.gene.largest_output(&[0, 0, 0, 1]) == 0 {
                score += 1.0;
            }
            if current_creature.gene.largest_output(&[1, 1, 0, 0]) == 3 {
                score += 1.0;
            }
            if current_creature.gene.largest_output(&[0, 1, 1, 0]) == 2 {
                score += 1.0;
            }
            if current_creature.gene.largest_output(&[0, 0, 1, 1]) == 0 {
                score += 1.0;
            }
            if current_creature.gene.largest_output(&[1, 0, 1, 0]) == 1 {
                score += 1.0;
            }
            if current_creature.gene.largest_output(&[0, 1, 0, 1]) == 2 {
                score += 1.0;
            }
            if current_creature.gene.largest_output(&[1, 0, 0, 1]) == 3 {
                score += 1.0;
            }
            if current_creature.gene.largest_output(&[1, 1, 1, 0]) == 1 {
                score += 1.0;
            }
            if current_creature.gene.largest_output(&[0, 1, 1, 1]) == 2 {
                score += 1.0;
            }
            if current_creature.gene.largest_output(&[1, 1, 0, 1]) == 2 {
                score += 1.0;
            }
            if current_creature.gene.largest_output(&[1, 0, 1, 1]) == 1 {
                score += 1.0;
            }
            if current_creature.gene.largest_output(&[1, 1, 1, 1]) == 0 {
                score += 1.0;
            }
            score
        });
        generation.update();
    }
}
