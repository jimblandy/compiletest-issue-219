extern crate compiletest_rs as compiletest;
use compiletest::common::Mode;
use std::env;
use std::path::PathBuf;

#[test]
fn compile_test() {
    compiletest::run_tests(&compiletest::Config {
        mode: Mode::CompileFail,
        src_base: PathBuf::from("compile-fail"),
        target_rustcflags: Some("--edition 2018".to_owned()),
        filter: env::var::<&str>("TESTNAME").ok(),
        .. compiletest::Config::default()
    });
}
