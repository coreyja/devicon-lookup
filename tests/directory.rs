extern crate assert_cmd;

#[cfg(test)]
mod integration {
    use std::fs::create_dir;

    use assert_cmd::Command;
    use temp_dir::TempDir;

    #[test]
    fn local_dir_test() {
        let d = TempDir::new().unwrap();
        create_dir(d.path().join("test_dir")).unwrap();

        let mut cmd = Command::cargo_bin("devicon-lookup").unwrap();
        cmd.current_dir(d.path())
            .write_stdin("test_dir\ntest_dir/".to_string())
            .assert()
            .stdout(" test_dir/\n test_dir/\n");
    }

    #[test]
    fn absolute_dir_test() {
        let d = TempDir::new().unwrap();
        let test_dir = d.path().join("test_dir");
        create_dir(&test_dir).unwrap();

        let mut cmd = Command::cargo_bin("devicon-lookup").unwrap();
        cmd.current_dir(d.path())
            .write_stdin(test_dir.to_str().unwrap().to_string())
            .assert()
            .stdout(format!(" {}/\n", test_dir.to_str().unwrap()));
    }
}
