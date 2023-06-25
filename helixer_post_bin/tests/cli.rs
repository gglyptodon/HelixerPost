use assert_cmd::Command;
use predicates::prelude::*;
use std::borrow::Cow;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

const PRG: &str = "helixer_post_bin";

#[cfg(not(windows))]
fn format_file_name(expected_file: &str) -> Cow<str> {
    expected_file.into()
}

fn run_fail_contains(args: &[&str], expected_stdout: &str, expected_stderr: &str) -> TestResult {
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected_stderr))
        .stdout(predicate::str::contains(expected_stdout));
    Ok(())
}

fn run(args: &[&str], observed_file: &str, expected_file: &str) -> TestResult {
    let exp_file = format_file_name(expected_file);
    if let Ok(exp_content) = fs::read_to_string(exp_file.as_ref()) {
        println!("exp_content {:?}", exp_content);
        let cmd = Command::cargo_bin(PRG)?.args(args).assert().success();
        let out = cmd.get_output();
        let _stdout = String::from_utf8(out.stdout.clone())?;
        let obs_file = format_file_name(observed_file);
        if let Ok(obs_content) = fs::read_to_string(obs_file.as_ref()) {
            assert_eq!(exp_content, obs_content);
        } else {
            return Err(format!("No output found here: {}", obs_file).into());
        }
    } else {
        return Err(format!("Error while opening {}.", exp_file).into());
    }
    Ok(())
}

//cli
#[test]
fn test_dies() -> TestResult {
    //let _expected_todo_stderr = "ERROR DESCRIPTION";
    let expected_stderr = "";
    let expected_stdout = "HelixerPost <genome.h5> <predictions.h5> <windowSize> <edgeThresh> <peakThresh> <minCodingLength> <gff>\n";
    run_fail_contains(&[""], expected_stdout, expected_stderr)
}

#[test]
fn test_example() -> TestResult {
    let expected_file = "tests/example/expected_output.gff3";
    let observed_file = "tests/example/out/test_output.gff3";
    run(
        &[
            "tests/example/genome_data.h5",
            "tests/example/predictions.h5",
            "100",
            "0.1",
            "0.8",
            "60",
            observed_file,
        ],
        observed_file,
        expected_file,
    )
}
