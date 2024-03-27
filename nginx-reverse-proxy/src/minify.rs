use std::{
    fs,
    io::{Error, ErrorKind},
    path::PathBuf,
};
extern crate minifier;
extern crate minify_html;

/// Performs minifcation on all html, css and js files within a directory tree and copies them
/// to output directoy
///
/// # Arguments
///
/// * `input_path` - A directory whoose contents will be minified
/// * `output_path` - A directory who will contain the produced minified content
///
/// # Panics
///
/// The function panics if any of the input directories or files cannot be copied to the output
/// directory or minified
///
pub fn minify_drectory(input_path: PathBuf, output_path: PathBuf) -> Result<(), std::io::Error> {
    if !output_path.exists() {
        fs::create_dir(&output_path).unwrap();
    }

    for dir_entry_result in input_path.read_dir()? {
        let entry = dir_entry_result?;
        let file_type = entry.file_type()?;
        let file_name = entry.file_name();

        if file_type.is_file() {
            let minifed_file_contents = minify_file_contents(entry.path())?;
            fs::write(&output_path.join(file_name), minifed_file_contents)?
        } else if file_type.is_dir() {
            let output_path_with_dir = output_path.join(file_name);
            minify_drectory(entry.path(), output_path_with_dir)?
        }
    }

    Ok(())
}

/// Performs minifcation of a files content if the file is html, css or js and returns the minified
/// content as a vector of bytes
///
/// # Arguments
///
/// * `input_file` - A file to be minified
///
/// # Panics
///
/// The function panics if the file cannot be read or contains html, css or js that cannot be minified
///
pub fn minify_file_contents(input_file: PathBuf) -> Result<Vec<u8>, std::io::Error> {
    let extension = input_file.extension().unwrap();

    match extension.to_str().unwrap() {
        "html" => {
            let file = fs::read(input_file)?;

            Ok(minify_html::minify(
                &file,
                &minify_html::Cfg::spec_compliant(),
            ))
        }
        "css" => {
            let file = fs::read_to_string(input_file)?;
            let minified =
                minifier::css::minify(&file).map_err(|_| Error::from(ErrorKind::InvalidInput))?;

            Ok(minified.to_string().as_bytes().to_vec())
        }
        "js" => {
            let file = fs::read_to_string(input_file)?;
            let minified = minifier::js::minify(&file);

            Ok(minified.to_string().as_bytes().to_vec())
        }
        _ => Ok(fs::read(input_file).unwrap()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::Builder;

    mod minify_directory_tests {
        use super::*;

        #[test]
        fn minify_directory_with_sub_directories() {
            let input_dir = Builder::new().prefix("example_input").tempdir().unwrap();
            let output_dir = Builder::new().prefix("example_output").tempdir().unwrap();
            let directory = input_dir.path().join("example_dir");

            fs::create_dir(&directory).unwrap();
            fs::write(
                &directory.as_path().join("example.js"),
                b"const test = \"Hello World!\"",
            )
            .unwrap();

            minify_drectory(
                input_dir.path().to_path_buf(),
                output_dir.path().to_path_buf(),
            )
            .unwrap();

            assert!(output_dir.path().join("example_dir/example.js").exists())
        }

        #[test]
        fn minify_directory_with_only_files() {
            let input_dir = Builder::new().prefix("example_input").tempdir().unwrap();
            let output_dir = Builder::new().prefix("example_output").tempdir().unwrap();

            fs::write(
                input_dir.path().join("example.css"),
                b".hello { color: red; }",
            )
            .unwrap();
            minify_drectory(
                input_dir.path().to_path_buf(),
                output_dir.path().to_path_buf(),
            )
            .unwrap();

            assert!(output_dir.path().join("example.css").exists())
        }
    }

    mod minify_file_contents_tests {
        use super::*;

        #[test]
        fn minifies_a_css_file() {
            let named_tempfile = Builder::new()
                .prefix("example")
                .suffix(".css")
                .tempfile()
                .unwrap();

            fs::write(&named_tempfile, ".class {{ color: red; }}").unwrap();

            let pre_minification_length = fs::read(&named_tempfile).unwrap().len();
            let minified_contents =
                minify_file_contents(named_tempfile.path().to_path_buf()).unwrap();

            assert!(pre_minification_length > minified_contents.len());
        }

        #[test]
        fn minifies_a_html_file() {
            let named_tempfile = Builder::new()
                .prefix("example")
                .suffix(".html")
                .tempfile()
                .unwrap();

            fs::write(&named_tempfile, "<a>Hello</a>   <div>World!</div>").unwrap();

            let pre_minification_length = fs::read(&named_tempfile).unwrap().len();
            let minified_contents =
                minify_file_contents(named_tempfile.path().to_path_buf()).unwrap();

            assert!(pre_minification_length > minified_contents.len());
        }

        #[test]
        fn minifies_a_js_file() {
            let named_tempfile = Builder::new()
                .prefix("example")
                .suffix(".js")
                .tempfile()
                .unwrap();

            fs::write(&named_tempfile, "const a = \"hello\"   const b = \"world\"").unwrap();

            let pre_minification_length = fs::read(&named_tempfile).unwrap().len();
            let minified_contents =
                minify_file_contents(named_tempfile.path().to_path_buf()).unwrap();

            assert!(pre_minification_length > minified_contents.len());
        }

        #[test]
        fn does_not_minify_txt_file() {
            let named_tempfile = Builder::new()
                .prefix("example")
                .suffix(".txt")
                .tempfile()
                .unwrap();

            fs::write(&named_tempfile, "hello   this is a text file").unwrap();

            let pre_minification_length = fs::read(&named_tempfile).unwrap().len();
            let minified_contents =
                minify_file_contents(named_tempfile.path().to_path_buf()).unwrap();

            assert!(pre_minification_length == minified_contents.len());
        }
    }
}
