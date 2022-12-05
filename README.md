
> **Note**
> This project is in it's early stages.

## run
You can by clicking this link: [github.io/no-name](https://askeladd123.github.io/no-name/).

## build native
You need *cargo* installed. Run ```cargo run --release```.
> **Alert** only tested on *Windows*

## build web
You need *wasm-pack* to build. You need a *http server runner* to load and run. Do commands:
- `wasm-pack build --target web`
- `http-server`

The program should run in [localhost:8080]().

## potential problems
- *thiserror* crate has an unstable feature:
  - run ```cargo clean``` and it magically works
- **python http.server** didn't work for me
