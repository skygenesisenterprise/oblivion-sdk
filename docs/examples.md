# Examples

This section provides complete examples of Oblivion UI applications, from simple to complex.

## Basic Counter App

```rust
use oblivion_ui::components::{Window, VStack, Button, Label};
use oblivion_ui::state::State;
use oblivion_ui::rendering::SDLEngine;
use oblivion_ui::themes::Theme;

fn main() -> Result<(), String> {
    // Create reactive state
    let counter = State::new("0".to_string());

    // Build UI
    let mut window = Window::new("Counter".to_string(), 300, 200);
    let mut vstack = VStack::new(20.0).padding(20.0);

    // Display counter
    let label = Label::new(counter.binding());
    vstack.add_child(Box::new(label));

    // Increment button
    let button = Button::new("Increment".to_string())
        .on_click(move || {
            let current: i32 = counter.get().parse().unwrap_or(0);
            counter.set((current + 1).to_string());
        });
    vstack.add_child(Box::new(button));

    window.add_child(Box::new(vstack));

    // Run app
    let theme = Theme::default();
    let mut engine = SDLEngine::new("Counter", 300, 200)?;
    engine.run(Box::new(window), &theme)
}
```

## Todo List App

```rust
use oblivion_ui::components::{Window, VStack, HStack, Button, Label, Input};
use oblivion_ui::state::{State, Binding};
use oblivion_ui::rendering::SDLEngine;
use oblivion_ui::themes::Theme;

#[derive(Clone)]
struct TodoItem {
    text: String,
    completed: bool,
}

fn main() -> Result<(), String> {
    let todos = State::new(Vec::<TodoItem>::new());
    let new_todo_text = State::new(String::new());

    let mut window = Window::new("Todo App".to_string(), 400, 500);
    let mut main_vstack = VStack::new(10.0).padding(20.0);

    // Input for new todos
    let input = Input::new(new_todo_text.binding(), "Add new todo...".to_string());
    main_vstack.add_child(Box::new(input));

    // Add button
    let add_button = Button::new("Add".to_string())
        .on_click({
            let todos = todos.clone();
            let new_todo_text = new_todo_text.clone();
            move || {
                let text = new_todo_text.get();
                if !text.is_empty() {
                    let mut current_todos = todos.get();
                    current_todos.push(TodoItem {
                        text: text.clone(),
                        completed: false,
                    });
                    todos.set(current_todos);
                    new_todo_text.set(String::new());
                }
            }
        });
    main_vstack.add_child(Box::new(add_button));

    // Todo list (simplified - in real app, you'd create a proper list component)
    let todo_list_placeholder = Label::new(Binding::new("Todo items would go here".to_string()));
    main_vstack.add_child(Box::new(todo_list_placeholder));

    window.add_child(Box::new(main_vstack));

    let theme = Theme::default();
    let mut engine = SDLEngine::new("Todo App", 400, 500)?;
    engine.run(Box::new(window), &theme)
}
```

## Settings Panel

