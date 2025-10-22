# API Reference

Complete API documentation for Oblivion UI.

## Components

### Component Trait

```rust
pub trait Component {
    fn render(&self, renderer: &mut dyn Renderer, theme: &Theme);
    fn handle_event(&mut self, event: &Event);
}
```

### Window

Root application container.

```rust
pub struct Window {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub children: Vec<Box<dyn Component>>,
}

impl Window {
    pub fn new(title: String, width: u32, height: u32) -> Self
    pub fn add_child(&mut self, child: Box<dyn Component>)
}
```

### VStack

Vertical layout container.

```rust
pub struct VStack {
    pub children: Vec<Box<dyn Component>>,
    pub spacing: f32,
    pub padding: f32,
    pub border: f32,
}

impl VStack {
    pub fn new(spacing: f32) -> Self
    pub fn padding(self, padding: f32) -> Self
    pub fn border(self, border: f32) -> Self
    pub fn add_child(&mut self, child: Box<dyn Component>)
}
```

### HStack

Horizontal layout container.

```rust
pub struct HStack {
    pub children: Vec<Box<dyn Component>>,
    pub spacing: f32,
    pub padding: f32,
    pub border: f32,
}

impl HStack {
    pub fn new(spacing: f32) -> Self
    pub fn padding(self, padding: f32) -> Self
    pub fn border(self, border: f32) -> Self
    pub fn add_child(&mut self, child: Box<dyn Component>)
}
```

### Grid

2D grid layout.

```rust
pub struct Grid {
    pub children: Vec<Vec<Option<Box<dyn Component>>>>,
    pub rows: usize,
    pub cols: usize,
    pub spacing: f32,
}

impl Grid {
    pub fn new(rows: usize, cols: usize, spacing: f32) -> Self
    pub fn set_child(&mut self, row: usize, col: usize, child: Box<dyn Component>)
}
```

### Panel

Container with border and padding.

```rust
pub struct Panel {
    pub child: Option<Box<dyn Component>>,
    pub border_width: f32,
    pub padding: f32,
}

impl Panel {
    pub fn new(border_width: f32, padding: f32) -> Self
    pub fn child(self, child: Box<dyn Component>) -> Self
}
```

### Button

Clickable button with text.

```rust
pub struct Button {
    pub label: String,
    pub on_click: Option<Box<dyn FnMut()>>,
    pub padding: f32,
    pub border: f32,
}

impl Button {
    pub fn new(label: String) -> Self
    pub fn on_click<F>(self, f: F) -> Self where F: FnMut() + 'static
    pub fn padding(self, padding: f32) -> Self
    pub fn border(self, border: f32) -> Self
}
```

### Label

Text display component.

```rust
pub struct Label {
    pub text: Binding<String>,
    pub padding: f32,
}

impl Label {
    pub fn new(text: Binding<String>) -> Self
    pub fn padding(self, padding: f32) -> Self
}
```

### Toggle

Boolean switch component.

```rust
pub struct Toggle {
    pub is_on: Binding<bool>,
    pub on_toggle: Option<Box<dyn FnMut(bool)>>,
}

impl Toggle {
    pub fn new(is_on: Binding<bool>) -> Self
    pub fn on_toggle<F>(self, f: F) -> Self where F: FnMut(bool) + 'static
}
```

### Input

Text input field.

```rust
pub struct Input {
    pub text: Binding<String>,
    pub placeholder: String,
}

impl Input {
    pub fn new(text: Binding<String>, placeholder: String) -> Self
}
```

## State Management

### State<T>

Reactive state container.

```rust
pub struct State<T> {
    value: Rc<RefCell<T>>,
}

impl<T> State<T> {
    pub fn new(initial: T) -> Self
    pub fn get(&self) -> T where T: Clone
    pub fn set(&self, new_value: T)
    pub fn binding(&self) -> Binding<T>
}
```

### Binding<T>

Read/write access to state.

```rust
pub struct Binding<T> {
    value: Rc<RefCell<T>>,
}

impl<T> Binding<T> {
    pub fn get(&self) -> T where T: Clone
    pub fn set(&self, new_value: T)
}
```

## Rendering

### Renderer Trait

Abstract rendering interface.

```rust
pub trait Renderer {
    fn draw_text(&mut self, text: &str, x: f32, y: f32);
    fn draw_rect(&mut self, x: f32, y: f32, w: f32, h: f32);
}
```

### SDLEngine

SDL2-based rendering engine.

```rust
pub struct SDLEngine {
    sdl_context: Sdl,
    canvas: Canvas<Window>,
    ttf_context: Sdl2TtfContext,
    font: sdl2::ttf::Font<'static, 'static>,
}

impl SDLEngine {
    pub fn new(title: &str, width: u32, height: u32) -> Result<Self, String>
    pub fn run(&mut self, root_component: Box<dyn Component>, theme: &Theme) -> Result<(), String>
}
```

## Themes

### Theme

Application theming.

```rust
#[derive(Clone)]
pub struct Theme {
    pub primary_color: (u8, u8, u8),
    pub secondary_color: (u8, u8, u8),
    pub background_color: (u8, u8, u8),
    pub text_color: (u8, u8, u8),
    pub font_size: u32,
}

impl Default for Theme {
    fn default() -> Self
}
```

## Events

### Event Enum

User interaction events.

```rust
pub enum Event {
    Click { x: f32, y: f32 },
    Hover { x: f32, y: f32 },
    KeyPress(char),
    Drag { dx: f32, dy: f32 },
}
```

## Modules

### lib.rs

Main library exports.

```rust
pub mod components;
pub mod state;
pub mod rendering;
pub mod themes;
```

## Type Aliases

No public type aliases defined.

## Constants

No public constants defined.

## Macros

No public macros defined.

## Error Types

Functions return `Result<T, String>` for errors.

## Platform-specific Notes

- **Linux**: Requires SDL2 and SDL2_ttf development libraries
- **macOS**: SDL2 available via Homebrew
- **Windows**: SDL2 available via vcpkg or MSYS2

## Memory Management

- Components use `Box<dyn Component>` for dynamic dispatch
- State uses `Rc<RefCell<T>>` for shared mutable access
- Bindings are lightweight clones of state references

## Thread Safety

Currently single-threaded. State is not `Send` or `Sync`.

## Performance Characteristics

- Rendering: O(n) where n is number of components
- State updates: O(1) for simple updates
- Event propagation: O(depth) of component tree

## Version Compatibility

API follows semantic versioning. Breaking changes will increment major version.

## Deprecation Notices

No deprecated APIs in current version.