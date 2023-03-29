extern crate minifier;
extern crate minify_html;
mod asset_minifier;

use std::fs;
use std::path::PathBuf;

fn main() {
    let pages_dir_input = fs::read_dir("pages").unwrap();
    let pages_dir_ouput_path: PathBuf = PathBuf::from("minified_pages/");

    if !pages_dir_ouput_path.exists() {
        fs::create_dir(&pages_dir_ouput_path).unwrap();
    }

    asset_minifier::copy_pages_directory(pages_dir_input, &pages_dir_ouput_path);

    let static_dir_input = fs::read_dir("static").unwrap();
    let static_dir_ouput_path: PathBuf = PathBuf::from("minified_static/");

    if !static_dir_ouput_path.exists() {
        fs::create_dir(&static_dir_ouput_path).unwrap();
    }

    asset_minifier::copy_static_directory(static_dir_input, &static_dir_ouput_path);
}