```rust
use oblivion_ui::components::{Window, VStack, HStack, Button, Label, Toggle, Panel};
use oblivion_ui::state::{State, Binding};
use oblivion_ui::rendering::SDLEngine;
use oblivion_ui::themes::Theme;

fn main() -> Result<(), String> {
    // Settings state
    let dark_mode = State::new(false);
    let notifications = State::new(true);
    let auto_save = State::new(false);

    let mut window = Window::new("Settings".to_string(), 400, 400);
    let mut main_vstack = VStack::new(15.0).padding(20.0);

    // Appearance section
    let appearance_panel = Panel::new(1.0, 10.0)
        .child(Box::new(VStack::new(10.0)
            .add_child(Box::new(Label::new(Binding::new("Appearance".to_string()))))
            .add_child(Box::new(HStack::new(10.0)
                .add_child(Box::new(Label::new(Binding::new("Dark Mode".to_string()))))
                .add_child(Box::new(Toggle::new(dark_mode.binding())))))));
    main_vstack.add_child(Box::new(appearance_panel));

    // Notifications section
    let notifications_panel = Panel::new(1.0, 10.0)
        .child(Box::new(VStack::new(10.0)
            .add_child(Box::new(Label::new(Binding::new("Notifications".to_string()))))
            .add_child(Box::new(HStack::new(10.0)
                .add_child(Box::new(Label::new(Binding::new("Enable Notifications".to_string()))))
                .add_child(Box::new(Toggle::new(notifications.binding())))))));
    main_vstack.add_child(Box::new(notifications_panel));

    // General section
    let general_panel = Panel::new(1.0, 10.0)
        .child(Box::new(VStack::new(10.0)
            .add_child(Box::new(Label::new(Binding::new("General".to_string()))))
            .add_child(Box::new(HStack::new(10.0)
                .add_child(Box::new(Label::new(Binding::new("Auto Save".to_string()))))
                .add_child(Box::new(Toggle::new(auto_save.binding())))))));
    main_vstack.add_child(Box::new(general_panel));

    // Save button
    let save_button = Button::new("Save Settings".to_string())
        .on_click(|| {
            // Save settings to disk
            println!("Settings saved!");
        });
    main_vstack.add_child(Box::new(save_button));

    window.add_child(Box::new(main_vstack));

    let theme = Theme::default();
    let mut engine = SDLEngine::new("Settings", 400, 400)?;
    engine.run(Box::new(window), &theme)
}
```

## Calculator App

```rust
use oblivion_ui::components::{Window, VStack, HStack, Button, Label, Grid};
use oblivion_ui::state::State;
use oblivion_ui::rendering::SDLEngine;
use oblivion_ui::themes::Theme;

fn main() -> Result<(), String> {
    let display = State::new("0".to_string());
    let first_operand = State::new(None::<f64>);
    let operation = State::new(None::<char>);

    let mut window = Window::new("Calculator".to_string(), 300, 400);
    let mut main_vstack = VStack::new(10.0).padding(10.0);

    // Display
    let display_label = Label::new(display.binding())
        .padding(10.0);
    main_vstack.add_child(Box::new(display_label));

    // Button grid
    let mut button_grid = Grid::new(5, 4, 5.0);

    let buttons = [
        ("7", 0, 0), ("8", 0, 1), ("9", 0, 2), ("/", 0, 3),
        ("4", 1, 0), ("5", 1, 1), ("6", 1, 2), ("*", 1, 3),
        ("1", 2, 0), ("2", 2, 1), ("3", 2, 2), ("-", 2, 3),
        ("0", 3, 0), (".", 3, 1), ("=", 3, 2), ("+", 3, 3),
        ("C", 4, 0), ("", 4, 1), ("", 4, 2), ("", 4, 3),
    ];

    for (label, row, col) in buttons.iter() {
        if !label.is_empty() {
            let button = create_calc_button(
                label,
                display.clone(),
                first_operand.clone(),
                operation.clone(),
            );
            button_grid.set_child(*row, *col, Box::new(button));
        }
    }

    main_vstack.add_child(Box::new(button_grid));
    window.add_child(Box::new(main_vstack));

    let theme = Theme::default();
    let mut engine = SDLEngine::new("Calculator", 300, 400)?;
    engine.run(Box::new(window), &theme)
}

fn create_calc_button(
    label: &str,
    display: State<String>,
    first_operand: State<Option<f64>>,
    operation: State<Option<char>>,
) -> Button {
    let label = label.to_string();
    let display = display.clone();
    let first_operand = first_operand.clone();
    let operation = operation.clone();

    Button::new(label.clone()).on_click(move || {
        let current_display = display.get();

        match label.as_str() {
            "C" => {
                display.set("0".to_string());
                first_operand.set(None);
                operation.set(None);
            }
            "=" => {
                if let (Some(first), Some(op)) = (first_operand.get(), operation.get()) {
                    if let Ok(second) = current_display.parse::<f64>() {
                        let result = match op {
                            '+' => first + second,
                            '-' => first - second,
                            '*' => first * second,
                            '/' => first / second,
                            _ => second,
                        };
                        display.set(result.to_string());
                        first_operand.set(None);
                        operation.set(None);
                    }
                }
            }
            "+" | "-" | "*" | "/" => {
                if let Ok(num) = current_display.parse::<f64>() {
                    first_operand.set(Some(num));
                    operation.set(Some(label.chars().next().unwrap()));
                    display.set("0".to_string());
                }
            }
            _ => {
                // Number or decimal point
                if current_display == "0" && label != "." {
                    display.set(label.clone());
                } else {
                    display.set(format!("{}{}", current_display, label));
                }
            }
        }
    })
}
```

