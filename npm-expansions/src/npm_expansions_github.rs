use reqwest;
use std::{fmt};
use crate::github_api::GithubApiAccess;
use rustrict::CensorStr;

pub struct NpmExpansionsGithubAccessor {
    client: Box<dyn GithubApiAccess>,
    repo_url: String,
}

impl NpmExpansionsGithubAccessor {
    pub fn repo_url(&self) -> &str {
        &self.repo_url
    }

    pub fn new(github_api_client: Box<dyn GithubApiAccess>) -> NpmExpansionsGithubAccessor {
        NpmExpansionsGithubAccessor {
            repo_url: "https://api.github.com/repos/npm/npm-expansions".to_string(),
            client: github_api_client,
        }
    }

    pub fn get_new_expansion_from_pr(&self, pr_number: &str) -> Result<Option<Vec<String>>, reqwest::Error> {
        let expansions_txt_raw_url = self.raw_url_of_pr_file(pr_number, "expansions.txt")?;
        
        match expansions_txt_raw_url {
            Some(url) => {
                let expansions_txt_string = self.client.fetch_pr_file_as_string(&url)?; 
                Ok(Some(self.clean_expansions_string(&expansions_txt_string)))
            },
            None => Ok(None)
        }
    }
    
    pub fn unused_open_pr(&self, used_pr_numbers: &[String]) -> Result<Option<String>, reqwest::Error> {
        let open_pr_numbers = self
            .client
            .open_pr_numbers(&self.repo_url)?;

        let new_pr_number = open_pr_numbers
            .iter()
            .find(|pr_number| !used_pr_numbers.contains(pr_number));

        Ok(new_pr_number.cloned())
    }
    
    fn raw_url_of_pr_file(&self, pr_url: &str, filename: &str) -> Result<Option<String>, reqwest::Error> {
        let pr_files = self
            .client
            .fetch_pr_raw_file_urls(&pr_url)?;

        Ok(pr_files.get(filename).cloned())
    }
    
    fn clean_expansions_string(&self, expansions_string: &str) -> Vec<String>{
        expansions_string.to_string()
        .lines()
        .filter(|expansion| !expansion.starts_with('#'))
        .filter(|expansion| !expansion.is_inappropriate())
        .map(|expansion| expansion.to_string())
        .collect()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::github_api_mock::GithubApiMock;

    #[test]
    fn correctly_returns_expansions() {
        let getter = NpmExpansionsGithubAccessor::new(Box::new(GithubApiMock::default()));
        let expansions = getter.get_new_expansion_from_pr("4302").unwrap();

        assert_eq!(expansions.unwrap(), vec!["node package manager", "no purpose much", "nice puppet master"])
    }

    #[test]
    fn unused_open_pr_returns_first_unused_number() {
        let getter = NpmExpansionsGithubAccessor::new(Box::new(GithubApiMock::default()));
        let expansions = getter.unused_open_pr(&vec!["4301".to_string()]).unwrap();

        assert_eq!(expansions.unwrap(), "4302")
    }

    #[test]
    fn no_unused_open_pr_returns_first_unused_number() {
        let getter = NpmExpansionsGithubAccessor::new(Box::new(GithubApiMock::default()));
        let expansions = getter.unused_open_pr(&vec!["4301".to_string(), "4302".to_string(), "4303".to_string()]).unwrap();

        assert!(expansions.is_none())
    }
}