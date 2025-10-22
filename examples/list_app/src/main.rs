use oblivion_ui::components::{Window, VStack, Text, View, ForEach};
use oblivion_ui::state::State;
use oblivion_ui::rendering::SDLEngine;
use oblivion_ui::themes::Theme;
use oblivion_ui::error::UiError;
use std::rc::Rc;
use std::cell::RefCell;

fn main() -> Result<(), UiError> {
    let (mut engine, redraw_trigger) = SDLEngine::new("List App", 800, 600)?;
    let items = vec!["Item 1".to_string(), "Item 2".to_string(), "Item 3".to_string()];

    let mut window = Window::new("List App".to_string(), 800, 600);

    let mut vstack = VStack::new(10.0);

    let list = ForEach::new(items, |item| {
        Box::new(Text::new(State::new(item, redraw_trigger.clone()).binding())) as Box<dyn View>
    });
    vstack.add_child(Box::new(list));

    window.add_child(Box::new(vstack));

    let theme = Theme::default();
    engine.run(Box::new(window) as Box<dyn View>, &theme, redraw_trigger)
}