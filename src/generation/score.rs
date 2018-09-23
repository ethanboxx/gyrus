use super::Creature;
use super::Generation;

impl Generation {
    pub fn score_update<F>(self, _f: F) -> Self
    where
        F: Fn(i32) -> i32,
    {
        let mut generation = self;

        generation = Self {
            genes: generation
                .genes
                .iter()
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

        generation
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
