#![feature(never_type)]
mod host;
mod host_function;
use host::load_string;
use host_function::println::host_println;
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
    let invm = vm.clone();

    let import = ImportObjectBuilder::new()
        .with_func::<(i32, i32), (), !>("host_println", host_println, None)?
        .with_func::<(i32, i32, i32, i32, i32, i32), (i32, i32), !>(
            "forward_call",
            move |caller: CallingFrame,
                  input: Vec<WasmValue>,
                  _: *mut c_void|
                  -> Result<Vec<WasmValue>, HostFuncError> {
                if input.len() != 6 {
                    return Err(HostFuncError::User(1));
                }

                let mod_name =
                    load_string(&caller, input[0].to_i32() as u32, input[1].to_i32() as u32);
                let fn_name =
                    load_string(&caller, input[2].to_i32() as u32, input[3].to_i32() as u32);
                println!("forward_call: `{}:{}`", mod_name, fn_name);

                let target_mod = invm.named_module(mod_name).unwrap();
                let target_fn = target_mod.func(fn_name).unwrap();
                let mut executor: wasmedge_sdk::Executor = caller.executor_mut().unwrap().into();
                let result = target_fn
                    .call(&mut executor, vec![input[4], input[5]])
                    .expect("call fail");

                let target_mem = target_mod.memory("memory").unwrap();
                let str = target_mem
                    .read_string(result[0].to_i32() as u32, result[1].to_i32() as u32)
                    .expect("fail to read from target");

                let mut mem = caller.memory_mut(0).unwrap();
                // take last address+1
                let final_addr = mem.size() + 1;
                // grow a page size
                mem.grow(1).expect("fail to grow caller memory");
                // put the returned string into new address
                mem.set_data(str.as_bytes(), final_addr)
                    .expect("fail to write back");

                Ok(vec![WasmValue::from_i32(final_addr as i32), result[1]])
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
