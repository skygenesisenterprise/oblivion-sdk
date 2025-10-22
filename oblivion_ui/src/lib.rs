pub mod components;
pub mod state;
pub mod rendering;
pub mod themes;
pub mod error;

#[cfg(test)]
mod tests {
    use super::*;

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
}