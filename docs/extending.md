# Extending the Framework

Oblivion UI is designed to be extensible. This guide shows how to add new components, renderers, themes, and integrate with system services.

## Creating Custom Components

### Basic Component Structure

All components must implement the `Component` trait:

```rust
use crate::components::{Component, Renderer, Event};
use crate::themes::Theme;
use crate::state::{State, Binding};

pub struct MyComponent {
    // Component state
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,

    // Reactive properties
    pub label: Binding<String>,
    pub enabled: Binding<bool>,

    // Event handlers
    pub on_click: Option<Box<dyn FnMut()>>,
}

impl MyComponent {
    pub fn new(label: Binding<String>) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 30.0,
            label,
            enabled: Binding::new(true),
            on_click: None,
        }
    }

    pub fn position(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn on_click<F>(mut self, callback: F) -> Self
    where
        F: FnMut() + 'static,
    {
        self.on_click = Some(Box::new(callback));
        self
    }

    fn contains_point(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.width &&
        y >= self.y && y <= self.y + self.height
    }
}
```

### Implementing Component Trait

```rust
impl Component for MyComponent {
    fn render(&self, renderer: &mut dyn Renderer, theme: &Theme) {
        // Choose color based on state
        let bg_color = if self.enabled.get() {
            theme.primary_color
        } else {
            theme.secondary_color
        };

        // Draw background
        renderer.draw_rect(self.x, self.y, self.width, self.height);

        // Draw border
        renderer.draw_rect_border(self.x, self.y, self.width, self.height, 1.0);

        // Draw text
        let text = self.label.get();
        let text_x = self.x + 10.0;
        let text_y = self.y + (self.height - 20.0) / 2.0;  // Center vertically
        renderer.draw_text(&text, text_x, text_y);
    }

    fn handle_event(&mut self, event: &Event) {
        match event {
            Event::Click { x, y } if self.enabled.get() && self.contains_point(*x, *y) => {
                if let Some(ref mut callback) = self.on_click {
                    callback();
                }
            }
            _ => {
                // Ignore other events
            }
        }
    }
}
```

### Builder Pattern

Use the builder pattern for fluent component construction:

```rust
impl MyComponent {
    pub fn enabled(mut self, enabled: Binding<bool>) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn style(mut self, style: ComponentStyle) -> Self {
        self.style = style;
        self
    }
}

pub struct ComponentStyle {
    pub background_color: Option<(u8, u8, u8)>,
    pub border_color: Option<(u8, u8, u8)>,
    pub border_width: f32,
    pub corner_radius: f32,
}

// Usage
let component = MyComponent::new(label_binding)
    .position(10.0, 20.0)
    .size(200.0, 40.0)
    .enabled(enabled_binding)
    .on_click(|| println!("Clicked!"));
```

## Advanced Components

### Composite Components

Build complex components from simpler ones:

```rust
pub struct LabeledInput {
    label: Label,
    input: Input,
    spacing: f32,
}

impl LabeledInput {
    pub fn new(label_text: &str, input_binding: Binding<String>) -> Self {
        Self {
            label: Label::new(Binding::new(label_text.to_string())),
            input: Input::new(input_binding, "".to_string()),
            spacing: 5.0,
        }
    }
}

impl Component for LabeledInput {
    fn render(&self, renderer: &mut dyn Renderer, theme: &Theme) {
        // Render label
        self.label.render(renderer, theme);

        // Position and render input below label
        let input_y = self.label.y + 25.0 + self.spacing;
        // Note: In real implementation, you'd need to modify input position
        self.input.render(renderer, theme);
    }

    fn handle_event(&mut self, event: &Event) {
        // Try input first (higher priority)
        self.input.handle_event(event);

        // Then label
        self.label.handle_event(event);
    }
}
```

### Stateful Components

Components with internal state:

```rust
pub struct ExpandablePanel {
    header: Button,
    content: Box<dyn Component>,
    expanded: State<bool>,
    height: f32,
}

impl ExpandablePanel {
    pub fn new(header_text: &str, content: Box<dyn Component>) -> Self {
        let expanded = State::new(false);

        let header = Button::new(header_text.to_string())
            .on_click({
                let expanded = expanded.clone();
                move || {
                    expanded.set(!expanded.get());
                }
            });

        Self {
            header,
            content,
            expanded,
            height: 30.0,  // Header height
        }
    }
}

impl Component for ExpandablePanel {
    fn render(&self, renderer: &mut dyn Renderer, theme: &Theme) {
        // Render header
        self.header.render(renderer, theme);

        // Render content if expanded
        if self.expanded.get() {
            // Position content below header
            self.content.render(renderer, theme);
        }
    }

    fn handle_event(&mut self, event: &Event) {
        self.header.handle_event(event);

        if self.expanded.get() {
            self.content.handle_event(event);
        }
    }
}
```

## Custom Renderers

### Implementing Renderer Trait

