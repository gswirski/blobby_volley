use std::collections::HashSet;
use glutin;
use glutin::{
    ElementState,
    Event,
    VirtualKeyCode,
};

pub struct Window {
    pub window: glutin::Window,
    pub should_close: bool,
    pub paused: bool,
    pub key_set: HashSet<VirtualKeyCode>,
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> Window {
        let window = glutin::WindowBuilder::new()
            .with_title(title.to_string())
            .with_dimensions(width, height)
            .build()
            .unwrap();

        unsafe { window.make_current() };

        Window {
            window: window,
            should_close: false,
            paused: false,
            key_set: HashSet::new()
        }
    }

    pub fn get_canvas_proportions(&self) -> f32 {
        match self.window.get_inner_size() {
            Some((w, h)) => (w as f32 / h as f32),
            None => 1.0
        }
    }

    pub fn handle_events(&mut self) {
        let events: Vec<Event> = self.window.poll_events().collect();
        for event in events {
            self.handle_event(event);
        }
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::KeyboardInput(_, _, Some(VirtualKeyCode::Escape)) =>
                self.close_window(),
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::P)) =>
                self.pause(),
            Event::KeyboardInput(ElementState::Pressed, _, Some(key)) =>
                self.key_set.insert(key),
            Event::KeyboardInput(ElementState::Released, _, Some(key)) =>
                self.key_set.remove(&key),
            _ =>
                false
        };
    }

    fn close_window(&mut self) -> bool {
        self.should_close = true;
        true
    }

    fn pause(&mut self) -> bool {
        self.paused = !self.paused;
        true
    }
}
