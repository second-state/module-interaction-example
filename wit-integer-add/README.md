## Description

In this example, we use `wit-bindgen` to generate a wasm module that export `add` function.
After that, we use `wit-bindgen` to generate host code to execute the module.

- `example.wit`: define the `add` function and the payload record with two `u32`s.
- `lib`: a wasm module that export an `add` function generated from `example.wit`.
- `host`: import the wasm module compiled from `lib` and then execute it with the payload.

## Usage

```
git clone https://github.com/second-state/module-interaction-example.git
cd module-interaction-example/wit-integer-add
cargo run --release
```

## Use `wasmedge`

Since the module genertaed by `wit-bindgen` only uses core wasm type,
we could run it using `wasmedge` runtime:

```
$ wasmedge target/wasm32-wasi/release/lib.wasm add 1 2
3
```