## File Browser

```rust
use oblivion_ui::components::{Window, VStack, HStack, Button, Label, Panel};
use oblivion_ui::state::{State, Binding};
use oblivion_ui::rendering::SDLEngine;
use oblivion_ui::themes::Theme;
use std::path::PathBuf;

fn main() -> Result<(), String> {
    let current_path = State::new(PathBuf::from("/"));
    let selected_file = State::new(None::<PathBuf>);

    let mut window = Window::new("File Browser".to_string(), 600, 400);
    let mut main_vstack = VStack::new(10.0).padding(10.0);

    // Navigation bar
    let nav_bar = HStack::new(10.0)
        .add_child(Box::new(Button::new("Up".to_string())
            .on_click({
                let current_path = current_path.clone();
                move || {
                    if let Some(parent) = current_path.get().parent() {
                        current_path.set(parent.to_path_buf());
                    }
                }
            })))
        .add_child(Box::new(Label::new(current_path.binding()
            .map(|path| path.to_string_lossy().to_string()))));
    main_vstack.add_child(Box::new(nav_bar));

    // File list placeholder
    let file_list = Panel::new(1.0, 5.0)
        .child(Box::new(Label::new(Binding::new("File list would go here".to_string()))));
    main_vstack.add_child(Box::new(file_list));

    // Status bar
    let status_bar = Label::new(selected_file.binding()
        .map(|file| {
            if let Some(path) = file {
                format!("Selected: {}", path.display())
            } else {
                "No file selected".to_string()
            }
        }));
    main_vstack.add_child(Box::new(status_bar));

    window.add_child(Box::new(main_vstack));

    let theme = Theme::default();
    let mut engine = SDLEngine::new("File Browser", 600, 400)?;
    engine.run(Box::new(window), &theme)
}
```

## Custom Component Example

```rust
use oblivion_ui::components::{Component, Renderer, Event};
use oblivion_ui::themes::Theme;
use oblivion_ui::state::Binding;

pub struct ProgressBar {
    progress: Binding<f32>,  // 0.0 to 1.0
    width: f32,
    height: f32,
}

impl ProgressBar {
    pub fn new(progress: Binding<f32>) -> Self {
        Self {
            progress,
            width: 200.0,
            height: 20.0,
        }
    }

    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }
}

impl Component for ProgressBar {
    fn render(&self, renderer: &mut dyn Renderer, theme: &Theme) {
        // Background
        renderer.draw_rect(0.0, 0.0, self.width, self.height);

        // Progress fill
        let fill_width = self.progress.get() * self.width;
        renderer.draw_rect(0.0, 0.0, fill_width, self.height);

        // Border
        renderer.draw_rect_border(0.0, 0.0, self.width, self.height, 1.0);

        // Text
        let percentage = (self.progress.get() * 100.0) as i32;
        let text = format!("{}%", percentage);
        renderer.draw_text(&text, self.width / 2.0 - 20.0, self.height / 2.0 - 8.0);
    }

    fn handle_event(&mut self, _event: &Event) {
        // Progress bars don't handle events
    }
}

// Usage
fn create_progress_app() -> Result<(), String> {
    let progress = State::new(0.5);

    let mut window = Window::new("Progress Demo".to_string(), 300, 100);
    let progress_bar = ProgressBar::new(progress.binding())
        .size(250.0, 30.0);

    window.add_child(Box::new(progress_bar));

    let theme = Theme::default();
    let mut engine = SDLEngine::new("Progress Demo", 300, 100)?;
    engine.run(Box::new(window), &theme)
}
```

