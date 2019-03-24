
extern crate assert_cmd;

#[cfg(test)]
mod integration {
    use assert_cmd::prelude::*;
    use colored::*;
    use std::process::Command;

    #[test]
    fn calling_devicon_lookup_with_single_colored_filename_unique_icon() {
        let mut cmd = Command::cargo_bin("devicon-lookup").unwrap();
        cmd.with_stdin()
            .buffer("test.rs".blue().to_string())
            .assert()
            .stdout(" \x1b[34mtest.rs\x1B[0m\n");
    }

    #[test]
    fn calling_devicon_lookup_with_multi_colored_filenames() {
        let mut cmd = Command::cargo_bin("devicon-lookup").unwrap();
        cmd.with_stdin()
            .buffer(format!("{}\n{}","test.rs".blue(), "test.rb".red()))
            .assert()
            .stdout(" \x1b[34mtest.rs\x1B[0m\n \x1b[31mtest.rb\x1b[0m\n");
    }
}
