#[macro_use]
extern crate criterion;

use assert_cmd::Command;
use criterion::Criterion;

fn run_cli() {
    let mut cmd = Command::cargo_bin("devicon-lookup").unwrap();
    cmd.pipe_stdin("tests/fixtures/all-types.txt").unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("benchmark_each_file_type_cli", |b| b.iter(|| run_cli()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
