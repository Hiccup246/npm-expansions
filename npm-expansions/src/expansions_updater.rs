use crate::history_model::HistoryModel;
use crate::{expansions_model::ExpansionsAccess, github_api::GithubApi};
use chrono::DateTime;
use chrono::Utc;
use core::time::Duration;
use rustrict::CensorStr;
use std::{
    fmt,
    path::Path,
    sync::{Arc, RwLock},
};

#[derive(Debug)]
pub struct UpdaterError {
    message: String,
}

impl UpdaterError {
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

static NPM_EXPANSIONS_REPO: &str = "https://api.github.com/repos/npm/npm-expansions";

pub fn add_new_expansion(
    history_model: &HistoryModel,
    expansions_model: Arc<RwLock<dyn ExpansionsAccess>>,
) -> Result<(), UpdaterError> {
    let github_api = GithubApi::new("npm-expansions.com")
        .map_err(|err| UpdaterError::from(err.to_string().as_str()))?;

    let merged_prs = history_model.pr_numbers();

    let repo_pr_numbers = github_api
        .repo_pr_numbers(NPM_EXPANSIONS_REPO)
        .map_err(|err| UpdaterError::from(err.to_string().as_str()))?;

    let new_pr_number = next_unused_pr_number(&repo_pr_numbers, &merged_prs)
        .ok_or(UpdaterError::from("No new prs to get!"))?;

    let pr_url = format!("{}/pulls/{}", NPM_EXPANSIONS_REPO, new_pr_number);
    let pr_files = github_api
        .fetch_pr_raw_file_urls(&pr_url)
        .map_err(|err| UpdaterError::from(err.to_string().as_str()))?;

    let expansions_file_url =
        pr_files
            .get("expansions.txt")
            .ok_or(UpdaterError::from(&format!(
                "No expansions.txt file for {pr_url}"
            )))?;

    let expansions_file = github_api
        .fetch_pr_file(expansions_file_url)
        .map_err(|err| UpdaterError::from(err.to_string().as_str()))?;
    let expansions_file_string = std::str::from_utf8(expansions_file.as_slice())
        .map_err(|err| UpdaterError::from(err.to_string().as_str()))?;

    let readable_expansions_model = expansions_model
        .read()
        .map_err(|err| UpdaterError::from(err.to_string().as_str()))?;

    let new_expansions: Vec<String> = expansions_file_string
        .lines()
        .filter(|expansion| !expansion.starts_with('#'))
        .filter(|expansion| !expansion.is_inappropriate())
        .map(|expansion| expansion.to_string())
        .filter(|expansion| !readable_expansions_model.all().contains(&expansion))
        .collect();

    let mut writeable_expansions_model = expansions_model
        .write()
        .map_err(|err| UpdaterError::from(err.to_string().as_str()))?;

    writeable_expansions_model.add_expansions(&new_expansions);

    writeable_expansions_model
        .update_expansions_file(new_expansions)
        .map_err(|err| UpdaterError::from(err.to_string().as_str()))?;

    history_model
        .update_history_file((chrono::Utc::now(), &new_pr_number))
        .map_err(|err| UpdaterError::from(err.to_string().as_str()))?;

    Ok(())
}

pub fn calc_sleep_time(history_model: &HistoryModel, current_date: DateTime<Utc>) -> Duration {
    let two_weeks = chrono::Duration::milliseconds(60000 * 60 * 24 * 14);
    let mut sleep_duration = chrono::Duration::milliseconds(0);

    if let Some((last_updated_at, _pr_number)) = history_model.latest_entry() {
        let time_since_last_update = current_date - *last_updated_at;

        if time_since_last_update < two_weeks {
            sleep_duration = two_weeks - time_since_last_update;
        };
    };

    Duration::from_millis(sleep_duration.num_milliseconds() as u64)
}

fn next_unused_pr_number<'a>(
    repo_pr_numbers: &'a Vec<String>,
    merged_prs: &Vec<&String>,
) -> Option<&'a String> {
    for pr_number in repo_pr_numbers {
        if !merged_prs.contains(&pr_number) {
            return Some(pr_number);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use chrono::Utc;
    use std::fs;
    use tempfile::Builder;

    #[test]
    fn calc_sleep_time_test() {
        let named_tempfile = Builder::new()
            .prefix("pr_numbers")
            .suffix(".txt")
            .tempfile()
            .unwrap();

        fs::write(named_tempfile.path(), "2022-02-02T00:00:00+00:00,4302\r\n").unwrap();
        let two_weeks_in_millis = 60000 * 60 * 24 * 14;

        let mut history_model = HistoryModel::new(named_tempfile.path());
        history_model.load_history().unwrap();

        let mock_date = Utc.with_ymd_and_hms(2022, 2, 2, 0, 0, 0).unwrap();
        assert_eq!(
            calc_sleep_time(&history_model, mock_date),
            Duration::from_millis(two_weeks_in_millis)
        );
    }

    #[test]
    fn calc_sleep_time_test_2() {
        let named_tempfile = Builder::new()
            .prefix("pr_numbers")
            .suffix(".txt")
            .tempfile()
            .unwrap();

        let two_weeks_in_millis = 60000 * 60 * 24 * 12;

        fs::write(named_tempfile.path(), "2022-02-01T00:00:00+00:00,4302\r\n").unwrap();

        let mut history_model = HistoryModel::new(named_tempfile.path());
        history_model.load_history().unwrap();

        let mock_date = Utc.with_ymd_and_hms(2022, 2, 3, 0, 0, 0).unwrap();
        assert_eq!(
            calc_sleep_time(&history_model, mock_date),
            Duration::from_millis(two_weeks_in_millis)
        );
    }

    #[test]
    fn calc_sleep_time_test_3() {
        let named_tempfile = Builder::new()
            .prefix("pr_numbers")
            .suffix(".txt")
            .tempfile()
            .unwrap();

        fs::write(named_tempfile.path(), "2022-02-01T00:00:00+00:00,4302\r\n").unwrap();

        let mut history_model = HistoryModel::new(named_tempfile.path());
        history_model.load_history().unwrap();

        let mock_date = Utc.with_ymd_and_hms(2022, 2, 15, 0, 0, 0).unwrap();
        assert_eq!(
            calc_sleep_time(&history_model, mock_date),
            Duration::from_millis(0)
        );
    }
}
