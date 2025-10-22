pub mod components;
pub mod state;
pub mod rendering;
pub mod themes;
pub mod error;
pub mod rso;

#[cfg(test)]
mod tests {
    use super::*;
    use components::View;

    #[test]
    fn test_state() {
        let redraw = std::rc::Rc::new(std::cell::RefCell::new(false));
        let state = state::State::new(42, redraw);
        assert_eq!(state.get(), 42);
        state.set(43);
        assert_eq!(state.get(), 43);
    }

    #[test]
    fn test_binding() {
        let redraw = std::rc::Rc::new(std::cell::RefCell::new(false));
        let state = state::State::new("hello".to_string(), redraw);
        let binding = state.binding();
        assert_eq!(binding.get(), "hello");
        binding.set("world".to_string());
        assert_eq!(state.get(), "world");
    }

    #[test]
    fn test_animated_view() {
        let redraw = std::rc::Rc::new(std::cell::RefCell::new(false));
        let state = state::State::new("test".to_string(), redraw);
        let text = components::Text::new(state.binding());
        let mut animated = components::AnimatedView::new(Box::new(text), 0.0, 100.0, 2.0);
        assert_eq!(animated.offset_x, 0.0);
        animated.update(1.0);
        assert_eq!(animated.offset_x, 50.0);
        animated.update(1.0);
        assert_eq!(animated.offset_x, 100.0);
        animated.update(1.0);
        assert_eq!(animated.offset_x, 100.0);
    }
}