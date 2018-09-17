set -euxo pipefail

main() {
    cargo test --target $TARGET
}

main
