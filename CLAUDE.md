# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with this rust-ray-cli codebase.

## About This Project

A macOS desktop application that receives debug events from Ray PHP/Laravel applications and displays them in a native GUI using the gpui framework.

## Build Commands

### Running the Application
- **Run:** `cargo run`
- **Test run:** `timeout 5s cargo run` (for quick testing)

### Development Commands
- **Test:** `cargo test`
- **Check:** `cargo check`
- **Format:** `cargo fmt`
- **Lint:** `cargo clippy`

## Architecture Overview

### Core Components

1. **HTTP Server** (`src/server.rs`): Listens on port 23517 for Ray events
2. **Event Processing** (`src/events/`): Three-layer architecture for processing events
3. **Event Storage** (`src/event_storage.rs`): Central storage with structured logging
4. **GUI Application** (`src/app.rs`): Main application using gpui framework
5. **UI Components** (`src/ui_components.rs`): Reusable UI elements and styling

### Event Processing Architecture

This codebase uses a **three-layer Event-Processor-Renderer architecture**:

```
JSON Event → Processor → Structured Data → Renderer → Markdown/UI
```

#### Layer 1: Event Processors (`src/events/processors/`)
- **Purpose:** Pure data extraction from JSON payloads
- **One processor per event type:** `cache.rs`, `http.rs`, `log.rs`, `query.rs`, `exception.rs`, `application_log.rs`, `table.rs`
- **Output:** Structured data types defined in `src/events/types.rs`

#### Layer 2: Structured Data Types (`src/events/types.rs`)
- **Purpose:** Clean data structures without presentation logic
- **Types:** `CacheEvent`, `HttpEvent`, `LogEvent`, `QueryEvent`, `ExceptionEvent`, `ApplicationLogEvent`, `TableEvent`

#### Layer 3: Event Renderers (`src/events/renderers/`)
- **Purpose:** Pure presentation logic, converts structured data to markdown
- **One renderer per event type:** Matching the processors
- **Output:** Formatted markdown content for display

### Event Flow

1. HTTP server receives JSON payload
2. Event type determines which processor to use
3. Processor extracts and structures data
4. Renderer converts structured data to markdown
5. Result stored in EventStorage and displayed in UI

## Adding New Event Types

To add a new event type, follow this pattern:

### 1. Create Data Type
Add to `src/events/types.rs`:
```rust
#[derive(Clone, Debug)]
pub struct MyEvent {
    pub field1: String,
    pub field2: Option<Value>,
    // ... other fields
}
```

Add variant to `ProcessedEvent` enum:
```rust
pub enum ProcessedEvent {
    // ... existing variants
    MyEvent(MyEvent),
}
```

### 2. Create Processor
Create `src/events/processors/my_event.rs`:
```rust
use crate::events::types::{MyEvent, ProcessedEvent};
use anyhow::Result;
use serde_json::Value;

pub fn process_my_event(content: &Value) -> Result<ProcessedEvent> {
    // Extract data from JSON
    let field1 = content.get("field1")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();
    
    Ok(ProcessedEvent::MyEvent(MyEvent {
        field1,
        field2: content.get("field2").cloned(),
    }))
}
```

### 3. Create Renderer
Create `src/events/renderers/my_event.rs`:
```rust
use crate::events::types::MyEvent;

pub fn render_my_event_markdown(event: &MyEvent) -> String {
    let mut markdown = String::from("## My Event\n\n");
    markdown.push_str(&format!("**Field 1:** {}\n", event.field1));
    // ... format other fields
    markdown
}

pub fn get_my_event_label(_event: &MyEvent) -> String {
    "My Event".to_string()
}

pub fn get_my_event_description(event: &MyEvent) -> String {
    event.field1.clone()
}
```

### 4. Update Module Files
- Add exports to `src/events/processors/mod.rs`
- Add exports to `src/events/renderers/mod.rs`
- Add new event type to factory pattern in main event processor

## UI Framework (GPUI)

### Requirements
- **Full Xcode installation** (for Metal shader compiler)
- **Nightly Rust compiler** (uses experimental trait upcasting)
- **macOS only** (gpui is currently macOS-specific)

### Resources
- **Examples:** https://github.com/zed-industries/zed/tree/main/crates/gpui/examples
- **Note:** Your knowledge about gpui may be outdated; always prefer examples from the official Zed repo

### UI Components
- Use components from `src/ui_components.rs` for consistent styling
- Follow existing patterns for event rendering and display

## Rust Coding Guidelines

### Code Quality
- **Prioritize correctness and clarity** over speed and efficiency
- **Avoid organizational comments** that summarize code
- **Write comments only to explain "why"**, not "what"

### Error Handling
- **Never use `unwrap()`** - use `?` to propagate errors
- **Never silently discard errors** with `let _ =`
- **Always handle errors appropriately:**
  - Propagate with `?` when caller should handle
  - Use explicit handling with `match` or `if let Err(...)`
  - Ensure async operation errors reach the UI layer

### File Organization
- **Prefer `src/module.rs`** over `src/module/mod.rs`
- **Implement functionality in existing files** unless it's a new logical component
- **Avoid creating many small files**

### Safety
- **Be careful with indexing** - check bounds to prevent panics
- **Handle async operation failures** with proper error propagation

## Key Design Principles

- **Single Responsibility:** Each processor handles exactly one event type
- **Clean Separation:** Data processing separate from presentation
- **Maintainable:** Easy to understand, modify, and extend
- **Consistent:** All event types follow the same architectural pattern
- **Testable:** Can test processing and rendering independently