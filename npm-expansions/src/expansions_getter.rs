struct NpmGithubRepo {
    client: GithubApi,
    repo_url: str,
}

impl NpmExpansionsGithubRepo {
    fn new() -> Result<NpmExpansionsGithubRepo, reqwest::Error> {
        NpmExpansionsGithubRepo {
            repo_url: "https://api.github.com/repos/npm/npm-expansions",
            client: GithubApi::new("npm-expansions.com")?
        }
    }

    /// Two options
    /// 1. We have pr number and expansions
    /// 2. We have pr number but no expansions
    pub fn get_new_expansion_from_pr(pr_number: &str) -> Vec<String> {
        let expansions_txt_raw_url = self.raw_url_of_pr_file(pr_number, "expansions.txt");
        let expansions_txt_string = self.stringifyed_file_contents(expansions_txt_raw_url);
        
        self.clean_expansions_string(expansions_txt_string)
    }
    
    pub fn unused_open_pr(&self, repo_url: &str, used_pr_numbers: Vec<&str>, ) {
        let open_pr_numbers = self
            .client
            .open_pr_numbers(repo_url)
            .map_err(|err| UpdaterError::from(err.to_string().as_str()))?;

        let new_pr_number = open_pr_numbers
            .iter()
            .find(|&pr_number| !used_pr_numbers.contains(&pr_number))
            .ok_or(UpdaterError::from("No new prs to get!"))?;

        Ok(new_pr_number.clone())
    }
    
    fn raw_url_of_pr_file(pr_url: &str, filename: &str) {
        let pr_files = self
            .client
            .fetch_pr_raw_file_urls(&pr_url)
            .map_err(|err| UpdaterError::from(err.to_string().as_str()))?;

        let expansions_file_url =
            pr_files
                .get(filename)
                .ok_or(UpdaterError::from(""))?;

        expansions_file_url
    }
    
    fn stringifyed_file_contents(&self, raw_file_url: &str) -> Result<String, UpdaterError> {
        let expansions_file = self
            .client
            .fetch_pr_file(raw_file_url)
            .map_err(|err| UpdaterError::from(err.to_string().as_str()))?;

        let expansions_file_string = std::str::from_utf8(expansions_file.as_slice())
            .map_err(|err| UpdaterError::from(err.to_string().as_str()))?;

        Ok(expansions_file_string.to_string())
    }
    
    fn clean_expansions_string(expansions_string: &str) -> Vec<String>{
        expansions_file
        .lines()
        .filter(|expansion| !expansion.starts_with('#'))
        .filter(|expansion| !expansion.is_inappropriate())
        .map(|expansion| expansion.to_string())
        .collect()
    }
}