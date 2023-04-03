use levenshtein::levenshtein;
use rand::Rng;
use std::fs;

pub struct NpmExpansions {
    expansions: Vec<String>,
}

pub trait ExpansionsGenerator {
    fn random_expansion(&self) -> String;
    fn expansions(&self) -> &Vec<String>;
    fn levenshtein_search(&self, query: &str) -> Vec<String>;
}

impl ExpansionsGenerator for NpmExpansions {
    fn expansions(&self) -> &Vec<String> {
        &self.expansions
    }

    fn random_expansion(&self) -> String {
        let random_index: usize = rand::thread_rng().gen_range(0..self.expansions.len());
        let expansion = self.expansions.get(random_index).unwrap();

        expansion.to_string()
    }

    fn levenshtein_search(&self, query: &str) -> Vec<String> {
        let mut scored_matches: Vec<(usize, &String)> = self
            .expansions
            .iter()
            .map(|expansion| (levenshtein(expansion, query), expansion))
            .collect();

        scored_matches.sort_by(|a, b| a.0.cmp(&b.0));

        let end_index = if scored_matches.len() < 10 {
            scored_matches.len()
        } else {
            10
        };

        scored_matches[0..end_index]
            .iter()
            .map(|expansions| expansions.1.clone())
            .collect::<Vec<String>>()
    }
}

impl NpmExpansions {
    pub fn build(path: &str) -> NpmExpansions {
        let expansions_string: Vec<String> = fs::read_to_string(path)
            .unwrap()
            .lines()
            .filter(|a| !a.starts_with('*') && !a.starts_with('#'))
            .map(|expansion| expansion.to_string())
            .collect();

        NpmExpansions {
            expansions: expansions_string,
        }
    }

    pub fn new(expansions_string: &str) -> NpmExpansions {
        let expansions: Vec<String> = expansions_string
            .lines()
            .filter(|a| !a.starts_with('*') && !a.starts_with('#'))
            .map(|expansion| expansion.to_string())
            .collect();

        NpmExpansions { expansions }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_valid_expansion() {
        let expansion_one = NpmExpansions::new("no please manager").random_expansion();

        assert!(!expansion_one.is_empty())
    }
}
