use std::collections::HashMap;
use tera::{Value, Result as TeraResult};

/// Convert a boolean to "on"/"off" for tmux config
pub fn bool_to_on_off(value: &Value, _args: &HashMap<String, Value>) -> TeraResult<Value> {
    match value.as_bool() {
        Some(true) => Ok(Value::String("on".into())),
        Some(false) => Ok(Value::String("off".into())),
        _ => Ok(value.clone()),
    }
}

/// Convert a boolean to "true"/"false" string
pub fn bool_to_str(value: &Value, _args: &HashMap<String, Value>) -> TeraResult<Value> {
    match value.as_bool() {
        Some(true) => Ok(Value::String("true".into())),
        Some(false) => Ok(Value::String("false".into())),
        _ => Ok(value.clone()),
    }
}

/// Escape single quotes for shell strings
pub fn shell_escape(value: &Value, _args: &HashMap<String, Value>) -> TeraResult<Value> {
    match value.as_str() {
        Some(s) => Ok(Value::String(s.replace('\'', "'\\''"))),
        _ => Ok(value.clone()),
    }
}

/// Convert a hex color like #0b0e14 to the palette index format
pub fn hex_strip(value: &Value, _args: &HashMap<String, Value>) -> TeraResult<Value> {
    match value.as_str() {
        Some(s) if s.starts_with('#') => Ok(Value::String(s[1..].to_string())),
        _ => Ok(value.clone()),
    }
}
