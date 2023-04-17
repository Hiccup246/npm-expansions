use chrono::TimeZone;
use chrono::Utc;
use std::fs::OpenOptions;
use std::io::Write;
use std::{fmt, fs, io, path::Path, path::PathBuf};

type HistoryEntry = (chrono::DateTime<Utc>, String, String);

///
pub struct HistoryModel {
    history_file: PathBuf,
    ///
    pub history_entries: Vec<HistoryEntry>,
}

///
#[derive(Debug)]
pub struct HistoryModelError {
    message: String,
}

impl HistoryModelError {
    ///
    pub fn from(message: &str) -> HistoryModelError {
        HistoryModelError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for HistoryModelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HistoryModelError: {:?}", self.message)
    }
}

impl HistoryModel {
    ///
    pub fn new(history_file: &Path) -> HistoryModel {
        HistoryModel {
            history_file: history_file.to_path_buf(),
            history_entries: load_history(history_file).expect("Failed to parse history file"),
        }
    }

    ///
    pub fn from(history_entries: Vec<HistoryEntry>) -> HistoryModel {
        HistoryModel {
            history_file: PathBuf::new(),
            history_entries,
        }
    }

    ///
    pub fn pr_numbers(&self) -> Vec<&String> {
        self.history_entries
            .iter()
            .map(|entry| &entry.1)
            .collect::<Vec<&String>>()
    }

    ///
    pub fn latest_entry(&self) -> Option<&HistoryEntry> {
        let mut entries = self.history_entries.clone();

        entries.sort();

        self.history_entries.last()
    }

    ///
    pub fn update_history_file(
        &self,
        entry: (chrono::DateTime<Utc>, &str, &str),
    ) -> Result<(), io::Error> {
        let mut history_file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(self.history_file.as_path())?;

        writeln!(
            history_file,
            "{},{},{}",
            entry.0.format("%+"),
            entry.1,
            entry.2
        )?;

        Ok(())
    }

    ///
    pub fn reload(&mut self) -> Result<(), HistoryModelError> {
        self.history_entries = load_history(&self.history_file)?;

        Ok(())
    }
}

