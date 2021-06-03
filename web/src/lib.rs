use once_cell::sync::OnceCell;
use todo_interface as todo;
use wapc_guest as guest;
use wasmcloud_actor_core as actor;
use wasmcloud_actor_http_server as http;

use guest::prelude::*;

const TODO_ACTOR: &str = "todo";

static TEMPLATE_INDEX: OnceCell<liquid::Template> = OnceCell::new();
const STYLES_INDEX: &str = include_str!("../html/styles/index.css");
const SCRIPTS_INDEX: &str = include_str!("../html/scripts/index.js");

#[actor::init]
fn init() {
    let parser = liquid::ParserBuilder::with_stdlib().build().unwrap();
    let template_index = parser.parse(include_str!("../html/index.liquid")).unwrap();
    let _ = TEMPLATE_INDEX.set(template_index);

    http::Handlers::register_handle_request(handle_request);
}

fn handle_request(req: http::Request) -> HandlerResult<http::Response> {
    match (req.method(), req.path_segments().as_slice()) {
        (http::Method::Get, [""]) => render_index(),
        (http::Method::Get, ["styles", "index.css"]) => render_styles_index(),
        (http::Method::Get, ["scripts", "index.js"]) => render_scripts_index(),
        _ => Ok(http::Response::not_found()),
    }
}

fn render_index() -> HandlerResult<http::Response> {
    let todos = todo::host(TODO_ACTOR).list(true)?;
    let globals = liquid::object!({ "todos": todos });
    let body = TEMPLATE_INDEX.get().unwrap().render(&globals)?;

    let mut res = http::Response::ok();
    res.header.insert("content-type".into(), "text/html".into());
    res.body = body.into();
    Ok(res)
}

fn render_styles_index() -> HandlerResult<http::Response> {
    let mut res = http::Response::ok();
    res.header.insert("content-type".into(), "text/css".into());
    res.body = STYLES_INDEX.into();
    Ok(res)
}

fn render_scripts_index() -> HandlerResult<http::Response> {
    let mut res = http::Response::ok();
    res.header
        .insert("content-type".into(), "application/javascript".into());
    res.body = SCRIPTS_INDEX.into();
    Ok(res)
}
