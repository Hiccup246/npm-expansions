#![feature(test)]

extern crate test;
use npm_expansions::npm_expansions::NpmExpansions;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn generating_random_expansions(b: &mut Bencher) {
        b.iter(|| NpmExpansions::random_expansion());
    }

    #[bench]
    fn generating_all_expansions(b: &mut Bencher) {
        b.iter(|| NpmExpansions::expansions());
    }

    #[bench]
    fn generating_all_new_expansions(b: &mut Bencher) {
        b.iter(|| NpmExpansions::new_expansions());
    }
}
