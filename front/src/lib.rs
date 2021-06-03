use wapc_guest as guest;
use wasmcloud_actor_core as actor;
use wasmcloud_actor_http_server as http;

use guest::prelude::*;

const API_ACTOR: &str = "api";
const WEB_ACTOR: &str = "web";

#[actor::init]
fn init() {
    http::Handlers::register_handle_request(handle_request);
}

fn handle_request(req: http::Request) -> HandlerResult<http::Response> {
    match req.path_segments().as_slice() {
        ["api", path @ ..] => forward_request(
            http::Request {
                path: join_path(path),
                ..req
            },
            API_ACTOR,
        ),
        _ => forward_request(req, WEB_ACTOR),
    }
}

fn join_path(path: &[&str]) -> String {
    if path.is_empty() {
        "/".into()
    } else {
        std::iter::once(String::new())
            .chain(path.iter().map(|x| (*x).into()))
            .collect::<Vec<String>>()
            .join("/")
    }
}

fn forward_request(req: http::Request, actor: &str) -> HandlerResult<http::Response> {
    actor::call_actor(actor, "HandleRequest", &req)
}
