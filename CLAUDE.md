# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## UI Framework Migration Status

✅ **COMPLETED**: The codebase has been successfully migrated from egui to gpui (Zed's UI framework).

### Requirements for gpui
- Full Xcode installation (for Metal shader compiler)
- Nightly Rust compiler (uses experimental trait upcasting)
- macOS (gpui is currently macOS-only)

### Current Status
- ✅ Dependencies updated to use gpui from Zed repository
- ✅ UI rewritten using gpui's declarative syntax
- ✅ Application builds and runs successfully
- ✅ Server functionality preserved
- ✅ TUI mode still available as fallback

### Known Issues
- Some UI methods (overflow scrolling) were removed due to API differences
- Keyboard navigation not yet implemented in new UI
- Some warnings about unused imports/methods

## Build Commands

### Building WASM Modules (Required before running)
```bash
./build-wasm.sh
```
This compiles all event processor crates to WebAssembly modules in the `wasm-modules/` directory.

### Running the Application
- GUI mode: `cargo run` (use `timeout 5s cargo run` for testing)
- TUI mode: `cargo run -- --tui`

### Development Commands
- Run tests: `cargo test`
- Check code: `cargo check`
- Format code: `cargo fmt`
- Lint code: `cargo clippy`

## Architecture Overview

This is a WASM-powered Ray event processor that receives debug events from Ray PHP/Laravel applications and processes them through dynamically loaded WebAssembly modules.

### Core Components

1. **HTTP Server** (src/server.rs): Listens on port 23517 for Ray events
2. **WASM Event Factory** (src/wasm_event_factory.rs): Loads and executes WASM modules based on event type
3. **Event Storage** (src/event_storage.rs): Central storage with structured logging support
4. **Dual UI**: GUI (src/app.rs) using egui/eframe and TUI (src/tui.rs) using ratatui

### Event Processing Flow

1. Server receives JSON payload with event data
2. Event type determines which WASM module to load from `wasm-modules/`
3. WASM module processes event and returns `EventEntry`
4. Result is stored in EventStorage and displayed in UI

### Adding New Event Types

1. Create new crate in `crates/` directory following existing pattern
2. Implement `EventProcessor` trait with `no_std` compatibility
3. Export required FFI functions: `process_event` and `free_string`
4. Add crate to workspace in root `Cargo.toml`
5. Run `./build-wasm.sh` to compile new module

### Key Design Decisions

- **WASM Modules**: Enable hot-loading and sandboxed execution of event processors
- **Dual Interface**: Support both desktop GUI and terminal environments
- **Structured Logging**: Events support multiple log levels (Info, Warning, Error, Debug)
- **Markdown Support**: Application logs can render markdown content
