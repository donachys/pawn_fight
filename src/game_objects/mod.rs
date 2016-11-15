mod board;
mod token;

mod human_player;
mod cpu_player;

#[derive(Clone, Copy, PartialEq)]
pub enum TokenStates {
    WAIT,
    PREP,
    READY,
    DEAD,
}
pub enum InputTypes {
    MOUSE,
    KEYBOARD,
}
pub enum KeyboardStates {
    MOVING,
    SELECTED,
}
pub mod player_constants {
    pub const MAX_TOKENS: i32 = 4;
}

pub use self::board::Board;

pub use self::token::Token;
pub use self::human_player::HumanPlayer;
pub use self::cpu_player::CpuPlayer;
