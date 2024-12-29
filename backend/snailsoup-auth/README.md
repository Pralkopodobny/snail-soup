# snail-soup

## Build Prerequisites
- rust (with cargo etc.)
- docker
- sqlx-cli (cargo install sqlx-cli); requires
    - build-essential
    - openssl (eg. libssl-dev on Ubuntu)
    - pkg-config
    - in case error happens READ terminal output!

## Db initialization!
- docker compose up snailsoup_auth_db
- sqlx database create
- sqlx migrate run

## Build with cargo
- Initialize DB! (sqlx checks schema to make sure that queries are type-safe)
- cargo build

## Run with cargo
- Cargo run