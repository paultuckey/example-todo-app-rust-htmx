# Example TODO app with Rust HTMx

A simple example TODO app build using:

- [htmx](https://htmx.org/)
- [Rust](https://www.rust-lang.org/)
- [Rocket](https://rocket.rs/) web framework using handlebars templates
- [SQLx](https://github.com/launchbadge/sqlx) and [SQLite](https://sqlite.org/)

## Run

```shell
cargo run
```

## Hot Reloading

Install cargo watch with `cargo install cargo-watch` then use:

```shell
cargo watch -x run
```

## Syntax Check

```shell
cargo clippy
```
