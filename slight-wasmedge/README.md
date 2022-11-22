## Description

This example is a POC to run `spidey` example from [spiderlightning] using [WasmEdge].
It only mocks the required functions but does not really store the key-value pair.

## Usage

```
$ cargo build --release
$ cargo run --release wasmedge
kv::open
-> name: my_folder
kv::set
-> key: hello-spiderlightning
-> value: Hello, SpiderLightning!
kv::get
-> key: hello-spiderlightning
resource_drop_kv
```

[spiderlightning]: https://github.com/deislabs/spiderlightning
[WasmEdge]: https://github.com/WasmEdge/WasmEdge
