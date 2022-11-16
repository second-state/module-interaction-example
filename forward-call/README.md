# forward call

`forward-call` this technology is meant to help arbitrary two module-instances exchange non-primitive data without a common interface. The current call interface looks like below

```rust
let json_str = forward_call(module_name, function_name, json_str);
```

`forward_call` is a host function in fact. Though wasm's type requirement, a host function must provide a static type to the runtime, that's possible to exchange complicated data types via JSON encoding/decoding.
