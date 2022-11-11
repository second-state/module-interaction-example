#![feature(never_type)]
mod host;
use std::{collections::HashMap, ffi::c_void, hash::Hash, ptr::null};

use host::load_string;
use wasmedge_sdk::{
    config::{CommonConfigOptions, ConfigBuilder, HostRegistrationConfigOptions},
    error::HostFuncError,
    host_function, Caller, CallingFrame, ImportObjectBuilder, Store, Vm, WasmValue,
};

#[host_function]
fn hello(_: &Caller, _: Vec<WasmValue>) -> Result<Vec<WasmValue>, HostFuncError> {
    println!("Host: say hello");

    Ok(vec![])
}

fn main() -> Result<(), anyhow::Error> {
    let config = ConfigBuilder::new(CommonConfigOptions::default())
        .with_host_registration_config(HostRegistrationConfigOptions::default().wasi(true))
        .build()?;

    let mut vm = Vm::new(Some(config))?
        .register_module_from_file("lib", "target/wasm32-wasi/release/lib.wasm")?
        .register_module_from_file("app", "target/wasm32-wasi/release/app.wasm")?;

    let mut hashmap = HashMap::new();
    let names = vm.instance_names()?;
    for n in names {
        hashmap.insert(n.clone(), vm.named_module(n.as_str())?);
    }

    let import = ImportObjectBuilder::new()
        .with_func::<(), (), !>("hello", hello, None)?
        .with_func::<(i32, i32, i32, i32), (), !>(
            "forward_call",
            move |caller: &CallingFrame,
                  input: Vec<WasmValue>,
                  _: *mut c_void|
                  -> Result<Vec<WasmValue>, HostFuncError> {
                println!("Rust: Entering `forward_call`");
                if input.len() != 4 {
                    return Err(HostFuncError::User(1));
                }

                let mod_name =
                    load_string(caller, input[0].to_i32() as u32, input[1].to_i32() as u32);
                println!("Rust: module name is: `{}`", mod_name);
                let fn_name =
                    load_string(caller, input[2].to_i32() as u32, input[3].to_i32() as u32);
                println!("Rust: function name is: `{}`", fn_name);

                let target_mod = hashmap.get(&mod_name).unwrap();
                let target_fn = target_mod.func(fn_name).unwrap();

                let mut executor = caller.executor_mut().unwrap();
                target_fn.call(&mut executor, vec![]);

                println!("Rust: Leaving `forward_call`");
                Ok(vec![])
            },
            None,
        )?
        .build("host")?;
    vm = vm.register_import_module(import)?;

    let result = vm.run_func(Some("app"), "start", None)?;
    println!("result: {}", result[0].to_i32());

    Ok(())
}
