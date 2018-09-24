//To run example `cargo run --example less-simple-example --release`
// Let user pick number of gen to process every time.

//TODO Print out alot more data
//TODO fix genetic diversity.
//TODO study if lines have the same diversity problem as nodes

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
        generation = generation.score_update(|current| {
            let mut score = 0.0;
            if largest_of_3(&current.gene.clone().output(&[0, 0, 0, 0])) == 0 {
                score += 1.0;
            }
            if largest_of_3(&current.gene.clone().output(&[1, 0, 0, 0])) == 3 {
                score += 1.0;
            }
            if largest_of_3(&current.gene.clone().output(&[0, 1, 0, 0])) == 1 {
                score += 1.0;
            }
            if largest_of_3(&current.gene.clone().output(&[0, 0, 1, 0])) == 2 {
                score += 1.0;
            }
            if largest_of_3(&current.gene.clone().output(&[0, 0, 0, 1])) == 0 {
                score += 1.0;
            }
            if largest_of_3(&current.gene.clone().output(&[1, 1, 0, 0])) == 3 {
                score += 1.0;
            }
            if largest_of_3(&current.gene.clone().output(&[0, 1, 1, 0])) == 2 {
                score += 1.0;
            }
            if largest_of_3(&current.gene.clone().output(&[0, 0, 1, 1])) == 0 {
                score += 1.0;
            }
            if largest_of_3(&current.gene.clone().output(&[1, 0, 1, 0])) == 1 {
                score += 1.0;
            }
            if largest_of_3(&current.gene.clone().output(&[0, 1, 0, 1])) == 2 {
                score += 1.0;
            }
            if largest_of_3(&current.gene.clone().output(&[1, 0, 0, 1])) == 3 {
                score += 1.0;
            }
            if largest_of_3(&current.gene.clone().output(&[1, 1, 1, 0])) == 1 {
                score += 1.0;
            }
            if largest_of_3(&current.gene.clone().output(&[0, 1, 1, 1])) == 2 {
                score += 1.0;
            }
            if largest_of_3(&current.gene.clone().output(&[1, 1, 0, 1])) == 2 {
                score += 1.0;
            }
            if largest_of_3(&current.gene.clone().output(&[1, 0, 1, 1])) == 1 {
                score += 1.0;
            }
            if largest_of_3(&current.gene.clone().output(&[1, 1, 1, 1])) == 0 {
                score += 1.0;
            }
            score
        });
        generation.update();
        generation.print_diverse_debug();
    }
}

fn largest_of_3(arr: &[f64]) -> i8 {
    if arr[0] > arr[1] && arr[0] > arr[2] && arr[0] > arr[3] {
        0
    } else if arr[1] > arr[0] && arr[1] > arr[2] && arr[1] > arr[3] {
        1
    } else if arr[2] > arr[0] && arr[2] > arr[1] && arr[2] > arr[3] {
        2
    } else if arr[3] > arr[0] && arr[3] > arr[1] && arr[3] > arr[2] {
        3
    } else {
        0
    }
}
