use std::{collections::HashMap, fs, sync::Arc};

use serde_json::Value;
use shared::{EventEntry, EventFactory, EventProcessor};

pub struct WasmEventFactory {
    processors: HashMap<String, Arc<dyn EventProcessor>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    #[test]
    fn test_make_lists_wasm_files() {
        // Create a temporary directory
        let temp_dir = TempDir::new().unwrap();
        let wasm_dir = temp_dir.path().join("wasm-modules");
        fs::create_dir(&wasm_dir).unwrap();

        // Create some test .wasm files
        fs::write(wasm_dir.join("test1.wasm"), "").unwrap();
        fs::write(wasm_dir.join("test2.wasm"), "").unwrap();

        // Create symlink to temp directory
        let symlink_path = PathBuf::from("../wasm-modules");
        if symlink_path.exists() {
            fs::remove_file(&symlink_path).unwrap();
        }
        std::os::unix::fs::symlink(&wasm_dir, &symlink_path).unwrap();

        // Test the make function
        let factory = WasmEventFactory::default();
        factory.make(&Value::Null);

        // Cleanup
        fs::remove_file(symlink_path).unwrap();
    }
}

impl Default for WasmEventFactory {
    fn default() -> Self {
        WasmEventFactory {
            processors: HashMap::default(),
        }
    }
}

impl EventFactory for WasmEventFactory {
    fn make(&self, event: &Value) -> Option<EventEntry> {
        if let Ok(entries) = fs::read_dir("../wasm-modules") {
            println!("Listing wasm modules:");
            for entry in entries {
                if let Ok(entry) = entry {
                    println!("  {}", entry.path().display());
                }
            }
        } else {
            println!("Failed to read ../wasm-modules directory");
        }
        None
    }
}
