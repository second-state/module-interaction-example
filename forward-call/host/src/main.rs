#![feature(never_type)]
mod host;
mod host_function;
use host::load_string;
use host_function::println::host_println;
use std::ffi::c_void;
use wasmedge_sdk::{
    config::{CommonConfigOptions, ConfigBuilder, HostRegistrationConfigOptions},
    error::HostFuncError,
    Caller, CallingFrame, ImportObjectBuilder, Vm, WasmValue,
};

fn extend_forward_call(
    io: ImportObjectBuilder,
    vm: Vm,
) -> Result<ImportObjectBuilder, anyhow::Error> {
    Ok(
        io.with_func::<(i32, i32, i32, i32, i32, i32), (i32, i32), !>(
            "forward_call",
            move |frame: CallingFrame,
                  input: Vec<WasmValue>,
                  _: *mut c_void|
                  -> Result<Vec<WasmValue>, HostFuncError> {
                if input.len() != 6 {
                    return Err(HostFuncError::User(1));
                }

                let mut mem = frame.memory_mut(0).unwrap();

                let mod_name =
                    load_string(&frame, input[0].to_i32() as u32, input[1].to_i32() as u32);
                let fn_name =
                    load_string(&frame, input[2].to_i32() as u32, input[3].to_i32() as u32);
                println!("forward_call: `{}:{}`", mod_name, fn_name);

                let target_mod = vm.named_module(mod_name).unwrap();
                let mut target_mem = target_mod.memory("memory").unwrap();

                let final_addr = target_mem.size() + 1;
                target_mem.grow(1).unwrap();
                let str = mem
                    .get_data(input[4].to_i32() as u32, input[5].to_i32() as u32)
                    .unwrap();
                target_mem.write(str, final_addr).unwrap();

                let target_fn = target_mod.func(fn_name).unwrap();
                let mut executor = Caller::new(frame).executor().unwrap();
                let result = target_fn
                    .call(
                        &mut executor,
                        vec![WasmValue::from_i32(final_addr as i32), input[5]],
                    )
                    .unwrap();

                let str = target_mem
                    .read_string(result[0].to_i32() as u32, result[1].to_i32() as u32)
                    .unwrap();

                // take last address+1
                let final_addr = mem.size() + 1;
                // grow a page size
                mem.grow(1).unwrap();
                // put the returned string into new address
                mem.set_data(str.as_bytes(), final_addr).unwrap();

                Ok(vec![WasmValue::from_i32(final_addr as i32), result[1]])
            },
            None,
        )?,
    )
}

fn main() -> Result<(), anyhow::Error> {
    let config = ConfigBuilder::new(CommonConfigOptions::default())
        .with_host_registration_config(HostRegistrationConfigOptions::default().wasi(true))
        .build()?;

    let mut vm = Vm::new(Some(config))?;

    let import = extend_forward_call(ImportObjectBuilder::new(), vm.clone())?
        .with_func::<(i32, i32), (), !>("host_println", host_println, None)?
        .build("host")?;

    vm = vm
        .register_import_module(import)?
        .register_module_from_file("lib", "target/wasm32-wasi/release/lib.wasm")?
        .register_module_from_file("app", "target/wasm32-wasi/release/app.wasm")?;

    let result = vm.run_func(Some("app"), "start", None)?;
    println!("result: {}", result[0].to_i32());

    Ok(())
}
