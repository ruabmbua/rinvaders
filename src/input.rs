use web_sys::KeyboardEvent;

pub struct Input {
    go_left: bool,
    go_right: bool,
    shoot: bool,
    gp_go_left: bool,
    gp_go_right: bool,
    gp_shoot: bool,
    event_queue: Vec<KeyEvent>,
}

impl Input {
    pub fn new() -> Self {
        Self {
            go_left: false,
            go_right: false,
            shoot: false,
            gp_go_left: false,
            gp_go_right: false,
            gp_shoot: false,
            event_queue: Vec::with_capacity(8),
        }
    }

    pub fn left(&self) -> bool {
        self.gp_go_left | self.go_left
    }

    pub fn right(&self) -> bool {
        self.gp_go_right | self.go_right
    }

    pub fn shoot(&self) -> bool {
        self.gp_shoot | self.shoot
    }

    pub fn set_gamepad_state(&mut self, left: bool, right: bool, shoot: bool) {
		self.gp_go_left = left;
        self.gp_go_right = right;
        self.gp_shoot = shoot;
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
