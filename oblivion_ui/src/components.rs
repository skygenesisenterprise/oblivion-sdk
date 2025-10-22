use crate::state::Binding;
use crate::themes::Theme;

pub trait Component {
    fn render(&self, renderer: &mut dyn Renderer, theme: &Theme, x: f32, y: f32);
    fn handle_event(&mut self, event: &Event);
}

pub struct Window {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub children: Vec<Box<dyn Component>>,
}

impl Window {
    pub fn new(title: String, width: u32, height: u32) -> Self {
        Window {
            title,
            width,
            height,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: Box<dyn Component>) {
        self.children.push(child);
    }
}

impl Component for Window {
    fn render(&self, renderer: &mut dyn Renderer, theme: &Theme, x: f32, y: f32) {
        // Render window frame
        let mut child_y = y;
        for child in &self.children {
            child.render(renderer, theme, x, child_y);
            child_y += 50.0; // Placeholder height
        }
    }

    fn handle_event(&mut self, event: &Event) {
        for child in &mut self.children {
            child.handle_event(event);
        }
    }
}

pub struct VStack {
    pub children: Vec<Box<dyn Component>>,
    pub spacing: f32,
    pub padding: f32,
    pub border: f32,
}

impl VStack {
    pub fn new(spacing: f32) -> Self {
        VStack {
            children: Vec::new(),
            spacing,
            padding: 0.0,
            border: 0.0,
        }
    }

    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    pub fn border(mut self, border: f32) -> Self {
        self.border = border;
        self
    }

    pub fn add_child(&mut self, child: Box<dyn Component>) {
        self.children.push(child);
    }
}

impl Component for VStack {
    fn render(&self, renderer: &mut dyn Renderer, theme: &Theme, x: f32, y: f32) {
        let mut current_y = y + self.padding + self.border;
        for child in &self.children {
            child.render(renderer, theme, x + self.padding, current_y);
            current_y += 30.0 + self.spacing; // Placeholder child height
        }
        // Render border if >0
        if self.border > 0.0 {
            renderer.draw_rect(x, y, 200.0, current_y - y); // Placeholder width
        }
    }

    fn handle_event(&mut self, event: &Event) {
        for child in &mut self.children {
            child.handle_event(event);
        }
    }
}

pub struct Button {
    pub label: String,
    pub on_click: Option<Box<dyn FnMut()>>,
    pub padding: f32,
    pub border: f32,
}

impl Button {
    pub fn new(label: String) -> Self {
        Button {
            label,
            on_click: None,
            padding: 5.0,
            border: 1.0,
        }
    }

    pub fn on_click<F: FnMut() + 'static>(mut self, f: F) -> Self {
        self.on_click = Some(Box::new(f));
        self
    }

    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    pub fn border(mut self, border: f32) -> Self {
        self.border = border;
        self
    }
}

impl Component for Button {
    fn render(&self, renderer: &mut dyn Renderer, theme: &Theme, x: f32, y: f32) {
        // Render button rect with border, then text
        renderer.draw_rect(x, y, 100.0, 30.0);
        renderer.draw_text(&self.label, x + self.padding, y + self.padding);
    }

    fn handle_event(&mut self, event: &Event) {
        if let Event::Click { .. } = event {
            if let Some(ref mut callback) = self.on_click {
                callback();
            }
        }
    }
}

pub struct Label {
    pub text: Binding<String>,
    pub padding: f32,
}

impl Label {
    pub fn new(text: Binding<String>) -> Self {
        Label { text, padding: 0.0 }
    }

    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }
}

impl Component for Label {
    fn render(&self, renderer: &mut dyn Renderer, theme: &Theme, x: f32, y: f32) {
        renderer.draw_text(&self.text.get(), x + self.padding, y + self.padding);
    }

    fn handle_event(&mut self, _event: &Event) {
        // Labels don't handle events
    }
}

pub struct HStack {
    pub children: Vec<Box<dyn Component>>,
    pub spacing: f32,
    pub padding: f32,
    pub border: f32,
}

