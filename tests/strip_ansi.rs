extern crate assert_cmd;

#[cfg(test)]
mod integration {
    use assert_cmd::Command;
    use colored::*;

    #[test]
    fn calling_devicon_lookup_without_strip_color_single_file() {
        let mut cmd = Command::cargo_bin("devicon-lookup").unwrap();
        cmd.write_stdin(format!("{}", "test.rs".blue()))
            .assert()
            .stdout(format!(" {}\n", "test.rs".blue()));
    }

    #[test]
    fn calling_devicon_lookup_with_strip_color_single_file() {
        let mut cmd = Command::cargo_bin("devicon-lookup").unwrap();
        cmd.arg("--color");
        cmd.write_stdin("test.rs".blue().to_string())
            .assert()
            .stdout(format!(" {}\n", "test.rs".blue()));
    }

    #[test]
    fn calling_devicon_lookup_with_strip_color_multi_file() {
        let mut cmd = Command::cargo_bin("devicon-lookup").unwrap();
        cmd.arg("-c");
        cmd.write_stdin(format!("{}\n{}", "test.rs".blue(), "test.rb".red()))
            .assert()
            .stdout(format!(" {}\n {}\n", "test.rs".blue(), "test.rb".red()));
    }
}
