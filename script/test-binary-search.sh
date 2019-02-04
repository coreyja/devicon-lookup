BASE_DIR="./results/$1/"

SIZES=("small" "large")

mkdir -p "$BASE_DIR"

function run_test_for_git_object {
  git checkout "$2"
  cargo build --release
  for size in "${SIZES[@]}"; do
    mkdir -p "$BASE_DIR/$size"
    hyperfine --warmup 3 --export-markdown "$BASE_DIR/$size/$1.md" "cat ~/tmp/$size.txt | ./target/release/devicon-lookup"
  done

}

run_test_for_git_object "baseline" "master"
run_test_for_git_object "plain" "b931ae1"
run_test_for_git_object "cached" "binary-search"
