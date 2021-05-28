use std::convert::TryInto as _;
use todo_interface as todo;
use wapc_guest as guest;
use wasmcloud_actor_core as actor;
use wasmcloud_actor_keyvalue as kv;

use guest::prelude::*;

const COUNT: &str = "count";
const PREFIX_TASK_TITLE: &str = "title";
const PREFIX_TASK_COMPLETED: &str = "completed";

#[actor::init]
fn init() {
    todo::Handlers::register_create(handle_create);
    todo::Handlers::register_list(handle_list);
    todo::Handlers::register_mark_completed(handle_mark_completed);
}

fn handle_create(title: String) -> HandlerResult<u32> {
    let store = kv::default();
    let res_id = store.add(COUNT.into(), 1)?;
    let id: u32 = res_id.value.try_into()?;
    store.set(format!("{}_{}", PREFIX_TASK_TITLE, id), title, 0)?;
    store.set(format!("{}_{}", PREFIX_TASK_COMPLETED, id), "".into(), 0)?;
    Ok(id)
}

fn handle_list(_dummy: bool) -> HandlerResult<Vec<todo::Todo>> {
    let store = kv::default();
    let res_count = store.get(COUNT.into())?;
    let count = res_count.value.parse::<u32>()?;
    let mut todos = Vec::new();
    for id in 0..=count {
        let res_title = store.get(format!("{}_{}", PREFIX_TASK_TITLE, id))?;
        let res_completed = store.get(format!("{}_{}", PREFIX_TASK_COMPLETED, id))?;
        todos.push(todo::Todo {
            id,
            title: res_title.value,
            completed: !res_completed.value.is_empty(),
        });
    }

    Ok(todos)
}

fn handle_mark_completed(id: u32) -> HandlerResult<()> {
    let store = kv::default();
    store.set(format!("{}_{}", PREFIX_TASK_COMPLETED, id), "1".into(), 0)?;
    Ok(())
}
