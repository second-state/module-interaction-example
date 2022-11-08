use wasmedge_sdk::{
    config::{CommonConfigOptions, ConfigBuilder, HostRegistrationConfigOptions},
    Vm,
};

fn main() {
    let config = ConfigBuilder::new(CommonConfigOptions::default())
        .with_host_registration_config(HostRegistrationConfigOptions::default().wasi(true))
        .build()
        .expect("fail to create config");
    let mut vm = Vm::new(Some(config)).expect("fail to create vm");
    vm = vm
        .register_module_from_file("module", "target/wasm32-wasi/release/module.wasm")
        .expect("fail to register module");
    vm = vm
        .register_module_from_file("app", "target/wasm32-wasi/release/app.wasm")
        .expect("fail to register app");
    let result = vm
        .run_func(Some("app"), "start", None)
        .expect("fail to run");
    println!("result: {}", result[0].to_i32());
}
