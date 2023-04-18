use crate::expansions_model::ExpansionsAccess;
use std::io;

/// A mock expansions model for testing purposes
pub struct MockExpansionsModel {
    expansions: Vec<String>,
}

impl ExpansionsAccess for MockExpansionsModel {
    fn all(&self) -> &Vec<String> {
        &self.expansions
    }

    fn random_expansion(&self) -> String {
        "no please manager".to_string()
    }

    fn search(&self, _query: &str) -> Vec<String> {
        self.expansions[0..10].to_owned()
    }

    fn update_expansions_file(&mut self, _expansions: &[String]) -> Result<Vec<String>, io::Error> {
        Ok(Vec::from(["node package manger".to_string()]))
    }
}

impl Default for MockExpansionsModel {
    fn default() -> Self {
        MockExpansionsModel {
            expansions: "Nacho Pizza Marinade\nNacho Portion Monitor\nNacho Portmanteau Meltdown\nNacho Printing Machine\nNachos Pillage Milwaukee\nNachos Preventing Motivation\nNadie Programa más\nNagging Penguin Matriarchs\nNahi Pata Mujhe!\nNail Polish Makeover\nNail Polishing Minions\nNaive Pac Man\nNaive Props Mutation\nNaive Puppets Marching"
                .lines()
                .map(|line| line.to_string())
                .collect()
        }
    }
}