fn load_history(path: &Path) -> Result<Vec<HistoryEntry>, HistoryModelError> {
    let entries = fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|entry| {
            let split_entry: Vec<&str> = entry.split(',').collect();

            let date = split_entry.first().ok_or(HistoryModelError::from(
                "Incorrect history file format. Missing date in entry.",
            ))?;
            let pr_number = split_entry.get(1).ok_or(HistoryModelError::from(
                "Incorrect history file format. Missing pr number in entry.",
            ))?;
            let status = split_entry.get(2).ok_or(HistoryModelError::from(
                "Incorrect history file format. Missing status in entry.",
            ))?;

            Ok((
                chrono::Utc
                    .datetime_from_str(date, "%+")
                    .map_err(|_err| HistoryModelError::from("Incorrect history file format."))?,
                pr_number.to_string(),
                status.to_string(),
            ))
        })
        .collect::<Result<Vec<HistoryEntry>, HistoryModelError>>();

    entries
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::Builder;

    mod new {
        use super::*;

        #[test]
        fn valid_file() {
            let tmpfile = Builder::new()
                .prefix("history")
                .suffix(".txt")
                .tempfile()
                .unwrap();

            fs::write(tmpfile.path(), "2022-02-02T00:00:00+00:00,4302,success\r\n").unwrap();

            let model = HistoryModel::new(tmpfile.path());

            assert_eq!(
                model.history_entries,
                vec![(
                    chrono::Utc.with_ymd_and_hms(2022, 2, 2, 0, 0, 0).unwrap(),
                    "4302".to_string(),
                    "success".to_string(),
                )]
            );
        }

        #[test]
        #[should_panic(expected = "Failed to parse history file")]
        fn file_with_missing_comma() {
            let tmpfile = Builder::new()
                .prefix("history")
                .suffix(".txt")
                .tempfile()
                .unwrap();

            fs::write(tmpfile.path(), "2022-02-02T00:00:00+00:004302,success\r\n").unwrap();

            HistoryModel::new(tmpfile.path());
        }

        #[test]
        #[should_panic(expected = "Failed to parse history file")]
        fn file_with_incorrect_date() {
            let tmpfile = Builder::new()
                .prefix("history")
                .suffix(".txt")
                .tempfile()
                .unwrap();

            fs::write(tmpfile.path(), "2022-02-02:00,4302\r\n").unwrap();

            HistoryModel::new(tmpfile.path());
        }

        #[test]
        #[should_panic(expected = "Failed to parse history file")]
        fn file_with_no_pr_number() {
            let tmpfile = Builder::new()
                .prefix("history")
                .suffix(".txt")
                .tempfile()
                .unwrap();

            fs::write(tmpfile.path(), "2022-02-02:00,,success\r\n").unwrap();

            HistoryModel::new(tmpfile.path());
        }

        #[test]
        #[should_panic(expected = "Failed to parse history file")]
        fn file_with_no_status() {
            let tmpfile = Builder::new()
                .prefix("history")
                .suffix(".txt")
                .tempfile()
                .unwrap();

            fs::write(tmpfile.path(), "2022-02-02:00,4302,\r\n").unwrap();

            HistoryModel::new(tmpfile.path());
        }
    }

    mod pr_numbers {
        use super::*;

        #[test]
        fn returns_pr_numbers() {
            let model = HistoryModel::from(vec![
                (
                    chrono::Utc::now(),
                    "4301".to_string(),
                    "success".to_string(),
                ),
                (
                    chrono::Utc::now(),
                    "4302".to_string(),
                    "success".to_string(),
                ),
            ]);

            assert_eq!(
                model.pr_numbers(),
                vec![&"4301".to_string(), &"4302".to_string()]
            )
        }
    }

    mod latest_entry {
        use super::*;

        #[test]
        fn returns_correct_date() {
            let model = HistoryModel::from(vec![
                (
                    chrono::Utc.with_ymd_and_hms(2022, 2, 2, 0, 0, 0).unwrap(),
                    "4301".to_string(),
                    "success".to_string(),
                ),
                (
                    chrono::Utc.with_ymd_and_hms(2022, 2, 3, 0, 0, 0).unwrap(),
                    "4302".to_string(),
                    "success".to_string(),
                ),
            ]);

            assert_eq!(
                model.latest_entry().unwrap().0,
                chrono::Utc.with_ymd_and_hms(2022, 2, 3, 0, 0, 0).unwrap()
            )
        }

        #[test]
        fn returns_correct_pr_number() {
            let model = HistoryModel::from(vec![
                (
                    chrono::Utc.with_ymd_and_hms(2022, 2, 2, 0, 0, 0).unwrap(),
                    "4301".to_string(),
                    "success".to_string(),
                ),
                (
                    chrono::Utc.with_ymd_and_hms(2022, 2, 3, 0, 0, 0).unwrap(),
                    "4302".to_string(),
                    "success".to_string(),
                ),
            ]);

            assert_eq!(model.latest_entry().unwrap().1, "4302")
        }
    }

    mod update_history_file {
        use super::*;

        #[test]
        fn writes_new_pr_number() {
            let tmpfile = Builder::new()
                .prefix("history")
                .suffix(".txt")
                .tempfile()
                .unwrap();

            let model = HistoryModel::new(tmpfile.path());
            let datetime = chrono::Utc::now();

            fs::write(tmpfile.path(), "").unwrap();
            model
                .update_history_file((datetime, "4302", "failure"))
                .unwrap();

            let file = fs::read_to_string(tmpfile.path()).unwrap();

            assert_eq!(
                file.lines().last().unwrap(),
                format!("{},4302,failure", datetime.format("%+"))
            )
        }

        #[test]
        fn writes_multiple_pr_numbers() {
            let tmpfile = Builder::new()
                .prefix("history")
                .suffix(".txt")
                .tempfile()
                .unwrap();

            let model = HistoryModel::new(tmpfile.path());
            let datetime = chrono::Utc::now();

            fs::write(tmpfile.path(), "").unwrap();

            model
                .update_history_file((datetime, "4302", "success"))
                .unwrap();
            model
                .update_history_file((datetime, "4303", "failure"))
                .unwrap();

            let file = fs::read_to_string(tmpfile.path()).unwrap();

            assert_eq!(
                file,
                format!(
                    "{},4302,success\n{},4303,failure\n",
                    datetime.format("%+"),
                    datetime.format("%+")
                )
            )
        }
    }
}
