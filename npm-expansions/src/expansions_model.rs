use levenshtein::levenshtein;
use rand::Rng;
use std::fs;

/// A struct representing a vector of npm expansion strings and methods to search them
pub struct ExpansionsModel {
    expansions: Vec<String>,
}

/// This trait represents the basic search functions that a expansions model should provide
pub trait ExpansionsAccess {
    /// Returns a random npm expansion
    fn random_expansion(&self) -> String;
    /// Returns all available npm expansions
    fn all(&self) -> &Vec<String>;
    /// Returns a curated list of npm expansions based on a given search query
    fn search(&self, query: &str) -> Vec<String>;
}

impl ExpansionsAccess for ExpansionsModel {
    fn all(&self) -> &Vec<String> {
        &self.expansions
    }

    fn random_expansion(&self) -> String {
        let random_index: usize = rand::thread_rng().gen_range(0..self.expansions.len());
        let expansion = self.expansions.get(random_index).unwrap();

        expansion.to_string()
    }

    fn search(&self, query: &str) -> Vec<String> {
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

impl ExpansionsModel {
    /// Takes a path to a txt file and constructs a ExpansionsModel with its
    /// expansions field populated by the expansions found in the txt file.
    ///
    /// The given text file should be in a format where each line
    /// that is not a comment i.e. start with a # or * () is a npm expansion
    pub fn build(path: &str) -> ExpansionsModel {
        let expansions_string: Vec<String> = fs::read_to_string(path)
            .unwrap()
            .lines()
            .filter(|a| !a.starts_with('*') && !a.starts_with('#'))
            .map(|expansion| expansion.to_string())
            .collect();

        ExpansionsModel {
            expansions: expansions_string,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::Builder;

    #[test]
    fn random_expansion() {
        let file = Builder::new().prefix("expansions.txt").tempfile().unwrap();
        let file_path = file.path().to_str().unwrap();

        fs::write(
            &file,
            b"Nacho Pizza Marinade \n Nacho Portion Monitor \n Nacho Portmanteau Meltdown",
        )
        .unwrap();

        let expansion = ExpansionsModel::build(file_path).random_expansion();

        assert!(!expansion.is_empty())
    }

    #[test]
    fn all_expansions() {
        let file = Builder::new().prefix("expansions.txt").tempfile().unwrap();
        let file_path = file.path().to_str().unwrap();

        fs::write(
            &file,
            b"Nacho Pizza Marinade \n Nacho Portion Monitor \n Nacho Portmanteau Meltdown",
        )
        .unwrap();

        let all_expansions = ExpansionsModel::build(file_path);

        assert_eq!(all_expansions.all().len(), 3)
    }

    #[test]
    fn search_expansions_exact_match() {
        let file = Builder::new().prefix("expansions.txt").tempfile().unwrap();
        let file_path = file.path().to_str().unwrap();

        fs::write(
            &file,
            "Nacho Pizza Marinade\nNacho Portion Monitor\nNacho Portmanteau Meltdown\nNacho Printing Machine\nNachos Pillage Milwaukee\nNachos Preventing Motivation\nNadie Programa más\nNagging Penguin Matriarchs\nNahi Pata Mujhe!\nNail Polish Makeover\nNail Polishing Minions\nNaive Pac Man\nNaive Props Mutation\nNaive Puppets Marching".as_bytes()
        ).unwrap();

        let expansions = ExpansionsModel::build(file_path).search("Nachos Pillage Milwaukee");

        assert_eq!(expansions.first().unwrap(), "Nachos Pillage Milwaukee")
    }

    #[test]
    fn search_expansions_returns_top_ten() {
        let file = Builder::new().prefix("expansions.txt").tempfile().unwrap();
        let file_path = file.path().to_str().unwrap();

        fs::write(
            &file,
            "Nacho Pizza Marinade\nNacho Portion Monitor\nNacho Portmanteau Meltdown\nNacho Printing Machine\nNachos Pillage Milwaukee\nNachos Preventing Motivation\nNadie Programa más\nNagging Penguin Matriarchs\nNahi Pata Mujhe!\nNail Polish Makeover\nNail Polishing Minions\nNaive Pac Man\nNaive Props Mutation\nNaive Puppets Marching".as_bytes()
        ).unwrap();

        let expansions = ExpansionsModel::build(file_path).search("Nachos Pillage Milwaukee");

        assert_eq!(expansions.len(), 10)
    }
}
