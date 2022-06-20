extern crate compiletest_rs as compiletest;
use compiletest::common::Mode;
use std::path::PathBuf;

#[test]
fn compile_test() {
    compiletest::run_tests(&compiletest::Config {
        mode: Mode::CompileFail,
        src_base: PathBuf::from("compile-fail"),
        target_rustcflags: Some("--edition 2018".to_owned()),
        .. compiletest::Config::default()
    });
}
