use std::fs::{DirEntry, ReadDir, File};
// Example custom build script.
use std::fs;
use std::path::{Path, PathBuf};
use minify_html::{Cfg, minify};
use minifier::css::minify as css_mimifier;
use minifier::js::minify as js_mimifier;

fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
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
    // File::create("target/hello-builder.txt").unwrap();

    // We have two key choices
    // 1. Macros which imbed the code
    //   - Hashmap access i.e. Assets::get("pages/index.html");
    //   - Const access i.e. INDEX_PAGE
    // 2. Simply imbed assets when we need them in each controller
    //   - include_bytes!("pages/index.html");
    // 3. Build time commands
    //   - Cargo builds and uglifies at run time and stores files in target
    //
    // Benefits and Drawbacks
    // 1. Access is standadised (uglification at run or compile time)
    // 2. Access on a per file basis (uglification at run or compile time)
    // 3. Binary cannot work standalone

    // 1 and 2 are identical in all aspects except access
    // Either way we will have to uglify at build time so we should just do that
}

fn copy_static_directory(input_dir: fs::ReadDir, output_dir: &PathBuf) {
    for dir_entry in input_dir {
        if let Ok(dir) = dir_entry {
            if dir.file_type().unwrap().is_file() {
                let new_file_name = output_dir.as_path().join(dir.file_name());
                fs::copy(dir.path(), new_file_name).unwrap();
            }
        }
    }
}

fn copy_pages_directory(dir: ReadDir, output_dir: &PathBuf) {
    for dir_entry in dir {
        if let Ok(dir) = dir_entry {
            if dir.file_type().unwrap().is_file() {
                let new_file_name = output_dir.as_path().join(dir.file_name());
                copy_file(&dir.path(), &new_file_name);
                // fs::copy(dir.path(), new_file_name).unwrap();
            } else if dir.file_type().unwrap().is_dir() {
                if !output_dir.as_path().join(dir.file_name()).exists() {
                    fs::create_dir(output_dir.as_path().join(dir.file_name())).unwrap();
                }
                copy_pages_directory(
                    fs::read_dir(dir.path()).unwrap(),
                    &output_dir.as_path().join(dir.file_name()),
                )
            }
        }
    }
}

fn copy_file(from: &Path, to: &Path) {
    File::create(to).unwrap();

    if from.extension().unwrap() == "html" {
        let file = fs::read(from).unwrap();
        let config = Cfg::new();
        let minified = minify(&file, &config);
        File::create(to).unwrap();
        fs::write(to, minified).unwrap();
    } else if from.extension().unwrap() == "css" {
        let file = fs::read_to_string(from).unwrap();
        let minified = css_mimifier(&file).unwrap();
        File::create(to).unwrap();
        fs::write(to, minified.to_string().as_bytes()).unwrap();
    } else if from.extension().unwrap() == "js" {
        let file = fs::read_to_string(from).unwrap();
        let minified = js_mimifier(&file).to_string();
        File::create(to).unwrap();
        fs::write(to, minified.as_bytes()).unwrap();
    } else {
        fs::copy(from, to).unwrap();
    }
}

fn minify_css() {

}