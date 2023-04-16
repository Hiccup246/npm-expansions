use std::{fmt, fs, path::PathBuf, path::Path};
use chrono::TimeZone;
use chrono::Utc;

struct HistoryModel {
    history_file: PathBuf,
    pub history_entries: Vec<(chrono::DateTime<Utc>, String)>
}

#[derive(Debug)]
struct HistoryModelError {
    message: String
}

impl HistoryModelError {
    pub fn from(message: &str) -> HistoryModelError {
        HistoryModelError { message: message.to_string() }
    }
}

impl fmt::Display for HistoryModelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "HistoryModelError: {:?}",
            self.message
        )
    }
}

impl HistoryModel {
    pub fn new(history_file: &Path) -> HistoryModel {
        HistoryModel { history_file: history_file.to_path_buf(), history_entries: Vec::new() }
    }

    pub fn from(history_entries: Vec<(chrono::DateTime<Utc>, String)>) -> HistoryModel {
        HistoryModel { history_file: PathBuf::new(), history_entries: history_entries }
    }

    pub fn load_history(&mut self) -> Result<(), HistoryModelError> {
        let entries = fs::read_to_string(&self.history_file)
            .unwrap()
            .lines()
            .map(|entry| {
                let (date, pr_number) = entry.split_once(",").ok_or(HistoryModelError::from("Incorrect history file format."))?;

                if pr_number.is_empty() {
                    return Err(HistoryModelError::from("Incorrect history file format."));
                }
                
                Ok((chrono::Utc.datetime_from_str(&date, "%+").map_err(|err| HistoryModelError::from("Incorrect history file format."))?, pr_number.to_string()))
            })
            .collect::<Result<Vec<(chrono::DateTime<Utc>, String)>, HistoryModelError>>();


        self.history_entries = entries?;

        Ok(())
    }

    pub fn pr_numbers(&self) -> Vec<&String> {
        self.history_entries.iter().map(|entry| &entry.1).collect::<Vec<&String>>()
    }

    pub fn latest_entry(&self) -> Option<&(chrono::DateTime<Utc>, String)> {
        let mut entries = self.history_entries.clone();
        
        entries.sort_by(|a, b| a.cmp(b));

        self.history_entries.last()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::Builder;

    mod load_history {
        use super::*;

        #[test]
        fn valid_file() {
            let tmpfile = Builder::new()
                .prefix("history")
                .suffix(".txt")
                .tempfile()
                .unwrap();

            fs::write(tmpfile.path(), "2022-02-02T00:00:00+00:00,4302\r\n").unwrap();

            let mut model = HistoryModel::new(tmpfile.path());

            assert!(model.load_history().is_ok())
        }

        #[test]
        fn file_with_no_comma() {
            let tmpfile = Builder::new()
                .prefix("history")
                .suffix(".txt")
                .tempfile()
                .unwrap();

            fs::write(tmpfile.path(), "2022-02-02T00:00:00+00:004302\r\n").unwrap();

            let mut model = HistoryModel::new(tmpfile.path());

            assert!(model.load_history().is_err())
        }

        #[test]
        fn file_with_incorrect_date() {
            let tmpfile = Builder::new()
                .prefix("history")
                .suffix(".txt")
                .tempfile()
                .unwrap();

            fs::write(tmpfile.path(), "2022-02-02:00,4302\r\n").unwrap();

            let mut model = HistoryModel::new(tmpfile.path());

            assert!(model.load_history().is_err())
        }

        #[test]
        fn file_with_no_pr_number() {
            let tmpfile = Builder::new()
                .prefix("history")
                .suffix(".txt")
                .tempfile()
                .unwrap();

            fs::write(tmpfile.path(), "2022-02-02:00,\r\n").unwrap();

            let mut model = HistoryModel::new(tmpfile.path());

            assert!(model.load_history().is_err())
        }
    }

    mod pr_numbers {
        use super::*;

        #[test]
        fn returns_pr_numbers() {
            let model = HistoryModel::from(vec![(chrono::Utc::now(), "4301".to_string()), (chrono::Utc::now(), "4302".to_string())]);

            assert_eq!(model.pr_numbers(), vec![&"4301".to_string(), &"4302".to_string()])
        } 
    }

    mod latest_entry {
        use super::*;

        #[test]
        fn returns_correct_date() {
            let model = HistoryModel::from(
                vec![
                    (chrono::Utc.with_ymd_and_hms(2022, 2, 2, 0, 0, 0).unwrap(), "4301".to_string()),
                    (chrono::Utc.with_ymd_and_hms(2022, 2, 3, 0, 0, 0).unwrap(), "4302".to_string())
                ]
            );

            assert_eq!(model.latest_entry().unwrap().0, chrono::Utc.with_ymd_and_hms(2022, 2, 3, 0, 0, 0).unwrap())
        }

        #[test]
        fn returns_correct_pr_number() {
            let model = HistoryModel::from(
                vec![
                    (chrono::Utc.with_ymd_and_hms(2022, 2, 2, 0, 0, 0).unwrap(), "4301".to_string()),
                    (chrono::Utc.with_ymd_and_hms(2022, 2, 3, 0, 0, 0).unwrap(), "4302".to_string())
                ]
            );

            assert_eq!(model.latest_entry().unwrap().1, "4302")
        }
    }
}