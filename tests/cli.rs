use assert_cmd::Command;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

const CMD: &str = "rqst";

#[test]
fn dies_no_args() -> TestResult {
    Command::cargo_bin(CMD)?
        .assert()
        .failure()
        .stderr(predicate::str::contains("error"));
    Ok(())
}
