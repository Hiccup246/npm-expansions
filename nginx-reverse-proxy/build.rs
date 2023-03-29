use std::fs;
use std::path::{PathBuf};

use npm_expansions_static::builder_utilities;

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