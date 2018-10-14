use super::{
    species::creature::{Creature, MadeFrom},
    Generation,
};

impl Generation {
    pub fn print_generation_info(&self) -> () {
        println!(
            "score: {:>width$} | How gene made: {} | gen before: {:>width$} | genes now: {:>width$}",
            self.top_score(),
            self.top_scorer_is_made_from(),
            self.generations_before(),
            self.number_of_creatures(),
            width = 6
        );
    }
    fn top_score(&self) -> f64 {
        self.sort()[self.sort().len() - 1].score
    }
    fn top_scorer_is_made_from(&self) -> &str {
        match self.sort()[self.sort().len() - 1].made_from.clone() {
            MadeFrom::Start => "   Start",
            MadeFrom::Breed => "   Breed",
            MadeFrom::Mutate => "  Mutate",
        }
    }
    fn generations_before(&self) -> u64 {
        self.generations_before
    }
    fn number_of_creatures(&self) -> usize {
        self.sort().len()
    }
    pub fn top_score_from_close<F>(&self, f: F) -> f64
    where
        F: Fn(&Creature) -> f64,
    {
        f(&self.sort()[self.sort().len() - 1])
    }
}
