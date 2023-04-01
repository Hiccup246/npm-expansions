use npm_expansions_static::asset_minifier;
use std::path::PathBuf;

fn main() {
    asset_minifier::minify_drectory(PathBuf::from("pages"), PathBuf::from("minified_pages"))
        .unwrap();
    asset_minifier::minify_drectory(PathBuf::from("static"), PathBuf::from("minified_static"))
        .unwrap();
}
