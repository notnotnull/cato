use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

fn temp_file_path(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time must be after unix epoch")
        .as_nanos();

    std::env::temp_dir().join(format!("cato-test-{}-{}", name, nanos))
}

#[test]
fn invalid_file_path() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cato")?;

    cmd.arg("does-not-exist.txt");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Unable to read file"));

    Ok(())
}

#[test]
fn stdin_then_file_with_numbering() -> Result<(), Box<dyn std::error::Error>> {
    let path = temp_file_path("numbering");
    fs::write(&path, "R1\nR2\n")?;

    let mut cmd = Command::cargo_bin("cato")?;
    cmd.args(["-n", "-"])
        .arg(&path)
        .write_stdin("X\nY\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("    1  X\n"))
        .stdout(predicate::str::contains("    2  Y\n"))
        .stdout(predicate::str::contains("    3  R1\n"))
        .stdout(predicate::str::contains("    4  R2\n"));

    let _ = fs::remove_file(path);
    Ok(())
}

#[test]
fn no_args_reads_stdin() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cato")?;

    cmd.write_stdin("A\nB\n")
        .assert()
        .success()
        .stdout(predicate::eq("A\nB\n"));

    Ok(())
}

#[test]
fn number_nonblank_only() -> Result<(), Box<dyn std::error::Error>> {
    let path = temp_file_path("number-nonblank");
    fs::write(&path, "A\n\nB\n")?;

    let mut cmd = Command::cargo_bin("cato")?;
    cmd.arg("-b")
        .arg(&path)
        .assert()
        .success()
        .stdout(predicate::eq("    1  A\n\n    2  B\n"));

    let _ = fs::remove_file(path);
    Ok(())
}

#[test]
fn number_nonblank_overrides_number_all() -> Result<(), Box<dyn std::error::Error>> {
    let path = temp_file_path("number-precedence");
    fs::write(&path, "A\n\nB\n")?;

    let mut cmd = Command::cargo_bin("cato")?;
    cmd.args(["-b", "-n"])
        .arg(&path)
        .assert()
        .success()
        .stdout(predicate::eq("    1  A\n\n    2  B\n"));

    let _ = fs::remove_file(path);
    Ok(())
}

#[test]
fn show_ends() -> Result<(), Box<dyn std::error::Error>> {
    let path = temp_file_path("show-ends");
    fs::write(&path, "A\n\nB\n")?;

    let mut cmd = Command::cargo_bin("cato")?;
    cmd.arg("-E")
        .arg(&path)
        .assert()
        .success()
        .stdout(predicate::eq("A$\n$\nB$\n"));

    let _ = fs::remove_file(path);
    Ok(())
}

#[test]
fn show_ends_with_n_option() -> Result<(), Box<dyn std::error::Error>> {
    let path = temp_file_path("show-ends-numbered");
    fs::write(&path, "A\n\nB\n")?;

    let mut cmd = Command::cargo_bin("cato")?;
    cmd.args(["-n", "-E"])
        .arg(&path)
        .assert()
        .success()
        .stdout(predicate::eq("    1  A$\n    2  $\n    3  B$\n"));

    let _ = fs::remove_file(path);
    Ok(())
}

#[test]
fn show_ends_with_e_option() -> Result<(), Box<dyn std::error::Error>> {
    let path = temp_file_path("show-ends-number-nonblank");
    fs::write(&path, "A\n\nB\n")?;

    let mut cmd = Command::cargo_bin("cato")?;
    cmd.args(["-b", "-E"])
        .arg(&path)
        .assert()
        .success()
        .stdout(predicate::eq("    1  A$\n$\n    2  B$\n"));

    let _ = fs::remove_file(path);
    Ok(())
}

#[test]
fn squeeze_blank() -> Result<(), Box<dyn std::error::Error>> {
    let path = temp_file_path("squeeze-blank");
    fs::write(&path, "A\n\n\nB\n")?;

    let mut cmd = Command::cargo_bin("cato")?;
    cmd.args(["-s"])
        .arg(&path)
        .assert()
        .success()
        .stdout(predicate::eq("A\n\nB\n"));

    let _ = fs::remove_file(path);
    Ok(())
}
