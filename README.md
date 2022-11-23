# module-interaction-example

The structure of this project is

```
.
├── forward-call
├── host-function
├── integer-add
├── slight-wasmedge
├── wit-integer-add
└── wit-string-hello
```

### WasmEdge extension

For the wasmedge related part, the order is

1. integer-add
2. host-function
3. forward-call

We use [integer-add](./integer-add/) to show how to call function in another module instance, [host-function](./host-function/) to show how to operate `caller` memory to fetch heap data, [forward-call](./forward-call/) is the final stage of module interaction. You can move to [forward-call](./forward-call/) to get more details.

### Component Model experiment

For component model related part, the order is

1. wit-integer-add: exchange primitive type(e.g. `i32`) but generates interface from `.wit`
2. wit-string-hello: exchange structure type and generates interface from `.wit`
3. slight-wasmedge: use WasmEdge host code to handle generated code from `.wit`
