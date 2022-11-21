use anyhow::Error;
use example::Person;
use wasmedge_sdk::{
    config::{CommonConfigOptions, ConfigBuilder, HostRegistrationConfigOptions},
    Vm, WasmValue,
};

wit_bindgen_wasmtime::import!("../wit/example.wit");

fn main() -> Result<(), Error> {
    // Register module and create VM.
    let config = ConfigBuilder::new(CommonConfigOptions::default())
        .with_host_registration_config(HostRegistrationConfigOptions::default().wasi(true))
        .build()?;
    let vm = Vm::new(Some(config))?
        .register_module_from_file("lib", "target/wasm32-wasi/release/lib.wasm")?;

    // Prepare arguments.
    let mut memory = vm.named_module("lib")?.memory("memory").unwrap();
    let person = Person {
        first_name: "John",
        last_name: "Doe",
    };
    let first_name_ptr = vm.run_func(
        Some("lib"),
        "canonical_abi_realloc",
        vec![
            WasmValue::from_i32(0),
            WasmValue::from_i32(0),
            WasmValue::from_i32(1),
            WasmValue::from_i32(person.first_name.len() as i32),
        ],
    )?[0];
    let last_name_ptr = vm.run_func(
        Some("lib"),
        "canonical_abi_realloc",
        vec![
            WasmValue::from_i32(0),
            WasmValue::from_i32(0),
            WasmValue::from_i32(1),
            WasmValue::from_i32(person.last_name.len() as i32),
        ],
    )?[0];
    memory.write(person.first_name.as_bytes(), first_name_ptr.to_i32() as u32)?;
    memory.write(person.last_name.as_bytes(), last_name_ptr.to_i32() as u32)?;

    // Call `lib.hello`.
    let result = vm.run_func(
        Some("lib"),
        "hello",
        vec![
            first_name_ptr,
            WasmValue::from_i32(person.first_name.len() as i32),
            last_name_ptr,
            WasmValue::from_i32(person.last_name.len() as i32),
        ],
    )?;

    // Construct result string.
    let result_string_ptr = u32::from_ne_bytes(
        memory
            .read(result[0].to_i32() as u32, 4)?
            .try_into()
            .unwrap_or_else(|_| panic!("Convert failed")),
    );
    let result_string_len = u32::from_ne_bytes(
        memory
            .read((result[0].to_i32() + 4) as u32, 4)?
            .try_into()
            .unwrap_or_else(|_| panic!("Convert failed")),
    );
    vm.run_func(
        Some("lib"),
        "canonical_abi_free",
        vec![
            WasmValue::from_i32(result_string_ptr as i32),
            WasmValue::from_i32(result_string_len as i32),
            WasmValue::from_i32(1),
        ],
    )?;

    let result_string_vec = memory.read(result_string_ptr, result_string_len)?;
    let result_string = std::str::from_utf8(&result_string_vec)?;
    println!("Person: {:?}", person);
    println!("Result: {}", result_string);

    Ok(())
}
