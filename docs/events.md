# Events

Events in Oblivion UI handle user interactions and system notifications. The framework provides a comprehensive event system for mouse, keyboard, and other inputs.

## Event Types

All events are represented by the `Event` enum:

```rust
pub enum Event {
    Click { x: f32, y: f32 },
    Hover { x: f32, y: f32 },
    KeyPress(char),
    Drag { dx: f32, dy: f32 },
}
```

## Event Propagation

Events follow a bubbling pattern:
1. Event occurs at the lowest component
2. Bubbles up through parent components
3. Each component can handle or ignore the event

```rust
impl Component for MyComponent {
    fn handle_event(&mut self, event: &Event) {
        match event {
            Event::Click { x, y } => {
                if self.contains_point(*x, *y) {
                    self.handle_click(*x, *y);
                }
            }
            Event::KeyPress(key) => {
                self.handle_keypress(*key);
            }
            _ => {
                // Pass to children or ignore
            }
        }
    }
}
```

## Mouse Events

### Click Events
```rust
Event::Click { x, y }
```
- Triggered on mouse button down
- Provides click coordinates
- Used for buttons, links, etc.

### Hover Events
```rust
Event::Hover { x, y }
```
- Triggered on mouse movement
- Provides current mouse position
- Used for tooltips, hover effects

### Drag Events
```rust
Event::Drag { dx, dy }
```
- Triggered during mouse drag operations
- Provides delta movement
- Used for sliders, drag-and-drop

## Keyboard Events

### Key Press Events
```rust
Event::KeyPress(char)
```
- Triggered on key down
- Provides the pressed character
- Used for text input, shortcuts

## Component Event Handling

### Button Events
```rust
impl Component for Button {
    fn handle_event(&mut self, event: &Event) {
        if let Event::Click { x, y } = event {
            if self.contains_point(*x, *y) {
                if let Some(ref mut callback) = self.on_click {
                    callback();
                }
            }
        }
    }
}
```

### Input Field Events
```rust
impl Component for Input {
    fn handle_event(&mut self, event: &Event) {
        match event {
            Event::Click { x, y } => {
                if self.contains_point(*x, *y) {
                    self.focused = true;
                }
            }
            Event::KeyPress(c) if self.focused => {
                let mut text = self.text.get();
                text.push(*c);
                self.text.set(text);
            }
            _ => {}
        }
    }
}
```

## Event Bubbling

Events bubble up the component hierarchy:

```rust
impl Component for Container {
    fn handle_event(&mut self, event: &Event) {
        // Handle at container level first
        match event {
            Event::Click { .. } => {
                // Container-specific handling
            }
            _ => {}
        }

        // Then pass to children
        for child in &mut self.children {
            child.handle_event(event);
        }
    }
}
```

## Event Filtering

Stop event propagation when handled:

```rust
impl Component for Modal {
    fn handle_event(&mut self, event: &Event) {
        match event {
            Event::Click { .. } => {
                // Handle modal click - don't pass to children
                self.close();
            }
            Event::KeyPress(key) if *key == '\x1b' => {  // Escape
                self.close();
            }
            _ => {
                // Pass other events to modal content
                self.content.handle_event(event);
            }
        }
    }
}
```

## Custom Events

Extend the event system for custom interactions:

```rust
#[derive(Clone)]
pub enum CustomEvent {
    Standard(Event),
    Custom(CustomEventData),
}

pub enum CustomEventData {
    Scroll { delta: f32 },
    Pinch { scale: f32 },
    Swipe { direction: Direction },
}

pub enum Direction {
    Up, Down, Left, Right,
}
```

## Event Handlers

### Closures
```rust
let button = Button::new("Click me".to_string())
    .on_click(|| {
        println!("Button clicked!");
    });
```

### State Updates
```rust
let count = State::new(0);
let increment_button = Button::new("+".to_string())
    .on_click(move || {
        count.set(count.get() + 1);
    });
```

### Complex Logic
```rust
let form_data = State::new(FormData::default());
let submit_button = Button::new("Submit".to_string())
    .on_click(move || {
        if validate_form(&form_data.get()) {
            submit_form(form_data.get());
        } else {
            show_validation_errors();
        }
    });
```

## Focus Management

Track which component has focus:

```rust
struct FocusManager {
    focused_component: Option<usize>,
    components: Vec<Box<dyn Component>>,
}

impl FocusManager {
    fn handle_event(&mut self, event: &Event) {
        match event {
            Event::Click { x, y } => {
                // Find clicked component and focus it
                for (i, component) in self.components.iter().enumerate() {
                    if component.contains_point(*x, *y) {
                        self.focused_component = Some(i);
                        break;
                    }
                }
            }
            Event::KeyPress(key) => {
                if let Some(focused) = self.focused_component {
                    self.components[focused].handle_event(event);
                }
            }
            _ => {}
        }
    }
}
```

## Gesture Recognition

Implement complex gestures from basic events:

```rust
struct GestureRecognizer {
    start_pos: Option<(f32, f32)>,
    current_pos: (f32, f32),
}

impl GestureRecognizer {
    fn handle_event(&mut self, event: &Event) -> Option<Gesture> {
        match event {
            Event::Click { x, y } => {
                self.start_pos = Some((*x, *y));
                self.current_pos = (*x, *y);
                None
            }
            Event::Hover { x, y } => {
                self.current_pos = (*x, *y);
                if let Some(start) = self.start_pos {
                    let distance = ((x - start.0).powi(2) + (y - start.1).powi(2)).sqrt();
                    if distance > 50.0 {
                        self.start_pos = None;
                        return Some(Gesture::Swipe {
                            direction: self.calculate_direction(start, (*x, *y)),
                        });
                    }
                }
                None
            }
            _ => None,
        }
    }

    fn calculate_direction(&self, start: (f32, f32), end: (f32, f32)) -> Direction {
        let dx = end.0 - start.0;
        let dy = end.1 - start.1;

        if dx.abs() > dy.abs() {
            if dx > 0.0 { Direction::Right } else { Direction::Left }
        } else {
            if dy > 0.0 { Direction::Down } else { Direction::Up }
        }
    }
}

pub enum Gesture {
    Swipe { direction: Direction },
    Pinch { scale: f32 },
    Tap,
}
```

## Event Debouncing

Prevent rapid event firing:

```rust
use std::time::{Duration, Instant};

struct Debouncer {
    last_event: Instant,
    delay: Duration,
}

impl Debouncer {
    fn new(delay_ms: u64) -> Self {
        Self {
            last_event: Instant::now(),
            delay: Duration::from_millis(delay_ms),
        }
    }

    fn should_handle(&mut self) -> bool {
        let now = Instant::now();
        if now.duration_since(self.last_event) >= self.delay {
            self.last_event = now;
            true
        } else {
            false
        }
    }
}

// Usage
impl Component for SearchInput {
    fn handle_event(&mut self, event: &Event) {
        if let Event::KeyPress(_) = event {
            if self.debouncer.should_handle() {
                self.perform_search();
            }
        }
    }
}
```

## Event Logging and Debugging

Log events for debugging:

```rust
impl Component for DebugComponent {
    fn handle_event(&mut self, event: &Event) {
        println!("Event received: {:?}", event);

        // Pass to actual handler
        self.child.handle_event(event);
    }
}
```

## Performance Considerations

### Event Throttling
```rust
struct EventThrottler {
    last_update: Instant,
    update_interval: Duration,
}

impl EventThrottler {
    fn should_update(&mut self) -> bool {
        let now = Instant::now();
        if now.duration_since(self.last_update) >= self.update_interval {
            self.last_update = now;
            true
        } else {
            false
        }
    }
}
```

### Event Prioritization
```rust
enum EventPriority {
    High,    // Immediate response required
    Normal,  // Standard UI updates
    Low,     // Background updates
}

struct PrioritizedEvent {
    event: Event,
    priority: EventPriority,
}
```

## Platform-specific Events

### Touch Events (Future)
```rust
pub enum TouchEvent {
    TouchStart { id: u32, x: f32, y: f32 },
    TouchMove { id: u32, x: f32, y: f32 },
    TouchEnd { id: u32 },
}
```

### Gamepad Events (Future)
```rust
pub enum GamepadEvent {
    ButtonPressed { button: GamepadButton },
    AxisMoved { axis: GamepadAxis, value: f32 },
}
```

## Best Practices

1. **Keep event handlers simple** - move complex logic elsewhere
2. **Use event bubbling appropriately** - don't over-handle
3. **Handle focus correctly** - manage keyboard focus
4. **Debounce rapid events** - prevent excessive updates
5. **Test event interactions** - cover edge cases
6. **Document custom events** - explain event contracts

## Common Patterns

### Event Delegation
```rust
struct EventDelegator {
    handlers: HashMap<String, Box<dyn Fn(&Event)>>,
}

impl EventDelegator {
    fn register_handler(&mut self, event_type: &str, handler: Box<dyn Fn(&Event)>) {
        self.handlers.insert(event_type.to_string(), handler);
    }

    fn handle_event(&self, event: &Event) {
        if let Some(handler) = self.handlers.get(&event_type(event)) {
            handler(event);
        }
    }
}

fn event_type(event: &Event) -> String {
    match event {
        Event::Click { .. } => "click".to_string(),
        Event::KeyPress(_) => "keypress".to_string(),
        _ => "unknown".to_string(),
    }
}
```

### State Machine Events
```rust
enum AppState {
    Loading,
    Ready,
    Error,
}

struct StateMachine {
    current_state: AppState,
    state_handlers: HashMap<AppState, Box<dyn Fn(&Event) -> Option<AppState>>>,
}

impl StateMachine {
    fn handle_event(&mut self, event: &Event) {
        if let Some(handler) = self.state_handlers.get(&self.current_state) {
            if let Some(new_state) = handler(event) {
                self.current_state = new_state;
            }
        }
    }
}
```

## Future Enhancements

- [ ] Touch and gesture events
- [ ] Gamepad/controller support
- [ ] Accessibility events (screen reader)
- [ ] Network events
- [ ] File system events
- [ ] Custom event system
- [ ] Event recording/playback for testing