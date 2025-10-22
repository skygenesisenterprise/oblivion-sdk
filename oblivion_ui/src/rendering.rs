use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;
use std::rc::Rc;
use std::cell::RefCell;
use crate::error::UiError;

use crate::components::{View, Renderer as UIRenderer};
use crate::themes::Theme;

pub struct SDLEngine {
    sdl_context: Sdl,
    canvas: Canvas<Window>,
}

impl SDLEngine {
    pub fn new(title: &str, width: u32, height: u32) -> Result<(Self, Rc<RefCell<bool>>), UiError> {
        let sdl_context = sdl2::init().map_err(|e| UiError::SdlError(e.to_string()))?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window(title, width, height)
            .position_centered()
            .build()
            .map_err(|e| UiError::SdlError(e.to_string()))?;

        let canvas = window.into_canvas().build().map_err(|e| UiError::SdlError(e.to_string()))?;

        let redraw_trigger = Rc::new(RefCell::new(true));

        Ok((SDLEngine {
            sdl_context,
            canvas,
        }, redraw_trigger))
    }

    pub fn run(&mut self, mut root_view: Box<dyn View>, theme: &Theme, redraw_trigger: Rc<RefCell<bool>>) -> Result<(), UiError> {
        let mut event_pump = self.sdl_context.event_pump()?;

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {
                        // Convert SDL event to our Event
                        let ui_event = self.convert_event(&event);
                        root_view.handle_event(&ui_event);
                    }
                }
            }

            if *redraw_trigger.borrow() {
                self.canvas.set_draw_color(Color::RGB(255, 255, 255));
                self.canvas.clear();

                root_view.render(&mut SDLRenderer { canvas: &mut self.canvas, theme }, theme, 0.0, 0.0);

                self.canvas.present();
                *redraw_trigger.borrow_mut() = false;
            }
        }

        Ok(())
    }

    fn convert_event(&self, event: &Event) -> crate::components::Event {
        match event {
            Event::MouseButtonDown { x, y, .. } => crate::components::Event::Click { x: *x as f32, y: *y as f32 },
            Event::MouseMotion { x, y, .. } => crate::components::Event::MouseMove { x: *x as f32, y: *y as f32 },
            Event::KeyDown { keycode: Some(key), .. } => crate::components::Event::KeyDown(*key),
            _ => crate::components::Event::Click { x: 0.0, y: 0.0 }, // Default
        }
    }
}

struct SDLRenderer<'a> {
    canvas: &'a mut Canvas<Window>,
    theme: &'a crate::themes::Theme,
}

impl<'a> UIRenderer for SDLRenderer<'a> {
    fn draw_text(&mut self, text: &str, x: f32, y: f32) {
        // Placeholder: draw a colored rectangle representing text
        self.canvas.set_draw_color(Color::RGB(self.theme.text_color.0, self.theme.text_color.1, self.theme.text_color.2));
        let rect = Rect::new(x as i32, y as i32, (text.len() * 10) as u32, 20);
        self.canvas.fill_rect(rect).unwrap();
    }

    fn draw_rect(&mut self, x: f32, y: f32, w: f32, h: f32) {
        self.canvas.set_draw_color(Color::RGB(self.theme.secondary_color.0, self.theme.secondary_color.1, self.theme.secondary_color.2));
        let rect = Rect::new(x as i32, y as i32, w as u32, h as u32);
        self.canvas.fill_rect(rect).unwrap();
    }
}