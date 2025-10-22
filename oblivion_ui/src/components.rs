use crate::state::{State, Binding};

pub trait Component {
    fn render(&self, renderer: &mut dyn Renderer);
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
    fn render(&self, renderer: &mut dyn Renderer) {
        // Render window frame
        for child in &self.children {
            child.render(renderer);
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
}

impl VStack {
    pub fn new(spacing: f32) -> Self {
        VStack {
            children: Vec::new(),
            spacing,
        }
    }

    pub fn add_child(&mut self, child: Box<dyn Component>) {
        self.children.push(child);
    }
}

impl Component for VStack {
    fn render(&self, renderer: &mut dyn Renderer) {
        let mut y = 0.0;
        for child in &self.children {
            // Position child at y
            child.render(renderer);
            y += self.spacing;
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
}

impl Button {
    pub fn new(label: String) -> Self {
        Button {
            label,
            on_click: None,
        }
    }

    pub fn on_click<F: FnMut() + 'static>(mut self, f: F) -> Self {
        self.on_click = Some(Box::new(f));
        self
    }
}

impl Component for Button {
    fn render(&self, renderer: &mut dyn Renderer) {
        // Render button with label
    }

    fn handle_event(&mut self, event: &Event) {
        if let Event::Click = event {
            if let Some(ref mut callback) = self.on_click {
                callback();
            }
        }
    }
}

pub struct Label {
    pub text: Binding<String>,
}

impl Label {
    pub fn new(text: Binding<String>) -> Self {
        Label { text }
    }
}

impl Component for Label {
    fn render(&self, renderer: &mut dyn Renderer) {
        // Render text
    }

    fn handle_event(&mut self, _event: &Event) {
        // Labels don't handle events
    }
}

// Placeholder for Renderer trait
pub trait Renderer {
    fn draw_text(&mut self, text: &str, x: f32, y: f32);
    fn draw_rect(&mut self, x: f32, y: f32, w: f32, h: f32);
}

// Placeholder for Event
pub enum Event {
    Click,
    Hover,
    KeyPress(char),
}