```rust
use crate::components::Renderer;

pub struct OpenGLRenderer {
    // OpenGL context, shaders, etc.
}

impl Renderer for OpenGLRenderer {
    fn draw_text(&mut self, text: &str, x: f32, y: f32) {
        // OpenGL text rendering implementation
        // - Load font texture
        // - Create vertex buffer
        // - Render quads for each character
    }

    fn draw_rect(&mut self, x: f32, y: f32, w: f32, h: f32) {
        // OpenGL rectangle rendering
        // - Create vertex buffer for quad
        // - Set shader uniforms
        // - Draw triangles
    }
}
```

### Integrating Custom Renderer

```rust
pub struct CustomEngine {
    renderer: OpenGLRenderer,
    // Other engine state
}

impl CustomEngine {
    pub fn run(&mut self, mut root_component: Box<dyn Component>, theme: &Theme) {
        loop {
            // Handle events...

            // Clear screen
            self.renderer.clear();

            // Render UI
            root_component.render(&mut self.renderer, theme);

            // Swap buffers
            self.renderer.present();
        }
    }
}
```

## Custom Themes

### Extending Theme Structure

```rust
#[derive(Clone)]
pub struct ExtendedTheme {
    pub base: Theme,
    pub accent_color: (u8, u8, u8),
    pub error_color: (u8, u8, u8),
    pub success_color: (u8, u8, u8),
    pub warning_color: (u8, u8, u8),
    pub spacing_unit: f32,
    pub border_radius: f32,
}

impl Default for ExtendedTheme {
    fn default() -> Self {
        Self {
            base: Theme::default(),
            accent_color: (255, 149, 0),
            error_color: (255, 59, 48),
            success_color: (52, 199, 89),
            warning_color: (255, 149, 0),
            spacing_unit: 8.0,
            border_radius: 4.0,
        }
    }
}
```

### Theme-aware Components

```rust
impl Component for ThemedButton {
    fn render(&self, renderer: &mut dyn Renderer, theme: &Theme) {
        // Try to cast to extended theme
        if let Some(extended) = theme.downcast_ref::<ExtendedTheme>() {
            // Use extended theme properties
            renderer.set_draw_color(extended.accent_color);
        } else {
            // Fall back to base theme
            renderer.set_draw_color(theme.primary_color);
        }

        // Render with rounded corners
        renderer.draw_rounded_rect(self.x, self.y, self.width, self.height, extended.border_radius);
    }
}
```

## System Services Integration

### Service Architecture

```rust
pub mod services {
    use std::sync::{Arc, Mutex};

    pub trait Service {
        fn initialize(&mut self) -> Result<(), String>;
        fn shutdown(&mut self);
    }

    pub struct ServiceRegistry {
        services: HashMap<String, Arc<Mutex<dyn Service>>>,
    }

    impl ServiceRegistry {
        pub fn register<T: Service + 'static>(&mut self, name: &str, service: T) {
            self.services.insert(name.to_string(), Arc::new(Mutex::new(service)));
        }

        pub fn get<T: Service + 'static>(&self, name: &str) -> Option<Arc<Mutex<T>>> {
            self.services.get(name)?.clone().downcast::<Mutex<T>>().ok()
        }
    }
}
```

### Notification Service

```rust
pub struct NotificationService {
    // Platform-specific notification implementation
}

impl NotificationService {
    pub fn show(&self, title: &str, message: &str) {
        // Show system notification
        #[cfg(target_os = "linux")]
        {
            // Use libnotify or similar
        }
    }
}

impl Service for NotificationService {
    fn initialize(&mut self) -> Result<(), String> {
        // Initialize notification system
        Ok(())
    }

    fn shutdown(&mut self) {
        // Clean up
    }
}
```

### Settings Service

```rust
pub struct SettingsService {
    settings: HashMap<String, String>,
}

impl SettingsService {
    pub fn get(&self, key: &str) -> Option<&String> {
        self.settings.get(key)
    }

    pub fn set(&mut self, key: &str, value: String) {
        self.settings.insert(key.to_string(), value);
        self.save_to_disk();
    }

    fn save_to_disk(&self) {
        // Persist settings
    }
}
```

### Using Services in Components

```rust
pub struct SettingsComponent {
    settings: Arc<Mutex<SettingsService>>,
    theme_binding: Binding<String>,
}

impl SettingsComponent {
    pub fn new(settings: Arc<Mutex<SettingsService>>) -> Self {
        let theme_binding = {
            let settings = settings.lock().unwrap();
            Binding::new(settings.get("theme").unwrap_or(&"light".to_string()).clone())
        };

        Self {
            settings,
            theme_binding,
        }
    }
}

impl Component for SettingsComponent {
    fn render(&self, renderer: &mut dyn Renderer, theme: &Theme) {
        // Render settings UI
    }

    fn handle_event(&mut self, event: &Event) {
        if let Event::Click { .. } = event {
            // Update setting
            let mut settings = self.settings.lock().unwrap();
            settings.set("theme", self.theme_binding.get());
        }
    }
}
```

## Custom Event Types

### Extending Event System

