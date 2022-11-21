use example::{Example, ExampleData, Person};
use wasi_common::WasiCtx;
use wasmtime::{Engine, Linker, Module, Store};
use wasmtime_wasi::sync::WasiCtxBuilder;
use wit_bindgen_wasmtime::anyhow::Result;

wit_bindgen_wasmtime::import!("../wit/example.wit");

fn main() -> Result<()> {
    // Prepare WASI environment.
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s: &mut WasiCtx| s)?;
    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()?
        .build();
    let mut store = Store::new(&engine, wasi);

    // Load module.
    let module = Module::from_file(&engine, "target/wasm32-wasi/release/lib.wasm")?;

    // Initiate Example object.
    static mut DATA: ExampleData = ExampleData {};
    let (example_object, _) =
        Example::instantiate(&mut store, &module, &mut linker, |_| unsafe { &mut DATA })?;

    // Call `hello` with person.
    let person = Person {
        first_name: "John",
        last_name: "Doe",
    };
    let result = example_object.hello(&mut store, person.clone())?;
    println!("Person: {:?}", person);
    println!("Result: {}", result);
    Ok(())
}
