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

pub fn add_new_expansion(expansions_model: Arc<RwLock<dyn ExpansionsAccess>>, history_model: Arc<RwLock<HistoryModel>>) {
    // Get new expansions
    // Error handle
    // write expansions model
    // write history model
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


    fn update_history_file(&self, pr_number: &str, status: &str) -> Result<(), UpdaterError> {
        let mut write_access_history_model = self
            .history_model
            .write()
            .map_err(|err| UpdaterError::from(err.to_string().as_str()))?;

        write_access_history_model
            .update_history_file((chrono::Utc::now(), pr_number, status))
            .map_err(|err| UpdaterError::from(err.to_string().as_str()))?;

        Ok(())
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

        Ok(_written_expansions)
    }
}

#[cfg(test)]
mod tests {}
