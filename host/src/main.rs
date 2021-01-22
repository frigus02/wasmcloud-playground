use std::collections::HashMap;
use wascc_host::{Actor, Host, NativeCapability};
use wascc_httpsrv::HttpServerProvider;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let host = Host::new();

    // TODO: try host.apply_manifest() instead of manually loading actors and capabilities

    let actor = Actor::from_file("../api/target/wasm32-unknown-unknown/debug/api_s.wasm")?;
    let actor_pub_key = actor.public_key();
    host.add_actor(actor)?;

    let http_server_cap = NativeCapability::from_instance(HttpServerProvider::new(), None)?;
    let http_server_cap_id = http_server_cap.descriptor().id.clone();
    host.add_native_capability(http_server_cap)?;

    host.set_binding(
        &actor_pub_key,
        &http_server_cap_id,
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
