# Rust Ray CLI

A high-performance debugging tool for monitoring application events and logs, built with Rust. Native macOS implementation of the Ray debugging tool with HTTP server and GPUI-based GUI.

![Application Screenshot](images/image1.jpeg)

## Features

- **Native macOS GUI**: Built with [GPUI](https://github.com/zed-industries/gpui) for optimal performance
- **HTTP Server**: Receives event payloads on port 23517
- **Event Types**: HTTP requests, cache operations, logs, queries, exceptions, and application logs
- **Real-time Filtering**: Filter events by type with optimized performance
- **Virtual Scrolling**: Handle thousands of events without performance degradation
- **Memory Efficient**: Arc-based storage minimizes cloning and memory usage
- **Performance Optimized**: Processes 1000+ events/second, maintains 60 FPS with 10,000+ events

## Requirements

- **macOS only** (GPUI is currently macOS-specific)
- **Xcode** (for Metal shader compiler)
- **Rust 1.70+** with nightly features

## Quick Start

```bash
# Clone and build
git clone https://github.com/yourusername/rust-ray-cli.git
cd rust-ray-cli
cargo build --release

# Run the application
cargo run --release
```

## Usage

1. **Start the application**: The GUI opens and HTTP server starts on `http://127.0.0.1:23517`

2. **Send events**: POST JSON payloads to the server:

```bash
curl -X POST http://127.0.0.1:23517/ \
  -H "Content-Type: application/json" \
  -d '{
    "payloads": [{
      "type": "log",
      "content": {
        "values": ["Debug message", "Additional info"]
      }
    }]
  }'
```

3. **View events**: Events appear in real-time with filtering options

### Ray PHP/Laravel Integration

Configure Ray to send events to `localhost:23517`:

```php
// In ray.php config
'port' => 23517,
'host' => 'localhost',
```

**Note:** You may need to patch `vendor/spatie/ray/src/ArgumentConverter.php` to bypass Symfony tags.

## Project Structure

```
src/
├── main.rs           # Application entry point
├── app.rs            # GUI application logic
├── server.rs         # HTTP server implementation
├── event_storage.rs  # Event storage and management
├── events/           # Event processing modules
│   ├── processors/   # JSON processors for each event type
│   └── types.rs      # Event data structures
└── ui_components.rs  # UI components and rendering
```

## Performance

Run benchmarks to test performance:

```bash
cargo bench
```

Optimizations include:
- Arc-based storage to minimize allocations
- Virtual scrolling for large event lists
- Cached filtering with hash-based invalidation
- Optimized JSON processing with size limits

## Development

```bash
# Run tests
cargo test

# Check code
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy
```

### Navigation

Use `↑` and `↓` arrow keys to navigate events in the GUI.

## Contributing

Contributions are welcome! Please follow the existing architecture patterns and code style.

## License

This project is open source and available under the [MIT License](LICENSE).

## Resources

- [GPUI Framework](https://github.com/zed-industries/zed/tree/main/crates/gpui)
- [Ray Documentation](https://spatie.be/docs/ray)
