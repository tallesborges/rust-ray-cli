# GPUI Development Guide

Based on official Zed examples from the gpui crate (current as of commit 8227c45a).

## Table of Contents

1. [Basic Setup](#basic-setup)
2. [Application Structure](#application-structure)
3. [Component/View Pattern](#componentview-pattern)
4. [Event Handling](#event-handling)
5. [Styling and Layout](#styling-and-layout)
6. [State Management](#state-management)
7. [Actions and Key Bindings](#actions-and-key-bindings)
8. [Common Patterns](#common-patterns)
9. [Advanced Features](#advanced-features)

## Basic Setup

### Essential Imports

```rust
use gpui::{
    App, Application, Bounds, Context, Window, WindowBounds, WindowOptions,
    div, prelude::*, px, rgb, size, actions
};
```

### Minimal Application

```rust
use gpui::{App, Application, Bounds, Context, Window, WindowBounds, WindowOptions, div, prelude::*, px, rgb, size};

struct HelloWorld;

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .bg(rgb(0x505050))
            .size(px(500.0))
            .justify_center()
            .items_center()
            .child("Hello, World!")
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| HelloWorld),
        ).unwrap();
        cx.activate(true);
    });
}
```

## Application Structure

### Application Lifecycle

1. **Application::new()** - Creates the app instance
2. **run()** - Starts the event loop with a closure receiving `&mut App`
3. **cx.open_window()** - Opens windows with options and a view factory
4. **cx.activate(true)** - Activates the application

### Window Creation Pattern

```rust
cx.open_window(
    WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(bounds)),
        titlebar: Some(TitlebarOptions {
            title: Some("My App".into()),
            ..Default::default()
        }),
        ..Default::default()
    },
    |_window, cx| cx.new(|_cx| MyView::new())
).unwrap();
```

## Component/View Pattern

### Basic Component Structure

```rust
struct MyComponent {
    // State fields
    data: String,
    selected: bool,
}

impl MyComponent {
    fn new() -> Self {
        Self {
            data: "Initial".into(),
            selected: false,
        }
    }
}

impl Render for MyComponent {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .child("Component content")
    }
}
```

### Component with State Updates

```rust
impl MyComponent {
    fn toggle_selection(&mut self, cx: &mut Context<Self>) {
        self.selected = !self.selected;
        cx.notify(); // Triggers re-render
    }
}
```

## Event Handling

### Mouse Events

```rust
div()
    .on_click(|event, window, cx| {
        // Handle click
    })
    .on_mouse_down(MouseButton::Left, cx.listener(|this, event, window, cx| {
        // Handle mouse down on component
        this.handle_mouse_down(cx);
    }))
    .on_mouse_move(cx.listener(|this, event, window, cx| {
        // Handle mouse move
    }))
```

### Hover States

```rust
div()
    .bg(rgb(0x007acc))
    .hover(|style| style.bg(rgb(0x005a9e)))
    .cursor_pointer()
```

### Focus Handling

```rust
use gpui::{FocusHandle, Focusable};

struct FocusableComponent {
    focus_handle: FocusHandle,
}

impl Focusable for FocusableComponent {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for FocusableComponent {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .track_focus(&self.focus_handle(cx))
            .key_context("MyComponent")
            .on_action(cx.listener(Self::handle_action))
    }
}
```

## Styling and Layout

### Flexbox Layout

```rust
div()
    .flex()              // Enable flexbox
    .flex_col()          // Column direction
    .flex_row()          // Row direction (default)
    .justify_center()    // Main axis alignment
    .items_center()      // Cross axis alignment
    .gap_4()            // Gap between items
    .flex_1()           // Flex grow
    .flex_none()        // No flex
```

### Sizing

```rust
div()
    .size_full()        // 100% width and height
    .w_full()           // 100% width
    .h_full()           // 100% height
    .size(px(200.0))    // Explicit size
    .w(px(300.0))       // Explicit width
    .h(px(150.0))       // Explicit height
    .size_32()          // Fixed size utilities
```

### Spacing

```rust
div()
    .p_4()              // Padding all sides
    .px_3()             // Horizontal padding
    .py_2()             // Vertical padding
    .pt_1()             // Padding top
    .m_4()              // Margin (similar pattern)
```

### Colors and Appearance

```rust
div()
    .bg(rgb(0x007acc))          // Background color
    .text_color(rgb(0xffffff))  // Text color
    .border_1()                 // Border width
    .border_color(rgb(0x000000)) // Border color
    .rounded_md()               // Border radius
    .shadow_lg()                // Drop shadow
```

### Color Utilities

```rust
use gpui::{red, green, blue, yellow, black, white, rgb, rgba, hsla};

div()
    .bg(gpui::blue())           // Predefined colors
    .bg(rgb(0x007acc))          // Hex colors
    .bg(rgba(0x007accff))       // RGBA
    .bg(hsla(0.6, 0.8, 0.5, 1.0)) // HSLA
```

### Overflow and Content Management

```rust
div()
    .overflow_hidden()      // Hide content that overflows (most common)
    .overflow_y_hidden()    // Hide overflow on Y axis only
    .overflow_x_hidden()    // Hide overflow on X axis only
```

**Overflow Handling Best Practices:**
- Use `.overflow_hidden()` on containers to prevent content from spilling out
- Essential for panels and fixed-size containers
- Prevents layout issues when content exceeds container bounds

**Layout Container Pattern:**
```rust
// Typical panel structure with proper overflow handling
div()
    .flex()
    .flex_col()
    .w_64()                 // Fixed width panel
    .h_full()               // Full height
    .overflow_hidden()      // Contain content
    .child(
        // Header section
        div()
            .p_4()
            .border_b_1()
            .child("Panel Header")
    )
    .child(
        // Scrollable content area
        div()
            .flex_1()           // Take remaining space
            .overflow_hidden()  // Contain list content
            .child(
                div()
                    .flex()
                    .flex_col()
                    .children(items.iter().map(|item| {
                        // Item rendering
                    }))
            )
    )
```

## State Management

### Local State

```rust
struct Counter {
    count: i32,
}

impl Counter {
    fn increment(&mut self, cx: &mut Context<Self>) {
        self.count += 1;
        cx.notify(); // Important: triggers re-render
    }
}
```

### Entity References

```rust
use gpui::Entity;

struct ParentComponent {
    child: Entity<ChildComponent>,
}

impl Render for ParentComponent {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().child(self.child.clone()) // Clone the entity reference
    }
}
```

### State Updates

```rust
// Update child component from parent
self.child.update(cx, |child, cx| {
    child.update_data("new data");
    cx.notify();
});
```

## Actions and Key Bindings

### Defining Actions

```rust
use gpui::actions;

actions!(my_app, [
    Quit,
    Save,
    Open,
    CustomAction,
]);
```

### Action Handlers

```rust
impl MyComponent {
    fn handle_quit(&mut self, _action: &Quit, _window: &mut Window, cx: &mut Context<Self>) {
        // Handle quit action
    }
}

impl Render for MyComponent {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .on_action(cx.listener(Self::handle_quit))
    }
}
```

### Key Bindings

```rust
use gpui::KeyBinding;

// In main function
cx.bind_keys([
    KeyBinding::new("cmd-q", Quit, None),
    KeyBinding::new("cmd-s", Save, None),
    KeyBinding::new("escape", Cancel, None),
]);

// Global action handlers
cx.on_action(|_: &Quit, cx| cx.quit());
```

## Common Patterns

### Button Component

```rust
fn button(text: &str, on_click: impl Fn(&mut Window, &mut App) + 'static) -> impl IntoElement {
    div()
        .px_4()
        .py_2()
        .bg(rgb(0x007acc))
        .text_color(rgb(0xffffff))
        .rounded_md()
        .cursor_pointer()
        .hover(|style| style.bg(rgb(0x005a9e)))
        .active(|style| style.opacity(0.85))
        .child(text.to_string())
        .on_click(move |_, window, cx| on_click(window, cx))
}
```

### List Rendering

```rust
div()
    .flex()
    .flex_col()
    .children(items.iter().enumerate().map(|(index, item)| {
        div()
            .p_2()
            .bg(if index % 2 == 0 { rgb(0xf0f0f0) } else { rgb(0xffffff) })
            .child(format!("Item: {}", item))
    }))
```

### Conditional Rendering

```rust
div()
    .when(condition, |div| {
        div.child("Conditional content")
    })
    .when_some(optional_value, |div, value| {
        div.child(format!("Value: {}", value))
    })
```

## Advanced Features

### Uniform Lists (Virtual Scrolling)

```rust
use gpui::uniform_list;

uniform_list(
    cx.entity().clone(),
    "list_items",
    item_count,
    |_this, range, _window, _cx| {
        range.map(|index| {
            div()
                .id(index)
                .child(format!("Item {}", index))
        }).collect()
    }
)
```

### Advanced Scrolling Patterns

**Basic Scrollable List Pattern:**
```rust
// Simple scrollable content area
div()
    .flex_1()
    .overflow_hidden()      // Contains content within bounds
    .child(
        div()
            .flex()
            .flex_col()
            .children(items.iter().enumerate().map(|(index, item)| {
                // Render each item
                div()
                    .id(("item", index))
                    .p_2()
                    .child(format!("Item {}: {}", index, item))
            }))
    )
```

**Scrolling with Selection State:**
```rust
// Managing selection in scrollable lists
div()
    .flex()
    .flex_col()
    .children(events.iter().enumerate().map(|(index, entry)| {
        let is_selected = self.selected_row == Some(index);
        let bg_color = if is_selected {
            rgb(0x094771)   // Selected color
        } else {
            rgb(0x252526)   // Default color
        };
        
        div()
            .id(("event", index))
            .p_2()
            .bg(bg_color)
            .hover(|style| style.bg(rgb(0x2a2d2e)))
            .cursor_pointer()
            .on_mouse_down(gpui::MouseButton::Left, cx.listener(move |this, _event, _window, cx| {
                this.select_row(index, cx);
            }))
            .child(entry.title.clone())
    }))
```

**Complex Scrolling with Track Handle:**
```rust
use gpui::UniformListScrollHandle;

struct ScrollableView {
    scroll_handle: UniformListScrollHandle,
}

impl ScrollableView {
    fn new() -> Self {
        Self {
            scroll_handle: UniformListScrollHandle::new(),
        }
    }
}

// In render method:
uniform_list(
    cx.entity().clone(),
    "scrollable_list",
    items.len(),
    |_this, range, _window, _cx| {
        // Generate items for visible range
        range.map(|index| {
            div().id(index).child(format!("Item {}", index))
        }).collect()
    }
)
.track_scroll(self.scroll_handle.clone())  // Enable scroll tracking
```

**Empty State Pattern:**
```rust
// Handling empty lists gracefully
div()
    .flex_1()
    .overflow_hidden()
    .child(
        if items.is_empty() {
            div()
                .flex()
                .items_center()
                .justify_center()
                .h_full()
                .text_color(rgb(0x969696))
                .child("No items yet...")
        } else {
            div()
                .flex()
                .flex_col()
                .children(items.iter().map(|item| {
                    // Render items
                }))
        }
    )
```

### Drag and Drop

```rust
div()
    .on_drag(drag_data, |data, position, _, cx| {
        cx.new(|_| DragPreview::new(data, position))
    })
    .on_drop(cx.listener(|this, data, _, _| {
        this.handle_drop(data);
    }))
```

### Custom Elements

For complex components, implement the `Element` trait directly:

```rust
impl Element for CustomElement {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn request_layout(&mut self, /* ... */) -> (LayoutId, Self::RequestLayoutState) {
        // Layout calculation
    }

    fn prepaint(&mut self, /* ... */) -> Self::PrepaintState {
        // Pre-paint setup
    }

    fn paint(&mut self, /* ... */) {
        // Actual painting
    }
}
```

### Window Management

```rust
// Open different window types
cx.open_window(WindowOptions {
    kind: WindowKind::PopUp,        // Popup window
    show: false,                    // Initially hidden
    is_movable: false,             // Can't be moved
    titlebar: None,                // Custom titlebar
    ..Default::default()
}, |_, cx| cx.new(|_| MyView));

// Window operations from within a view
window.remove_window();            // Close window
window.resize(size(px(800.0), px(600.0))); // Resize
```

### Async Operations

```rust
// Spawn async task
window.spawn(cx, async move |cx| {
    // Async work
    Timer::after(Duration::from_secs(1)).await;
    
    // Update UI from async context
    cx.update(|_, cx| {
        // UI updates
    })
}).detach();
```

## Best Practices

### Core Development Practices
1. **Always call `cx.notify()`** after state changes that should trigger re-renders
2. **Use `Entity<T>`** for component references and shared state
3. **Implement `Focusable`** for components that need keyboard input
4. **Use actions** instead of direct method calls for user interactions
5. **Prefer declarative styling** over imperative DOM manipulation
6. **Clone entity references** when passing to child components
7. **Use `when()` and `when_some()`** for conditional rendering
8. **Implement proper key contexts** for keyboard handling

### Layout and UI Best Practices
9. **Always use `.overflow_hidden()`** on fixed-size containers to prevent layout issues
10. **Use `.flex_1()`** for elements that should take remaining space in flex containers
11. **Combine `.h_full()` and `.overflow_hidden()`** for proper panel height management
12. **Use explicit IDs** for list items: `.id(("item_type", index))` for stable identity
13. **Handle empty states** gracefully with conditional rendering
14. **Use consistent color schemes** with predefined color variables
15. **Test with different content lengths** to ensure overflow handling works

### Event Handling Best Practices
16. **Use `cx.listener()` for component method callbacks** instead of direct closures
17. **Capture necessary state before closures** when using uniform_list or complex rendering
18. **Use `move` keyword in closures** when capturing values for event handlers
19. **Prefer `on_mouse_down` over `on_click`** for immediate response in lists

### Performance Considerations
20. **Use `uniform_list` for large datasets** (1000+ items) with virtual scrolling
21. **Use simple div rendering for small lists** (< 100 items) for simplicity
22. **Clone data efficiently** - use `Arc<T>` for shared immutable data
23. **Avoid deep nesting** in render methods to improve readability and performance

## Debugging Tips

### General Debugging
1. **Use `cx.notify()` liberally** during development to ensure re-renders
2. **Check focus handling** with `track_focus()` and proper focus handles
3. **Use explicit IDs** with `.id()` for elements that need stable identity
4. **Test different window bounds** and screen sizes
5. **Verify key bindings** with simple debug actions first

### Layout and Overflow Issues
6. **Check container heights** - use `.h_full()` on parent containers
7. **Verify overflow settings** - ensure `.overflow_hidden()` is applied to containers
8. **Test with varying content** - both empty and full lists to verify layout
9. **Use browser-like inspect** - add temporary border colors to debug layout:
   ```rust
   div()
       .border_1()
       .border_color(rgb(0xff0000))  // Temporary red border for debugging
   ```

### Event Handling Issues
10. **Check closure captures** - ensure all needed values are moved into closures
11. **Verify entity updates** - use `.update()` correctly when modifying state from callbacks
12. **Test click areas** - ensure clickable elements have proper size and hover states
13. **Debug with print statements** - add temporary logging in event handlers

### Performance Debugging
14. **Monitor re-render frequency** - excessive `cx.notify()` calls can cause performance issues
15. **Check list rendering performance** - switch between simple div and uniform_list approaches
16. **Profile with large datasets** - test with 100+ items to identify bottlenecks

### Common Pitfalls
17. **Missing `.clone()` calls** on data passed to closures
18. **Incorrect entity references** in async or callback contexts
19. **Forgetting `.into_any_element()`** when mixing different element types
20. **State not updating** - usually missing `cx.notify()` after state changes

## API Evolution Notes

This guide is based on gpui commit `8227c45a` from the Zed repository. The API is actively evolving, so:

- **Check official examples** in the Zed repository for the latest patterns
- **Test API changes** when updating gpui versions
- **Refer to Zed's UI components** for real-world usage examples
- **Join Zed's Discord** for community support and updates

The patterns documented here have been tested in production and should remain stable, but always verify against the latest examples when in doubt.