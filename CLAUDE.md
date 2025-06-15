# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

### Examples and Resources
- Here you can find up-to-date examples of how to use the gpui crate: https://github.com/zed-industries/zed/tree/main/crates/gpui/examples
- Your knowledge about gpui is outdated, so when you need some examples always prefer to use the examples from the repo

## Build Commands

### Running the Application
- Run: `cargo run` (use `timeout 5s cargo run` for testing)

### Development Commands
- Run tests: `cargo test`
- Check code: `cargo check`
- Format code: `cargo fmt`
- Lint code: `cargo clippy`

## Architecture Overview

This is a simplified Ray event processor that receives debug events from Ray PHP/Laravel applications and processes them through direct Rust implementations.

### Core Components

1. **HTTP Server** (src/server.rs): Listens on port 23517 for Ray events
2. **Event Processing** (src/events/): Direct Rust implementations for each event type
3. **Event Storage** (src/event_storage.rs): Central storage with structured logging support
4. **GUI**: (src/app.rs) using gpui framework

### Event Processing Flow

1. Server receives JSON payload with event data
2. Event type is matched to a processor in `src/events/`
3. Direct Rust function processes event and returns `EventEntry`
4. Result is stored in EventStorage and displayed in UI

### Adding New Event Types

1. Create new module in `src/events/` directory following existing pattern:
   ```
   src/events/my_event/mod.rs
   ```
2. Implement `EventProcessor` trait:
   ```rust
   impl EventProcessor for MyEventProcessor {
       fn process(&self, payload: &Value) -> Result<EventEntry> { ... }
       fn display_name(&self) -> &'static str { "My Event" }
   }
   ```
3. Add to the factory in `src/events/mod.rs`:
   ```rust
   "my_event" => Some(Box::new(my_event::MyEventProcessor)),
   ```

### Key Design Decisions

- **Direct Processing**: Simple Rust function calls for fast, debuggable event processing
- **GUI Interface**: macOS desktop application using gpui
- **Structured Logging**: Events support multiple log levels (Info, Warning, Error, Debug)
- **Markdown Support**: Application logs can render markdown content

# Rust coding guidelines

* Prioritize code correctness and clarity. Speed and efficiency are secondary priorities unless otherwise specified.
* Do not write organizational or comments that summarize the code. Comments should only be written in order to explain "why" the code is written in some way in the case there is a reason that is tricky / non-obvious.
* Prefer implementing functionality in existing files unless it is a new logical component. Avoid creating many small files.
* Avoid using functions that panic like `unwrap()`, instead use mechanisms like `?` to propagate errors.
* Be careful with operations like indexing which may panic if the indexes are out of bounds.
* Never silently discard errors with `let _ =` on fallible operations. Always handle errors appropriately:
  - Propagate errors with `?` when the calling function should handle them
  - Use `.log_err()` or similar when you need to ignore errors but want visibility
  - Use explicit error handling with `match` or `if let Err(...)` when you need custom logic
  - Example: avoid `let _ = client.request(...).await?;` - use `client.request(...).await?;` instead
* When implementing async operations that may fail, ensure errors propagate to the UI layer so users get meaningful feedback.
* Never create files with `mod.rs` paths - prefer `src/some_module.rs` instead of `src/some_module/mod.rs`.

# GPUI

GPUI is a UI framework which also provides primitives for state and concurrency management.
