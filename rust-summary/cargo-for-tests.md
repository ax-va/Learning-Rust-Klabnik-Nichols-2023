# Cargo

## Cargo for tests

Compile code in test mode and run the resultant test binary to execute 
all the tests *in parallel* and capture output generated during test runs in the test results.

```unix
$ cargo test 
```

Some command line options go to `cargo test`, and some go to the resultant test binary:

- `cargo test --help` displays the options to use with `cargo test`;

- `cargo test -- --help` displays the options to use with the resultant test binary.

### Running tests in parallel or consecutively

By default, all the tests run in parallel. 
Set the number of threads to the test binary.

```unix
$ cargo test -- --test-threads=1
```

Running the tests using one thread will take longer than running them in parallel, 
but the tests will not interfere with each other if they share state.

### Showing function output

By default, the `println!` output will not be printed for passing tests.
If a test fails, we will see whatever was printed to standard output 
in the failed test with the rest of the failure message.

Set the option to see printed values for passing tests as well.

```unix
$ cargo test -- --show-output
```
