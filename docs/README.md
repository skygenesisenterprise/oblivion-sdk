# Oblivion UI Documentation

Welcome to the official documentation for Oblivion UI, a SwiftUI-inspired Rust framework for building native user interfaces on OblivionOS.

## What is Oblivion UI?

Oblivion UI is a declarative, component-based UI framework written in Rust. It provides a SwiftUI-like API for creating native applications with reactive state management, event handling, and hardware-accelerated rendering via SDL2.

## Key Concepts

### Declarative Programming
Instead of manually manipulating UI elements, you describe what your UI should look like based on your application state. The framework handles the rest.

### Reactive State
UI automatically updates when underlying data changes, eliminating the need for manual DOM manipulation.

### Component-Based Architecture
Build complex UIs by composing smaller, reusable components.

## Quick Start

```rust
use oblivion_ui::components::{Window, VStack, Button, Label};
use oblivion_ui::state::State;
use oblivion_ui::rendering::SDLEngine;
use oblivion_ui::themes::Theme;

fn main() -> Result<(), String> {
    // Create reactive state
    let counter = State::new("0".to_string());

    // Build UI hierarchy
    let mut window = Window::new("Counter App".to_string(), 400, 300);
    let mut vstack = VStack::new(20.0).padding(20.0);

    // Add components
    let label = Label::new(counter.binding());
    vstack.add_child(Box::new(label));

    let button = Button::new("Increment".to_string())
        .on_click(move || {
            let current: i32 = counter.get().parse().unwrap_or(0);
            counter.set((current + 1).to_string());
        });
    vstack.add_child(Box::new(button));

    window.add_child(Box::new(vstack));

    // Run the app
    let theme = Theme::default();
    let mut engine = SDLEngine::new("Counter App", 400, 300)?;
    engine.run(Box::new(window), &theme)
}
```

## Documentation Structure

- **[Components](components.md)**: Available UI components and how to use them
- **[State Management](state.md)**: Reactive state with @State and @Binding
- **[Rendering](rendering.md)**: How rendering works and customization
- **[Themes](themes.md)**: Customizing appearance
- **[Events](events.md)**: Handling user interactions
- **[Extending the Framework](extending.md)**: Creating custom components
- **[Examples](examples.md)**: Complete application examples
- **[API Reference](api.md)**: Detailed API documentation

## Architecture Overview

Oblivion UI follows a layered architecture:

1. **Components Layer**: UI building blocks (buttons, labels, layouts)
2. **State Layer**: Reactive data management
3. **Rendering Layer**: SDL2-based graphics rendering
4. **Event Layer**: Input handling and propagation

## Platform Support

- **Primary**: Linux (OblivionOS/Debian-based)
- **Architecture**: x86_64, ARM64
- **Rendering**: SDL2 with OpenGL/Vulkan support

## Contributing

See the main README for contribution guidelines.

## License

MIT License