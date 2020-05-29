use std::f64;


use piston::input::*;
use piston_window::{Graphics, Context, clear};

use rand::{self, ThreadRng};
use crate::game_objects::Board;
use crate::game_objects::HumanPlayer;
use crate::game_objects::CpuPlayer;
use crate::game_objects::InputTypes;

use crate::drawing::color;
use crate::drawing::screen;

const SCREEN_WIDTH: i64 = screen::WIDTH;
const SCREEN_HEIGHT: i64 = screen::HEIGHT;
const BOARD_SIZE: i32 = screen::SIZE;

const CELL_HEIGHT: f64 = (SCREEN_HEIGHT as f64 / BOARD_SIZE as f64) as f64;
const CELL_WIDTH: f64 = (SCREEN_WIDTH as f64 / BOARD_SIZE as f64) as f64;

const NUM_CPU_PLAYERS: i32 = 1;
const NUM_HUM_PLAYERS: i32 = 1;
const NUM_PLAYERS: i32 = NUM_HUM_PLAYERS + NUM_CPU_PLAYERS;

#[derive(Default)]
struct Timers {
    current_time: f64,
}

pub struct Game {
    /// A random number generator
    rng: ThreadRng,
    timers: Timers,
    board: Board,
    hum_players: Vec<HumanPlayer>,
    cpu_players: Vec<CpuPlayer>,
}

impl Game {
    pub fn new() -> Game {
        let rng = rand::thread_rng();
        let board = Board::new(NUM_PLAYERS);
        let mut hum_players = Vec::with_capacity(NUM_HUM_PLAYERS as usize);
        let cpu_players = Vec::with_capacity(NUM_HUM_PLAYERS as usize);

        hum_players.push(HumanPlayer::new(0, InputTypes::MOUSE));
        hum_players.push(HumanPlayer::new(1, InputTypes::KEYBOARD));

        Game {
            rng: rng,
            timers: Timers::default(),
            board: board,
            hum_players: hum_players,
            cpu_players: cpu_players,
        }
    }
    pub fn handle_mouse_click(&mut self, _b: MouseButton, c: [f64; 2]) {
        let cell_row: i32 = (c[0] / CELL_WIDTH) as i32;
        let cell_col: i32 = (c[1] / CELL_HEIGHT) as i32;
        for human in self.hum_players.iter_mut() {
            match human.input_type {
                InputTypes::MOUSE => human.handle_mouse_click((cell_row, cell_col)),
                InputTypes::KEYBOARD => {}
            }
        }
        // println!("Mouse cursor ({}, {}) clicked row '{}' col '{}'",
        //              c[0], c[1], cell_row, cell_col);
    }
    pub fn handle_key_press(&mut self, b: Key) {
        for human in self.hum_players.iter_mut() {
            match human.input_type {
                InputTypes::MOUSE => {}
                InputTypes::KEYBOARD => human.handle_key_press(b),
            }
        }
    }
    pub fn render<G: Graphics>(&mut self, c: &Context, g: &mut G) {
        // Clear everything
        clear(color::BLACK, g);
        self.board.draw(&c, g);
        for human in self.hum_players.iter() {
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
        for human in self.hum_players.iter_mut() {
            human.update(&mut self.board);
        }
        self.board.update(dt);
    }
}
