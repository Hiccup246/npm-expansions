use std::fs::File;
// Example custom build script.
fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=pages");

    File::create("target/hello-builder.txt").unwrap();
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
