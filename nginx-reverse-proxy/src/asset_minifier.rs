use std::{fs, path::Path, io::Error, io::ErrorKind, path::PathBuf};
extern crate minifier;
extern crate minify_html;

pub fn copy_static_directory(input_dir: fs::ReadDir, output_dir: &Path) {
    input_dir
        .map(|dir_entry| dir_entry.unwrap())
        .for_each(|dir_entry| {
            if dir_entry.file_type().unwrap().is_file() {
                let new_file_name = output_dir.join(dir_entry.file_name());
                copy_file(&dir_entry.path(), &new_file_name);
            }
        });
}

pub fn minify_drectory(input_path: PathBuf, output_path: PathBuf) -> Result<(), std::io::Error> {
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

pub fn minify_file_contents(input_file: PathBuf) -> Result<Vec<u8>, std::io::Error> {
    let extension = input_file.extension().unwrap();

    match extension.to_str().unwrap() {
        "html" => {
            let file = fs::read(input_file)?;
            
            return Ok(minify_html::minify(&file, &minify_html::Cfg::new()));
        }
        "css" => {
            let file = fs::read_to_string(input_file)?;
            let minified = minifier::css::minify(&file).map_err(|_| Error::from(ErrorKind::InvalidInput))?;
            
            return Ok(minified.to_string().as_bytes().to_vec());
        }
        "js" => {
            let file = fs::read_to_string(input_file)?;
            let minified = minifier::js::minify(&file);
            
            return Ok(minified.to_string().as_bytes().to_vec())
        }
        _ => {
            return Ok(fs::read(input_file).unwrap());
        }
    };
}



pub fn copy_pages_directory(directory: fs::ReadDir, output_dir: &Path) {
    directory
        .map(|dir_entry| dir_entry.unwrap())
        .for_each(|dir_entry| {
            let file_type = dir_entry.file_type().unwrap();

            if file_type.is_file() {
                let new_file_name = output_dir.join(dir_entry.file_name());
                copy_file(&dir_entry.path(), &new_file_name);
            } else if file_type.is_dir() {
                let new_directory_path = output_dir.join(dir_entry.file_name());

                if !new_directory_path.exists() {
                    fs::create_dir(new_directory_path).unwrap();
                }

                copy_pages_directory(
                    fs::read_dir(dir_entry.path()).unwrap(),
                    &output_dir.join(dir_entry.file_name()),
                )
            }
        });
}

fn copy_file(from: &Path, to: &Path) {
    let extension = from.extension().unwrap();

    match extension.to_str().unwrap() {
        "html" => {
            let file = fs::read(from).unwrap();
            let minified = minify_html::minify(&file, &minify_html::Cfg::new());

            fs::File::create(to).unwrap();
            fs::write(to, minified).unwrap();
        }
        "css" => {
            let file = fs::read_to_string(from).unwrap();
            let minified = minifier::css::minify(&file).unwrap();

            fs::File::create(to).unwrap();
            fs::write(to, minified.to_string().as_bytes()).unwrap();
        }
        "js" => {
            let file = fs::read_to_string(from).unwrap();
            let minified = minifier::js::minify(&file).to_string();

            fs::File::create(to).unwrap();
            fs::write(to, minified.as_bytes()).unwrap();
        }
        _ => {
            fs::File::create(to).unwrap();
            fs::copy(from, to).unwrap();
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    mod minify_directory_tests {
        // Create output directory as tmp directory
        // Create input directory as tmp directory with files
        // Check that after action output directory contains correct paths and files
    }

    mod minify_file_contents_tests {
        extern crate tempfile;

        use super::*;
        use std::io::Write;
        use tempfile::Builder;

        #[test]
        fn minifies_a_css_file() {
            let named_tempfile = Builder::new()
            .prefix("example")
            .suffix(".css")
            .tempfile().unwrap();

            write!(&named_tempfile, ".class {{ color: red; }}").unwrap();

            let pre_minification_length = fs::read(&named_tempfile).unwrap().len();
            let minified_contents = minify_file_contents(named_tempfile.path().to_path_buf()).unwrap();

            assert!(pre_minification_length > minified_contents.len());
        }

        #[test]
        fn minifies_a_html_file() {
            let named_tempfile = Builder::new()
            .prefix("example")
            .suffix(".html")
            .tempfile().unwrap();

            write!(&named_tempfile, "<a>Hello</a>   <div>World!</div>").unwrap();

            let pre_minification_length = fs::read(&named_tempfile).unwrap().len();
            let minified_contents = minify_file_contents(named_tempfile.path().to_path_buf()).unwrap();

            assert!(pre_minification_length > minified_contents.len());
        }

        #[test]
        fn minifies_a_js_file() {
            let named_tempfile = Builder::new()
            .prefix("example")
            .suffix(".js")
            .tempfile().unwrap();

            write!(&named_tempfile, "const a = \"hello\"   const b = \"world\"").unwrap();

            let pre_minification_length = fs::read(&named_tempfile).unwrap().len();
            let minified_contents = minify_file_contents(named_tempfile.path().to_path_buf()).unwrap();

            assert!(pre_minification_length > minified_contents.len());
        }

        #[test]
        fn does_not_minify_txt_file() {
            let named_tempfile = Builder::new()
            .prefix("example")
            .suffix(".txt")
            .tempfile().unwrap();

            write!(&named_tempfile, "hello   this is a text file").unwrap();

            let pre_minification_length = fs::read(&named_tempfile).unwrap().len();
            let minified_contents = minify_file_contents(named_tempfile.path().to_path_buf()).unwrap();

            assert!(pre_minification_length == minified_contents.len());
        }
    }
}