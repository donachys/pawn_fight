mod board;
mod token;

mod human_player;

#[derive(Clone, Copy, PartialEq)]
pub enum TokenStates {
    Wait,
    Prep,
    Ready,
    Dead,
}

pub enum InputTypes {
    Mouse,
    Keyboard,
}

pub enum KeyboardStates {
    Moving,
    Selected,
}

pub mod player_constants {
    pub const MAX_TOKENS: i32 = 4;
}

pub use self::board::Board;

pub use self::human_player::HumanPlayer;
pub use self::token::Token;
