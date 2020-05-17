# Test case for compiletest #219

If I run `TERM=dumb cargo test; echo $?` in this repo, compiletest complains
about unexpected failures in `compile-fail/fail.rs`, but then the command exits
with a success status.

```
$ TERM=dumb cargo test
    Finished test [unoptimized + debuginfo] target(s) in 0.04s
     Running target/debug/deps/testcase-eb91742bc7d596e4

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/runtests-4ffd32f600a544a7

running 1 test

running 1 test

error: compile-fail/fail.rs:1: unexpected error: '1:10: 1:11: expected `;` or `{`, found `<eof>`'

error: compile-fail/fail.rs:1: expected error not found: slurve

error: 1 unexpected errors found, 1 expected errors not found
status: exit code: 1
command: "rustc" "compile-fail/fail.rs" "-L" "/tmp" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/fail.stage-id" "-A" "unused" "--edition" "2018" "-L" "/tmp/fail.stage-id.aux"
unexpected errors (from JSON output): [
    Error {
        line_num: 1,
        kind: Some(
            Error,
        ),
        msg: "1:10: 1:11: expected `;` or `{`, found `<eof>`",
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

thread '[compile-fail] compile-fail/fail.rs' panicked at 'explicit panic', /home/jimb/.cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.5.0/src/runtest.rs:1091:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test [compile-fail] compile-fail/fail.rs ... test compile_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ echo $?
0
$
```
