# Host function

The structure of this project

```
.
├── Cargo.lock
├── Cargo.toml
├── README.md
├── app
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
└── host
    ├── Cargo.toml
    └── src
        └── main.rs
```

The **host** will invoke **app**, and provides a host function.

### Run command

The only command need to remember here is

```sh
cargo run --release
```
