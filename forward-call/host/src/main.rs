#![feature(never_type)]
mod host;
use host::load_string;
use std::ffi::c_void;
use wasmedge_sdk::{
    config::{CommonConfigOptions, ConfigBuilder, HostRegistrationConfigOptions},
    error::HostFuncError,
    CallingFrame, ImportObjectBuilder, Vm, WasmValue,
};

fn main() -> Result<(), anyhow::Error> {
    let config = ConfigBuilder::new(CommonConfigOptions::default())
        .with_host_registration_config(HostRegistrationConfigOptions::default().wasi(true))
        .build()?;

    let mut vm = Vm::new(Some(config))?;
    let invm = Box::new(vm.clone());

    let import = ImportObjectBuilder::new()
        .with_func::<(i32, i32, i32, i32), i32, !>(
            "forward_call",
            move |caller: CallingFrame,
                  input: Vec<WasmValue>,
                  _: *mut c_void|
                  -> Result<Vec<WasmValue>, HostFuncError> {
                println!("Rust: Entering `forward_call`");
                if input.len() != 4 {
                    return Err(HostFuncError::User(1));
                }

                let mod_name =
                    load_string(&caller, input[0].to_i32() as u32, input[1].to_i32() as u32);
                println!("Rust: module name is: `{}`", mod_name);
                let fn_name =
                    load_string(&caller, input[2].to_i32() as u32, input[3].to_i32() as u32);
                println!("Rust: function name is: `{}`", fn_name);

                let target_mod = invm.to_owned().named_module(mod_name).unwrap();
                let target_fn = target_mod.func(fn_name).unwrap();

                let mut executor: wasmedge_sdk::Executor = caller.executor_mut().unwrap().into();
                let result = target_fn.call(&mut executor, vec![]).expect("call fail");

                println!("Rust: Leaving `forward_call`");
                Ok(result)
            },
            None,
        )?
        .build("host")?;

    vm = vm
        .register_import_module(import)?
        .register_module_from_file("lib", "target/wasm32-wasi/release/lib.wasm")?
        .register_module_from_file("app", "target/wasm32-wasi/release/app.wasm")?;

    let result = vm.run_func(Some("app"), "start", None)?;
    println!("result: {}", result[0].to_i32());

    Ok(())
}
