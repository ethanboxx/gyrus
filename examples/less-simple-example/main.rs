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

extern crate gyrus;
use gyrus::generation::Generation;

//TODO lets get some scores to se how easy this is

fn main() {
    let mut generation = Generation::new(4, 4, 8, 3, true, 100);
    println!("Each creature is given sets of 4 binary inputs. The creature must return the correct number to get score for each input. Will it learn how to get the highest score?");
    loop {
        generation.print_generation_info();

        //TODO score needs to be caluclated with more decimals so the flow is better
        generation = generation.score_update(|current_creature| {
            let intended = [
                ([0, 0, 0, 0], 0),
                ([1, 0, 0, 0], 3),
                ([0, 1, 0, 0], 1),
                ([0, 0, 1, 0], 2),
                ([0, 0, 0, 1], 0),
                ([1, 1, 0, 0], 3),
                ([0, 1, 1, 0], 2),
                ([0, 0, 1, 1], 0),
                ([1, 0, 1, 0], 1),
                ([0, 1, 0, 1], 2),
                ([1, 0, 0, 1], 3),
                ([1, 1, 1, 0], 1),
                ([0, 1, 1, 1], 2),
                ([1, 1, 0, 1], 2),
                ([1, 0, 1, 1], 1),
                ([1, 1, 1, 1], 0),
            ];
            let mut score = 0.0;
            for intend in &intended {
                if current_creature.gene.largest_output(&intend.0) == intend.1 {
                    score += 1.0;
                }
            }
            score
        });
        generation.update();
    }
}
