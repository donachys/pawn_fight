use piston_window::{clear, Context, Graphics};

use crate::game_objects::{Board, HumanPlayer, InputTypes};
use crate::drawing::{color, screen};

const SCREEN_WIDTH: i64 = screen::WIDTH;
const SCREEN_HEIGHT: i64 = screen::HEIGHT;
const BOARD_SIZE: i32 = screen::SIZE;

const CELL_HEIGHT: f64 = (SCREEN_HEIGHT as f64 / BOARD_SIZE as f64) as f64;
const CELL_WIDTH: f64 = (SCREEN_WIDTH as f64 / BOARD_SIZE as f64) as f64;

const NUM_PLAYERS: i32 = 2;

#[derive(Default)]
struct Timers {
    current_time: f64,
}

pub struct Game {
    timers: Timers,
    board: Board,
    players: Vec<HumanPlayer>,
}

impl Game {
    pub fn new() -> Game {
        let board = Board::new(NUM_PLAYERS);
        let mut players = Vec::with_capacity(NUM_PLAYERS as usize);

        players.push(HumanPlayer::new(0, InputTypes::Mouse));
        players.push(HumanPlayer::new(1, InputTypes::Keyboard));

        Game {
            timers: Timers::default(),
            board,
            players,
        }
    }
    pub fn handle_mouse_click(&mut self, _b: piston_window::MouseButton, c: [f64; 2]) {
        let cell_row: i32 = (c[0] / CELL_WIDTH) as i32;
        let cell_col: i32 = (c[1] / CELL_HEIGHT) as i32;
        for human in self.players.iter_mut() {
            match human.input_type {
                InputTypes::Mouse => human.handle_mouse_click((cell_row, cell_col)),
                InputTypes::Keyboard => {}
            }
        }
        // println!("Mouse cursor ({}, {}) clicked row '{}' col '{}'",
        //              c[0], c[1], cell_row, cell_col);
    }
    pub fn handle_key_press(&mut self, b: piston_window::Key) {
        for human in self.players.iter_mut() {
            match human.input_type {
                InputTypes::Mouse => {}
                InputTypes::Keyboard => human.handle_key_press(b),
            }
        }
    }
    pub fn render<G: Graphics>(&mut self, c: &Context, g: &mut G) {
        // Clear everything
        clear(color::BLACK, g);
        self.board.draw(&c, g);
        for human in self.players.iter() {
            human.draw_selection(c, g);
        }
    }
    pub fn update(&mut self, dt: f64) {
        self.timers.current_time += dt;
        let player_count = self.board.players_remaining();
        if player_count == 1 {
            println!("VICTORY");
            return;
        }
        for human in self.players.iter_mut() {
            human.update(&mut self.board);
        }
        self.board.update(dt);
    }
}
