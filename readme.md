# WASM-Powered Ray Event Processor

A fast, flexible Ray event processor built with Rust and WASM. Tired of freezes in the official Ray app? This project offers a reliable alternative with a plugin architecture for custom event handling.

![Application Screenshot](images/image1.jpeg)

**Key Features:**

*   **WASM Plugins:** Extend functionality with `.wasm` modules. Just drop them into the `wasm-modules/` directory!
*   **Hot-Loading:** New event types are automatically detected.
*   **Sandboxed Execution:** WASM modules run in isolated environments for enhanced security.
*   **Native UI:** Built with [egui](https://github.com/emilk/egui) for a responsive, cross-platform experience.

**Development Highlights:**

This project served as a learning experience in Rust and WASM, resulting in a move from a native Rust implementation to a WASM-based plugin architecture. This enabled hot-reloading and sandboxed execution. Key steps included dynamic processor loading via FFI, WASM module hot-loading, and the creation of a common event processing interface.

**Usage:**

1.  **Ray Integration:** Apply the following patch to `vendor/spatie/ray/src/ArgumentConverter.php` to bypass Symfony tags:

    ```php
    // Bypass Synphony tags for direct processing
    return $argument;
    ```

2.  **Navigation:** Use the `↑` and `↓` keys to navigate table rows.

**Roadmap:**

-   ✅ Redis cache support
-   Line numbering
-   Label filtering
-   ✅ Request details in preview

**Evaluation:**

The following technologies/features are being evaluated for potential integration:

*   `egui_tracing`: For enhanced debugging and profiling.
*   `egui_code_editor`: For improved code display and editing within the UI.
*   `gpui`:  Exploring alternative UI frameworks for potential performance or feature benefits.
