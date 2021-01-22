use std::collections::HashMap;
use wascc_host::{Actor, Host, NativeCapability};
use wascc_httpsrv::HttpServerProvider;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let host = Host::new();

    // TODO: try host.apply_manifest() instead of manually loading actors and capabilities

    host.add_actor(Actor::from_file(
        "../api/target/wasm32-unknown-unknown/debug/api_s.wasm",
    )?)?;

    let http_server_provider = NativeCapability::from_instance(HttpServerProvider::new(), None)?;
    let http_server_cap_id = &http_server_provider.descriptor().id.clone();
    host.add_native_capability(http_server_provider)?;

    host.set_binding(
        "MC5PD6ZD4WAIWQW5E6U5RHZBR2PJAZ5KSAKDOGHECKREUYR6HWKFFCBO",
        http_server_cap_id,
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
