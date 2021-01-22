use std::collections::HashMap;
use wascc_host::{Actor, Host, NativeCapability};

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let _ = env_logger::try_init();
    let host = Host::new();
    host.add_actor(Actor::from_file(
        "../../api/target/wasm32-unknown-unknown/debug/api_signed.wasm",
    )?)?;
    host.add_native_capability(NativeCapability::from_file(
        "../../wascc/wascc-host/examples/.assets/libwascc_httpsrv.so",
        None,
    )?)?;

    host.set_binding(
        "MCYQSR5WIOABHZP6Z3SG67REVC2QDCYAHUVXHUSSLFWNO55OZ3O33MKR",
        "wascc:http_server",
        None,
        generate_port_config(8081),
    )?;

    std::thread::park();

    Ok(())
}

fn generate_port_config(port: u16) -> HashMap<String, String> {
    let mut hm = HashMap::new();
    hm.insert("PORT".to_string(), port.to_string());
    hm
}
