#[macro_use]
extern crate criterion;

use assert_cmd::prelude::*;
use assert_cmd::Command;
use criterion::Criterion;

fn run_cli_plain() {
    Command::cargo_bin("devicon-lookup")
        .unwrap()
        .pipe_stdin("tests/fixtures/all-types-large.txt")
        .unwrap();
}

fn run_cli_regex() {
    Command::cargo_bin("devicon-lookup")
        .unwrap()
        .arg("--regex")
        .arg("^(.*):")
        .pipe_stdin("tests/fixtures/all-types-grep-large.txt")
        .unwrap();
}

fn run_cli_prefix() {
    Command::cargo_bin("devicon-lookup")
        .unwrap()
        .arg("--prefix")
        .arg(":")
        .pipe_stdin("tests/fixtures/all-types-grep-large.txt")
        .unwrap();
}

fn run_cli_color() {
    Command::cargo_bin("devicon-lookup")
        .unwrap()
        .arg("--color")
        .pipe_stdin("tests/fixtures/all-types-large.txt")
        .unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("benchmark_each_file_type_plain_cli", |b| {
        b.iter(run_cli_plain)
    });
    c.bench_function("benchmark_each_file_type_color_cli", |b| {
        b.iter(run_cli_color)
    });
    c.bench_function("benchmark_each_file_type_regex_cli", |b| {
        b.iter(|| run_cli_prefix())
    });
    c.bench_function("benchmark_each_file_type_prefix_cli", |b| {
        b.iter(|| run_cli_prefix())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
