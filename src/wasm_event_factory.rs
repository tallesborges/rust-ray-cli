use std::{ffi::CStr, fs, env, io::{self, Write}};
use chrono::Local;
use serde_json::Value;
use shared::{EventEntry, EventFactory};
use wasmtime::{Engine, Instance, Module, Store};

#[derive(Default)]
pub struct WasmEventFactory;

impl WasmEventFactory {
    fn is_tui_mode(&self) -> bool {
        // Check if we're running in TUI mode
        env::args().any(|arg| arg == "--tui")
    }
    
    fn log_info(&self, message: &str) {
        // Only log if not in TUI mode
        if !self.is_tui_mode() {
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            let log_line = format!("[{}] [INFO] [WasmFactory] {}", timestamp, message);
            
            let mut stdout = io::stdout();
            let _ = writeln!(stdout, "{}", log_line);
            let _ = stdout.flush();
        }
    }
    
    fn log_error(&self, message: &str) {
        // Only log if not in TUI mode
        if !self.is_tui_mode() {
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            let log_line = format!("[{}] [ERROR] [WasmFactory] {}", timestamp, message);
            
            let mut stderr = io::stderr();
            let _ = writeln!(stderr, "{}", log_line);
            let _ = stderr.flush();
        }
    }
}

impl EventFactory for WasmEventFactory {
    fn make(&self, event: &Value) -> Option<EventEntry> {
        let event_type = event
            .get("type")
            .and_then(Value::as_str)
            .unwrap_or("unknown")
            .to_string();
        
        self.log_info(&format!("Processing event type: {}", event_type));
        self.log_info(&format!("Event payload: {}", event.to_string()));

        let entries = match fs::read_dir("wasm-modules/") {
            Ok(entries) => entries,
            Err(e) => {
                self.log_error(&format!("Failed to read wasm-modules directory: {}", e));
                return None;
            }
        };

        for entry in entries {
            let path = entry.unwrap().path();
            let file_name = path.file_name().unwrap().to_str().unwrap();

            let wasm_event_type = file_name.replace("event_", "");
            let wasm_event_type = wasm_event_type.replace(".wasm", "");
            // println!("Supported type: {}", wasm_event_type);

            if wasm_event_type.trim() == event_type.trim() {
                let wasm_path = format!("wasm-modules/{}", file_name);
                self.log_info(&format!("Loading WASM module: {}", wasm_path));

                let engine = Engine::default();
                let mut store = Store::new(&engine, ());
                
                let module = match Module::from_file(&engine, &wasm_path) {
                    Ok(module) => module,
                    Err(e) => {
                        self.log_error(&format!("Failed to load WASM module {}: {}", wasm_path, e));
                        return None;
                    }
                };
                
                let instance = match Instance::new(&mut store, &module, &[]) {
                    Ok(instance) => instance,
                    Err(e) => {
                        self.log_error(&format!("Failed to instantiate WASM module: {}", e));
                        return None;
                    }
                };

                let func = match instance.get_typed_func::<(i32, i32), i32>(&mut store, "process_event") {
                    Ok(func) => func,
                    Err(e) => {
                        self.log_error(&format!("Failed to get process_event function: {}", e));
                        return None;
                    }
                };

                let input = event.to_string();
                let memory = match instance.get_memory(&mut store, "memory") {
                    Some(memory) => memory,
                    None => {
                        self.log_error("Failed to get memory export from WASM module");
                        return None;
                    }
                };

                let offset = 0;
                if let Err(e) = memory.write(&mut store, offset, input.as_bytes()) {
                    self.log_error(&format!("Failed to write to WASM memory: {}", e));
                    return None;
                }

                let result_ptr = match func.call(&mut store, (offset as i32, input.len() as i32)) {
                    Ok(ptr) => ptr,
                    Err(e) => {
                        self.log_error(&format!("Failed to call WASM process_event function: {}", e));
                        return None;
                    }
                };

                let memory_slice = memory.data(&store);
                let result_str = match unsafe {
                    CStr::from_ptr(&memory_slice[result_ptr as usize] as *const u8 as *const i8)
                        .to_str()
                } {
                    Ok(s) => s,
                    Err(e) => {
                        self.log_error(&format!("Invalid UTF-8 in WASM result: {}", e));
                        return None;
                    }
                };
                
                let result: shared::EventEntry = match serde_json::from_str(&result_str) {
                    Ok(entry) => entry,
                    Err(e) => {
                        self.log_error(&format!("Invalid JSON in WASM result: {}", e));
                        return None;
                    }
                };

                self.log_info(&format!("Successfully processed {} event with WASM", event_type));
                self.log_info(&format!("Result: label={}, content_type={}, content length={}", 
                    result.label, result.content_type, result.content.len()));
                return Some(result);
            }
        }

        self.log_error(&format!("No suitable WASM module found for event type: {}", event_type));
        self.log_error(&format!("Available modules searched: wasm-modules/event_{}.wasm", event_type));
        None
    }
}
