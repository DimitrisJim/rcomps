extern crate compiletest_rs as compiletest;

use std::path::PathBuf;

fn run_mode(mode: &'static str, custom_dir: Option<&'static str>) {
    let mut config = compiletest::Config::default();
    let cfg_mode = mode.parse().expect("Invalid mode");

    config.mode = cfg_mode;

    let dir = custom_dir.unwrap_or(mode);
    config.src_base = PathBuf::from(format!("tests/{}", dir));
    config.link_deps();

    // NOTE: clean_rmeta seems to trigger a recompilation each time
    // cargo test is executed.
    // TODO: Is it necessary?
    // config.clean_rmeta();

    compiletest::run_tests(&config);
}

// List tests run by compiletest_rs
#[test]
fn compile_test() {
    run_mode("compile-fail", None);
    run_mode("ui", None);
}
