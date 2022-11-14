#![feature(never_type)]
mod host;
use host::load_string;
use wasmedge_sdk::{
    config::{CommonConfigOptions, ConfigBuilder, HostRegistrationConfigOptions},
    error::HostFuncError,
    host_function, Caller, ImportObjectBuilder, ValType, Vm, WasmValue,
};

#[host_function]
fn host_add(_caller: Caller, input: Vec<WasmValue>) -> Result<Vec<WasmValue>, HostFuncError> {
    if input.len() != 2 {
        return Err(HostFuncError::User(1));
    }

    let a = if input[0].ty() == ValType::I32 {
        input[0].to_i32()
    } else {
        return Err(HostFuncError::User(2));
    };

    let b = if input[1].ty() == ValType::I32 {
        input[1].to_i32()
    } else {
        return Err(HostFuncError::User(3));
    };

    let c = a + b;
    println!("Rust: calcuating in `host_add` c: {:?}", c);

    Ok(vec![WasmValue::from_i32(c)])
}

#[host_function]
fn host_println(caller: Caller, input: Vec<WasmValue>) -> Result<Vec<WasmValue>, HostFuncError> {
    let addr = input[0].to_i32() as u32;
    let size = input[1].to_i32() as u32;
    let s = load_string(&caller, addr, size);
    println!("Rust: `host_println` is printing: \"{}\"", s);

    Ok(vec![])
}

#[host_function]
fn host_suffix(caller: Caller, input: Vec<WasmValue>) -> Result<Vec<WasmValue>, HostFuncError> {
    println!("Rust: Entering `host_suffix`");
    if input.len() != 2 {
        return Err(HostFuncError::User(1));
    }

    let addr = input[0].to_i32() as u32;
    let size = input[1].to_i32() as u32;
    let mut s = load_string(&caller, addr, size);
    println!("Rust: Get: {}", s);

    // add suffix
    s.push_str("_suffix");

    let mut mem = caller.memory(0).unwrap();
    // take last address+1
    let final_addr = mem.size() + 1;
    // grow a page size
    mem.grow(1).expect("fail to grow memory");
    // put the returned string into new address
    mem.write(s.as_bytes(), final_addr)
        .expect("fail to write returned string");

    println!("Rust: Leaving `host_suffix`");
    Ok(vec![
        WasmValue::from_i32(final_addr as i32),
        WasmValue::from_i32(s.len() as i32),
    ])
}

fn main() -> Result<(), anyhow::Error> {
    let config = ConfigBuilder::new(CommonConfigOptions::default())
        .with_host_registration_config(HostRegistrationConfigOptions::default().wasi(true))
        .build()?;

    let import = ImportObjectBuilder::new()
        .with_func::<(i32, i32), i32, !>("host_add", host_add, None)?
        .with_func::<(i32, i32), (), !>("host_println", host_println, None)?
        .with_func::<(i32, i32), (i32, i32), !>("host_suffix", host_suffix, None)?
        .build("host")?;
    let vm = Vm::new(Some(config))?
        .register_import_module(import)?
        .register_module_from_file("app", "target/wasm32-wasi/release/app.wasm")?;

    let result = vm.run_func(Some("app"), "start", None)?;
    println!("result: {}", result[0].to_i32());

    Ok(())
}
