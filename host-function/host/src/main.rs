#![feature(never_type)]

use wasmedge_sdk::{
    config::{CommonConfigOptions, ConfigBuilder, HostRegistrationConfigOptions},
    error::HostFuncError,
    host_function, Caller, ImportObjectBuilder, ValType, Vm, WasmValue,
};

#[host_function]
fn real_add(_caller: &Caller, input: Vec<WasmValue>) -> Result<Vec<WasmValue>, HostFuncError> {
    println!("Rust: Entering Rust function real_add");

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
    println!("Rust: calcuating in real_add c: {:?}", c);

    println!("Rust: Leaving Rust function real_add");
    Ok(vec![WasmValue::from_i32(c)])
}

#[host_function]
fn real_println(caller: &Caller, input: Vec<WasmValue>) -> Result<Vec<WasmValue>, HostFuncError> {
    println!("Rust: Entering Rust function real_println");
    if input.len() != 2 {
        return Err(HostFuncError::User(1));
    }

    let addr = input[0].to_i32();
    let size = input[1].to_i32();
    let m = caller.memory(0).unwrap();
    println!(
        "Rust: get string: {}",
        String::from_utf8_lossy(&m.read(addr as u32, size as u32).expect("test"))
    );

    println!("Rust: Leaving Rust function real_println");
    Ok(vec![])
}

fn main() -> Result<(), anyhow::Error> {
    let config = ConfigBuilder::new(CommonConfigOptions::default())
        .with_host_registration_config(HostRegistrationConfigOptions::default().wasi(true))
        .build()?;

    let import = ImportObjectBuilder::new()
        .with_func::<(i32, i32), i32, !>("real_add", real_add, None)?
        .with_func::<(i32, i32), (), !>("real_println", real_println, None)?
        .build("env")?;
    let vm = Vm::new(Some(config))?
        .register_import_module(import)?
        .register_module_from_file("app", "target/wasm32-wasi/release/app.wasm")?;

    let result = vm.run_func(Some("app"), "start", None)?;
    println!("result: {}", result[0].to_i32());

    Ok(())
}
