use serde_json::json;
use wapc_guest as guest;
use wasmcloud_actor_core as actor;
use wasmcloud_actor_http_server as http;

use guest::prelude::*;

#[actor::init]
fn init() {
    http::Handlers::register_handle_request(hello_world);
}

fn hello_world(_req: http::Request) -> HandlerResult<http::Response> {
    let result = json!({ "hello": "world", "data": 21});
    Ok(http::Response::json(&result, 200, "OK"))
}
