use reqwest;
use serde_json;
use std::collections::HashMap;

/// A struct allowing HTTP requests to be made to the github developer REST API found at https://docs.github.com/en/rest
pub struct GithubApi {
    client: reqwest::blocking::Client,
}

pub trait GithubApiAccess {
    fn open_pr_numbers(&self, repo_url: &str) -> Result<Vec<String>, reqwest::Error>;
    fn fetch_pr_raw_file_urls(
        &self,
        pr_url: &str,
    ) -> Result<HashMap<String, String>, reqwest::Error>;
    fn fetch_pr_file_as_string(&self, raw_file_url: &str) -> Result<String, reqwest::Error>;
}

impl GithubApiAccess for GithubApi {
    /// Returns the unique numbers of all open pull requests for a given github repository
    ///
    /// # Arguments
    ///
    /// * `repo_url` - the url of a github repository
    fn open_pr_numbers(&self, repo_url: &str) -> Result<Vec<String>, reqwest::Error> {
        let repo_prs_url = repo_url.to_owned() + "/pulls?state=open";
        let repo_prs: Vec<serde_json::Value> = self
            .client
            .get(repo_prs_url)
            .send()?
            .json::<Vec<serde_json::Value>>()?;

        Ok(repo_prs.iter().map(|pr| pr["number"].to_string()).collect())
    }

    /// Returns all files associated with a given pull request in a HashMap of format
    /// ```json
    /// { "filename": "raw_url" }
    /// ```
    ///
    /// # Arguments
    ///
    /// * `pr_url` - url of a github pull request
    fn fetch_pr_raw_file_urls(
        &self,
        pr_url: &str,
    ) -> Result<HashMap<String, String>, reqwest::Error> {
        let pr_files_url = pr_url.to_owned() + "/files";

        let repo_files: Vec<serde_json::Value> = self
            .client
            .get(pr_files_url)
            .send()?
            .json::<Vec<serde_json::Value>>()?;

        let name_and_raw_url = repo_files
            .iter()
            .map(|github_file| {
                (
                    github_file["filename"].as_str(),
                    github_file["raw_url"].as_str(),
                )
            })
            .filter_map(|github_file_tuple| {
                if let (Some(key), Some(value)) = (github_file_tuple.0, github_file_tuple.1) {
                    Some((key.to_string(), value.to_string()))
                } else {
                    None
                }
            });

        Ok(HashMap::from_iter(name_and_raw_url))
    }

    /// Returns the bytes of an individual raw github file
    ///
    /// # Arguments
    ///
    /// * `raw_file_url` - a raw file github url
    fn fetch_pr_file_as_string(&self, raw_file_url: &str) -> Result<String, reqwest::Error> {
        Ok(self.client.get(raw_file_url).send()?.text_with_charset("utf-8")?)
    }
}

