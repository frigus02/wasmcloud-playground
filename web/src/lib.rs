use handlebars::Handlebars;
use todo_interface as todo;
use wapc_guest as guest;
use wasmcloud_actor_core as actor;
use wasmcloud_actor_http_server as http;

use guest::prelude::*;

const TODO_ACTOR: &str = "todo";

#[actor::init]
fn init() {
    http::Handlers::register_handle_request(handle_request);
}

fn handle_request(req: http::Request) -> HandlerResult<http::Response> {
    if req.path_segments().len() != 1
        || !req.path_segments().first().unwrap().is_empty()
        || req.method() != http::Method::Get
    {
        return Ok(http::Response::not_found());
    }

    let todos = todo::host(TODO_ACTOR).list(true)?;

    let reg = Handlebars::new();
    let body = reg.render_template(include_str!("../html/index.hb"), &todos)?;

    let mut res = http::Response::ok();
    res.header.insert("content-type".into(), "text/html".into());
    res.body = body.into();
    Ok(res)
}
