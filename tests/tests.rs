extern crate compiletest_rs as compiletest;

use std::path::PathBuf;

fn run_mode(mode: &'static str, custom_dir: Option<&'static str>) {
    // NOTES:
    // - clean_rmeta seems to trigger a recompilation each time
    // - link_deps fails on macos. see:
    // https://github.com/laumann/compiletest-rs/issues/179
    // config.link_deps();

    let mut config = compiletest::Config::default();
    let cfg_mode = mode.parse().expect("Invalid mode");

    config.mode = cfg_mode;

    let dir = custom_dir.unwrap_or(mode);
    config.src_base = PathBuf::from(format!("tests/{}", dir));
    config.target_rustcflags = Some("-L target/debug/deps".to_string());
    compiletest::run_tests(&config);
}

// List tests run by compiletest_rs
#[test]
fn compile_test() {
    run_mode("compile-fail", None);
    run_mode("ui", None);
}
