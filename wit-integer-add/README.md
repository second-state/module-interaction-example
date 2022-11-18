## Description

In this example, we use `wit-bindgen` to generate a wasm module that export `add` function.
After that, we use `wit-bindgen` to generate host code to execute the module.

- `example.wit`: define the `add` function and the payload record with two `u32`s.
- `lib`: a wasm module that export an `add` function generated from `example.wit`.
- `host`: import the wasm module compiled from `lib` and then execute it with the payload.

## Usage

```
git clone https://github.com/second-state/module-interaction-example.git
cd module-interaction-example/integer-add
cargo run --release
```
