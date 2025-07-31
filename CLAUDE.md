# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

```bash
# Build the project (requires macOS + Xcode for Metal shaders)
cargo build --release

# Run the application
cargo run --release

# Run tests
cargo test

# Run benchmarks
cargo bench

# Check code without building
cargo check

# Format code
cargo fmt

# Lint with clippy
cargo clippy

# Run specific test
cargo test test_name -- --exact

# Build with performance profiling
cargo build --profile release-fast
```

## Architecture Overview

This is a native macOS debugging tool built with Rust and GPUI. The application consists of:

### Core Components

1. **HTTP Server** (`server.rs`)
   - Runs on port 23517
   - Receives JSON event payloads from Ray clients
   - Processes events asynchronously with Tokio
   - Graceful shutdown via oneshot channel

2. **GUI Application** (`app.rs`, `main.rs`)
   - Built with GPUI framework (macOS-only)
   - Virtual scrolling for performance with large event lists
   - Real-time event filtering by type
   - Keyboard navigation support

3. **Event Storage** (`event_storage.rs`)
   - Arc-based shared storage for thread safety
   - Minimizes allocations and cloning
   - Supports multiple event types with enum dispatch
   - Performance-optimized with caching

4. **Event Processing** (`events/` module)
   - Modular processors for each event type
   - Event types: HTTP, Cache, Log, Query, Exception, ApplicationLog
   - JSON deserializers with validation
   - Unified `EventEntry` structure

### Key Design Patterns

- **Shared State**: `Arc<EventStorage>` shared between server and GUI threads
- **Message Passing**: Server processes events and stores them; GUI polls for updates
- **Virtual Rendering**: Only visible events are rendered (handles 10K+ events)
- **Filter Caching**: Hash-based cache invalidation for filtered event lists
- **Zero-Copy Where Possible**: Arc usage minimizes string/data cloning

### Performance Considerations

- Target: 1000+ events/second processing
- Maintain 60 FPS with 10,000+ events displayed
- Memory-efficient Arc-based storage
- Optimized JSON processing with size limits (10MB max)
- Virtual scrolling prevents rendering all events

### Important Implementation Notes

1. **Rust Nightly Required**: The project uses nightly Rust (see `rust-toolchain.toml`)
2. **macOS Only**: GPUI framework is macOS-specific and requires Xcode
3. **Event Processing**: All events flow through `process_event()` in `event_storage.rs`
4. **UI Updates**: GUI polls event storage; no direct messaging from server
5. **Error Handling**: Most functions use `Result` types; avoid `.unwrap()` in production code

### Testing Strategy

- Unit tests for event processors in `events/processors/`
- Integration tests for server endpoints
- Performance benchmarks in `benches/performance_benchmarks.rs`
- Manual validation tests in `tests/`

### Common Development Tasks

When adding a new event type:
1. Add variant to `EventType` enum in `events/event_type.rs`
2. Create processor module in `events/processors/`
3. Add processor to match statement in `event_storage::process_event()`
4. Update `EventType::all()` method
5. Add corresponding data structure in `events/types.rs`

When optimizing performance:
1. Run benchmarks first: `cargo bench`
2. Profile with Instruments (macOS)
3. Check for unnecessary cloning or allocations
4. Consider Arc usage for shared data
5. Validate with performance tests in `tests/`