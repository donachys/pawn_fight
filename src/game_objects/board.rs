use std::mem;

use graphics;
use opengl_graphics::GlGraphics;

use drawing::color;
use drawing::screen;

use game_objects::Token;
use game_objects::TokenStates;
use game_objects::player_constants;

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
    num_players: i32
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
        for (i, inum) in (0..np).enumerate() {
            for (j, jnum) in (0..BOARD_SIZE).enumerate() {
                for (k, knum) in (0..BOARD_SIZE).enumerate() {
                    out.push(Option::None);
                }
            }
        }
        //TODO: more generic token placement? owned by player?
        for (j, jnum) in (0..MAX_TOKENS).enumerate() {
            out[(0 * SQ_BOARD_SIZE
                  + (jnum + TOKEN_OFFSET) * BOARD_SIZE
                  + 0) as usize] = Some(Token::new(color::GREEN));
             out[(1 * SQ_BOARD_SIZE
                  + (jnum + TOKEN_OFFSET) * BOARD_SIZE
                  + (BOARD_SIZE - 1)) as usize] = Some(Token::new(color::ORANGE));
        }
        Board {
            rows: BOARD_SIZE,
            cols: BOARD_SIZE,
            surface: out,
            num_players: np
        }
    }
    pub fn check_and_move_token(&mut self, p: i32, from: (i32, i32), to: (i32, i32)) {
        //check that the move is legal
        if self.is_legal(p, from, to) {
            //move the token, and change states as needed
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
                None => return
            }
            self.surface.swap(from_ind, to_ind);
            // println!("Swapped! From: ({}, {}) To: ({}, {})", from.0, from.1, to.0, to.1);
    }
    fn is_legal(&mut self, p: i32, from: (i32, i32), to: (i32, i32)) -> bool {
        match self.surface[self.surface_index(p, from)].as_ref() {
            Some(t) => {
                if t.state != TokenStates::READY {
                            return false 
                }
            }
            None => return false
        }
        // check destination in all player layers for occupancy
        let mut occupied: bool = false;
        let mut occupying_player: i32 = -1;
        for i in 0 .. self.num_players {
            match self.surface[self.surface_index(i, to)].as_ref() {
                Some(t) => {occupied = true; occupying_player = i;}
                None => continue
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
        Board::is_diagonal(from, to)       // diagonal direction
    }
    fn kill_token_at(&mut self, loc: (i32, i32)) {
        for (i, inum) in (0 .. self.num_players).enumerate() {
            let loc_ind = self.surface_index(inum, loc);
            match self.surface[loc_ind].as_mut() {
                Some(t) => t.state = TokenStates::DEAD,
                None => continue
            }
            self.surface[loc_ind] = None;
        }
    }
    fn surface_index(&self, p: i32, loc: (i32, i32)) -> usize {
        (p * SQ_BOARD_SIZE + loc.1 * BOARD_SIZE+ loc.0) as usize
    }
    pub fn players_remaining(&self) -> i32 {
        let mut player_count: i32 = 0;
        for inum in 0..self.num_players {
            for jnum in (inum * SQ_BOARD_SIZE) .. ((inum + 1) * SQ_BOARD_SIZE) {
                match self.surface[jnum as usize].as_ref() {
                    Some(t) => {player_count += 1; break},
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
    pub fn draw(&self, c: &graphics::Context, g: &mut GlGraphics) {
        let box_width = SCREEN_WIDTH as f64 / self.cols as f64;
        let box_height = SCREEN_HEIGHT as f64 / self.rows as f64;
        let mut colored_row = true;
        let mut colored_col = false;
        for row in 0..self.rows{
            colored_row = !colored_row;
            if colored_row{
                colored_row = !colored_row;
                colored_col = !colored_col;
            }
            for col in 0..self.cols {
                colored_col = !colored_col;
                if colored_col {
                    let xpos = col as f64 * SCREEN_WIDTH as f64 / self.cols as f64;
                    let ypos = row as f64 * SCREEN_HEIGHT as f64 / self.rows as f64;
                    graphics::Rectangle::new(color::VIOLET).draw(
                                    [xpos, ypos, box_width, box_height],
                                    &c.draw_state, c.transform, g);
                }
            }
        }
        for row in 1..self.rows {
            let ypos = row as f64 * SCREEN_HEIGHT as f64 / self.rows as f64;
            graphics::Line::new(color::LIGHTGREY, 1.0).draw(
                [0.0, ypos, SCREEN_WIDTH as f64, ypos],
                &c.draw_state, c.transform, g);    
        }
        for col in 1..self.cols {
            let xpos = col as f64 * SCREEN_WIDTH as f64 / self.cols as f64;
            graphics::Line::new(color::LIGHTGREY, 1.0).draw(
                [xpos, 0.0, xpos, SCREEN_HEIGHT as f64],
                &c.draw_state, c.transform, g);
        }
        for (i, inum) in (0..self.num_players).enumerate() {
            for (j, jnum) in (0..BOARD_SIZE).enumerate() {
                for (k, knum) in (0..BOARD_SIZE).enumerate() {
                    match self.surface[self.surface_index(inum, (jnum, knum))].as_ref() {
                        Some(t) => t.draw_at(c, g, (jnum, knum)),
                        None => {}
                    }
                }
            }
        }
    }
}
    // pub fn init_tokens(&mut self){
    //     for token_num in 0..MAX_TOKENS{
    //         self.p1_tokens.push(Token::new((0, token_num+TOKEN_OFFSET), color::GREEN));
    //     }
    //     for token_num in 0..MAX_TOKENS{
    //         self.p2_tokens.push(Token::new((BOARD_SIZE-1, token_num+TOKEN_OFFSET), color::ORANGE));
    //     }
    // }
    // pub fn select_if_owned_by(&mut self, p: i64, c: (i64, i64)) -> bool{
    //     if p == 0{
    //         // println!("player 0");
    //         for token_num in 0..self.p1_tokens.len(){
    //             if self.p1_tokens[token_num].pos.0 == c.0 as i32
    //                 && self.p1_tokens[token_num].pos.1 == c.1 as i32{
    //                     self.p1_selection = token_num;
    //                     return true;
    //             }
    //         }
    //     }
    //     // this probably won't happen
    //     // if p == 1{
    //     // }
    //     return false;
    // }
    // pub fn move_if_permitted(&mut self, p: i64, c: (i64, i64)) -> bool {
    //     let mut token_moved = false;
    //     //if p == 0
    //     //if the click is adjacent
    //     // same x value indicates in the same column
        
    //     let mut is_empty: bool = self.cell_is_empty(c);
    //     if c.0 == self.p1_tokens[self.p1_selection].pos.0 as i64
    //             && (c.1 - self.p1_tokens[self.p1_selection].pos.1 as i64).abs() == 1 {
    //         if self.p1_tokens[self.p1_selection].state == TokenStates::READY{
    //             if is_empty {
    //                 self.p1_tokens[self.p1_selection].set_pos((c.0 as i32, c.1 as i32));
    //                 self.p1_tokens[self.p1_selection].reset_time();
    //                 self.p1_tokens[self.p1_selection].set_state(TokenStates::PREP);
    //                 token_moved = true;
    //             }
    //         }
    //     }
    //     // same y value indicates same row
    //     else if c.1 == self.p1_tokens[self.p1_selection].pos.1 as i64
    //             && (c.0 - self.p1_tokens[self.p1_selection].pos.0 as i64).abs() == 1 {
    //         if self.p1_tokens[self.p1_selection].state == TokenStates::READY{
    //             if is_empty {
    //                 self.p1_tokens[self.p1_selection].set_pos((c.0 as i32, c.1 as i32));
    //                 self.p1_tokens[self.p1_selection].reset_time();
    //                 self.p1_tokens[self.p1_selection].set_state(TokenStates::PREP);
    //                 token_moved = true;
    //             }
    //         }
    //     }
    //     return token_moved;
    // }
    // pub fn move_if_killable(&mut self, p: i64, c: (i64, i64)) -> bool {
    //     // let mut is_empty: bool = self.cell_is_empty(c);
    //     let mut is_enemy: bool = self.cell_has_enemy(p, c);
    //     if is_enemy
    //         && (c.0 - self.p1_tokens[self.p1_selection].pos.0 as i64).abs() == 1
    //         && (c.1 - self.p1_tokens[self.p1_selection].pos.1 as i64).abs() == 1 {
    //         if self.p1_tokens[self.p1_selection].state == TokenStates::READY {
    //             for token_num in 0..self.p2_tokens.len() {
    //                 if self.p2_tokens[token_num].pos.0 == c.0 as i32 && self.p2_tokens[token_num].pos.1 == c.1 as i32 {
    //                     self.p1_tokens[self.p1_selection].pos = (c.0 as i32, c.1 as i32);
    //                     self.p2_tokens[token_num].state = TokenStates::DEAD;
    //                 }
    //             }
    //         }
    //     }
    //     return false;
    // }
    // pub fn cell_is_empty(&mut self, c: (i64, i64)) -> bool {
    //     for token_num in 0..self.p1_tokens.len(){
    //         let ref token = self.p1_tokens[token_num];
    //         if token.pos.0 == c.0 as i32 && token.pos.1 == c.1 as i32 {
    //             return false;
    //         }
    //     }
    //     for token_num in 0..self.p2_tokens.len(){
    //         let ref token = self.p2_tokens[token_num];
    //         if token.pos.0 == c.0 as i32 && token.pos.1 == c.1 as i32 {
    //             return false;
    //         }   
    //     }
    //     return true;
    // }
    // pub fn cell_has_enemy(&mut self, p: i64, c: (i64, i64)) -> bool {
    //     if p == 0 {
    //         for token_num in 0..self.p2_tokens.len() {
    //             let ref token = self.p2_tokens[token_num];
    //             if token.pos.0 == c.0 as i32 && token.pos.1 == c.1 as i32 {
    //                 return true;
    //             }   
    //         }
    //     }
    //     else if p == 1 {
    //         for token_num in 0..self.p1_tokens.len() {
    //             let ref token = self.p1_tokens[token_num];
    //             if token.pos.0 == c.0 as i32 && token.pos.1 == c.1 as i32 {
    //                 return true;
    //             }
    //         }
    //     }
    //     return false;
    // }
    // pub fn update(&mut self, dt: f64) {
    //     for token in self.p1_tokens.iter_mut(){
    //         token.update(dt);
    //     }
    //     for token in self.p2_tokens.iter_mut(){
    //         token.update(dt);
    //     }
    // }
    // pub fn draw(&self, c: &graphics::Context, g: &mut GlGraphics) {
    //     let box_width = SCREEN_WIDTH as f64 / self.cols as f64;
    //     let box_height = SCREEN_HEIGHT as f64 / self.rows as f64;
    //     let mut colored_row = true;
    //     let mut colored_col = false;
    //     for row in 0..self.rows{
    //         colored_row = !colored_row;
    //         if colored_row{
    //             colored_row = !colored_row;
    //             colored_col = !colored_col;
    //         }
    //         for col in 0..self.cols{
    //             colored_col = !colored_col;
    //             if colored_col {
    //                 let xpos = col as f64 * SCREEN_WIDTH as f64 / self.cols as f64;
    //                 let ypos = row as f64 * SCREEN_HEIGHT as f64 / self.rows as f64;
    //                 graphics::Rectangle::new(color::VIOLET).draw(
    //                                 [xpos, ypos, box_width, box_height],
    //                                 &c.draw_state, c.transform, g);
    //             }
    //         }
    //     }
    //     for row in 1..self.rows{
    //         let ypos = row as f64 * SCREEN_HEIGHT as f64 / self.rows as f64;
    //         graphics::Line::new(color::BLACK, 1.0).draw(
    //             [0.0, ypos, SCREEN_WIDTH as f64, ypos],
    //             &c.draw_state, c.transform, g);    
    //     }
    //     for col in 1..self.cols{
    //         let xpos = col as f64 * SCREEN_WIDTH as f64 / self.cols as f64;
    //         graphics::Line::new(color::BLACK, 1.0).draw(
    //             [xpos, 0.0, xpos, SCREEN_HEIGHT as f64],
    //             &c.draw_state, c.transform, g);
    //     }
    //     self.p1_tokens[self.p1_selection].draw_selection(c, g);
    //     self.p2_tokens[self.p2_selection].draw_selection(c, g);
    //     for token in self.p1_tokens.iter(){
    //         token.draw(c, g);
    //     }
    //     for token in self.p2_tokens.iter(){
    //         token.draw(c, g);
    //     }
    // }

