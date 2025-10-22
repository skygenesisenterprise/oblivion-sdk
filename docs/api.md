# API Reference

Complete API documentation for Oblivion UI.

## Views

### View Trait

```rust
pub trait View {
    fn render(&self, renderer: &mut dyn Renderer, theme: &Theme, x: f32, y: f32);
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
    pub children: Vec<Box<dyn View>>,
}

impl Window {
    pub fn new(title: String, width: u32, height: u32) -> Self
    pub fn add_child(&mut self, child: Box<dyn View>)
}
```

### VStack

Vertical layout container.

```rust
pub struct VStack {
    pub children: Vec<Box<dyn View>>,
    pub spacing: f32,
    pub padding: f32,
    pub border: f32,
}

impl VStack {
    pub fn new(spacing: f32) -> Self
    pub fn padding(self, padding: f32) -> Self
    pub fn border(self, border: f32) -> Self
    pub fn add_child(&mut self, child: Box<dyn View>)
}
```

### HStack

Horizontal layout container.

```rust
pub struct HStack {
    pub children: Vec<Box<dyn View>>,
    pub spacing: f32,
    pub padding: f32,
    pub border: f32,
}

impl HStack {
    pub fn new(spacing: f32) -> Self
    pub fn padding(self, padding: f32) -> Self
    pub fn border(self, border: f32) -> Self
    pub fn add_child(&mut self, child: Box<dyn View>)
}
```

### Grid

2D grid layout.

```rust
pub struct Grid {
    pub children: Vec<Vec<Option<Box<dyn View>>>>,
    pub rows: usize,
    pub cols: usize,
    pub spacing: f32,
}

impl Grid {
    pub fn new(rows: usize, cols: usize, spacing: f32) -> Self
    pub fn set_child(&mut self, row: usize, col: usize, child: Box<dyn View>)
}
```

### Panel

Container with border and padding.

```rust
pub struct Panel {
    pub child: Option<Box<dyn View>>,
    pub border_width: f32,
    pub padding: f32,
}

impl Panel {
    pub fn new(border_width: f32, padding: f32) -> Self
    pub fn child(self, child: Box<dyn View>) -> Self
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

### Text

Text display component.

```rust
pub struct Text {
    pub text: Binding<String>,
}

impl Text {
    pub fn new(text: Binding<String>) -> Self
}
```

### Spacer

Flexible space component.

```rust
pub struct Spacer {
    pub min_length: f32,
}

impl Spacer {
    pub fn new() -> Self
    pub fn min_length(self, len: f32) -> Self
}
```

### Divider

Visual separator.

```rust
pub struct Divider {}

impl Divider {
    pub fn new() -> Self
}
```

### Image

Image display component.

```rust
pub struct Image {
    pub width: f32,
    pub height: f32,
}

impl Image {
    pub fn new(width: f32, height: f32) -> Self
}
```

## Modifiers

### ViewModifier Trait

```rust
pub trait ViewModifier {
    fn modify_render(&self, view: &dyn View, renderer: &mut dyn Renderer, theme: &Theme, x: f32, y: f32);
    fn modify_event(&self, view: &mut dyn View, event: &Event);
}
```

### ModifiedContent

```rust
pub struct ModifiedContent<V: View, M: ViewModifier> {
    pub view: V,
    pub modifier: M,
}
```

### ViewExt Trait

```rust
pub trait ViewExt: View + Sized {
    fn padding(self, p: f32) -> ModifiedContent<Self, PaddingModifier>
    fn background(self, color: (u8, u8, u8)) -> ModifiedContent<Self, BackgroundModifier>
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

### Slider

Value slider component.

```rust
pub struct Slider {
    pub value: Binding<f32>,
    pub min: f32,
    pub max: f32,
    pub on_change: Option<Box<dyn FnMut(f32)>>,
}

impl Slider {
    pub fn new(value: Binding<f32>, min: f32, max: f32) -> Self
    pub fn on_change<F>(self, f: F) -> Self where F: FnMut(f32) + 'static
}
```

### MenuBar

Horizontal menu bar.

```rust
pub struct MenuBar {
    pub items: Vec<String>,
    pub on_select: Option<Box<dyn FnMut(usize)>>,
}

impl MenuBar {
    pub fn new(items: Vec<String>) -> Self
    pub fn on_select<F>(self, f: F) -> Self where F: FnMut(usize) + 'static
}
```

### ZStack

Z-axis stack for overlaying views.

```rust
pub struct ZStack {
    pub children: Vec<Box<dyn View>>,
}

impl ZStack {
    pub fn new() -> Self
    pub fn add_child(&mut self, child: Box<dyn View>)
}
```

### List

Vertical list of views.

```rust
pub struct List {
    pub items: Vec<Box<dyn View>>,
}

impl List {
    pub fn new(items: Vec<Box<dyn View>>) -> Self
}
```

### ProgressBar

Progress indicator.

```rust
pub struct ProgressBar {
    pub progress: Binding<f32>,
}

impl ProgressBar {
    pub fn new(progress: Binding<f32>) -> Self
}
```

### TabView

Tabbed interface.

```rust
pub struct TabView {
    pub tabs: Vec<String>,
    pub selected: Binding<usize>,
    pub content: Vec<Box<dyn View>>,
}

impl TabView {
    pub fn new(tabs: Vec<String>, selected: Binding<usize>, content: Vec<Box<dyn View>>) -> Self
}
```

### Canvas

Custom drawing area.

```rust
pub struct Canvas {
    pub width: f32,
    pub height: f32,
    pub draw_callback: Option<Box<dyn Fn(&mut dyn Renderer)>>,
}

impl Canvas {
    pub fn new(width: f32, height: f32) -> Self
    pub fn on_draw<F>(self, f: F) -> Self where F: Fn(&mut dyn Renderer) + 'static
}
```

## Integration

### SDLEngine

```rust
impl SDLEngine {
    pub fn render_view(&mut self, view: &dyn View, theme: &Theme) -> Result<(), UiError>
    pub fn handle_event(&mut self, sdl_event: &sdl2::event::Event, view: &mut dyn View)
}
```

### ViewExt Trait

Extension methods for modifiers.

```rust
pub trait ViewExt: View + Sized {
    fn padding(self, p: f32) -> ModifiedContent<Self, PaddingModifier>
    fn background(self, color: (u8, u8, u8)) -> ModifiedContent<Self, BackgroundModifier>
    fn frame(self, width: f32, height: f32) -> ModifiedContent<Self, FrameModifier>
    fn foreground_color(self, color: (u8, u8, u8)) -> ModifiedContent<Self, ForegroundColorModifier>
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
    MouseMove { x: f32, y: f32 },
    KeyDown(sdl2::keyboard::Keycode),
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