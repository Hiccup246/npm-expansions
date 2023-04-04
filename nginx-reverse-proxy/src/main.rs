use asset_minifier::minify;
use std::path::PathBuf;

fn main() {
    minify::minify_drectory(PathBuf::from("pages"), PathBuf::from("minified_pages")).unwrap();
    minify::minify_drectory(PathBuf::from("static"), PathBuf::from("minified_static")).unwrap();
}
