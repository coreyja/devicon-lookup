extern crate assert_cmd;

#[cfg(test)]
mod integration {
    use assert_cmd::Command;

    #[test]
    fn calling_devicon_with_non_ut8_forwards_bytes() {
        let input_non_utf8 = vec![
            0xC0, 0xC1, 0xF5, 0xF6, 0xF7, 0xF8, 0xF9, 0xFA, 0xFB, 0xFC, 0xFD, 0xFE, 0xFF,
        ];
        let mut cmd = Command::cargo_bin("devicon-lookup").unwrap();
        cmd.write_stdin(input_non_utf8.clone())
            .assert()
            .stdout(input_non_utf8);
    }

    #[test]
    fn calling_devicon_with_random_contents_does_not_error() {
        Command::cargo_bin("devicon-lookup")
            .unwrap()
            .write_stdin(include_bytes!("fixtures/random_bytes.bin").to_vec())
            .assert()
            .success();
    }
}
