# Components

Components are the building blocks of your Oblivion UI applications. They represent visual elements that can be composed together to create complex user interfaces.

## Component Hierarchy

All components implement the `Component` trait:

```rust
pub trait Component {
    fn render(&self, renderer: &mut dyn Renderer, theme: &Theme);
    fn handle_event(&mut self, event: &Event);
}
```

## Layout Components

Layout components control how other components are positioned and sized.

### Window

The root component that represents the application window.

```rust
let window = Window::new("My App".to_string(), 800, 600);
// Add child components
window.add_child(Box::new(some_component));
```

**Properties:**
- `title`: Window title
- `width`, `height`: Window dimensions

### VStack

Arranges child components vertically with optional spacing.

```rust
let mut vstack = VStack::new(10.0)  // 10px spacing
    .padding(20.0)                  // 20px padding
    .border(2.0);                   // 2px border

vstack.add_child(Box::new(button));
vstack.add_child(Box::new(label));
```

**Properties:**
- `spacing`: Space between children
- `padding`: Internal padding
- `border`: Border width

### HStack

Arranges child components horizontally.

```rust
let mut hstack = HStack::new(15.0)
    .padding(10.0);

hstack.add_child(Box::new(icon));
hstack.add_child(Box::new(text));
```

**Properties:** Same as VStack

### Grid

Arranges components in a 2D grid.

```rust
let mut grid = Grid::new(3, 4, 5.0);  // 3 rows, 4 cols, 5px spacing

grid.set_child(0, 0, Box::new(component1));
grid.set_child(1, 2, Box::new(component2));
```

**Properties:**
- `rows`, `cols`: Grid dimensions
- `spacing`: Space between cells

### Panel

A container component with optional border and background.

```rust
let panel = Panel::new(1.0, 10.0)  // 1px border, 10px padding
    .child(Box::new(content));
```

**Properties:**
- `border_width`: Border thickness
- `padding`: Internal padding

## Interactive Components

These components respond to user input.

### Button

A clickable button with text label.

```rust
let button = Button::new("Click Me".to_string())
    .padding(8.0)
    .border(1.0)
    .on_click(|| {
        println!("Button clicked!");
    });
```

**Events:**
- `on_click`: Closure executed on click

### Label

Displays text, optionally bound to reactive state.

```rust
// Static text
let label = Label::new(Binding::new("Hello World".to_string()));

// Reactive text
let text_state = State::new("Dynamic text".to_string());
let label = Label::new(text_state.binding());
```

**Properties:**
- `text`: Text to display (via Binding)

### Toggle

A switch component for boolean values.

```rust
let is_enabled = State::new(false);
let toggle = Toggle::new(is_enabled.binding())
    .on_toggle(|enabled| {
        println!("Toggle is now: {}", enabled);
    });
```

**Events:**
- `on_toggle`: Closure called when state changes

### Input

A text input field.

```rust
let input_text = State::new(String::new());
let input = Input::new(
    input_text.binding(),
    "Enter text here...".to_string()
);
```

**Properties:**
- `text`: Input text (via Binding)
- `placeholder`: Placeholder text when empty

## Component Lifecycle

### Rendering

Components are rendered every frame. The `render` method is called with:
- `renderer`: Drawing context
- `theme`: Current theme

### Event Handling

Events bubble up from child to parent components. The `handle_event` method receives:
- `event`: The event that occurred

## Custom Components

To create custom components, implement the `Component` trait:

```rust
pub struct MyComponent {
    pub x: f32,
    pub y: f32,
}

impl Component for MyComponent {
    fn render(&self, renderer: &mut dyn Renderer, theme: &Theme) {
        renderer.draw_rect(self.x, self.y, 100.0, 50.0);
    }

    fn handle_event(&mut self, event: &Event) {
        match event {
            Event::Click { x, y } if self.contains_point(*x, *y) => {
                // Handle click
            }
            _ => {}
        }
    }
}

impl MyComponent {
    fn contains_point(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + 100.0 &&
        y >= self.y && y <= self.y + 50.0
    }
}
```

## Best Practices

1. **Keep components small and focused** on a single responsibility
2. **Use reactive state** for dynamic content
3. **Compose layouts** using VStack/HStack/Grid
4. **Handle events appropriately** - don't block the main thread
5. **Use themes** for consistent styling

## Common Patterns

### Form Layout
```rust
let mut form = VStack::new(10.0).padding(20.0);

form.add_child(Box::new(Label::new(Binding::new("Name:".to_string()))));
form.add_child(Box::new(Input::new(name_binding, "Enter name".to_string())));

form.add_child(Box::new(Label::new(Binding::new("Email:".to_string()))));
form.add_child(Box::new(Input::new(email_binding, "Enter email".to_string())));

form.add_child(Box::new(Button::new("Submit".to_string()).on_click(submit)));
```

### Card Layout
```rust
let card = Panel::new(1.0, 15.0)
    .child(Box::new(VStack::new(10.0)
        .add_child(Box::new(title_label))
        .add_child(Box::new(content_label))
        .add_child(Box::new(action_button))));
```

### Grid of Items
```rust
let mut grid = Grid::new(2, 3, 10.0);
for i in 0..2 {
    for j in 0..3 {
        let item = create_item(i * 3 + j);
        grid.set_child(i, j, Box::new(item));
    }
}
```