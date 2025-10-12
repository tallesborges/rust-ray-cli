# RustRay CLI

A native macOS debugging tool built with Rust and GPUI that receives and displays events from Ray clients.

## Core Commands

- Build (requires macOS + Xcode for Metal shaders): `cargo build --release`
- Run application: `cargo run --release`
- Run full test suite: `cargo test`
- Run benchmarks: `cargo bench`
- Type-check without building: `cargo check`
- Auto-fix style: `cargo fmt`
- Lint: `cargo clippy`
- Run specific test: `cargo test test_name -- --exact`
- Build with performance profiling: `cargo build --profile release-fast`

## Project Layout

```
├── src/
│   ├── main.rs          → Entry point, spawns server and GUI threads
│   ├── app.rs           → GPUI application with virtual scrolling
│   ├── server.rs        → HTTP server on port 23517
│   ├── event_storage.rs → Arc-based shared event storage
│   └── events/
│       ├── event_type.rs     → EventType enum
│       ├── processors/       → Event type processors
│       └── types.rs          → Event data structures
├── tests/               → Integration and validation tests
├── benches/            → Performance benchmarks
└── target/             → Build artifacts (gitignored)
```

- All event processing logic lives in `src/events/processors/`
- GUI code is exclusively in `src/app.rs`
- Server code is exclusively in `src/server.rs`
- Shared state management in `src/event_storage.rs`

## Development Patterns & Constraints

### Coding Style
- Rust nightly required (see `rust-toolchain.toml`)
- Follow standard Rust conventions: snake_case for functions/variables, PascalCase for types
- Avoid `.unwrap()` in production code; use `Result` types for error handling
- Minimize cloning; prefer `Arc` for shared data
- Use `async`/`await` with Tokio for concurrent operations

### Platform Requirements
- **macOS only**: GPUI framework requires macOS and Xcode for Metal shader compilation
- Rust nightly toolchain must be installed
- Xcode command line tools required

### Performance Targets
- Process 1000+ events/second
- Maintain 60 FPS with 10,000+ displayed events
- Virtual scrolling prevents rendering all events
- JSON payload size limit: 10MB per event

### Architecture Patterns
- **Shared State**: `Arc<EventStorage>` shared between server and GUI threads
- **Message Passing**: Server processes events and stores them; GUI polls for updates
- **Virtual Rendering**: Only visible events are rendered for performance
- **Filter Caching**: Hash-based cache invalidation for filtered event lists
- **Zero-Copy**: Arc usage minimizes string/data cloning

## Adding New Event Types

When adding a new event type:

1. Add variant to `EventType` enum in `src/events/event_type.rs`
2. Create processor module in `src/events/processors/` (e.g., `my_event.rs`)
3. Add processor to match statement in `event_storage::process_event()`
4. Update `EventType::all()` method to include new type
5. Add corresponding data structure in `src/events/types.rs`
6. Create unit tests in the processor module
7. Add integration test if needed

Example processor structure:
```rust
use serde_json::Value;
use crate::events::types::{EventEntry, EventType};

pub fn process(payload: &Value) -> Option<EventEntry> {
    // Deserialize and validate
    // Return EventEntry with appropriate type
}
```

## Testing Strategy

- **Unit tests**: Test event processors in `src/events/processors/`
- **Integration tests**: Test server endpoints in `tests/`
- **Performance benchmarks**: Run `cargo bench` to validate performance targets
- **Manual validation**: Use test scripts as needed

Before submitting changes:
1. Run `cargo clippy` and fix all warnings
2. Run `cargo test` and ensure all tests pass
3. Run `cargo bench` if performance-critical changes were made
4. Verify application launches and receives events correctly

## Git Workflow Essentials

1. Branch from `main` with descriptive name: `feature/<name>` or `fix/<name>`
2. Run `cargo clippy` and `cargo test` before committing
3. Keep commits focused and atomic
4. Use conventional commit messages when possible (e.g., `feat:`, `fix:`, `refactor:`)

## External Services & Configuration

- HTTP server runs on `localhost:23517`
- No external services or API keys required
- No environment variables needed for basic operation
- Server accepts JSON payloads from Ray clients

## Gotchas & Common Issues

- **Metal shaders**: Building requires Xcode to be installed; build will fail without it
- **GPUI platform**: Code will not compile on Linux or Windows
- **Event ordering**: Events are displayed in order received; no server-side reordering
- **Memory growth**: With 10K+ events, memory usage increases; consider implementing event pruning for production
- **Nightly Rust**: Must use nightly toolchain; stable will not work
- **JSON size limits**: Events larger than 10MB are rejected to prevent memory issues

## Performance Optimization

When optimizing performance:

1. Run benchmarks first: `cargo bench` to establish baseline
2. Profile with Instruments (macOS) or `cargo flamegraph`
3. Check for unnecessary cloning (search for `.clone()`)
4. Consider `Arc` usage for shared data
5. Validate changes with performance tests
6. Ensure virtual scrolling still works correctly

Target metrics:
- Event processing: < 1ms per event
- UI frame time: < 16ms (60 FPS)
- Memory per event: < 1KB average
