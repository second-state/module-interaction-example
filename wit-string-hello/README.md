## Description

In this example, we use `wit-bindgen` to generate a wasm module that export `hello` function.
After that, we use `wit-bindgen` to generate host code to execute the module.

- `example.wit`: define the `hello` function and the `person` record with two `string`s.
- `lib`: a wasm module that export an `hello` function generated from `example.wit`.
- `host`: import the wasm module compiled from `lib` and then execute it.

## Usage

```
git clone https://github.com/second-state/module-interaction-example.git
cd module-interaction-example/wit-string-hello
cargo run --release
```
