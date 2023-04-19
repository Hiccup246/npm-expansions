use crate::history_model::HistoryModel;
use crate::{expansions_model::ExpansionsAccess, github_api::GithubApi};
use std::{
    fmt,
    sync::{Arc, RwLock},
};
use crate::npm_expansions_github::NpmExpansionsGithubAccessor;

///
#[derive(Debug)]
pub struct AdderError {
    message: String,
}

impl AdderError {
    pub fn from(message: &str) -> AdderError {
        AdderError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for AdderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AdderError: {:?}", self.message)
    }
}
pub struct ExpansionsAdder {}

fn log_message(message: &str) {
    println!("[EXPANSIONS_ADDER] {}", message);
}

pub fn add_new_expansion(epxansions_model: Arc<RwLock<dyn ExpansionsAccess>>, history_model: Arc<RwLock<HistoryModel>>, npm_expansions_github: NpmExpansionsGithubAccessor) -> Result<(), AdderError> {
    log_message("Starting process of adding a new expansion...");

    let unused_pr_number = get_unused_pr_number(history_model, npm_expansions_github)?;

    match unused_pr_number {
        Some(pr_number) => {
            let pr_expansions = (pr_number, get_expansions_for_pr(&pr_number, npm_expansions_github));
            add_expanasions_to_models(pr_expansions, epxansions_model, history_model)?;
        },
        None => {
            log_message(&format!("No unused pr numbers for github repo: {}", {npm_expansions_github.repo_url()}));
        }
    };

    log_message("Process of adding a new expansion complete");

    Ok(())
}

pub fn add_expanasions_to_models(expansions: (String, Option<Vec<String>>), epx_model: Arc<RwLock<dyn ExpansionsAccess>>, history_model: Arc<RwLock<HistoryModel>>) -> Result<(), AdderError> {
    let mut write_model = history_model.write()
        .map_err(|err| AdderError::from(&format!("Unable to gain write access to history model: {}", err)))?;

    match expansions.1 {
        Some(exp) => {
            let mut write_exp_model = epx_model.write()
                .map_err(|err| AdderError::from(&format!("Unable to gain write access to expansions model: {}", err)))?;

            write_exp_model.update_expansions_file(&exp)
                .map_err(|err| AdderError::from(&format!("Unable to write expansions to expansions model: {}", err)))?;
            write_model.update_history_file((chrono::Utc::now(), &expansions.0, "success"))
            .map_err(|err| AdderError::from(&format!("Unable to write history entry to history model: {}", err)))?;

            log_message(&format!("Added expansions {:?}, from pull request {}", expansions.1, expansions.0));
            Ok(())
        },
        None => {
            write_model.update_history_file((chrono::Utc::now(), &expansions.0, "failure"))
            .map_err(|err| AdderError::from(&format!("Unable to write history entry to history model: {}", err)))?;
            log_message(&format!("No expansions from pull request {} to add", expansions.0));
            Ok(())
        }
    }
}

pub fn get_unused_pr_number(history_model: Arc<RwLock<HistoryModel>>, npm_expansions_github: NpmExpansionsGithubAccessor) -> Result<Option<String>, AdderError> {
    let used_pr_numbers = history_model.read()
        .map_err(|err| AdderError::from(&format!("Unable to gain read access to history model: {}", err)))?
        .pr_numbers();

    let unused_pr_number = npm_expansions_github
        .unused_open_pr(&used_pr_numbers)
        .map_err(|err| AdderError::from("Could not access to npm expansions github pull requests!"))?;

    Ok(unused_pr_number)
}

fn get_expansions_for_pr(pr_number: &str, npm_expansions_github: NpmExpansionsGithubAccessor) -> Option<Vec<String>> {
    let expansions = npm_expansions_github.get_new_expansion_from_pr(&pr_number);

    match expansions {
        Ok(Some(exp)) => {
            Some(exp)
        },
        _ => {
            log_message(&format!("Github pr {} for repo {} contains no appropriate expansions", {pr_number}, {npm_expansions_github.repo_url()}));
            None
        }
    }   
}

#[cfg(test)]
mod tests {}
