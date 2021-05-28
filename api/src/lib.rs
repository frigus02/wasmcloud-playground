use serde::{Deserialize, Serialize};
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

#[derive(Deserialize)]
struct NewTodoRequest {
    title: String,
}

#[derive(Serialize)]
struct NewTodoResponse {
    id: u32,
}

fn handle_request(req: http::Request) -> HandlerResult<http::Response> {
    if req.path_segments().len() != 1 {
        return Ok(http::Response::not_found());
    }

    let path = req.path_segments().first().unwrap().to_string();
    if path.is_empty() {
        match req.method() {
            http::Method::Get => {
                let todos = todo::host(TODO_ACTOR).list(true)?;
                Ok(http::Response::json(todos, 200, "OK"))
            }
            http::Method::Post => {
                let new_todo: NewTodoRequest = serde_json::from_slice(&req.body)?;
                let id = todo::host(TODO_ACTOR).create(new_todo.title)?;
                Ok(http::Response::json(NewTodoResponse { id }, 201, "Created"))
            }
            _ => Ok(http::Response::not_found()),
        }
    } else {
        let id: u32 = path.parse()?;
        match req.method() {
            http::Method::Put => {
                todo::host(TODO_ACTOR).mark_completed(id)?;
                Ok(http::Response::json((), 204, "No Content"))
            }
            _ => Ok(http::Response::not_found()),
        }
    }
}
