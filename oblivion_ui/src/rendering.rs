use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;

use crate::components::{Component, Renderer as UIRenderer};

pub struct SDLEngine {
    sdl_context: Sdl,
    canvas: Canvas<Window>,
}

impl SDLEngine {
    pub fn new(title: &str, width: u32, height: u32) -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window(title, width, height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        Ok(SDLEngine {
            sdl_context,
            canvas,
        })
    }

    pub fn run(&mut self, mut root_component: Box<dyn Component>) -> Result<(), String> {
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
                        root_component.handle_event(&ui_event);
                    }
                }
            }

            self.canvas.set_draw_color(Color::RGB(255, 255, 255));
            self.canvas.clear();

            root_component.render(&mut SDLRenderer { canvas: &mut self.canvas });

            self.canvas.present();
        }

        Ok(())
    }

    fn convert_event(&self, event: &Event) -> crate::components::Event {
        match event {
            Event::MouseButtonDown { .. } => crate::components::Event::Click,
            Event::MouseMotion { .. } => crate::components::Event::Hover,
            Event::KeyDown { keycode: Some(key), .. } => {
                if let Some(c) = key.to_string().chars().next() {
                    crate::components::Event::KeyPress(c)
                } else {
                    crate::components::Event::KeyPress(' ')
                }
            }
            _ => crate::components::Event::Click, // Default
        }
    }
}

struct SDLRenderer<'a> {
    canvas: &'a mut Canvas<Window>,
}

impl<'a> UIRenderer for SDLRenderer<'a> {
    fn draw_text(&mut self, text: &str, x: f32, y: f32) {
        // Placeholder: SDL2 doesn't have built-in text rendering
        // In real implementation, use SDL2_ttf or similar
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        let rect = Rect::new(x as i32, y as i32, (text.len() * 10) as u32, 20);
        self.canvas.fill_rect(rect).unwrap();
    }

    fn draw_rect(&mut self, x: f32, y: f32, w: f32, h: f32) {
        self.canvas.set_draw_color(Color::RGB(200, 200, 200));
        let rect = Rect::new(x as i32, y as i32, w as u32, h as u32);
        self.canvas.fill_rect(rect).unwrap();
    }
}