extern crate assert_cmd;

#[cfg(test)]
mod integration {
    use assert_cmd::Command;
    use predicates::prelude::*;
    use temp_dir::TempDir;

    #[test]
    fn calling_devicon_lookup_with_a_catchall_regex_works() {
        let mut cmd = Command::cargo_bin("devicon-lookup").unwrap();
        cmd.arg("--regex")
            .arg("(.*)")
            .write_stdin("test.rs".to_string())
            .assert()
            .stdout(" test.rs\n");
    }

    #[test]
    fn calling_devicon_lookup_with_grep_syntax_and_regex_works() {
        let mut cmd = Command::cargo_bin("devicon-lookup").unwrap();
        cmd.arg("--regex")
            .arg("^(.*):")
            .arg("-s")
            .write_stdin("test.rs: someline of the file".to_string())
            .assert()
            .stdout(" test.rs: someline of the file\n");
    }

    #[test]
    fn calling_devicon_lookup_with_zoxide_score_syntax_and_regex_works() {
        let d = TempDir::new().unwrap();
        let root_path = d.path().to_str().unwrap().to_string();
        std::fs::create_dir(d.path().join("one")).unwrap();
        std::fs::create_dir_all(d.path().join("foo").join("bar")).unwrap();

        let mut cmd = Command::cargo_bin("devicon-lookup").unwrap();
        cmd.current_dir(d.path())
            .arg("--regex")
            .arg(r"\d (.*)")
            .arg("-s")
            .write_stdin(format!(
                "  16.0 {root_path}/one
   8.0 {root_path}/foo
   4.0 {root_path}/foo/bar
"
            ))
            .assert()
            .stdout(format!(
                "  16.0  {root_path}/one/
   8.0  {root_path}/foo/
   4.0  {root_path}/foo/bar/
",
            ));
    }

    #[test]
    fn calling_devicon_lookup_with_malformed_regex_fails() {
        let mut cmd = Command::cargo_bin("devicon-lookup").unwrap();
        cmd.arg("-r")
            .arg("^(.")
            .write_stdin("test.rs: someline of the file".to_string())
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "The provided regex could not be parsed",
            ));
    }

    #[test]
    fn calling_devicon_lookup_with_input_that_doesnt_match_errors() {
        let mut cmd = Command::cargo_bin("devicon-lookup").unwrap();
        cmd.arg("--regex")
            .arg("^(.*):")
            .write_stdin("test.rs".to_string())
            .assert()
            .failure()
            .stderr(predicate::str::contains("Couldn't get captures from input"));
    }

    #[test]
    fn calling_devicon_lookup_with_regex_that_doesnt_have_capture_group() {
        let mut cmd = Command::cargo_bin("devicon-lookup").unwrap();
        cmd.arg("--regex")
            .arg("^.*")
            .write_stdin("test.rs".to_string())
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "The provided regex did not have a first capture group",
            ));
    }
}
