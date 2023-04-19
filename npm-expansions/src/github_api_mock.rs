use reqwest;
use std::collections::HashMap;
use crate::github_api::GithubApiAccess;

pub struct GithubApiMock {}

impl GithubApiAccess for GithubApiMock {
    fn open_pr_numbers(&self, repo_url: &str) -> Result<Vec<String>, reqwest::Error> {
        Ok(vec!["4301".to_string(), "4302".to_string(), "4303".to_string()])
    }

    fn fetch_pr_raw_file_urls(
        &self,
        pr_url: &str,
    ) -> Result<HashMap<String, String>, reqwest::Error> {
        Ok(
            HashMap::from([
                ("expansions.txt".to_string(), "https://npm.com/expansions.txt".to_string()),
                ("metadata.txt".to_string(), "expansions are fun!".to_string())
            ])
        )
    }

    fn fetch_pr_file_as_string(&self, raw_file_url: &str) -> Result<String, reqwest::Error> {
        Ok("node package manager\r\nno purpose much\r\nnice puppet master\r\n".to_string())
    }
}

impl GithubApiMock {
    pub fn default() -> Self {
        GithubApiMock {}
    }
}