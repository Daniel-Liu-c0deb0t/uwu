set -e

# note: cargo clean to make sure everything rebuilds every time

files="enwik8 enwik9"

for f in $files; do
    echo "1 thread uwu $f"
    cargo clean
    cargo run --quiet --release -- test/$f test/${f}_uwu -t 1
    echo ""

    echo "2 thread uwu $f"
    cargo clean
    cargo run --quiet --release -- test/$f test/${f}_uwu -t 2
    echo ""

    echo "4 thread uwu $f"
    cargo clean
    cargo run --quiet --release -- test/$f test/${f}_uwu -t 4
    echo ""

    echo "8 thread uwu $f"
    cargo clean
    cargo run --quiet --release -- test/$f test/${f}_uwu -t 8
    echo ""

    echo "copy $f"
    cargo clean
    cargo build --quiet --release
    time cp test/$f test/${f}_uwu
    echo ""
done