## Advanced Example: Drawing App

```rust
use oblivion_ui::components::{Window, VStack, HStack, Button, Panel};
use oblivion_ui::state::State;
use oblivion_ui::rendering::SDLEngine;
use oblivion_ui::themes::Theme;

#[derive(Clone)]
struct Point {
    x: f32,
    y: f32,
}

fn main() -> Result<(), String> {
    let drawing_points = State::new(Vec::<Point>::new());
    let is_drawing = State::new(false);

    let mut window = Window::new("Drawing App".to_string(), 600, 500);
    let mut main_vstack = VStack::new(10.0).padding(10.0);

    // Toolbar
    let toolbar = HStack::new(10.0)
        .add_child(Box::new(Button::new("Clear".to_string())
            .on_click({
                let drawing_points = drawing_points.clone();
                move || {
                    drawing_points.set(Vec::new());
                }
            })))
        .add_child(Box::new(Button::new("Save".to_string())
            .on_click(|| {
                // Save drawing
                println!("Drawing saved!");
            })));
    main_vstack.add_child(Box::new(toolbar));

    // Drawing canvas (simplified)
    let canvas = DrawingCanvas::new(drawing_points.binding(), is_drawing.binding());
    main_vstack.add_child(Box::new(canvas));

    window.add_child(Box::new(main_vstack));

    let theme = Theme::default();
    let mut engine = SDLEngine::new("Drawing App", 600, 500)?;
    engine.run(Box::new(window), &theme)
}

struct DrawingCanvas {
    points: Binding<Vec<Point>>,
    is_drawing: Binding<bool>,
}

impl DrawingCanvas {
    fn new(points: Binding<Vec<Point>>, is_drawing: Binding<bool>) -> Self {
        Self { points, is_drawing }
    }
}

impl Component for DrawingCanvas {
    fn render(&self, renderer: &mut dyn Renderer, _theme: &Theme) {
        // Draw canvas background
        renderer.draw_rect(0.0, 0.0, 580.0, 400.0);

        // Draw points
        let points = self.points.get();
        for point in points.iter() {
            renderer.draw_rect(point.x - 2.0, point.y - 2.0, 4.0, 4.0);
        }
    }

    fn handle_event(&mut self, event: &Event) {
        match event {
            Event::Click { x, y } => {
                // Start drawing
                self.is_drawing.set(true);
                let mut points = self.points.get();
                points.push(Point { x: *x, y: *y });
                self.points.set(points);
            }
            Event::Hover { x, y } => {
                if self.is_drawing.get() {
                    // Continue drawing
                    let mut points = self.points.get();
                    points.push(Point { x: *x, y: *y });
                    self.points.set(points);
                }
            }
            _ => {}
        }
    }
}
```

## Running Examples

1. Add examples to your `Cargo.toml`:
```toml
[[example]]
name = "counter"
path = "examples/counter.rs"
```

2. Run with:
```bash
cargo run --example counter
```

## Best Practices for Examples

1. **Keep them simple** - Focus on demonstrating one concept
2. **Use realistic data** - Avoid "foo/bar" examples
3. **Handle errors** - Show proper error handling
4. **Comment code** - Explain complex logic
5. **Test on target platforms** - Ensure cross-platform compatibility
6. **Follow conventions** - Use standard Oblivion UI patterns