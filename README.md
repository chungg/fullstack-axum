[![gate](https://github.com/chungg/fullstack-axum/actions/workflows/gate.yml/badge.svg)](https://github.com/chungg/fullstack-axum/actions/workflows/gate.yml)

# axum + htmx + bulma

sample app which provides:
- backend
  - backend via [axum](https://github.com/tokio-rs/axum)
  - tables via [polars](https://pola.rs/)
- ui
  - frontend via [htmx](https://github.com/bigskysoftware/htmx) and [minijinja](https://github.com/mitsuhiko/minijinja)
  - styling via [bulma](https://github.com/jgthms/bulma) and [bulma-extensions](https://wikiki.github.io/)
  - charting via [chartjs](https://www.chartjs.org/)
  - tables via [tabulator](https://tabulator.info/)

## setup
- install [rust](https://www.rust-lang.org/tools/install)
- `cargo r`

## running tests

### styling

- `cargo fmt`

### unit

- install [nextest](https://nexte.st/book/pre-built-binaries.html) (optional)
- `cargo nextest r` or `cargo test`

## resources
- https://www.shuttle.rs/blog/2023/12/06/using-axum-rust
- testing
  - https://www.youtube.com/watch?v=_cYIhG_3qSo
  - https://github.com/rstropek/rust-samples/blob/master/axum-di-testing/axum-di/src/main.rs
- openapi
  - https://github.com/tamasfe/aide
  - https://github.com/ProbablyClem/utoipauto
- alt frameworks
  - https://github.com/poem-web/poem
  - https://github.com/LukeMathWalker/pavex

# todo
- openapi
- auth support
