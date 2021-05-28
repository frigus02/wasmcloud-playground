extern crate rmp_serde as rmps;
use rmps::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[cfg(feature = "guest")]
extern crate wapc_guest as guest;
#[cfg(feature = "guest")]
use guest::prelude::*;

#[cfg(feature = "guest")]
pub struct Host {
    binding: String,
}

#[cfg(feature = "guest")]
impl Default for Host {
    fn default() -> Self {
        Host {
            binding: "default".to_string(),
        }
    }
}

/// Creates a named host binding
#[cfg(feature = "guest")]
pub fn host(binding: &str) -> Host {
    Host {
        binding: binding.to_string(),
    }
}

/// Creates the default host binding
#[cfg(feature = "guest")]
pub fn default() -> Host {
    Host::default()
}

#[cfg(feature = "guest")]
impl Host {
    /// Create a new todo. Returns the id of the new todo.
    pub fn create(&self, title: String) -> HandlerResult<u32> {
        host_call(&self.binding, "todo", "Create", &serialize(title)?)
            .map(|vec| {
                let resp = deserialize::<u32>(vec.as_ref()).unwrap();
                resp
            })
            .map_err(|e| e.into())
    }
    /// List all todos. The dummy parameter is necessary because a the code generator
    /// seems to have problems if the method has no params.
    pub fn list(&self, dummy: bool) -> HandlerResult<Vec<Todo>> {
        host_call(&self.binding, "todo", "List", &serialize(dummy)?)
            .map(|vec| {
                let resp = deserialize::<Vec<Todo>>(vec.as_ref()).unwrap();
                resp
            })
            .map_err(|e| e.into())
    }
    /// Mark the todo with the given id as completed.
    pub fn mark_completed(&self, id: u32) -> HandlerResult<()> {
        host_call(&self.binding, "todo", "MarkCompleted", &serialize(id)?)
            .map(|_vec| ())
            .map_err(|e| e.into())
    }
}

#[cfg(feature = "guest")]
pub struct Handlers {}

#[cfg(feature = "guest")]
impl Handlers {
    /// Create a new todo. Returns the id of the new todo.
    pub fn register_create(f: fn(String) -> HandlerResult<u32>) {
        *CREATE.write().unwrap() = Some(f);
        register_function(&"Create", create_wrapper);
    }
    /// List all todos. The dummy parameter is necessary because a the code generator
    /// seems to have problems if the method has no params.
    pub fn register_list(f: fn(bool) -> HandlerResult<Vec<Todo>>) {
        *LIST.write().unwrap() = Some(f);
        register_function(&"List", list_wrapper);
    }
    /// Mark the todo with the given id as completed.
    pub fn register_mark_completed(f: fn(u32) -> HandlerResult<()>) {
        *MARK_COMPLETED.write().unwrap() = Some(f);
        register_function(&"MarkCompleted", mark_completed_wrapper);
    }
}

#[cfg(feature = "guest")]
lazy_static::lazy_static! {
static ref CREATE: std::sync::RwLock<Option<fn(String) -> HandlerResult<u32>>> = std::sync::RwLock::new(None);
static ref LIST: std::sync::RwLock<Option<fn(bool) -> HandlerResult<Vec<Todo>>>> = std::sync::RwLock::new(None);
static ref MARK_COMPLETED: std::sync::RwLock<Option<fn(u32) -> HandlerResult<()>>> = std::sync::RwLock::new(None);
}

#[cfg(feature = "guest")]
fn create_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<String>(input_payload)?;
    let lock = CREATE.read().unwrap().unwrap();
    let result = lock(input)?;
    serialize(result)
}

#[cfg(feature = "guest")]
fn list_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<bool>(input_payload)?;
    let lock = LIST.read().unwrap().unwrap();
    let result = lock(input)?;
    serialize(result)
}

#[cfg(feature = "guest")]
fn mark_completed_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<u32>(input_payload)?;
    let lock = MARK_COMPLETED.read().unwrap().unwrap();
    let result = lock(input)?;
    serialize(result)
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct Todo {
    #[serde(rename = "id")]
    pub id: u32,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "completed")]
    pub completed: bool,
}

/// The standard function for serializing codec structs into a format that can be
/// used for message exchange between actor and host. Use of any other function to
/// serialize could result in breaking incompatibilities.
pub fn serialize<T>(
    item: T,
) -> ::std::result::Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>>
where
    T: Serialize,
{
    let mut buf = Vec::new();
    item.serialize(&mut Serializer::new(&mut buf).with_struct_map())?;
    Ok(buf)
}

/// The standard function for de-serializing codec structs from a format suitable
/// for message exchange between actor and host. Use of any other function to
/// deserialize could result in breaking incompatibilities.
pub fn deserialize<'de, T: Deserialize<'de>>(
    buf: &[u8],
) -> ::std::result::Result<T, Box<dyn std::error::Error + Send + Sync>> {
    let mut de = Deserializer::new(Cursor::new(buf));
    match Deserialize::deserialize(&mut de) {
        Ok(t) => Ok(t),
        Err(e) => Err(format!("Failed to de-serialize: {}", e).into()),
    }
}
