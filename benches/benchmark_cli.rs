#[macro_use]
extern crate criterion;

use assert_cmd::prelude::*;
use criterion::Criterion;
use std::process::Command;

fn run_cli() {
    let mut cmd = Command::cargo_bin("devicon-lookup").unwrap();
    cmd.with_stdin()
        .path("tests/fixtures/all-types.txt")
        .unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("benchmark_each_file_type_cli", |b| b.iter(|| run_cli()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
