use web_sys::KeyboardEvent;

pub struct Input {
    pub go_left: bool,
    pub go_right: bool,
    pub shoot: bool,
    event_queue: Vec<KeyEvent>,
}

impl Input {
    pub fn new() -> Self {
        Self {
            go_left: false,
            go_right: false,
            shoot: false,
            event_queue: Vec::with_capacity(8),
        }
    }

    pub fn update(&mut self, ts: u32) {
        for e in self.event_queue.drain(0..self.event_queue.len()) {
            log!("{:?}", e);
            match e {
                KeyEvent::Down(Key::Left) => self.go_left = true,
                KeyEvent::Up(Key::Left) => self.go_left = false,
                KeyEvent::Down(Key::Right) => self.go_right = true,
                KeyEvent::Up(Key::Right) => self.go_right = false,
                KeyEvent::Down(Key::Shoot) => self.shoot = true,
                KeyEvent::Up(Key::Shoot) => self.shoot = false,
            }
        }
    }

    pub fn keyboard_event(&mut self, is_down: bool, e: KeyboardEvent) {
        let key = match e.key().as_str() {
            "a" | "ArrowLeft" => Key::Left,
            "d" | "ArrowRight" => Key::Right,
            " " => Key::Shoot,
            _ => return,
        };
        let event = if is_down {
            KeyEvent::Down(key)
        } else {
            KeyEvent::Up(key)
        };
        self.event_queue.push(event);
    }
}

#[derive(Debug)]
pub enum KeyEvent {
    Up(Key),
    Down(Key),
}

#[derive(Debug)]
pub enum Key {
    Left,
    Right,
    Shoot,
}
