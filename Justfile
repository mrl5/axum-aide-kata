local-app:
    cargo run --package app

lint: fmt
    cargo clippy --fix --allow-staged

fmt:
    rustfmt crates/**/src/*.rs
