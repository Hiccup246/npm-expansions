use std::fs;
use std::fs::{File, ReadDir};
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=pages");

    let pages_dir_input = fs::read_dir("pages").unwrap();
    let pages_dir_ouput_path: PathBuf = PathBuf::from("target/pages/");

    if !pages_dir_ouput_path.exists() {
        fs::create_dir(&pages_dir_ouput_path).unwrap();
    }

    copy_pages_directory(pages_dir_input, &pages_dir_ouput_path);

    println!("cargo:rerun-if-changed=static");

    let static_dir_input = fs::read_dir("static").unwrap();
    let static_dir_ouput_path: PathBuf = PathBuf::from("target/static/");

    if !static_dir_ouput_path.exists() {
        fs::create_dir(&static_dir_ouput_path).unwrap();
    }

    copy_static_directory(static_dir_input, &static_dir_ouput_path);
}

fn copy_static_directory(input_dir: fs::ReadDir, output_dir: &Path) {
    input_dir
        .map(|dir_entry| dir_entry.unwrap())
        .for_each(|dir_entry| {
            if dir_entry.file_type().unwrap().is_file() {
                let new_file_name = output_dir.join(dir_entry.file_name());
                copy_file(&dir_entry.path(), &new_file_name);
            }
        });
}

fn copy_pages_directory(directory: ReadDir, output_dir: &Path) {
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

            File::create(to).unwrap();
            fs::write(to, minified).unwrap();
        }
        "css" => {
            let file = fs::read_to_string(from).unwrap();
            let minified = minifier::css::minify(&file).unwrap();

            File::create(to).unwrap();
            fs::write(to, minified.to_string().as_bytes()).unwrap();
        }
        "js" => {
            let file = fs::read_to_string(from).unwrap();
            let minified = minifier::js::minify(&file).to_string();

            File::create(to).unwrap();
            fs::write(to, minified.as_bytes()).unwrap();
        }
        _ => {
            File::create(to).unwrap();
            fs::copy(from, to).unwrap();
        }
    };
}