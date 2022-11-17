# module-interaction-example

The structure of this project is

```
.
├── forward-call
├── host-function
└── integer-add
```

The order is

1. integer-add
2. host-function
3. forward-call

We use [integer-add](./integer-add/) to show how to call function in another module instance, [host-function](./host-function/) to show how to operate `caller` memory to fetch heap data, [forward-call](./forward-call/) is the final stage of module interaction. You can move to [forward-call](./forward-call/) to get more details.
