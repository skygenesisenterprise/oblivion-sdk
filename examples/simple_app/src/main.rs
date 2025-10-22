use oblivion_ui::components::{Window, VStack, Button, Label};
use oblivion_ui::state::State;
use oblivion_ui::rendering::SDLEngine;
use oblivion_ui::themes::Theme;

fn main() -> Result<(), String> {
    let counter = State::new("0".to_string());

    let mut window = Window::new("Simple App".to_string(), 800, 600);

    let mut vstack = VStack::new(10.0);

    let label = Label::new(counter.binding());
    vstack.add_child(Box::new(label));

    let button = Button::new("Increment".to_string()).on_click(move || {
        let current: i32 = counter.get().parse().unwrap_or(0);
        counter.set((current + 1).to_string());
    });
    vstack.add_child(Box::new(button));

    window.add_child(Box::new(vstack));

    let theme = Theme::default();
    let mut engine = SDLEngine::new("Simple App", 800, 600)?;
    engine.run(Box::new(window), &theme)
}