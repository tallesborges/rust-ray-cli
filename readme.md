# Project Overview

Born from frustration with frequent freezes in the official Ray application, this project implements a WASM-based plugin architecture for reliable event processing. Originally created before and later enhanced during a Rust/Substrate course.

![Application Screenshot](images/image1.jpeg)

**Architecture Highlights**:
- 🖥️ **egui UI** - Built with [egui](https://github.com/emilk/egui) for native cross-platform rendering
- 🧩 WASM-powered plugins - Add new event processors by simply dropping `.wasm` files in the `wasm-modules/` directory
- ⚡ Hot-loading - New event types are automatically detected and integrated  
- 🔐 Sandboxed execution - WASM modules run in isolated environments for security

The WASM implementation was refined during the course, applying learnings about portable, secure execution environments to create a flexible plugin system that avoids traditional compilation cycles.

## Development Journey

### Key Milestones
- 🌱 **Initial Implementation**: Created while learning Rust as a native application with hardcoded event processors
- 🎓 **Course Evolution**: During Rust/Substrate training, migrated to WASM-based plugins for:
  - Hot-reloading capabilities
  - Sandboxed execution environments
  - Substrate-inspired runtime module loading
- 🚀 **WASM Integration**: Achieved dynamic processor loading through:
  - FFI interface macros
  - WASM module hot-loading
  - Common event processing interface

## Features

### UI Features
- [x] ✅ Colorized content preview
- [x] ✅ Copy button for content
- [x] ✅ Keyboard navigation (arrow keys)
- [x] ✅ CommonMark rendering via egui_commonmark (chosen for full Markdown support including code blocks and easy customization)

### WASM Integration
- [x] ✅ Timestamp handling
- [x] ✅ Exception support

### Upcoming Features
- [ ] Redis cache support
- [ ] Line numbering
- [ ] Label filtering
- [ ] egui_tracing integration
- [ ] egui_code_editor evaluation
- [ ] Request details in responses

## Usage Guide

### Ray Integration
While significantly faster than the official [Ray app](https://myray.app/), this requires a one-time modification to `vendor/spatie/ray/src/ArgumentConverter.php`:
```php
// Bypass Synphony tags for direct processing
return $argument;
```

### Application Controls
| Key       | Action                  |
|-----------|-------------------------|
| ↑ ↓       | Navigate table rows     |
