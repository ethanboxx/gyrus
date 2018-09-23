//To run example `cargo run --example less-simple-example --release`
// Let user pick number of gen to process every time.

//TODO Print out alot more data
//TODO fix genetic diversity.
//TODO study if have the same  diversity problem as nodes

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
use ai_graph::generation::creature::Creature;
use ai_graph::generation::Generation;
use rayon::prelude::*;
//TODO lets get some scores to se how easy this is

fn main() {
    let mut generation = Generation::new_rand_simple_custom(100);
    // println!("Random start generation {:#?}", generation);

    loop {
        generation = Generation {
            genes: generation
                .genes
                .par_iter()
                .map(|current| {
                    Creature {
                        score: {
                            let mut score = 0.0;
                            if largest_of_3(&current.gene.clone().output(&[0, 0, 0, 0])) == 0 {
                                score = score + 1.0;
                            }
                            if largest_of_3(&current.gene.clone().output(&[1, 0, 0, 0])) == 3 {
                                score = score + 1.0;
                            }
                            if largest_of_3(&current.gene.clone().output(&[0, 1, 0, 0])) == 1 {
                                score = score + 1.0;
                            }
                            if largest_of_3(&current.gene.clone().output(&[0, 0, 1, 0])) == 2 {
                                score = score + 1.0;
                            }
                            if largest_of_3(&current.gene.clone().output(&[0, 0, 0, 1])) == 0 {
                                score = score + 1.0;
                            }
                            if largest_of_3(&current.gene.clone().output(&[1, 1, 0, 0])) == 3 {
                                score = score + 1.0;
                            }
                            if largest_of_3(&current.gene.clone().output(&[0, 1, 1, 0])) == 2 {
                                score = score + 1.0;
                            }
                            if largest_of_3(&current.gene.clone().output(&[0, 0, 1, 1])) == 0 {
                                score = score + 1.0;
                            }
                            if largest_of_3(&current.gene.clone().output(&[1, 0, 1, 0])) == 1 {
                                score = score + 1.0;
                            }
                            if largest_of_3(&current.gene.clone().output(&[0, 1, 0, 1])) == 2 {
                                score = score + 1.0;
                            }
                            if largest_of_3(&current.gene.clone().output(&[1, 0, 0, 1])) == 3 {
                                score = score + 1.0;
                            }
                            if largest_of_3(&current.gene.clone().output(&[1, 1, 1, 0])) == 1 {
                                score = score + 1.0;
                            }
                            if largest_of_3(&current.gene.clone().output(&[0, 1, 1, 1])) == 2 {
                                score = score + 1.0;
                            }
                            if largest_of_3(&current.gene.clone().output(&[1, 1, 0, 1])) == 2 {
                                score = score + 1.0;
                            }
                            if largest_of_3(&current.gene.clone().output(&[1, 0, 1, 1])) == 1 {
                                score = score + 1.0;
                            }
                            if largest_of_3(&current.gene.clone().output(&[1, 1, 1, 1])) == 0 {
                                score = score + 1.0;
                            }

                            score
                        }, //TODO Calc score form gene
                        ..current.clone()
                    }.clone()
                }).collect(),
            generations_before: generation.generations_before + 1,
            ..generation
        };
        // println!("Random middle generation {:#?}", generation);
        // println!("Random middle len {:#?}", generation.genes.len());

        generation.update();
        generation.sort();
        println!(
            "score {:>width$} {:?} {:>width$}",
            generation.genes[generation.genes.len() - 1].score,
            generation.genes[generation.genes.len() - 1].made_from,
            generation.generations_before,
            width = 6
        );
        //Code used to check genetic diversity. It failed.
        println!(
            "data {:?}",
            generation.genes[generation.genes.len() - 1].key,
        );
        println!(
            "data {:?}",
            generation.genes[generation.genes.len() - 2].key,
        );

        //Code used to check genetic diversity. It failed.
        // println!(
        //     "data {:#?} {:#?}",
        //     generation.genes[generation.genes.len() - 1],
        //     generation.genes[generation.genes.len() - 2],
        // );
        // println!("Random end generation {:#?}", generation);
        // println!("Random end len {:#?}", generation.genes.len());
    }
}

fn largest_of_3(arr: &Vec<f64>) -> i8 {
    if arr[0] > arr[1] && arr[0] > arr[2] && arr[0] > arr[3] {
        0
    } else if arr[1] > arr[0] && arr[1] > arr[2] && arr[1] > arr[3] {
        1
    } else if arr[2] > arr[0] && arr[2] > arr[1] && arr[2] > arr[3] {
        2
    } else if arr[3] > arr[0] && arr[3] > arr[1] && arr[3] > arr[2] {
        2
    } else {
        0
    }
}