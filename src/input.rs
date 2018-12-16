//! Input handling module.
//! This module takes care of all user input. It essentially transforms DOM keyboard / gamepad events
//! into requested movement / action state for the player (in rust types).
//!
//! TODO: Move all direct DOM access in here, allow more input methods (mobile), and do something
//! about gamepad amappings / keyboard layout.

use web_sys::KeyboardEvent;

/// Input state. Contains current requested horizontal movement (two bool flags), shoot.
/// Everything is duplicated for all attached gamepads. Do not ask why \--(o,O)--/.
/// There is also an event queue, which is populated by DOM event handlers, and then used
/// to calculate the state of the movement / action flags.
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
    /// Create new **Input** object, with default user input state and empty event queue.
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

    /// Signals if player should go left.
    pub fn left(&self) -> bool {
        self.gp_go_left | self.go_left
    }

    /// Signals if player should go right.
    pub fn right(&self) -> bool {
        self.gp_go_right | self.go_right
    }

    /// Signals if player should shoot.
    pub fn shoot(&self) -> bool {
        self.gp_shoot | self.shoot
    }

    /// Set the gamepad state. This will be called from Javascript code, which evalutes gamepad state.
    ///
    /// TODO: Move the Javascript gamepad code into this module.
    pub fn set_gamepad_state(&mut self, left: bool, right: bool, shoot: bool) {
        self.gp_go_left = left;
        self.gp_go_right = right;
        self.gp_shoot = shoot;
    }

    /// Update function for the input module.
    /// This takes care of reading the event queue, and transforming the DOM event data into the
    /// new state for the flags in the **Input** object.
    pub fn update(&mut self, _: u32) {
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

    /// Add keyboard event to event queue. Matches on the DOM **KeyboardEvent** to extract all
    /// relevant keys, transforms them into the **Key** enum, and finally adds them to the queue (as a KeyEvent)
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

/// Key event enum, which describes all possible key transitions (Up, Down).
#[derive(Debug)]
pub enum KeyEvent {
    Up(Key),
    Down(Key),
}

/// Enum of all the relevant keyboard keys for the game (described with the actual action for the player).
#[derive(Debug)]
pub enum Key {
    Left,
    Right,
    Shoot,
}