impl GithubApi {
    /// Creates a new instance of GithubApi with a reqwest client
    ///
    /// # Arguments
    ///
    /// * `user_agent` - user_agent string which will be imbedded intp all HTTP requests
    pub fn new(user_agent: &str) -> Result<GithubApi, reqwest::Error> {
        let client = reqwest::blocking::Client::builder()
            .user_agent(user_agent)
            .build()?;

        Ok(GithubApi { client })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito;

    mod open_pr_numbers {
        use super::*;

        #[test]
        fn calls_github_prs_url() {
            let github_api = GithubApi::new("npm-expansions").unwrap();
            let mut mock_server = mockito::Server::new();
            let repo_url = mock_server.url() + "/repos/npm/npm-expansions";

            let mock = mock_server
                .mock("GET", "/repos/npm/npm-expansions/pulls?state=open")
                .with_status(200)
                .with_body("[{\"number\": 4301},{\"number\": 4302},{\"number\": 4303}]")
                .create();

            github_api.open_pr_numbers(repo_url.as_str()).unwrap();

            mock.assert();
        }

        #[test]
        fn returns_pr_numbers() {
            let github_api = GithubApi::new("npm-expansions").unwrap();
            let mut mock_server = mockito::Server::new();
            let repo_url = mock_server.url() + "/repos/npm/npm-expansions";

            mock_server
                .mock("GET", "/repos/npm/npm-expansions/pulls?state=open")
                .with_status(200)
                .with_body("[{\"number\":4301},{\"number\":4302},{\"number\":4303}]")
                .create();

            let pr_numbers = github_api.open_pr_numbers(repo_url.as_str()).unwrap();

            assert_eq!(pr_numbers, vec!["4301", "4302", "4303"])
        }

        #[test]
        fn invalid_repo_url() {
            let github_api = GithubApi::new("npm-expansions").unwrap();
            let mut mock_server = mockito::Server::new();
            let repo_url = mock_server.url() + "/repos/npm";

            mock_server
                .mock("GET", "/repos/npm/npm-expansions/pulls?state=open")
                .with_status(200)
                .with_body("[{\"number\":4301},{\"number\":4302},{\"number\":4303}]")
                .create();

            let pr_numbers = github_api.open_pr_numbers(repo_url.as_str());

            assert!(pr_numbers.is_err())
        }

        #[test]
        fn url_returns_invalid_json() {
            let github_api = GithubApi::new("npm-expansions").unwrap();
            let mut mock_server = mockito::Server::new();
            let repo_url = mock_server.url() + "/repos/npm/npm-expansions";

            mock_server
                .mock("GET", "/repos/npm/npm-expansions/pulls?state=open")
                .with_status(200)
                .with_body("number\":4301 \"number\":4302 \"number\":4303}]")
                .create();

            let pr_numbers = github_api.open_pr_numbers(repo_url.as_str());

            assert!(pr_numbers.is_err())
        }

        #[test]
        fn url_returns_400() {
            let github_api = GithubApi::new("npm-expansions").unwrap();
            let mut mock_server = mockito::Server::new();
            let repo_url = mock_server.url() + "/repos/npm/npm-expansions";

            mock_server
                .mock("GET", "/repos/npm/npm-expansions/pulls?state=open")
                .with_status(400)
                .create();

            let pr_numbers = github_api.open_pr_numbers(repo_url.as_str());

            assert!(pr_numbers.is_err())
        }

        #[test]
        fn repo_has_no_prs() {
            let github_api = GithubApi::new("npm-expansions").unwrap();
            let mut mock_server = mockito::Server::new();
            let repo_url = mock_server.url() + "/repos/npm/npm-expansions";

            mock_server
                .mock("GET", "/repos/npm/npm-expansions/pulls?state=open")
                .with_status(200)
                .with_body("[]")
                .create();

            let pr_numbers = github_api.open_pr_numbers(repo_url.as_str()).unwrap();

            assert_eq!(pr_numbers, Vec::new() as Vec<String>)
        }
    }

    mod fetch_pr_raw_file_urls {
        use super::*;

        #[test]
        fn fetches_files() {
            let github_api = GithubApi::new("npm-expansions").unwrap();
            let mut mock_server = mockito::Server::new();

            mock_server.mock("GET", "/repos/npm/npm-expansions/pulls/4302/files")
                .with_status(200)
                .with_body("[{\"filename\": \"expansions.txt\", \"raw_url\": \"url-to-file\"}, {\"filename\":\"hello-world.txt\", \"raw_url\": \"url-to-file\"}]")
                .create();

            let repo_pr = mock_server.url() + "/repos/npm/npm-expansions/pulls/4302";

            let files = github_api.fetch_pr_raw_file_urls(repo_pr.as_str()).unwrap();

            assert_eq!(
                files,
                HashMap::from([
                    ("expansions.txt".to_string(), "url-to-file".to_string()),
                    ("hello-world.txt".to_string(), "url-to-file".to_string())
                ])
            )
        }

        #[test]
        fn invalid_json_response() {
            let github_api = GithubApi::new("npm-expansions").unwrap();
            let mut mock_server = mockito::Server::new();

            mock_server.mock("GET", "/repos/npm/npm-expansions/pulls/4302/files")
                .with_status(200)
                .with_body("\"filename\": \"expansions.txt\", \"raw_url\": \"url-to-file\"}, {\"filename\":\"hello-world.txt\", \"raw_url\": \"url-to-file\"}]")
                .create();

            let repo_pr = mock_server.url() + "/repos/npm/npm-expansions/pulls/4302";

            let response = github_api.fetch_pr_raw_file_urls(repo_pr.as_str());

            assert!(response.is_err())
        }

        #[test]
        fn url_returns_400() {
            let github_api = GithubApi::new("npm-expansions").unwrap();
            let mut mock_server = mockito::Server::new();

            mock_server
                .mock("GET", "/repos/npm/npm-expansions/pulls/4302/files")
                .with_status(400)
                .create();

            let repo_pr = mock_server.url() + "/repos/npm/npm-expansions/pulls/4302";

            let response = github_api.fetch_pr_raw_file_urls(repo_pr.as_str());

            assert!(response.is_err())
        }

        #[test]
        fn blank_filename() {
            let github_api = GithubApi::new("npm-expansions").unwrap();
            let mut mock_server = mockito::Server::new();

            mock_server.mock("GET", "/repos/npm/npm-expansions/pulls/4302/files")
                .with_status(200)
                .with_body("[{\"filename\": \"\", \"raw_url\": \"url-to-file\"}, {\"filename\":\"hello-world.txt\", \"raw_url\": \"url-to-file\"}]")
                .create();

            let repo_pr = mock_server.url() + "/repos/npm/npm-expansions/pulls/4302";

            let files = github_api.fetch_pr_raw_file_urls(repo_pr.as_str()).unwrap();

            assert_eq!(
                files,
                HashMap::from([
                    ("".to_string(), "url-to-file".to_string()),
                    ("hello-world.txt".to_string(), "url-to-file".to_string())
                ])
            )
        }

        #[test]
        fn blank_raw_url() {
            let github_api = GithubApi::new("npm-expansions").unwrap();
            let mut mock_server = mockito::Server::new();

            mock_server.mock("GET", "/repos/npm/npm-expansions/pulls/4302/files")
                .with_status(200)
                .with_body("[{\"filename\": \"expansions.txt\", \"raw_url\": \"\"}, {\"filename\":\"hello-world.txt\", \"raw_url\": \"url-to-file\"}]")
                .create();

            let repo_pr = mock_server.url() + "/repos/npm/npm-expansions/pulls/4302";

            let files = github_api.fetch_pr_raw_file_urls(repo_pr.as_str()).unwrap();

            assert_eq!(
                files,
                HashMap::from([
                    ("expansions.txt".to_string(), "".to_string()),
                    ("hello-world.txt".to_string(), "url-to-file".to_string())
                ])
            )
        }

        #[test]
        fn no_raw_url() {
            let github_api = GithubApi::new("npm-expansions").unwrap();
            let mut mock_server = mockito::Server::new();

            mock_server.mock("GET", "/repos/npm/npm-expansions/pulls/4302/files")
                .with_status(200)
                .with_body("[{\"filename\": \"expansions.txt\"}, {\"filename\":\"hello-world.txt\", \"raw_url\": \"url-to-file\"}]")
                .create();

            let repo_pr = mock_server.url() + "/repos/npm/npm-expansions/pulls/4302";

            let files = github_api.fetch_pr_raw_file_urls(repo_pr.as_str()).unwrap();

            assert_eq!(
                files,
                HashMap::from([("hello-world.txt".to_string(), "url-to-file".to_string())])
            )
        }

        #[test]
        fn no_filename() {
            let github_api = GithubApi::new("npm-expansions").unwrap();
            let mut mock_server = mockito::Server::new();

            mock_server.mock("GET", "/repos/npm/npm-expansions/pulls/4302/files")
                .with_status(200)
                .with_body("[{\"raw_url\": \"url-to-file\"}, {\"filename\":\"hello-world.txt\", \"raw_url\": \"url-to-file\"}]")
                .create();

            let repo_pr = mock_server.url() + "/repos/npm/npm-expansions/pulls/4302";

            let files = github_api.fetch_pr_raw_file_urls(repo_pr.as_str()).unwrap();

            assert_eq!(
                files,
                HashMap::from([("hello-world.txt".to_string(), "url-to-file".to_string())])
            )
        }
    }

    mod fetch_pr_file {
        use super::*;

        #[test]
        fn fetch_file() {
            let github_api = GithubApi::new("npm-expansions").unwrap();
            let mut mock_server = mockito::Server::new();

            mock_server
                .mock("GET", "/example.txt")
                .with_status(200)
                .with_body(b"Hello World!")
                .create();

            let repo_pr = mock_server.url() + "/example.txt";

            let file_bytes = github_api.fetch_pr_file_as_string(repo_pr.as_str()).unwrap();

            assert_eq!(file_bytes, "Hello World!")
        }
    }
}
