use std::f64::consts;

use piston::input::keyboard::Key;
use piston_window::{CircleArc, Context, Graphics};

use crate::game_objects::Board;
use crate::game_objects::InputTypes;
use crate::game_objects::KeyboardStates;
use crate::game_objects::Token;

use crate::drawing::color;
use crate::drawing::screen;

const SCREEN_WIDTH: i64 = screen::WIDTH;
const SCREEN_HEIGHT: i64 = screen::HEIGHT;
const BOARD_SIZE: i32 = screen::SIZE;

const CELL_HEIGHT: f64 = (SCREEN_HEIGHT / BOARD_SIZE as i64) as f64;
const CELL_WIDTH: f64 = (SCREEN_WIDTH / BOARD_SIZE as i64) as f64;

const TOKEN_SIZE: i32 = ((0.6 * (CELL_HEIGHT + CELL_WIDTH)) as i32 / 2) as i32;
const SELECTOR_SIZE: i32 = (0.85 * (CELL_HEIGHT + CELL_WIDTH) / 2.0) as i32;
const SELECTOR_OFFSET: i32 = (0.5 * (SELECTOR_SIZE - TOKEN_SIZE) as f64) as i32;

pub struct HumanPlayer {
    selection: Option<(i32, i32)>,
    moving_selection: Option<(i32, i32)>,
    player_num: i32,
    move_buffer: Option<((i32, i32), (i32, i32))>,
    pub input_type: InputTypes,
    kb_state: KeyboardStates,
}

impl HumanPlayer {
    pub fn new(p: i32, it: InputTypes) -> HumanPlayer {
        HumanPlayer {
            selection: None,
            moving_selection: None,
            player_num: p,
            move_buffer: None,
            input_type: it,
            kb_state: KeyboardStates::MOVING,
        }
    }
    pub fn handle_mouse_click(&mut self, click_pos: (i32, i32)) {
        // TODO: more generic input (kb/mouse)
        if self.selection.is_some() && !self.is_selection(click_pos) {
            self.move_buffer = Some((self.selection.expect("Nothing in Move Buffer"), click_pos));
            self.selection = None;
            return;
        }
        self.selection = Some(click_pos);
    }
    pub fn handle_key_press(&mut self, key: Key) {
        match self.moving_selection {
            Some(_s) => {}
            None => self.moving_selection = Some((BOARD_SIZE / 2, BOARD_SIZE / 2)),
        }
        match key {
            Key::Right => {
                let pos = self.moving_selection.expect("no moving selection");
                if self.in_bounds((pos.0 + 1, pos.1)) {
                    self.moving_selection = Some((pos.0 + 1, pos.1))
                }
            }
            Key::Left => {
                let pos = self.moving_selection.expect("no moving selection");
                if self.in_bounds((pos.0 - 1, pos.1)) {
                    self.moving_selection = Some((pos.0 - 1, pos.1))
                }
            }
            Key::Down => {
                let pos = self.moving_selection.expect("no moving selection");
                if self.in_bounds((pos.0, pos.1 + 1)) {
                    self.moving_selection = Some((pos.0, pos.1 + 1))
                }
            }
            Key::Up => {
                let pos = self.moving_selection.expect("no moving selection");
                if self.in_bounds((pos.0, pos.1 - 1)) {
                    self.moving_selection = Some((pos.0, pos.1 - 1))
                }
            }
            Key::Return => match self.kb_state {
                KeyboardStates::MOVING => {
                    self.selection = self.moving_selection;
                    self.kb_state = KeyboardStates::SELECTED
                }
                KeyboardStates::SELECTED => {
                    let pos = self.moving_selection.expect("no moving selection");
                    if !self.is_selection(pos) {
                        self.move_buffer = Some((
                            self.selection.expect("Nothing in Move Buffer"),
                            self.moving_selection.expect("no moving selection"),
                        ));
                        self.selection = None;
                        self.kb_state = KeyboardStates::MOVING
                    }
                }
            },
            _ => {}
        }
    }
    fn is_selection(&self, pos: (i32, i32)) -> bool {
        match self.selection {
            Some(p) => return p == pos,
            None => return false,
        }
    }
    pub fn update(&mut self, board: &mut Board) {
        match self.move_buffer {
            Some(m) => {
                board.check_and_move_token(self.player_num, m.0, m.1);
                self.move_buffer = None;
            }
            None => {}
        }
    }
    pub fn draw_selection<G: Graphics>(&self, c: &Context, g: &mut G) {
        if let Some(sel) = self.selection {
            let canv_pos = Token::to_canv_pos(sel);
            CircleArc::new(color::BRIGHTBLUE, 2.0, 0.0, 1.9999 * consts::PI).draw(
                [
                    (canv_pos.0 - SELECTOR_OFFSET) as f64,
                    (canv_pos.1 - SELECTOR_OFFSET) as f64,
                    (SELECTOR_SIZE) as f64,
                    (SELECTOR_SIZE) as f64,
                ],
                &c.draw_state,
                c.transform,
                g,
            );
        }
        if let Some(sel) = self.moving_selection {
            let canv_pos = Token::to_canv_pos(sel);
            CircleArc::new(color::BRIGHTBLUE, 2.0, 0.0, 1.9999 * consts::PI).draw(
                [
                    (canv_pos.0 - SELECTOR_OFFSET) as f64,
                    (canv_pos.1 - SELECTOR_OFFSET) as f64,
                    (SELECTOR_SIZE) as f64,
                    (SELECTOR_SIZE) as f64,
                ],
                &c.draw_state,
                c.transform,
                g,
            );
        }
    }
    fn in_bounds(&self, loc: (i32, i32)) -> bool {
        loc.0 >= 0 && loc.0 < BOARD_SIZE && loc.1 >= 0 && loc.1 < BOARD_SIZE
    }
}
