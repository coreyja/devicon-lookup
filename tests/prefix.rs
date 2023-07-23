extern crate assert_cmd;

#[cfg(test)]
mod integration {
    use assert_cmd::Command;

    #[test]
    fn calling_devicon_lookup_with_grep_syntax_and_prefix_works() {
        let mut cmd = Command::cargo_bin("devicon-lookup").unwrap();
        cmd.arg("--prefix")
            .arg(":")
            .write_stdin("test.rs: someline of the file".to_string())
            .assert()
            .stdout(" test.rs: someline of the file\n");
    }

    #[test]
    fn calling_devicon_lookup_with_multichar_prefix_works() {
        let mut cmd = Command::cargo_bin("devicon-lookup").unwrap();
        cmd.arg("--prefix")
            .arg("pre")
            .write_stdin("test.rspre stuff after\ntest.rs\nrust.txtpr".to_string())
            .assert()
            .stdout(" test.rspre stuff after\n test.rs\n rust.txtpr\n");
    }

    #[test]
    fn calling_devicon_lookup_with_not_found_prefix_defaults_to_full_lines() {
        let mut cmd = Command::cargo_bin("devicon-lookup").unwrap();
        cmd.arg("-p")
            .arg("@")
            .write_stdin("test.rs: someline of the file\ntest.rs".to_string())
            .assert()
            .success()
            .stdout(" test.rs: someline of the file\n test.rs\n".to_string());
    }
}
