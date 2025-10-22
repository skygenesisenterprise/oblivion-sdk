# Rendering

Oblivion UI uses SDL2 for hardware-accelerated rendering. This provides cross-platform graphics capabilities with OpenGL/Vulkan support.

## Rendering Pipeline

1. **Application State** → 2. **Component Tree** → 3. **Render Commands** → 4. **SDL2 Canvas** → 5. **Display**

## SDLEngine

The `SDLEngine` manages the SDL2 context and main loop:

```rust
use oblivion_ui::rendering::SDLEngine;

let mut engine = SDLEngine::new("My App", 800, 600)?;
engine.run(root_component, &theme)?;
```

### Initialization
- Creates SDL2 context
- Initializes video subsystem
- Sets up window and canvas
- Loads rendering resources (fonts, etc.)

### Main Loop
```rust
'running: loop {
    // Process events
    for event in event_pump.poll_iter() {
        // Convert SDL events to UI events
        let ui_event = convert_event(&event);
        root_component.handle_event(&ui_event);
    }

    // Clear canvas
    canvas.set_draw_color(background_color);
    canvas.clear();

    // Render UI
    root_component.render(&mut renderer, &theme);

    // Present to screen
    canvas.present();
}
```

## Renderer Trait

All rendering goes through the `Renderer` trait:

```rust
pub trait Renderer {
    fn draw_text(&mut self, text: &str, x: f32, y: f32);
    fn draw_rect(&mut self, x: f32, y: f32, w: f32, h: f32);
}
```

### SDLRenderer Implementation

The SDL2 implementation provides:
- Text rendering (placeholder - uses filled rectangles)
- Rectangle drawing
- Future: Image rendering, gradients, etc.

## Coordinate System

- **Origin**: Top-left corner (0,0)
- **X-axis**: Increases right
- **Y-axis**: Increases down
- **Units**: Pixels (f32 for sub-pixel positioning)

## Component Rendering

Each component implements `render()`:

```rust
impl Component for MyComponent {
    fn render(&self, renderer: &mut dyn Renderer, theme: &Theme) {
        // Draw background
        renderer.draw_rect(self.x, self.y, self.width, self.height);

        // Draw text
        renderer.draw_text(&self.text, self.x + 10.0, self.y + 10.0);
    }
}
```

## Theming Integration

Themes provide colors and styling:

```rust
impl Component for Button {
    fn render(&self, renderer: &mut dyn Renderer, theme: &Theme) {
        // Use theme colors
        renderer.set_color(theme.primary_color);
        renderer.draw_rect(self.x, self.y, self.width, self.height);

        // Use theme font settings
        renderer.set_font_size(theme.font_size);
        renderer.draw_text(&self.label, self.x + 5.0, self.y + 5.0);
    }
}
```

## Layout and Positioning

Components are responsible for positioning their children:

```rust
impl Component for VStack {
    fn render(&self, renderer: &mut dyn Renderer, theme: &Theme) {
        let mut y = self.y + self.padding;

        for child in &self.children {
            // Position child
            child.set_position(self.x + self.padding, y);

            // Render child
            child.render(renderer, theme);

            // Move to next position
            y += child.height() + self.spacing;
        }

        // Draw border if specified
        if self.border > 0.0 {
            renderer.draw_rect_border(self.x, self.y, self.width, self.height, self.border);
        }
    }
}
```

## Performance Considerations

### Redraw Optimization
- Currently redraws entire UI every frame
- Future: Implement dirty rectangle rendering
- Future: Component-level diffing

### Resource Management
- Fonts and textures are loaded once
- Reuse renderers across frames
- Minimize allocations in render loop

## Custom Renderers

Implement the `Renderer` trait for different backends:

```rust
struct OpenGLRenderer {
    shader_program: GLuint,
    vbo: GLuint,
    // ... OpenGL state
}

impl Renderer for OpenGLRenderer {
    fn draw_text(&mut self, text: &str, x: f32, y: f32) {
        // OpenGL text rendering implementation
    }

    fn draw_rect(&mut self, x: f32, y: f32, w: f32, h: f32) {
        // OpenGL rectangle drawing
    }
}
```

## Text Rendering

### Current Implementation
- Placeholder: Draws filled rectangles for text blocks
- Size estimation: `text.len() * 10` pixels width

### Future: SDL_ttf Integration
```rust
// Load font
let font = ttf_context.load_font("font.ttf", 16)?;

// Render text to surface
let surface = font.render(text).blended(color)?;

// Create texture
let texture = texture_creator.create_texture_from_surface(&surface)?;

// Render texture to canvas
canvas.copy(&texture, None, rect)?;
```

## Graphics Primitives

### Rectangles
```rust
// Filled rectangle
renderer.draw_rect(x, y, width, height);

// Bordered rectangle
renderer.draw_rect_border(x, y, width, height, border_width);
```

### Future Primitives
- Circles/ellipses
- Lines
- Polygons
- Gradients
- Images
- Rounded rectangles

## Animation Support

### Frame-based Animation
```rust
struct AnimatedComponent {
    start_time: Instant,
    duration: Duration,
    from_value: f32,
    to_value: f32,
}

impl AnimatedComponent {
    fn current_value(&self) -> f32 {
        let elapsed = self.start_time.elapsed();
        let progress = (elapsed.as_millis() as f32) / self.duration.as_millis() as f32;
        let clamped_progress = progress.min(1.0);

        self.from_value + (self.to_value - self.from_value) * clamped_progress
    }
}
```

### Future Animation System
- Easing functions
- Keyframe animations
- Physics-based animations
- Animation curves

## Multi-threading

### Current: Single-threaded
All rendering happens on the main thread.

### Future: Multi-threaded Rendering
- UI logic on main thread
- Rendering on separate thread
- Command buffer pattern

## Platform-specific Rendering

### Linux/X11
- Uses SDL2's X11 backend
- Hardware acceleration via OpenGL

### Wayland (Future)
- SDL3 support for Wayland
- Native Wayland rendering

### ARM Devices
- OpenGL ES support
- Optimized for mobile/embedded

## Debugging Rendering

### Visual Debugging
```rust
// Draw component bounds
renderer.set_color(Color::RED);
renderer.draw_rect_outline(component.x, component.y, component.width, component.height);
```

### Performance Profiling
```rust
let start = Instant::now();
// Render code
let duration = start.elapsed();
println!("Render time: {:?}", duration);
```

## Best Practices

1. **Minimize render calls** - batch operations when possible
2. **Use appropriate data types** - f32 for positions, avoid conversions
3. **Cache expensive operations** - font loading, texture creation
4. **Profile rendering performance** regularly
5. **Handle render errors** gracefully
6. **Test on target platforms** early

## Troubleshooting

### Common Issues

**Black screen:**
- Check SDL2 installation
- Verify OpenGL drivers
- Ensure window is not minimized

**Text not rendering:**
- SDL_ttf not installed
- Font file missing
- Incorrect font path

**Performance issues:**
- Too many draw calls per frame
- Large textures
- Inefficient component hierarchy

**Coordinate issues:**
- Check coordinate system (top-left origin)
- Verify viewport size
- Account for DPI scaling

## Future Roadmap

- [ ] Advanced text rendering with SDL_ttf
- [ ] Image and texture support
- [ ] GPU-accelerated rendering
- [ ] Vector graphics (SVG)
- [ ] 3D rendering capabilities
- [ ] VR/AR support
- [ ] Rendering performance profiling tools