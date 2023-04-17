use crate::history_model::HistoryModel;
use crate::{expansions_model::ExpansionsAccess, github_api::GithubApi};
use chrono::DateTime;
use chrono::Utc;
use core::time::Duration;
use rustrict::CensorStr;
use std::{
    fmt,
    sync::{Arc, RwLock},
};

///
#[derive(Debug)]
pub struct UpdaterError {
    message: String,
}

impl UpdaterError {
    ///
    pub fn from(message: &str) -> UpdaterError {
        UpdaterError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for UpdaterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UpdaterError: {:?}", self.message)
    }
}

///
pub struct ExpansionsUpater {
    epxansions_model: Arc<RwLock<dyn ExpansionsAccess>>,
    history_model: Arc<RwLock<HistoryModel>>,
    github_api: GithubApi,
    repo_url: String,
    update_interval_millis: i64,
}

impl ExpansionsUpater {
    ///
    pub fn new(
        epxansions_model: Arc<RwLock<dyn ExpansionsAccess>>,
        history_model: Arc<RwLock<HistoryModel>>,
        repo_url: String,
        update_interval_millis: i64,
    ) -> ExpansionsUpater {
        ExpansionsUpater {
            epxansions_model,
            history_model,
            github_api: GithubApi::new("expansions_updater")
                .expect("Failed to create github api for ExpansionsUpdater"),
            repo_url,
            update_interval_millis,
        }
    }

    ///
    pub fn add_new_expansions(&mut self) -> Result<Vec<String>, UpdaterError> {
        let new_pr_number = self.find_new_pr_number()?;

        self.update_history_file(&new_pr_number, "success")?;

        let expansions_file = self.get_expansions_file(&new_pr_number)?;
        let appropriate_expansions = collate_appropriate_expansions(&expansions_file);

        let written_expansions = self.update_expansions_model(&appropriate_expansions)?;

        Ok(written_expansions)
    }

    ///
    pub fn time_to_next_update(
        &self,
        current_date: DateTime<Utc>,
    ) -> Result<Duration, UpdaterError> {
        let read_access_history_model = self
            .history_model
            .read()
            .map_err(|err| UpdaterError::from(err.to_string().as_str()))?;

        let mut sleep_duration = chrono::Duration::milliseconds(0);
        let update_interval_duration = chrono::Duration::milliseconds(self.update_interval_millis);

        if let Some((last_updated_at, _pr_number)) = read_access_history_model.latest_entry() {
            let time_since_last_update = current_date - *last_updated_at;

            if time_since_last_update < update_interval_duration {
                sleep_duration = update_interval_duration - time_since_last_update;
            };
        };

        Ok(Duration::from_millis(
            sleep_duration.num_milliseconds() as u64
        ))
    }

    fn find_new_pr_number(&self) -> Result<String, UpdaterError> {
        let read_access_history_model = self
            .history_model
            .read()
            .map_err(|err| UpdaterError::from(err.to_string().as_str()))?;

        let merged_prs = read_access_history_model.pr_numbers();

        let repo_pr_numbers = self
            .github_api
            .repo_pr_numbers(&self.repo_url)
            .map_err(|err| UpdaterError::from(err.to_string().as_str()))?;

        let new_pr_number = repo_pr_numbers
            .iter()
            .find(|&pr_number| !merged_prs.contains(&pr_number))
            .ok_or(UpdaterError::from("No new prs to get!"))?;

        Ok(new_pr_number.clone())
    }

    fn update_history_file(&self, pr_number: &String, _status: &str) -> Result<(), UpdaterError> {
        let write_access_history_model = self
            .history_model
            .write()
            .map_err(|err| UpdaterError::from(err.to_string().as_str()))?;

        write_access_history_model
            .update_history_file((chrono::Utc::now(), pr_number))
            .map_err(|err| UpdaterError::from(err.to_string().as_str()))?;

        Ok(())
    }

    fn get_expansions_file(&self, pr_number: &String) -> Result<String, UpdaterError> {
        let pr_url = format!("{}/pulls/{}", self.repo_url, pr_number);

        let pr_files = self
            .github_api
            .fetch_pr_raw_file_urls(&pr_url)
            .map_err(|err| UpdaterError::from(err.to_string().as_str()))?;

        let expansions_file_url =
            pr_files
                .get("expansions.txt")
                .ok_or(UpdaterError::from(&format!(
                    "No expansions.txt file for {pr_url}"
                )))?;

        let expansions_file = self
            .github_api
            .fetch_pr_file(expansions_file_url)
            .map_err(|err| UpdaterError::from(err.to_string().as_str()))?;

        let expansions_file_string = std::str::from_utf8(expansions_file.as_slice())
            .map_err(|err| UpdaterError::from(err.to_string().as_str()))?;

        Ok(expansions_file_string.to_string())
    }

    fn update_expansions_model(
        &self,
        new_expansions: &[String],
    ) -> Result<Vec<String>, UpdaterError> {
        let mut writeable_expansions_model = self
            .epxansions_model
            .write()
            .map_err(|err| UpdaterError::from(err.to_string().as_str()))?;

        let _written_expansions = writeable_expansions_model
            .update_expansions_file(new_expansions)
            .map_err(|err| UpdaterError::from(err.to_string().as_str()))?;

        writeable_expansions_model
            .reload()
            .map_err(|err| UpdaterError::from(err.to_string().as_str()))?;

        Ok(_written_expansions)
    }
}

fn collate_appropriate_expansions(expansions_file: &str) -> Vec<String> {
    let new_expansions: Vec<String> = expansions_file
        .lines()
        .filter(|expansion| !expansion.starts_with('#'))
        .filter(|expansion| !expansion.is_inappropriate())
        .map(|expansion| expansion.to_string())
        .collect();

    new_expansions
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use chrono::TimeZone;
    // use chrono::Utc;
    // use std::fs;
    // use tempfile::Builder;
}
