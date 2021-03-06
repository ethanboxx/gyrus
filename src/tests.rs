#[cfg(test)]
use {
    crate::generation::{
        species::creature::gene::{mutate, Gene},
        Generation,
    },
    rand::Rng,
};

//TODO make broken gene and check it. Test if it fails

#[test]
fn random_gene_is_valid() {
    let test_gene = Gene::new(4, 4, 8, 3);
    if !test_gene.validate() {
        panic!("Gene is not valid")
    };
}

// To get output run cargo test tests::random_breed_is_valid -- --nocapture
#[test]

fn random_breed_is_valid() {
    let mut test_gene = Gene::new(4, 4, 8, 3);
    println!("test_gene{:#?}", test_gene);
    let test_gene2 = Gene::new(4, 4, 8, 3);;
    println!("test_gene2{:#?}", test_gene2);
    test_gene = Gene::breed(&test_gene, &test_gene2);
    println!("test_gene2 mut{:#?}", test_gene);
    if !test_gene.validate() {
        panic!("Gene is not valid")
    };
}

// To get output run cargo test tests::mutate_is_valid -- --nocapture
#[test]
fn mutate_is_valid() {
    let mut test_gene = Gene::new(4, 4, 8, 3);
    println!("test_gene{:#?}", test_gene);
    test_gene = test_gene.mutate(&mutate::Type::Strong, 10);
    println!("test_gene mutated mut{:#?}", test_gene);
    if !test_gene.validate() {
        panic!("Gene is not valid")
    };
}

// To get output run cargo test tests::random_mutate_is_valid -- --nocapture
#[test]
fn random_mutate_is_valid() {
    let mut test_gene = Gene::new(4, 4, 8, 3);
    println!("test_gene{:#?}", test_gene);
    test_gene = test_gene.mutate(
        {
            let mut rng = rand::thread_rng();
            let line_types = [&mutate::Type::Strong, &mutate::Type::OnlyValues];
            *rng.choose(&line_types).unwrap()
        },
        rand::thread_rng().gen_range(i8::min_value(), i8::max_value()),
    );
    println!("test_gene mutated mut{:#?}", test_gene);
    if !test_gene.validate() {
        panic!("Gene is not valid")
    };
}

// To get output run cargo test tests::random_generation_is_valid -- --nocapture
#[test]
fn random_generation_is_valid() {
    let test_generation = Generation::new(4, 4, 8, 3, true, 100);
    if !test_generation.validate() {
        panic!("Gene is not valid")
    };
}

// To get output run cargo test tests::random_generation_update_is_valid -- --nocapture
#[test]
fn random_generation_update_is_valid() {
    let mut test_generation = Generation::new(4, 4, 8, 3, true, 100);

    if !test_generation.validate() {
        panic!("Gene is not valid before update")
    };
    test_generation.update();
    if !test_generation.validate() {
        panic!("Gene is not valid after update")
    };
}
// To get output run cargo test tests::species_sort -- --nocapture
#[test]
fn species_sort() {
    let mut test_generation = Generation::new(4, 4, 8, 3, true, 100);
    println!("Before {:#?}", test_generation);
    test_generation = test_generation.resort_species();
    println!("After {:#?}", test_generation);
}

//test