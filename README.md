# Oblivion UI SDK

A SwiftUI-inspired Rust SDK for building native user interfaces on OblivionOS, a Debian-based Linux distribution inspired by macOS.

## Overview

Oblivion UI provides a declarative, component-based framework for creating native applications with a SwiftUI-like API. It supports reactive state management, event handling, and rendering via SDL2.

## Features

- **Declarative Components**: Window, VStack, HStack, Grid, Panel, Button, Label, Toggle, Input
- **Reactive State**: @State and @Binding for automatic UI updates
- **Event Handling**: Click, hover, keyboard, drag events
- **Theming**: Customizable themes with colors and fonts
- **Layout**: Spacing, padding, borders for flexible layouts
- **Rendering**: SDL2-based native rendering with OpenGL/Vulkan support
- **Cross-Platform**: Designed for Desktop and ARM architectures

## Architecture

The SDK is structured as a Cargo workspace:

```
oblivion-sdk/
├── oblivion_ui/          # Core library crate
│   ├── src/
│   │   ├── lib.rs        # Main library entry
│   │   ├── components.rs # UI components
│   │   ├── state.rs      # State management
│   │   ├── rendering.rs  # SDL2 rendering engine
│   │   └── themes.rs     # Theme definitions
│   └── Cargo.toml
├── examples/             # Example applications
│   └── simple_app/
└── Cargo.toml            # Workspace configuration
```

## Getting Started

### Prerequisites

- Rust 1.70+
- SDL2 development libraries
- SDL2_ttf for text rendering

On Ubuntu/Debian:
```bash
sudo apt-get install libsdl2-dev libsdl2-ttf-dev
```

### Adding to Your Project

Add to your `Cargo.toml`:
```toml
[dependencies]
oblivion_ui = { path = "../oblivion_ui" }
```

### Basic Example

```rust
use oblivion_ui::components::{Window, VStack, Button, Label};
use oblivion_ui::state::State;
use oblivion_ui::rendering::SDLEngine;
use oblivion_ui::themes::Theme;

fn main() -> Result<(), String> {
    let counter = State::new("0".to_string());

    let mut window = Window::new("My App".to_string(), 800, 600);
    let mut vstack = VStack::new(10.0).padding(20.0);

    let label = Label::new(counter.binding());
    vstack.add_child(Box::new(label));

    let button = Button::new("Increment".to_string())
        .on_click(move || {
            let current: i32 = counter.get().parse().unwrap_or(0);
            counter.set((current + 1).to_string());
        });
    vstack.add_child(Box::new(button));

    window.add_child(Box::new(vstack));

    let theme = Theme::default();
    let mut engine = SDLEngine::new("My App", 800, 600)?;
    engine.run(Box::new(window), &theme)
}
```

## Components

### Window

The root container for your application.

```rust
let window = Window::new("Title".to_string(), width, height);
```

### Layout Components

#### VStack
Vertical stack of components.

```rust
let mut vstack = VStack::new(spacing)
    .padding(10.0)
    .border(2.0);
vstack.add_child(Box::new(component));
```

#### HStack
Horizontal stack of components.

```rust
let mut hstack = HStack::new(spacing)
    .padding(10.0)
    .border(2.0);
hstack.add_child(Box::new(component));
```

#### Grid
2D grid layout.

```rust
let mut grid = Grid::new(rows, cols, spacing);
grid.set_child(row, col, Box::new(component));
```

#### Panel
Container with optional border and padding.

```rust
let panel = Panel::new(border_width, padding)
    .child(Box::new(component));
```

### Interactive Components

#### Button
Clickable button with label.

```rust
let button = Button::new("Click me".to_string())
    .padding(5.0)
    .border(1.0)
    .on_click(|| println!("Clicked!"));
```

#### Label
Displays text, bound to reactive state.

```rust
let label = Label::new(state.binding())
    .padding(5.0);
```

#### Toggle
On/off switch.

```rust
let toggle = Toggle::new(state.binding())
    .on_toggle(|is_on| println!("Toggled: {}", is_on));
```

#### Input
Text input field.

```rust
let input = Input::new(text_binding, "Placeholder".to_string());
```

## State Management

### @State
Local reactive state.

```rust
let count = State::new(0);
count.set(count.get() + 1); // Triggers redraw
```

### @Binding
Shared state between components.

```rust
let binding = state.binding();
let value = binding.get();
binding.set(new_value);
```

## Theming

Customize appearance with themes.

```rust
let theme = Theme {
    primary_color: (255, 0, 0),
    background_color: (255, 255, 255),
    text_color: (0, 0, 0),
    font_size: 14,
};
```

