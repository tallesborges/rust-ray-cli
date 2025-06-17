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
JSON Event → Processor → Structured Data → UI Renderer → gpui Components
```

#### Layer 1: Event Processors (`src/events/processors/`)
- **Purpose:** Pure data extraction from JSON payloads
- **One processor per event type:** `cache.rs`, `http.rs`, `log.rs`, `query.rs`, `exception.rs`, `application_log.rs`, `table.rs`
- **Output:** Structured data types defined in `src/events/types.rs`

#### Layer 2: Structured Data Types (`src/events/types.rs`)
- **Purpose:** Clean data structures without presentation logic
- **Types:** `CacheEvent`, `HttpEvent`, `LogEvent`, `QueryEvent`, `ExceptionEvent`, `ApplicationLogEvent`, `TableEvent`

#### Layer 3: Event Renderers (UI Functions in `src/events/`)
- **Purpose:** Pure presentation logic, converts structured data to UI components
- **One renderer per event type:** Located in main event files (e.g., `log.rs`, `query.rs`, `exception.rs`)
- **Output:** gpui Div components with minimalist styling for display

### Event Flow

1. HTTP server receives JSON payload
2. Event type determines which processor to use
3. Processor extracts and structures data
4. UI renderer converts structured data to gpui components with minimalist styling
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

### 3. Create UI Renderer
Add to the main event file (e.g., `src/events/my_event.rs`):
```rust
use crate::ui_components::{border_color, text_primary_color, text_secondary_color};
use gpui::prelude::*;
use gpui::{div, Context, Div};

pub fn render_my_event(entry: &EventEntry, _cx: &mut Context<crate::app::MyApp>) -> Div {
    div()
        .flex()
        .flex_col()
        .gap_6()
        .child(render_my_event_details(entry))
        .child(render_origin_info(entry))
}

fn render_my_event_details(entry: &EventEntry) -> Div {
    div()
        .flex()
        .flex_col()
        .gap_2()
        .child(
            div()
                .text_sm()
                .text_color(text_primary_color())
                .child("Event content here")
        )
}

fn render_origin_info(entry: &EventEntry) -> Div {
    // Standard origin info with minimal styling
    if let Some(origin) = entry.raw_payload.get("origin") {
        // ... implement origin display
    } else {
        div()
    }
}
```

### 4. Update Module Files
- Add exports to `src/events/processors/mod.rs`
- Add the new event file to `src/events/mod.rs`
- Add new event type to factory pattern in main event processor
- Register the UI renderer function in the renderer factory

## UI Framework (GPUI)

### Requirements
- **Full Xcode installation** (for Metal shader compiler)
- **Nightly Rust compiler** (uses experimental trait upcasting)
- **macOS only** (gpui is currently macOS-specific)

### Resources
- **Examples:** https://github.com/zed-industries/zed/tree/main/crates/gpui/examples
- **Note:** Your knowledge about gpui may be outdated; always prefer examples from the official Zed repo

### UI Design Philosophy
- **Minimalist, shadcn-inspired aesthetic** with clean typography and generous whitespace
- **Color palette:** zinc-based (zinc-950 background, zinc-800 borders, zinc-50/zinc-400 text)
- **Typography-first approach:** Use font weight, size, and color for hierarchy instead of heavy styling
- **Subtle interactions:** Text-only buttons with opacity/color hover effects
- **No cards or excessive borders:** Use spacing for visual separation
- **Unified background:** Single background color throughout the application

### UI Components
- Use components from `src/ui_components.rs` for consistent styling
- Follow existing patterns for event rendering and display
- **Key principles:**
  - Avoid emoji icons - use clean typography instead
  - Use opacity (0.5, 0.7, 0.8, 0.9) for subtle hierarchy
  - Minimal hover states with color/opacity changes only
  - Generous padding and spacing (px-4, px-8, py-3, py-6, gap-4, gap-6)
  - Clean borders only for true content separation (border_color())

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