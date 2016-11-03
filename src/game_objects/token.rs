use std::f64::consts;

use graphics;
use opengl_graphics::GlGraphics;

use drawing::color;
use drawing::screen;
use drawing::token;

use game_objects::Board;
use game_objects::TokenStates;

const SCREEN_WIDTH: i64 = screen::WIDTH;
const SCREEN_HEIGHT: i64 = screen::HEIGHT;
const BOARD_SIZE: i32 = screen::SIZE;

const CELL_HEIGHT: f64 = (SCREEN_HEIGHT / BOARD_SIZE as i64) as f64;
const CELL_WIDTH: f64 = (SCREEN_WIDTH / BOARD_SIZE as i64) as f64;

const TOKEN_SIZE: i32 = ((0.6 * (CELL_HEIGHT + CELL_WIDTH)) as i32 / 2) as i32;
const SELECTOR_SIZE: i32 = (0.85 * (CELL_HEIGHT + CELL_WIDTH) / 2.0) as i32;
const SELECTOR_OFFSET: i32 = (0.5 * (SELECTOR_SIZE - TOKEN_SIZE) as f64) as i32;

const ARC_RESOLUTION: u32 = token::ARC_RESOLUTION;
const TIMEOUT: f64 = 10.0;
const INITIAL_WAIT: f64 = 1.0;

pub struct Token {
    time: f64,
    color: [f32; 4],
    pub state: TokenStates,
    pub wait_time: f64
}

impl Token {
    pub fn to_canv_pos(pos: (i32, i32)) -> (i32, i32) {
        let mut x = pos.0 as i64 * SCREEN_WIDTH / BOARD_SIZE as i64;
        let mut y = pos.1 as i64 * SCREEN_HEIGHT / BOARD_SIZE as i64;
        x = x + ((CELL_WIDTH - (TOKEN_SIZE as f64)) / 2.0) as i64;
        y = y + ((CELL_HEIGHT - (TOKEN_SIZE as f64)) / 2.0) as i64;
        (x as i32, y as i32)
    }

    pub fn new(color: [f32; 4]) -> Token {
        Token{time: 0.0, color: color, state: TokenStates::WAIT, wait_time: INITIAL_WAIT}
    }
    pub fn set_color(&mut self, c: [f32; 4]) {
        self.color = c;
    }
    pub fn set_wait_time(&mut self, t: f64) {
        self.wait_time = t;
    }
    pub fn set_state(&mut self, s: TokenStates) {
        self.state = s;
    }
    pub fn update(&mut self, dt: f64) {
        self.time += dt;
        //if in wait state
        match self.state{
            TokenStates::WAIT => //see if initial wait has passed
                if self.time >= self.wait_time {
                    self.reset_time();
                    self.state = TokenStates::PREP;
                },
            TokenStates::PREP => 
                if self.time >= TIMEOUT {
                    self.reset_time();
                    self.state = TokenStates::READY;
                },
            TokenStates::READY => 
                if self.time >= TIMEOUT {
                    self.reset_time();
                    self.state = TokenStates::PREP;
                },
            TokenStates::DEAD => {}
        }
    }
    pub fn draw_at(&self, c: &graphics::Context, g: &mut GlGraphics, pos: (i32, i32)) {
        let canv_pos = Token::to_canv_pos(pos);
        match self.state{
            TokenStates::PREP => graphics::CircleArc::new(color::YELLOW, 2.0, 0.0, 1.9999*consts::PI * self.time / TIMEOUT)
                                    .resolution(ARC_RESOLUTION)
                                    .draw([canv_pos.0 as f64, canv_pos.1 as f64, TOKEN_SIZE as f64, TOKEN_SIZE as f64],
                                    &c.draw_state, c.transform, g),
            TokenStates::WAIT => graphics::CircleArc::new(color::RED, 2.0, 0.0, 1.9999*consts::PI - (1.9999*consts::PI * self.time / self.wait_time))
                                    .resolution(ARC_RESOLUTION)
                                    .draw([canv_pos.0 as f64, canv_pos.1 as f64, TOKEN_SIZE as f64, TOKEN_SIZE as f64],
                                    &c.draw_state, c.transform, g),
            TokenStates::READY => graphics::CircleArc::new(color::BRIGHTGREEN, 4.0, 0.0, 1.9999*consts::PI)
                                    .resolution(ARC_RESOLUTION)
                                    .draw([canv_pos.0 as f64, canv_pos.1 as f64, TOKEN_SIZE as f64, TOKEN_SIZE as f64],
                                    &c.draw_state, c.transform, g),
            TokenStates::DEAD => {}
        }
        graphics::Ellipse::new(self.color).resolution(ARC_RESOLUTION)
            .draw([canv_pos.0 as f64, canv_pos.1 as f64, TOKEN_SIZE as f64, TOKEN_SIZE as f64],
            &c.draw_state, c.transform, g);
    }
    pub fn reset_time(&mut self) {
        self.time = 0.0;
    }
}