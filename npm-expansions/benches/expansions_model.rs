#![feature(test)]

extern crate test;
use npm_expansions::expansions_model::ExpansionsModel;

#[cfg(test)]
mod tests {
    use super::*;
    use npm_expansions::expansions_model::ExpansionsAccess;
    use std::path::Path;
    use test::Bencher;

    #[bench]
    fn random_expansions(b: &mut Bencher) {
        let expansions_generator = ExpansionsModel::build(Path::new("rsc/expansions.txt"));
        b.iter(|| expansions_generator.random_expansion());
    }

    #[bench]
    fn all_expansions(b: &mut Bencher) {
        let expansions_generator = ExpansionsModel::build(Path::new("rsc/expansions.txt"));

        b.iter(|| expansions_generator.all());
    }

    #[bench]
    fn search_expansions(b: &mut Bencher) {
        let expansions_generator = ExpansionsModel::build(Path::new("rsc/expansions.txt"));

        b.iter(|| expansions_generator.search("Nacho Person Manager"));
    }
}
