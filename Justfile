set export

DOCKER_COMPOSE := "docker compose"

local-app:
    cargo run --package app

db-only:
    ${DOCKER_COMPOSE} up db

lint: fmt
    cargo clippy --fix --allow-staged

fmt:
    rustfmt crates/**/src/*.rs