impl HStack {
    pub fn new(spacing: f32) -> Self {
        HStack {
            children: Vec::new(),
            spacing,
            padding: 0.0,
            border: 0.0,
        }
    }

    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    pub fn border(mut self, border: f32) -> Self {
        self.border = border;
        self
    }

    pub fn add_child(&mut self, child: Box<dyn Component>) {
        self.children.push(child);
    }
}

impl Component for HStack {
    fn render(&self, renderer: &mut dyn Renderer, theme: &Theme, x: f32, y: f32) {
        let mut current_x = x + self.padding + self.border;
        for child in &self.children {
            child.render(renderer, theme, current_x, y + self.padding);
            current_x += 100.0 + self.spacing; // Placeholder width
        }
        // Render border
    }

    fn handle_event(&mut self, event: &Event) {
        for child in &mut self.children {
            child.handle_event(event);
        }
    }
}

pub struct Grid {
    pub children: Vec<Vec<Option<Box<dyn Component>>>>,
    pub rows: usize,
    pub cols: usize,
    pub spacing: f32,
}

impl Grid {
    pub fn new(rows: usize, cols: usize, spacing: f32) -> Self {
        let mut children = Vec::with_capacity(rows);
        for _ in 0..rows {
            let mut row = Vec::with_capacity(cols);
            for _ in 0..cols {
                row.push(None);
            }
            children.push(row);
        }
        Grid {
            children,
            rows,
            cols,
            spacing,
        }
    }

    pub fn set_child(&mut self, row: usize, col: usize, child: Box<dyn Component>) {
        if row < self.rows && col < self.cols {
            self.children[row][col] = Some(child);
        }
    }
}

impl Component for Grid {
    fn render(&self, renderer: &mut dyn Renderer, theme: &Theme, x: f32, y: f32) {
        for (row_idx, row) in self.children.iter().enumerate() {
            for (col_idx, child_opt) in row.iter().enumerate() {
                if let Some(child) = child_opt {
                    let child_x = x + col_idx as f32 * (100.0 + self.spacing);
                    let child_y = y + row_idx as f32 * (30.0 + self.spacing);
                    child.render(renderer, theme, child_x, child_y);
                }
            }
        }
    }

    fn handle_event(&mut self, event: &Event) {
        for row in &mut self.children {
            for child_opt in row {
                if let Some(child) = child_opt {
                    child.handle_event(event);
                }
            }
        }
    }
}

pub struct Panel {
    pub child: Option<Box<dyn Component>>,
    pub border_width: f32,
    pub padding: f32,
}

impl Panel {
    pub fn new(border_width: f32, padding: f32) -> Self {
        Panel {
            child: None,
            border_width,
            padding,
        }
    }

    pub fn child(mut self, child: Box<dyn Component>) -> Self {
        self.child = Some(child);
        self
    }
}

impl Component for Panel {
    fn render(&self, renderer: &mut dyn Renderer, theme: &Theme, x: f32, y: f32) {
        // Render border
        renderer.draw_rect(x, y, 200.0, 200.0);
        if let Some(ref child) = self.child {
            child.render(renderer, theme, x + self.padding, y + self.padding);
        }
    }

    fn handle_event(&mut self, event: &Event) {
        if let Some(ref mut child) = self.child {
            child.handle_event(event);
        }
    }
}

pub struct Toggle {
    pub is_on: Binding<bool>,
    pub on_toggle: Option<Box<dyn FnMut(bool)>>,
}

impl Toggle {
    pub fn new(is_on: Binding<bool>) -> Self {
        Toggle {
            is_on,
            on_toggle: None,
        }
    }

    pub fn on_toggle<F: FnMut(bool) + 'static>(mut self, f: F) -> Self {
        self.on_toggle = Some(Box::new(f));
        self
    }
}

impl Component for Toggle {
    fn render(&self, renderer: &mut dyn Renderer, theme: &Theme, x: f32, y: f32) {
        // Render toggle switch
        let state = if self.is_on.get() { "ON" } else { "OFF" };
        renderer.draw_text(state, x, y);
    }

