use std::collections::HashMap;
use std::{fs, io, path::PathBuf};

/// Injects environment variables into all html files within a directory tree and copies them
/// to output directoy
///
/// # Arguments
///
/// * `input_path` - A directory whoose contents will have environment variables injected
/// * `output_path` - A directory who will contain the produced environment variabled injected content
/// * `env_variables` A hashmap of environment variables
///
/// # Panics
///
/// The function panics if any of the input directories or files cannot be copied to the output
/// directory or minified
///
pub fn inject_envs_into_drectory(
    input_path: PathBuf,
    output_path: PathBuf,
    env_variables: &HashMap<String, String>,
) -> Result<(), io::Error> {
    if !output_path.exists() {
        fs::create_dir(&output_path).unwrap();
    }

    for dir_entry_result in input_path.read_dir()? {
        let entry = dir_entry_result?;
        let file_type = entry.file_type()?;
        let file_name = entry.file_name();

        if file_type.is_file() {
            let minifed_file_contents = inject_envs_into_file(entry.path(), env_variables)?;
            fs::write(&output_path.join(file_name), minifed_file_contents)?
        } else if file_type.is_dir() {
            let output_path_with_dir = output_path.join(file_name);
            inject_envs_into_drectory(entry.path(), output_path_with_dir, env_variables)?
        }
    }

    Ok(())
}

/// Injects environment variables into a files content if the file is html and returns the injected file
/// content as a vector of bytes
///
/// # Arguments
///
/// * `input_file` - A file to have environment variables injected
/// * `env_variables` A hashmap of environment variables
///
/// # Panics
///
/// The function panics if the file cannot be read or contains html or js that cannot have environment variables injected
///
pub fn inject_envs_into_file(
    input_file: PathBuf,
    env_variables: &HashMap<String, String>,
) -> Result<Vec<u8>, io::Error> {
    let extension = input_file.extension().unwrap().to_str().unwrap();
    let supported_extensions = ["html"];
    if supported_extensions.contains(&extension) {
        let mut file = fs::read_to_string(input_file)?;

        for (key, val) in env_variables.iter() {
            let env_key = format!("{{{{ ${} }}}}", key);
            let env_val = format!("\"{}\"", val);

            file = file.replace(env_key.as_str(), env_val.as_str());
        }

        Ok(file.as_bytes().to_vec())
    } else {
        Ok(fs::read(input_file).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::Builder;

    mod inject_envs_into_drectory_tests {
        use super::*;

        #[test]
        fn inject_directory_with_sub_directories() {
            let input_dir = Builder::new().prefix("example_input").tempdir().unwrap();
            let output_dir = Builder::new().prefix("example_output").tempdir().unwrap();
            let directory = input_dir.path().join("example_dir");
            let env_variables = HashMap::from([("TRACKING_TAG".to_string(), "abc".to_string())]);

            fs::create_dir(&directory).unwrap();
            fs::write(
                &directory.as_path().join("example.html"),
                b"Hello World! TRACKING_TAG  {{ $TRACKING_TAG }}",
            )
            .unwrap();

            inject_envs_into_drectory(
                input_dir.path().to_path_buf(),
                output_dir.path().to_path_buf(),
                &env_variables,
            )
            .unwrap();

            assert!(output_dir.path().join("example_dir/example.html").exists())
        }

        #[test]
        fn inject_directory_with_only_files() {
            let input_dir = Builder::new().prefix("example_input").tempdir().unwrap();
            let output_dir = Builder::new().prefix("example_output").tempdir().unwrap();
            let env_variables = HashMap::from([("TRACKING_TAG".to_string(), "1234".to_string())]);

            fs::write(
                input_dir.path().join("example.html"),
                b"<div> TRACKING_TAG  {{ $TRACKING_TAG }} </div>",
            )
            .unwrap();
            inject_envs_into_drectory(
                input_dir.path().to_path_buf(),
                output_dir.path().to_path_buf(),
                &env_variables,
            )
            .unwrap();

            assert!(output_dir.path().join("example.html").exists())
        }
    }

    mod inject_envs_into_file_tests {
        use super::*;
        use std::str;

        #[test]
        fn injects_into_html() {
            let named_tempfile = Builder::new()
                .prefix("example")
                .suffix(".html")
                .tempfile()
                .unwrap();
            let env_variables = HashMap::from([("DATABASE_URL".to_string(), "1234".to_string())]);

            fs::write(
                &named_tempfile,
                "<a>Hello</a>   <div>{{ $DATABASE_URL }}</div>",
            )
            .unwrap();

            let injected_contents =
                inject_envs_into_file(named_tempfile.path().to_path_buf(), &env_variables).unwrap();
            let converted_string = str::from_utf8(injected_contents.as_slice())
                .unwrap()
                .to_string();

            assert!(converted_string.find("<div>\"1234\"</div>").is_some());
        }

        #[test]
        fn does_not_inject_into_css() {
            let named_tempfile = Builder::new()
                .prefix("example")
                .suffix(".css")
                .tempfile()
                .unwrap();
            let env_variables = HashMap::from([("DATABASE_URL".to_string(), "1234".to_string())]);

            fs::write(&named_tempfile, ".class {{ $DATABASE_URL }}").unwrap();

            let pre_injection_length = fs::read(&named_tempfile).unwrap().len();
            let injected_contents =
                inject_envs_into_file(named_tempfile.path().to_path_buf(), &env_variables).unwrap();

            assert!(pre_injection_length == injected_contents.len());
        }

        #[test]
        fn does_not_inject_into_js() {
            let named_tempfile = Builder::new()
                .prefix("example")
                .suffix(".js")
                .tempfile()
                .unwrap();
            let env_variables = HashMap::from([("DATABASE_URL".to_string(), "1234".to_string())]);

            fs::write(
                &named_tempfile,
                "const a = \"hello\" {{ $DATABASE_URL }}  const b = \"world\"",
            )
            .unwrap();

            let pre_injection_length = fs::read(&named_tempfile).unwrap().len();
            let injected_contents =
                inject_envs_into_file(named_tempfile.path().to_path_buf(), &env_variables).unwrap();

            assert!(pre_injection_length == injected_contents.len());
        }

        #[test]
        fn does_not_inject_into_txt() {
            let named_tempfile = Builder::new()
                .prefix("example")
                .suffix(".txt")
                .tempfile()
                .unwrap();
            let env_variables = HashMap::from([("DATABASE_URL".to_string(), "1234".to_string())]);

            fs::write(
                &named_tempfile,
                "hello   {{ $DATABASE_URL }} this is a text file",
            )
            .unwrap();

            let pre_injection_length = fs::read(&named_tempfile).unwrap().len();
            let injected_contents =
                inject_envs_into_file(named_tempfile.path().to_path_buf(), &env_variables).unwrap();

            assert!(pre_injection_length == injected_contents.len());
        }
    }
}