```rust
#[derive(Clone)]
pub enum ExtendedEvent {
    Base(Event),
    Custom(CustomEventData),
}

pub enum CustomEventData {
    Scroll { delta_x: f32, delta_y: f32 },
    Gesture { kind: GestureKind },
    Network { data: Vec<u8> },
}

pub enum GestureKind {
    Pinch { scale: f32 },
    Rotate { angle: f32 },
    Swipe { direction: Direction },
}
```

### Gesture Recognizer Component

```rust
pub struct GestureRecognizer {
    child: Box<dyn Component>,
    gesture_handlers: HashMap<GestureKind, Box<dyn Fn(GestureData)>>,
}

impl GestureRecognizer {
    pub fn on_gesture<F>(mut self, kind: GestureKind, handler: F) -> Self
    where
        F: Fn(GestureData) + 'static,
    {
        self.gesture_handlers.insert(kind, Box::new(handler));
        self
    }
}

impl Component for GestureRecognizer {
    fn render(&self, renderer: &mut dyn Renderer, theme: &Theme) {
        self.child.render(renderer, theme);
    }

    fn handle_event(&mut self, event: &Event) {
        // Detect gestures from basic events
        match event {
            Event::Click { .. } => {
                // Detect tap
            }
            Event::Drag { dx, dy } => {
                // Detect swipe
                let direction = if dx.abs() > dy.abs() {
                    if dx > 0.0 { Direction::Right } else { Direction::Left }
                } else {
                    if dy > 0.0 { Direction::Down } else { Direction::Up }
                };

                if let Some(handler) = self.gesture_handlers.get(&GestureKind::Swipe { direction }) {
                    handler(GestureData::Swipe { direction });
                }
            }
            _ => {}
        }

        // Pass event to child
        self.child.handle_event(event);
    }
}
```

## Plugin System

### Plugin Interface

```rust
pub trait Plugin {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn initialize(&mut self, app: &mut AppContext) -> Result<(), String>;
    fn shutdown(&mut self);

    // Component registration
    fn register_components(&self, registry: &mut ComponentRegistry);

    // Service registration
    fn register_services(&self, registry: &mut ServiceRegistry);
}

pub struct ComponentRegistry {
    factories: HashMap<String, Box<dyn Fn() -> Box<dyn Component>>>,
}

impl ComponentRegistry {
    pub fn register<F>(&mut self, name: &str, factory: F)
    where
        F: Fn() -> Box<dyn Component> + 'static,
    {
        self.factories.insert(name.to_string(), Box::new(factory));
    }

    pub fn create(&self, name: &str) -> Option<Box<dyn Component>> {
        self.factories.get(name).map(|factory| factory())
    }
}
```

### Loading Plugins

```rust
use libloading::{Library, Symbol};

pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
    libraries: Vec<Library>,
}

impl PluginManager {
    pub fn load_plugin(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            let lib = Library::new(path)?;
            let constructor: Symbol<fn() -> Box<dyn Plugin>> = lib.get(b"create_plugin")?;
            let plugin = constructor();

            // Initialize plugin
            plugin.initialize(&mut self.app_context)?;

            self.plugins.push(plugin);
            self.libraries.push(lib);
        }

        Ok(())
    }
}
```

## Testing Extensions

### Unit Testing Components

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::rendering::MockRenderer;

    #[test]
    fn test_button_click() {
        let mut clicked = false;
        let mut button = Button::new("Test".to_string())
            .on_click(|| clicked = true);

        button.handle_event(&Event::Click { x: 10.0, y: 10.0 });

        assert!(clicked);
    }

    #[test]
    fn test_button_render() {
        let button = Button::new("Test".to_string());
        let mut renderer = MockRenderer::new();

        button.render(&mut renderer, &Theme::default());

        assert_eq!(renderer.draw_calls.len(), 2);  // Background + text
    }
}

pub struct MockRenderer {
    pub draw_calls: Vec<DrawCall>,
}

pub enum DrawCall {
    Rect { x: f32, y: f32, w: f32, h: f32 },
    Text { text: String, x: f32, y: f32 },
}

impl Renderer for MockRenderer {
    fn draw_text(&mut self, text: &str, x: f32, y: f32) {
        self.draw_calls.push(DrawCall::Text {
            text: text.to_string(),
            x,
            y,
        });
    }

    fn draw_rect(&mut self, x: f32, y: f32, w: f32, h: f32) {
        self.draw_calls.push(DrawCall::Rect { x, y, w, h });
    }
}
```

## Best Practices

1. **Follow existing patterns** - Use similar APIs to built-in components
2. **Document extensions** - Provide clear usage examples
3. **Handle errors gracefully** - Don't crash on invalid input
4. **Test thoroughly** - Cover edge cases and error conditions
5. **Version carefully** - Consider backward compatibility
6. **Performance matters** - Profile and optimize rendering
7. **Accessibility** - Support screen readers and keyboard navigation

## Future Extension Points

- [ ] WebAssembly components
- [ ] Native platform widgets
- [ ] 3D rendering support
- [ ] Advanced animation system
- [ ] Internationalization framework
- [ ] Component marketplace
- [ ] Visual component editor