use std::{borrow::BorrowMut, collections::HashMap, ffi::CStr, fs};

use serde_json::Value;
use shared::{EventEntry, EventFactory};
use wasmtime::{Engine, Instance, Module, Store};

#[derive(Default)]
pub struct WasmEventFactory;

impl EventFactory for WasmEventFactory {
    fn make(&self, event: &Value) -> Option<EventEntry> {
        let event_type = event
            .get("type")
            .and_then(Value::as_str)
            .unwrap()
            .to_string();
        println!("Processing event type: {}", event_type);
        println!("Event: {}", event);

        let entries = fs::read_dir("wasm-modules/").expect("Failed to read wasm-modules directory");

        for entry in entries {
            let path = entry.unwrap().path();
            let file_name = path.file_name().unwrap().to_str().unwrap();

            let wasm_event_type = file_name.replace("event_", "");
            let wasm_event_type = wasm_event_type.replace(".wasm", "");
            println!("Supported type: {}", wasm_event_type);

            if wasm_event_type.trim() == event_type.trim() {
                let wasm_path = format!("wasm-modules/{}", file_name);
                println!("Loading {}, path {}", file_name, wasm_path);

                let engine = Engine::default();
                let mut store = Store::new(&engine, ());
                let module = Module::from_file(&engine, wasm_path).unwrap();
                let instance = Instance::new(&mut store, &module, &[]).unwrap();

                let func = instance
                    .get_typed_func::<(i32, i32), i32>(&mut store, "process_event")
                    .expect("Failed to get process_event function");

                let input = event.to_string();
                let memory = instance.get_memory(&mut store, "memory").unwrap();

                let offset = 0;
                memory.write(&mut store, offset, input.as_bytes()).unwrap();

                let result_ptr = func
                    .call(&mut store, (offset as i32, input.len() as i32))
                    .unwrap();

                let memory_slice = memory.data(&store);
                let result_str = unsafe {
                    CStr::from_ptr(&memory_slice[result_ptr as usize] as *const u8 as *const i8)
                        .to_str()
                        .expect("Invalid UTF-8")
                };
                let result: shared::EventEntry =
                    serde_json::from_str(&result_str).expect("Invalid JSON");

                return Some(result);
            }
        }

        None
    }
}
