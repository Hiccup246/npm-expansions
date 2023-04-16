use rand::Rng;
use std::fs::OpenOptions;
use std::io::Write;
use std::{
    fs::{self, File},
    io,
    path::Path,
    path::PathBuf,
};
use strsim::jaro_winkler;

/// A struct representing a vector of npm expansion strings and methods to search them
pub struct ExpansionsModel {
    expansions_file: PathBuf,
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

    /// Adds an array of expansions to an existing list of expansions
    fn add_expansions(&mut self, expansions: &Vec<String>);

    fn update_expansions_file(&self, expansions: Vec<String>) -> Result<(), io::Error>;
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
        let mut scored_matches: Vec<(f64, &String)> = self
            .expansions
            .iter()
            .map(|expansion| (jaro_winkler(expansion, query), expansion))
            .collect();

        scored_matches.sort_by(|a, b| b.0.total_cmp(&a.0));

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

    fn add_expansions(&mut self, expansions: &Vec<String>) {
        self.expansions.extend(expansions.to_owned())
    }

    fn update_expansions_file(&self, expansions: Vec<String>) -> Result<(), io::Error> {
        let mut expansions_file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(self.expansions_file.as_path())?;

        for expansion in expansions {
            writeln!(expansions_file, "{}", expansion)?;
        }

        Ok(())
    }
}

impl ExpansionsModel {
    /// Takes a path to a txt file and constructs a ExpansionsModel with its
    /// expansions field populated by the expansions found in the txt file.
    ///
    /// The given text file should be in a format where each line
    /// that is not a comment i.e. start with a # or * () is a npm expansion
    pub fn build(path: &Path) -> ExpansionsModel {
        let expansions_string: Vec<String> = fs::read_to_string(path)
            .unwrap()
            .lines()
            .filter(|expansion| !expansion.starts_with("#"))
            .map(|expansion| expansion.to_string())
            .collect();

        ExpansionsModel {
            expansions_file: path.to_path_buf(),
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

        fs::write(
            file.path(),
            b"Nacho Pizza Marinade \n Nacho Portion Monitor \n Nacho Portmanteau Meltdown",
        )
        .unwrap();

        let expansion = ExpansionsModel::build(file.path()).random_expansion();

        assert!(!expansion.is_empty())
    }

    #[test]
    fn all_expansions() {
        let file = Builder::new().prefix("expansions.txt").tempfile().unwrap();

        fs::write(
            file.path(),
            b"Nacho Pizza Marinade \n Nacho Portion Monitor \n Nacho Portmanteau Meltdown",
        )
        .unwrap();

        let all_expansions = ExpansionsModel::build(file.path());

        assert_eq!(all_expansions.all().len(), 3)
    }

    #[test]
    fn search_expansions_exact_match() {
        let file = Builder::new().prefix("expansions.txt").tempfile().unwrap();

        fs::write(
            file.path(),
            "Nacho Pizza Marinade\nNacho Portion Monitor\nNacho Portmanteau Meltdown\nNacho Printing Machine\nNachos Pillage Milwaukee\nNachos Preventing Motivation\nNadie Programa más\nNagging Penguin Matriarchs\nNahi Pata Mujhe!\nNail Polish Makeover\nNail Polishing Minions\nNaive Pac Man\nNaive Props Mutation\nNaive Puppets Marching".as_bytes()
        ).unwrap();

        let expansions = ExpansionsModel::build(file.path()).search("Nachos Pillage Milwaukee");

        assert_eq!(expansions.first().unwrap(), "Nachos Pillage Milwaukee")
    }

    #[test]
    fn search_expansions_returns_top_ten() {
        let file = Builder::new().prefix("expansions.txt").tempfile().unwrap();

        fs::write(
            file.path(),
            "Nacho Pizza Marinade\nNacho Portion Monitor\nNacho Portmanteau Meltdown\nNacho Printing Machine\nNachos Pillage Milwaukee\nNachos Preventing Motivation\nNadie Programa más\nNagging Penguin Matriarchs\nNahi Pata Mujhe!\nNail Polish Makeover\nNail Polishing Minions\nNaive Pac Man\nNaive Props Mutation\nNaive Puppets Marching".as_bytes()
        ).unwrap();

        let expansions = ExpansionsModel::build(file.path()).search("Nachos Pillage Milwaukee");

        assert_eq!(expansions.len(), 10)
    }

    #[test]
    fn writes_new_expansions() {
        let file = Builder::new().prefix("expansions.txt").tempfile().unwrap();
        let model = ExpansionsModel::build(file.path());

        fs::write(file.path(), b"").unwrap();
        model
            .update_expansions_file(vec![
                "no manager please".to_string(),
                "nix program mistress".to_string(),
            ])
            .unwrap();

        let file_contents = fs::read_to_string(file.path()).unwrap();

        assert_eq!(file_contents, "no manager please\nnix program mistress\n")
    }
}
