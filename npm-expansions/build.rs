use std::fs::File;
// Example custom build script.
fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=pages");

    File::create("target/hello-builder.txt").unwrap();
    // We have two key choices
    // 1. Macros which imbed the code and uglifies it at compile time
    //   - Hashmap access i.e. Assets::get("pages/index.html");
    //   - Const access i.e. INDEX_PAGE
    // 2. Simply imbed assets when we need them in each controller and uglify at runtime
    //   - include_bytes!("pages/index.html");
    // 3. Build time commands
    //   - Cargo builds and uglifies at run time and stores files in target
    //
    // Benefits and Drawbacks
    // 1. Binary works standalone without any extra files
    // 2. Extra run time compilation
    // 3. Binary cannot work standalone
}
