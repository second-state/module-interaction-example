#![feature(never_type)]
mod host;
use host::load_string;
use std::ffi::c_void;
use wasmedge_sdk::{
    config::{CommonConfigOptions, ConfigBuilder, HostRegistrationConfigOptions},
    error::HostFuncError,
    host_function, Caller, CallingFrame, ImportObjectBuilder, Vm, WasmValue,
};

#[host_function]
pub fn host_println(
    caller: Caller,
    input: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, HostFuncError> {
    let addr = input[0].to_i32() as u32;
    let size = input[1].to_i32() as u32;
    let s = load_string(&caller, addr, size);
    println!("Rust: `host_println` is printing: \"{}\"", s);

    Ok(vec![])
}

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
                let caller = Caller::new(frame);

                if input.len() != 6 {
                    return Err(HostFuncError::User(1));
                }

                let mut mem = caller.memory(0).unwrap();

                let mod_name =
                    load_string(&caller, input[0].to_i32() as u32, input[1].to_i32() as u32);
                let fn_name =
                    load_string(&caller, input[2].to_i32() as u32, input[3].to_i32() as u32);
                println!("forward_call: `{}:{}`", mod_name, fn_name);

                let target_mod = vm.named_module(mod_name).unwrap();
                let mut target_mem = target_mod.memory("memory").unwrap();

                let final_addr = target_mem.size() + 1;
                target_mem.grow(1).unwrap();
                let str = mem
                    .read(input[4].to_i32() as u32, input[5].to_i32() as u32)
                    .unwrap();
                target_mem.write(str, final_addr).unwrap();

                let target_fn = target_mod.func(fn_name).unwrap();
                let mut executor = caller.executor().unwrap();
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
                mem.write(str.as_bytes(), final_addr).unwrap();

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
