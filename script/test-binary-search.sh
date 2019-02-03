BASE_DIR="./results/$1/"

mkdir -p "$BASE_DIR"

function run_test_for_git_object {
  git checkout "$2"
  cargo build --release
  hyperfine --warmup 3 --export-markdown "$BASE_DIR/$1.md" 'cat ~/tmp/large.txt | ./target/release/devicon-lookup'
}

# First Baseline Run Against Master
run_test_for_git_object "baseline" "master"

# Second Plain Binary Search
run_test_for_git_object "plain" "b931ae1"
# git checkout b931ae1
# cargo build --release
# hyperfine --warmup 3 --export-markdown "$BASE_DIR/plain.md" 'cat ~/tmp/large.txt | ./target/release/devicon-lookup'

# Second Plain Binary Search
run_test_for_git_object "cached" "binary-search"
# git checkout binary-search
# cargo build --release
# hyperfine --warmup 3 --export-markdown "$BASE_DIR/cached.md" 'cat ~/tmp/large.txt | ./target/release/devicon-lookup'
