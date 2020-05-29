// use graphics;
// use opengl_graphics::GlGraphics;

use piston_window::{Context, Graphics, Line, Rectangle};

use crate::drawing::color;
// use conrod::color;
use crate::drawing::screen;

use crate::game_objects::player_constants;
use crate::game_objects::Token;
use crate::game_objects::TokenStates;

const SCREEN_WIDTH: i64 = screen::WIDTH;
const SCREEN_HEIGHT: i64 = screen::HEIGHT;
const BOARD_SIZE: i32 = screen::SIZE;
const SQ_BOARD_SIZE: i32 = BOARD_SIZE * BOARD_SIZE;

const MAX_TOKENS: i32 = player_constants::MAX_TOKENS;
const TOKEN_OFFSET: i32 = (BOARD_SIZE - MAX_TOKENS) / 2;

pub struct Board {
    pub rows: i32,
    pub cols: i32,
    surface: Vec<Option<Token>>,
    num_players: i32,
}

impl Board {
    pub fn manhattan(p1: (i32, i32), p2: (i32, i32)) -> i32 {
        (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
    }
    pub fn is_diagonal(p1: (i32, i32), p2: (i32, i32)) -> bool {
        ((p1.0 - p2.0).abs() == 1) && ((p1.1 - p2.1).abs() == 1)
    }
    pub fn new(np: i32) -> Board {
        let mut out = Vec::with_capacity((np * SQ_BOARD_SIZE) as usize);
        for (_i, _inum) in (0..np).enumerate() {
            for (_j, _jnum) in (0..BOARD_SIZE).enumerate() {
                for (_k, _knum) in (0..BOARD_SIZE).enumerate() {
                    out.push(Option::None);
                }
            }
        }
        // TODO: more generic token placement? owned by player?
        for (_j, jnum) in (0..MAX_TOKENS).enumerate() {
            out[(0 * SQ_BOARD_SIZE + (jnum + TOKEN_OFFSET) * BOARD_SIZE + 0) as usize] =
                Some(Token::new(color::GREEN));
            out[(1 * SQ_BOARD_SIZE + (jnum + TOKEN_OFFSET) * BOARD_SIZE + (BOARD_SIZE - 1))
                as usize] = Some(Token::new(color::ORANGE));
        }
        Board {
            rows: BOARD_SIZE,
            cols: BOARD_SIZE,
            surface: out,
            num_players: np,
        }
    }
    pub fn check_and_move_token(&mut self, p: i32, from: (i32, i32), to: (i32, i32)) {
        // check that the move is legal
        if self.is_legal(p, from, to) {
            // move the token, and change states as needed
            if Board::is_diagonal(from, to) {
                self.kill_token_at(to);
                self.move_token(p, from, to);
            } else {
                self.move_token(p, from, to);
            }
        }
    }
    fn move_token(&mut self, p: i32, from: (i32, i32), to: (i32, i32)) {
        let from_ind = self.surface_index(p, from);
        let to_ind = self.surface_index(p, to);
        match self.surface[from_ind].as_mut() {
            Some(t) => {
                t.state = TokenStates::WAIT;
                t.set_wait_time(4.0);
            }
            None => return,
        }
        self.surface.swap(from_ind, to_ind);
        // println!("Swapped! From: ({}, {}) To: ({}, {})", from.0, from.1, to.0, to.1);
    }
    fn is_legal(&mut self, p: i32, from: (i32, i32), to: (i32, i32)) -> bool {
        match self.surface[self.surface_index(p, from)].as_ref() {
            Some(t) => {
                if t.state != TokenStates::READY {
                    return false;
                }
            }
            None => return false,
        }
        // check destination in all player layers for occupancy
        let mut occupied: bool = false;
        let mut occupying_player: i32 = -1;
        for i in 0..self.num_players {
            match self.surface[self.surface_index(i, to)].as_ref() {
                Some(_t) => {
                    occupied = true;
                    occupying_player = i;
                }
                None => continue,
            }
        }
        // RULES :
        // regular move
        (Board::manhattan(from, to) == 1   // horizontal or vertical one space
        &&                                 // and
        !occupied)                         // empty
        ||                                 // or
                        // kill move
        occupied                           // destination is occupied
        &&                                 // and
        occupying_player != p              // enemy player occupies the space
        &&                                 // and
        Board::manhattan(from, to) == 2    // exactly 2 distance
        &&                                 // and
        Board::is_diagonal(from, to) // diagonal direction
    }
    fn kill_token_at(&mut self, loc: (i32, i32)) {
        for (_i, inum) in (0..self.num_players).enumerate() {
            let loc_ind = self.surface_index(inum, loc);
            match self.surface[loc_ind].as_mut() {
                Some(t) => t.state = TokenStates::DEAD,
                None => continue,
            }
            self.surface[loc_ind] = None;
        }
    }
    fn surface_index(&self, p: i32, loc: (i32, i32)) -> usize {
        (p * SQ_BOARD_SIZE + loc.1 * BOARD_SIZE + loc.0) as usize
    }
    pub fn players_remaining(&self) -> i32 {
        let mut player_count: i32 = 0;
        for inum in 0..self.num_players {
            for jnum in (inum * SQ_BOARD_SIZE)..((inum + 1) * SQ_BOARD_SIZE) {
                match self.surface[jnum as usize].as_ref() {
                    Some(_t) => {
                        player_count += 1;
                        break;
                    }
                    None => {}
                }
            }
        }
        player_count
    }
    pub fn update(&mut self, dt: f64) {
        for token in self.surface.iter_mut() {
            match token.as_mut() {
                Some(t) => t.update(dt),
                None => {}
            }
        }
    }
    pub fn draw<G: Graphics>(&self, c: &Context, g: &mut G) {
        let box_width = SCREEN_WIDTH as f64 / self.cols as f64;
        let box_height = SCREEN_HEIGHT as f64 / self.rows as f64;
        let mut colored_row = true;
        let mut colored_col = false;
        for row in 0..self.rows {
            colored_row = !colored_row;
            if colored_row {
                colored_row = !colored_row;
                colored_col = !colored_col;
            }
            for col in 0..self.cols {
                colored_col = !colored_col;
                if colored_col {
                    let xpos = col as f64 * SCREEN_WIDTH as f64 / self.cols as f64;
                    let ypos = row as f64 * SCREEN_HEIGHT as f64 / self.rows as f64;
                    Rectangle::new(color::VIOLET).draw(
                        [xpos, ypos, box_width, box_height],
                        &c.draw_state,
                        c.transform,
                        g,
                    );
                }
            }
        }
        for row in 1..self.rows {
            let ypos = row as f64 * SCREEN_HEIGHT as f64 / self.rows as f64;
            Line::new(color::LIGHTGREY, 1.0).draw(
                [0.0, ypos, SCREEN_WIDTH as f64, ypos],
                &c.draw_state,
                c.transform,
                g,
            );
        }
        for col in 1..self.cols {
            let xpos = col as f64 * SCREEN_WIDTH as f64 / self.cols as f64;
            Line::new(color::LIGHTGREY, 1.0).draw(
                [xpos, 0.0, xpos, SCREEN_HEIGHT as f64],
                &c.draw_state,
                c.transform,
                g,
            );
        }
        for (_i, inum) in (0..self.num_players).enumerate() {
            for (_j, jnum) in (0..BOARD_SIZE).enumerate() {
                for (_k, knum) in (0..BOARD_SIZE).enumerate() {
                    match self.surface[self.surface_index(inum, (jnum, knum))].as_ref() {
                        Some(t) => t.draw_at(c, g, (jnum, knum)),
                        None => {}
                    }
                }
            }
        }
    }
}
