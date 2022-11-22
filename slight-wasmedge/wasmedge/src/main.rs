#![feature(never_type)]
use anyhow::Error;
use wasmedge_sdk::{
    config::{CommonConfigOptions, ConfigBuilder, HostRegistrationConfigOptions},
    error::HostFuncError,
    host_function, Caller, ImportObjectBuilder, Vm, WasmValue,
};

fn load_string(caller: &Caller, addr: u32, size: u32) -> String {
    let mem = caller.memory(0).unwrap();
    let data = mem.read(addr, size).expect("fail to get string");
    String::from_utf8_lossy(&data).to_string()
}

#[host_function]
fn kv_open(caller: Caller, args: Vec<WasmValue>) -> Result<Vec<WasmValue>, HostFuncError> {
    println!("kv::open");
    let param = load_string(&caller, args[0].to_i32() as u32, args[1].to_i32() as u32);
    println!("-> name: {}", param);
    Ok(vec![])
}

#[host_function]
fn kv_set(caller: Caller, args: Vec<WasmValue>) -> Result<Vec<WasmValue>, HostFuncError> {
    println!("kv::set");
    let key = load_string(&caller, args[1].to_i32() as u32, args[2].to_i32() as u32);
    let value = load_string(&caller, args[3].to_i32() as u32, args[4].to_i32() as u32);
    println!("-> key: {}", key);
    println!("-> value: {}", value);
    Ok(vec![])
}

#[host_function]
fn kv_get(caller: Caller, args: Vec<WasmValue>) -> Result<Vec<WasmValue>, HostFuncError> {
    println!("kv::get");
    let key = load_string(&caller, args[1].to_i32() as u32, args[2].to_i32() as u32);
    println!("-> key: {}", key);
    Ok(vec![])
}

#[host_function]
fn resource_drop_kv(_: Caller, _: Vec<WasmValue>) -> Result<Vec<WasmValue>, HostFuncError> {
    println!("resource_drop_kv");
    Ok(vec![])
}

fn main() -> Result<(), Error> {
    // Register module and create VM.
    let config = ConfigBuilder::new(CommonConfigOptions::default())
        .with_host_registration_config(HostRegistrationConfigOptions::default().wasi(true))
        .build()?;
    let kv_module = ImportObjectBuilder::new()
        .with_func::<(i32, i32, i32), (), !>("kv::open", kv_open, None)?
        .with_func::<(i32, i32, i32, i32, i32, i32), (), !>("kv::set", kv_set, None)?
        .with_func::<(i32, i32, i32, i32), (), !>("kv::get", kv_get, None)?
        .build("kv")?;
    let canon_module = ImportObjectBuilder::new()
        .with_func::<i32, (), !>("resource_drop_kv", resource_drop_kv, None)?
        .build("canonical_abi")?;
    let vm = Vm::new(Some(config))?
        .register_import_module(kv_module)?
        .register_import_module(canon_module)?
        .register_module_from_file("spidey", "target/wasm32-wasi/release/spidey.wasm")?;

    // Execute spidey.
    vm.run_func(Some("spidey"), "_start", None)?;

    Ok(())
}
