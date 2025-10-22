# State Management

Oblivion UI uses a reactive state management system inspired by SwiftUI's @State and @Binding. This allows UI to automatically update when underlying data changes.

## Core Concepts

### Reactive Programming
Instead of manually updating UI elements when data changes, you declare relationships between data and UI. The framework handles updates automatically.

### State vs Binding
- **State**: Owned, mutable data that triggers UI updates
- **Binding**: Reference to state that can read/write the value

## @State

`State<T>` represents mutable, reactive data owned by a component.

```rust
use oblivion_ui::state::State;

// Create state with initial value
let count = State::new(0);

// Read current value
let current = count.get();  // 0

// Update value (triggers UI redraw)
count.set(5);
```

**Key Points:**
- State is owned by the component that creates it
- Changes to state automatically trigger re-rendering
- State can be any type that implements `Clone`

### State Lifetime

State lives as long as the component that owns it. When a component is dropped, its state is also dropped.

```rust
fn create_counter() -> impl Component {
    let count = State::new(0);  // State owned by this function's scope

    Button::new("Increment".to_string())
        .on_click(move || {
            count.set(count.get() + 1);
        })
}
```

## @Binding

`Binding<T>` provides read/write access to state without owning it.

```rust
use oblivion_ui::state::Binding;

// Get a binding from state
let count = State::new(10);
let count_binding = count.binding();

// Read through binding
let value = count_binding.get();  // 10

// Write through binding
count_binding.set(20);
```

**Use Cases:**
- Passing state to child components
- Sharing state between multiple components
- Creating reusable components that accept state

### Binding from State

```rust
let state = State::new("Hello".to_string());
let binding = state.binding();  // Binding<String>
```

## Component State

Components can have their own internal state:

```rust
pub struct CounterButton {
    count: State<i32>,
    label: String,
}

impl CounterButton {
    pub fn new(label: String) -> Self {
        Self {
            count: State::new(0),
            label,
        }
    }
}

impl Component for CounterButton {
    fn render(&self, renderer: &mut dyn Renderer, theme: &Theme) {
        let text = format!("{}: {}", self.label, self.count.get());
        renderer.draw_text(&text, 0.0, 0.0);
    }

    fn handle_event(&mut self, event: &Event) {
        if let Event::Click { .. } = event {
            self.count.set(self.count.get() + 1);
        }
    }
}
```

## Shared State

Use bindings to share state between components:

```rust
fn create_shared_counter() -> Box<dyn Component> {
    let shared_count = State::new(0);

    let mut vstack = VStack::new(10.0);

    // Display component
    let display = CounterDisplay::new(shared_count.binding());
    vstack.add_child(Box::new(display));

    // Control component
    let control = CounterControl::new(shared_count.binding());
    vstack.add_child(Box::new(control));

    Box::new(vstack)
}

struct CounterDisplay {
    count: Binding<i32>,
}

impl CounterDisplay {
    fn new(count: Binding<i32>) -> Self {
        Self { count }
    }
}

impl Component for CounterDisplay {
    fn render(&self, renderer: &mut dyn Renderer, theme: &Theme) {
        let text = format!("Count: {}", self.count.get());
        renderer.draw_text(&text, 0.0, 0.0);
    }

    fn handle_event(&mut self, _event: &Event) {
        // Display only, no event handling
    }
}

struct CounterControl {
    count: Binding<i32>,
}

impl CounterControl {
    fn new(count: Binding<i32>) -> Self {
        Self { count }
    }
}

impl Component for CounterControl {
    fn render(&self, renderer: &mut dyn Renderer, theme: &Theme) {
        renderer.draw_text("Increment", 0.0, 0.0);
    }

    fn handle_event(&mut self, event: &Event) {
        if let Event::Click { .. } = event {
            self.count.set(self.count.get() + 1);
        }
    }
}
```

## State Updates and Redrawing

### Automatic Redraw
When state changes, the entire component tree is re-rendered:

```rust
let state = State::new("Initial".to_string());

// This triggers a redraw
state.set("Updated".to_string());
```

### Performance Considerations
- Only changed components are conceptually re-rendered (diffing is minimal)
- Avoid creating new state in render methods
- Use bindings for read-only access when possible

## Advanced Patterns

### Computed State
Create derived state from other state:

```rust
let first_name = State::new("John".to_string());
let last_name = State::new("Doe".to_string());

// Note: This is a simplified example
// In practice, you'd need a way to compute derived values
let full_name = format!("{} {}", first_name.get(), last_name.get());
```

### State Validation
Validate state changes:

```rust
let age = State::new(25);

let validated_age = age.binding().map(|value| {
    if value < 0 {
        0
    } else if value > 150 {
        150
    } else {
        value
    }
});
```

### State Persistence
For persistent state, integrate with system services:

```rust
// Pseudo-code for future implementation
let settings = State::new(load_settings_from_disk());

settings.set(new_settings);
// Automatically save to disk
```

## Best Practices

1. **Keep state local** when possible - use bindings for sharing
2. **Use descriptive names** for state variables
3. **Initialize state** with sensible defaults
4. **Avoid deep nesting** of state updates
5. **Test state changes** thoroughly
6. **Use types that implement Clone** for state values

## Common Pitfalls

### Forgetting to Clone
State values must be `Clone`:

```rust
// This won't work - String is not Copy
let state = State::new(String::from("hello"));

// This works - String implements Clone
let state = State::new("hello".to_string());
```

### State Ownership Issues
```rust
// Wrong - state is dropped immediately
let binding = {
    let state = State::new(0);
    state.binding()  // state goes out of scope here
};

// Correct - state lives long enough
let state = State::new(0);
let binding = state.binding();
```

### Unnecessary Re-renders
```rust
// Avoid this - creates new state on every render
impl Component for MyComponent {
    fn render(&self, renderer: &mut dyn Renderer, theme: &Theme) {
        let state = State::new(0);  // Bad!
        // ...
    }
}
```

## Migration from Imperative UI

### Before (Imperative)
```rust
// Manual UI updates
let mut label = create_label("Count: 0");
let mut count = 0;

button.on_click(|| {
    count += 1;
    label.set_text(format!("Count: {}", count));
});
```

### After (Reactive)
```rust
// Declarative UI
let count = State::new(0);
let label = Label::new(count.binding().map(|c| format!("Count: {}", c)));
let button = Button::new("Increment".to_string())
    .on_click(move || count.set(count.get() + 1));
```

## Future Enhancements

- Observable objects for complex state
- State persistence
- Undo/redo functionality
- State debugging tools