use std::rc::Rc;
use std::cell::RefCell;

pub struct State<T> {
    value: Rc<RefCell<T>>,
}

impl<T> State<T> {
    pub fn new(initial: T) -> Self {
        State {
            value: Rc::new(RefCell::new(initial)),
        }
    }

    pub fn get(&self) -> T
    where
        T: Clone,
    {
        self.value.borrow().clone()
    }

    pub fn set(&self, new_value: T) {
        *self.value.borrow_mut() = new_value;
        // Trigger redraw
    }

    pub fn binding(&self) -> Binding<T> {
        Binding {
            value: Rc::clone(&self.value),
        }
    }
}

pub struct Binding<T> {
    value: Rc<RefCell<T>>,
}

impl<T> Binding<T> {
    pub fn get(&self) -> T
    where
        T: Clone,
    {
        self.value.borrow().clone()
    }

    pub fn set(&self, new_value: T) {
        *self.value.borrow_mut() = new_value;
        // Trigger redraw
    }
}

impl<T> Clone for Binding<T> {
    fn clone(&self) -> Self {
        Binding {
            value: Rc::clone(&self.value),
        }
    }
}