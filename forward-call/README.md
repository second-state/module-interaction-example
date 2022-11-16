# forward call

`forward-call` this technology is meant to help arbitrary two module-instances exchange non-primitive data without a common interface. The current call interface looks like below

```rust
let json_str = forward_call(module_name, function_name, json_str);
```

`forward_call` is a host function in fact. Though wasm's type requirement, a host function must provide a static type to the runtime, that's possible to exchange complicated data types via JSON encoding/decoding.

### Usage

Here is the selected code from sample to show the idea.

#### Caller

```rust
let person = Person {
    name: "August".into(),
    age: 16,
};
let json = serde_json::to_string(&person).unwrap();
let host_str = forward_call("lib".into(), "growup".into(), json.as_str().into());
host_println(host_str.clone());
```

#### Callee

```rust
pub unsafe extern "wasm" fn growup(str: HostString) -> HostString {
    host_println(str.clone());
    let mut person: Person = serde_json::from_str(Into::<String>::into(str).as_str()).unwrap();
    person.age += 3;
    let json = serde_json::to_string(&person).unwrap();
    json.as_str().into()
}
```

#### The data type to exchange

```rust
#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
}
```

#### Future

The above code is able to wrap with a macro `forward_call`, then caller part would be simpler.

```rust
#[forward_call(module = "lib")]
extern "wasm" {
    fn growup(str: String) -> String;
}
```

### Motivation

We want to make every module instance can behave as a unit of microservice, and call each others' functions.

### Road

We think that the whole core of component-model is about the across-module calls, WIT is not that new but just a gRPC's `*.proto` file, component-model is RPC framework. This leads us to try gRPC and thrift, but their compiler cannot generate well C++ for wasm, there have tons of helper functions that have to implement.

So we step back, @dm4 build an across-module call that only about the primitive data, [example](../integer-add/) can be found in this repository. I feel that would be easy to figure out string passing! But that's not true, let's see what happened. To exchange a string in wasm, one must break it into two parts

1. pointer
2. size

Hence, a function that tries to return a string became

```rust
fn foo() -> (*const u8, usize)
```

At the call site, we declare a `extern` for it

```rust
#![feature(wasm_abi)]

extern "wasm" {
    fn foo() -> (*const u8, usize)
}
```

But if we try to dereference the pointer, it will be an out-of-bound access, because an address valid in a module will be invalid in another module. Or even worse, the module gets bad data from bad address.

After discussion, we found we can use **host function** to access and modify the memory, this is why originally JS can access web wasm's string(refers to `wasm_bindgen`). The next section will introduce the details.

### Technology details

#### A host function in WasmEdge

The signature of a **host function** in WasmEdge is

```rust
Fn (caller: CallingFrame, input: Vec<WasmValue>, _: *mut c_void) -> Result<Vec<WasmValue>, HostFuncError>
```

#### Read instance memory

One can use `caller` parameter to access caller module's memory, and fetching data out

```rust
let mem = caller.memory_mut(0).unwrap();
let data = mem.get_data(addr, size).unwrap();
String::from_utf8_lossy(&data).to_string()
```

#### Write instance memory

To write data into a module instance, it's

```rust
let final_addr = mem.size() + 1;
mem.grow(1).unwrap();
mem.set_data(str.as_bytes(), final_addr).unwrap();
```

#### Host function `forward-call` and `HostString` in the use site

```rust
#[link(wasm_import_module = "host")]
extern "wasm" {
    fn forward_call(
        module_name: HostString,
        funciton_name: HostString,
        data: HostString,
    ) -> HostString;
}

#[derive(Clone, Debug, PartialEq)]
#[repr(C)]
pub struct HostString(*const u8, usize);
impl From<&str> for HostString {
    fn from(value: &str) -> Self {
        Self(value.as_ptr(), value.len())
    }
}
impl HostString {
    pub fn len(&self) -> usize {
        self.1
    }
}
```
