# Test case for compiletest #219

Setting the `TERM` environment variable to `dumb` causes `compiletest`
not to report missing errors that the tests expect to be there.

If I run `TERM=dumb cargo test; echo $?` in this repo, compiletest
claims all tests have succeeded. If I leave `TERM` set to
`xterm-256color`, then the tests run and detect the missing error
message, as they should.

## Things are broken with TERM=dumb

If set `TERM` to `dumb`, then the tests pass, and `cargo test` exits
with a zero status:

```
$ TERM=dumb cargo test; echo $?
    Finished test [unoptimized + debuginfo] target(s) in 0.01s
     Running unittests src/main.rs (target/debug/deps/testcase-5fb9ff995428c77d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/runtests.rs (target/debug/deps/runtests-16f8ff5e2b5c4843)

running 1 test

running 1 test
test [compile-fail] compile-fail/fail.rs ... test compile_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.09s

0
$ 
```

If I leave `TERM` set to `xterm-256color`, then the tests fail, and
`cargo test` exits with an error status:

```
$ cargo test; echo $?
    Finished test [unoptimized + debuginfo] target(s) in 0.01s
     Running unittests src/main.rs (target/debug/deps/testcase-5fb9ff995428c77d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/runtests.rs (target/debug/deps/runtests-16f8ff5e2b5c4843)

running 1 test

running 1 test
test [compile-fail] compile-fail/fail.rs ... FAILED

failures:

failures:
    [compile-fail] compile-fail/fail.rs

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s

test compile_test ... FAILED

failures:

---- compile_test stdout ----

error: compile-fail/fail.rs:1: unexpected error: '1:10: 1:11: expected one of `->`, `where`, or `{`, found `<eof>`'

error: compile-fail/fail.rs:1: expected error not found: slurve

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
command: "rustc" "compile-fail/fail.rs" "-L" "/tmp" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/fail.stage-id" "-A" "unused" "--edition" "2018" "-L" "/tmp/fail.stage-id.aux"
unexpected errors (from JSON output): [
    Error {
        line_num: 1,
        kind: Some(
            Error,
        ),
        msg: "1:10: 1:11: expected one of `->`, `where`, or `{`, found `<eof>`",
    },
]

not found errors (from test file): [
    Error {
        line_num: 1,
        kind: Some(
            Error,
        ),
        msg: "slurve",
    },
]

thread '[compile-fail] compile-fail/fail.rs' panicked at 'explicit panic', /home/jimb/.cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.8.0/src/runtest.rs:1092:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'compile_test' panicked at 'Some tests failed', /home/jimb/.cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.8.0/src/lib.rs:111:22


failures:
    compile_test

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.09s

error: test failed, to rerun pass '--test runtests'
101
$ 
```