## Events

Components receive events through the `handle_event` method.

Supported events:
- `Click { x, y }`: Mouse click
- `Hover { x, y }`: Mouse hover
- `KeyPress(char)`: Keyboard input
- `Drag { dx, dy }`: Drag motion

## Rendering

The SDK uses SDL2 for rendering. The `SDLEngine` manages the event loop and canvas.

```rust
let mut engine = SDLEngine::new(title, width, height)?;
engine.run(root_component, &theme)?;
```

## Implementing Custom Components

To create a custom component:

1. Implement the `Component` trait:

```rust
use crate::components::{Component, Renderer, Event};
use crate::themes::Theme;

pub struct MyComponent {
    // fields
}

impl Component for MyComponent {
    fn render(&self, renderer: &mut dyn Renderer, theme: &Theme) {
        // Draw your component
    }

    fn handle_event(&mut self, event: &Event) {
        // Handle events
    }
}
```

2. Add to your layout:

```rust
let my_component = Box::new(MyComponent::new());
vstack.add_child(my_component);
```

## Extending the SDK

### Adding New Components

Add new structs and implementations in `components.rs`.

### Custom Rendering

Extend the `Renderer` trait for different backends.

### System Services

Add services in a new `services.rs` module:

```rust
pub mod services {
    pub struct Auth;
    pub struct Store;
    // etc.
}
```

## Examples

See `examples/simple_app/` for a complete application.

## Building and Running

```bash
# Build the SDK
cargo build

# Run an example
cargo run -p simple_app

# Build for ARM
cargo build --target aarch64-unknown-linux-gnu
```

## API Reference

### Components

#### Trait Component
```rust
pub trait Component {
    fn render(&self, renderer: &mut dyn Renderer, theme: &Theme);
    fn handle_event(&mut self, event: &Event);
}
```

#### Struct Window
- `new(title: String, width: u32, height: u32) -> Self`
- `add_child(&mut self, child: Box<dyn Component>)`

#### Struct VStack
- `new(spacing: f32) -> Self`
- `padding(self, padding: f32) -> Self`
- `border(self, border: f32) -> Self`
- `add_child(&mut self, child: Box<dyn Component>)`

#### Struct HStack
- Similar to VStack

#### Struct Grid
- `new(rows: usize, cols: usize, spacing: f32) -> Self`
- `set_child(&mut self, row: usize, col: usize, child: Box<dyn Component>)`

#### Struct Panel
- `new(border_width: f32, padding: f32) -> Self`
- `child(self, child: Box<dyn Component>) -> Self`

#### Struct Button
- `new(label: String) -> Self`
- `on_click<F>(self, f: F) -> Self where F: FnMut() + 'static`
- `padding(self, padding: f32) -> Self`
- `border(self, border: f32) -> Self`

#### Struct Label
- `new(text: Binding<String>) -> Self`
- `padding(self, padding: f32) -> Self`

#### Struct Toggle
- `new(is_on: Binding<bool>) -> Self`
- `on_toggle<F>(self, f: F) -> Self where F: FnMut(bool) + 'static`

#### Struct Input
- `new(text: Binding<String>, placeholder: String) -> Self`

### State

#### Struct State<T>
- `new(initial: T) -> Self`
- `get(&self) -> T where T: Clone`
- `set(&self, new_value: T)`
- `binding(&self) -> Binding<T>`

#### Struct Binding<T>
- `get(&self) -> T where T: Clone`
- `set(&self, new_value: T)`

### Rendering

#### Trait Renderer
- `draw_text(&mut self, text: &str, x: f32, y: f32)`
- `draw_rect(&mut self, x: f32, y: f32, w: f32, h: f32)`

#### Struct SDLEngine
- `new(title: &str, width: u32, height: u32) -> Result<Self, String>`
- `run(&mut self, root_component: Box<dyn Component>, theme: &Theme) -> Result<(), String>`

### Themes

#### Struct Theme
- `primary_color: (u8, u8, u8)`
- `secondary_color: (u8, u8, u8)`
- `background_color: (u8, u8, u8)`
- `text_color: (u8, u8, u8)`
- `font_size: u32`

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Submit a pull request

## License

This project is licensed under the MIT License.

## Roadmap

- [ ] Animation system
- [ ] System services (Auth, Store, Settings, Notifications)
- [ ] WebView integration
- [ ] Vulkan renderer
- [ ] More components (List, ScrollView, etc.)
- [ ] Accessibility support
- [ ] Internationalization
