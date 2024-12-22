#![cfg_attr(all(target_arch = "wasm32", not(test)), no_std, no_main)]
extern crate alloc;

use alloc::string::ToString;
use serde_json::Value;
use shared::{implement_ffi_interface, process_common_event, EventEntry, EventProcessor};

#[derive(Default)]
pub struct QueryEvent;

impl EventProcessor for QueryEvent {
    fn process(&self, payload: &str) -> EventEntry {
        let mut entry = process_common_event("query");
        if let Ok(v) = serde_json::from_str::<Value>(payload) {
            if let Some(content) = v.get("content") {
                let sql = content
                    .get("sql")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                let connection = content
                    .get("connection_name")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                let time = content
                    .get("time")
                    .and_then(|v| v.as_f64())
                    .unwrap_or_default();

                entry.content = alloc::format!(
                    r#"**Connection:** {}

**Execution Time:** {:.2}ms

```sql
{}
```"#,
                    connection,
                    time,
                    sql,
                );
            }
        }
        entry.content_type = "markdown".to_string();
        entry
    }
}

implement_ffi_interface!(QueryEvent);
