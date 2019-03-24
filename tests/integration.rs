extern crate assert_cmd;

#[cfg(test)]
mod integration {
    use assert_cmd::prelude::*;
    use colored::*;
    use std::process::Command;

    #[test]
    fn calling_devicon_lookup_with_single_file_default_icon() {
        let mut cmd = Command::cargo_bin("devicon-lookup").unwrap();
        cmd.with_stdin()
            .buffer("test.txt")
            .assert()
            .stdout(" test.txt\n");
    }

    #[test]
    fn calling_devicon_lookup_with_single_file_unique_icon() {
        let mut cmd = Command::cargo_bin("devicon-lookup").unwrap();
        cmd.with_stdin()
            .buffer("test.rs")
            .assert()
            .stdout(" test.rs\n");
    }

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

    #[test]
    fn calling_devicon_lookup_with_single_path_unique_icon() {
        let mut cmd = Command::cargo_bin("devicon-lookup").unwrap();
        cmd.with_stdin()
            .buffer("/some/cool/directories/here/test.rs")
            .assert()
            .stdout(" /some/cool/directories/here/test.rs\n");
    }

    #[test]
    fn calling_devicon_lookup_with_multiple_files() {
        let mut cmd = Command::cargo_bin("devicon-lookup").unwrap();
        cmd.with_stdin()
            .buffer("test.rs\nrandom.rb\nsome_cool_file.js\nmore_things.py")
            .assert()
            .stdout(" test.rs\n random.rb\n some_cool_file.js\n more_things.py\n");
    }

    #[test]
    fn calling_devicon_lookup_with_large_fixture_file() {
        let mut cmd = Command::cargo_bin("devicon-lookup").unwrap();
        cmd
          .with_stdin()
          .path("tests/fixtures/all-types.txt").unwrap()
          .assert()
          .stdout(" example.ai\n example.awk\n example.bash\n example.bat\n example.bmp\n example.c++\n example.c\n example.cc\n example.clj\n example.cljc\n example.cljs\n example.coffee\n example.conf\n example.cp\n example.cpp\n example.csh\n example.css\n example.cxx\n example.d\n example.dart\n example.db\n example.diff\n example.dump\n example.edn\n example.ejs\n example.erl\n example.f#\n example.fish\n example.fs\n example.fsi\n example.fsscript\n example.fsx\n example.gif\n example.go\n example.h\n example.hbs\n example.hpp\n example.hrl\n example.hs\n example.htm\n example.html\n example.hxx\n example.ico\n example.ini\n example.java\n example.jl\n example.jpeg\n example.jpg\n example.js\n example.json\n example.jsx\n example.ksh\n example.less\n example.lhs\n example.lua\n example.markdown\n example.md\nλ example.ml\nλ example.mli\n example.mustache\n example.php\n example.pl\n example.pm\n example.png\n example.pp\n example.ps1\n example.psb\n example.psd\n example.py\n example.pyc\n example.pyd\n example.pyo\n example.rb\n example.rlib\n example.rmd\n example.rs\n example.rss\n example.sass\n example.scala\n example.scss\n example.sh\n example.slim\n example.sln\n example.sql\n example.styl\n example.suo\n example.t\n example.ts\n example.tsx\n example.twig\n example.vi\n example.vim\n﵂ example.vue\n example.xul\n example.yaml\n example.yml\n example.zsh\n");
    }
}
