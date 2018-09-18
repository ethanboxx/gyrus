use crate::mutate;
#[cfg(test)]
use crate::Gene;
use rand::Rng;
#[test]
fn random_gene_is_valid() {
    let test_gene = Gene::new_random_gene();
    if !test_gene.validate() {
        panic!("Gene is not valid")
    };
}

// To get output run cargo test tests::random_breed_is_valid -- --nocapture
#[test]

fn random_breed_is_valid() {
    let mut test_gene = Gene::new_random_gene();
    println!("test_gene{:#?}", test_gene);
    let test_gene2 = Gene::new_random_gene();
    println!("test_gene2{:#?}", test_gene2);
    test_gene = Gene::breed(&test_gene, &test_gene2);
    println!("test_gene2 mut{:#?}", test_gene);
    if !test_gene.validate() {
        panic!("Gene is not valid")
    };
}

// To get output run cargo test tests::merge_is_valid -- --nocapture
#[test]
fn merge_is_valid() {
    let mut test_gene = Gene::new_random_gene();
    println!("test_gene{:#?}", test_gene);
    test_gene = test_gene.mutate(&mutate::Type::Strong, 10);
    println!("test_gene mutated mut{:#?}", test_gene);
    if !test_gene.validate() {
        panic!("Gene is not valid")
    };
}
