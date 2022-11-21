## Description

In this example, we use `wit-bindgen` to generate a wasm module that export `hello` function.
After that, we use `wit-bindgen` to generate host code to execute the module with both `wasmtime` and `wasmedge`.

- `example.wit`: define the `hello` function and the `person` record with two `string`s.
- `lib`: a wasm module that export an `hello` function generated from `example.wit`.
- `host-wasmtime`: import the wasm module compiled from `lib` and then execute it using `wasmtime`.
- `host-wasmedge`: import the wasm module compiled from `lib` and then execute it using `wasmedge`.

## Usage

```
git clone https://github.com/second-state/module-interaction-example.git
cd module-interaction-example/wit-string-hello
cargo run --release --bin host-wasmtime
```

## Use `wasmedge`

Since the module genertaed by `wit-bindgen` contains some high-level types,
we need to deal with Canonical ABI properly when using `wasmedge`.

```
$ cargo run --release --bin host-wasmedge
Person: Person { first-name: "John", last-name: "Doe" }
Result: Hello, John Doe
```
