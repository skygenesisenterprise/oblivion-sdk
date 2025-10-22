use oblivion_ui::components::{Window, VStack, Button, Text, View, AnimatedView};
use oblivion_ui::state::State;
use oblivion_ui::rendering::SDLEngine;
use oblivion_ui::themes::Theme;
use oblivion_ui::error::UiError;
use std::rc::Rc;
use std::cell::RefCell;

fn main() -> Result<(), UiError> {
    let (mut engine, redraw_trigger) = SDLEngine::new("Simple App", 800, 600)?;
    let counter = State::new("0".to_string(), redraw_trigger.clone());

    let mut window = Window::new("Simple App".to_string(), 800, 600);

    let mut vstack = VStack::new(10.0);

    let label = Text::new(counter.binding());
    vstack.add_child(Box::new(label));

    let button = Button::new("Increment".to_string())
        .on_click(move || {
            let current: i32 = counter.get().parse().unwrap_or(0);
            counter.set((current + 1).to_string());
        })
        .padding(10.0);
    let animated_button = AnimatedView::new(Box::new(button), -200.0, 0.0, 2.0);
    vstack.add_child(Box::new(animated_button));

    window.add_child(Box::new(vstack));

    let theme = Theme::default();
    engine.run(Box::new(window) as Box<dyn View>, &theme, redraw_trigger)
}