    fn handle_event(&mut self, event: &Event) {
        if let Event::Click { .. } = event {
            let current = self.is_on.get();
            self.is_on.set(!current);
            if let Some(ref mut callback) = self.on_toggle {
                callback(!current);
            }
        }
    }
}

pub struct Input {
    pub text: Binding<String>,
    pub placeholder: String,
}

impl Input {
    pub fn new(text: Binding<String>, placeholder: String) -> Self {
        Input { text, placeholder }
    }
}

impl Component for Input {
    fn render(&self, renderer: &mut dyn Renderer, theme: &Theme, x: f32, y: f32) {
        // Render input field with text
        let text = if self.text.get().is_empty() { &self.placeholder } else { &self.text.get() };
        renderer.draw_text(text, x, y);
        // Draw border
        renderer.draw_rect(x, y, 200.0, 25.0);
    }

    fn handle_event(&mut self, event: &Event) {
        match event {
            Event::KeyDown(key) => {
                match key {
                    sdl2::keyboard::Keycode::Backspace => {
                        let mut current = self.text.get();
                        current.pop();
                        self.text.set(current);
                    }
                    _ => {
                        if let Some(c) = key.to_string().chars().next() {
                            if c.is_alphanumeric() || c.is_whitespace() {
                                let mut current = self.text.get();
                                current.push(c);
                                self.text.set(current);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

pub struct Slider {
    pub value: Binding<f32>,
    pub min: f32,
    pub max: f32,
    pub on_change: Option<Box<dyn FnMut(f32)>>,
}

impl Slider {
    pub fn new(value: Binding<f32>, min: f32, max: f32) -> Self {
        Slider {
            value,
            min,
            max,
            on_change: None,
        }
    }

    pub fn on_change<F: FnMut(f32) + 'static>(mut self, f: F) -> Self {
        self.on_change = Some(Box::new(f));
        self
    }
}

impl Component for Slider {
    fn render(&self, renderer: &mut dyn Renderer, theme: &Theme, x: f32, y: f32) {
        // Render slider bar and knob
        renderer.draw_rect(x, y + 10.0, 100.0, 5.0); // Bar
        let knob_x = x + (self.value.get() - self.min) / (self.max - self.min) * 100.0;
        renderer.draw_rect(knob_x - 5.0, y, 10.0, 25.0); // Knob
    }

    fn handle_event(&mut self, event: &Event) {
        if let Event::Click { x, .. } = event {
            let new_value = self.min + (*x / 100.0) * (self.max - self.min);
            let clamped = new_value.max(self.min).min(self.max);
            self.value.set(clamped);
            if let Some(ref mut callback) = self.on_change {
                callback(clamped);
            }
        }
    }
}

pub struct MenuBar {
    pub items: Vec<String>,
    pub on_select: Option<Box<dyn FnMut(usize)>>,
}

impl MenuBar {
    pub fn new(items: Vec<String>) -> Self {
        MenuBar {
            items,
            on_select: None,
        }
    }

    pub fn on_select<F: FnMut(usize) + 'static>(mut self, f: F) -> Self {
        self.on_select = Some(Box::new(f));
        self
    }
}

impl Component for MenuBar {
    fn render(&self, renderer: &mut dyn Renderer, theme: &Theme, x: f32, y: f32) {
        let mut current_x = x;
        for item in &self.items {
            renderer.draw_text(item, current_x, y);
            current_x += 50.0; // Placeholder width
        }
    }

    fn handle_event(&mut self, event: &Event) {
        if let Event::Click { x, .. } = event {
            let index = (*x / 50.0) as usize;
            if index < self.items.len() {
                if let Some(ref mut callback) = self.on_select {
                    callback(index);
                }
            }
        }
    }
}

// Placeholder for Renderer trait
pub trait Renderer {
    fn draw_text(&mut self, text: &str, x: f32, y: f32);
    fn draw_rect(&mut self, x: f32, y: f32, w: f32, h: f32);
}

// Placeholder for Event
pub enum Event {
    Click { x: f32, y: f32 },
    MouseMove { x: f32, y: f32 },
    KeyDown(sdl2::keyboard::Keycode),
    KeyPress(char),
    Drag { dx: f32, dy: f32 },
}