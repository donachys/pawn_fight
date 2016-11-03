// use find_folder;

// use itertools;
use std::f64;
use graphics;
use opengl_graphics::GlGraphics;
// use opengl_graphics::glyph_cache::GlyphCache;
use piston::input::*;
use conrod;

use rand::{self, ThreadRng};
use game_objects::Board;
use game_objects::HumanPlayer;
use game_objects::CpuPlayer;
use game_objects::InputTypes;

use drawing::color;
use drawing::screen;

use traits::Player;

const SCREEN_WIDTH: i64 = screen::WIDTH;
const SCREEN_HEIGHT: i64 = screen::HEIGHT;
const BOARD_SIZE: i32 = screen::SIZE;

const CELL_HEIGHT: f64 = (SCREEN_HEIGHT as f64 / BOARD_SIZE as f64) as f64;
const CELL_WIDTH: f64 = (SCREEN_WIDTH as f64 / BOARD_SIZE as f64) as f64;

const NUM_CPU_PLAYERS: i32 = 1;
const NUM_HUM_PLAYERS: i32 = 1;
const NUM_PLAYERS: i32 = NUM_HUM_PLAYERS + NUM_CPU_PLAYERS;


widget_ids! {
    struct Ids {
        canvas,
        button,
        title
    }
}

#[derive(Default)]
struct Timers {
    current_time: f64
}

pub struct Game {
    /// A random number generator
    rng: ThreadRng,
    timers: Timers,
    board: Board,
    hum_players: Vec<HumanPlayer>,
    cpu_players: Vec<CpuPlayer>
}

impl Game {

    pub fn new() -> Game {
        let mut rng = rand::thread_rng();
        let mut board = Board::new(NUM_PLAYERS);
        let mut hum_players = Vec::with_capacity(NUM_HUM_PLAYERS as usize);
        let mut cpu_players = Vec::with_capacity(NUM_HUM_PLAYERS as usize);

            // Create a texture to use for efficiently caching text on the GPU.
        // let mut text_texture_cache =
        //     conrod::backend::piston_window::GlyphCache::new(&mut window, SCREEN_WIDTH, SCREEN_HEIGHT);

        // The image map describing each of our widget->image mappings (in our case, none).
        // let image_map = conrod::image::Map::new();
        // construct our `Ui`.
        // let mut ui = conrod::UiBuilder::new().build();

        // let mut ids = Ids::new(ui.widget_id_generator());
        // // Convert the piston event to a conrod event.
        // if let Some(event) = conrod::backend::piston_window::convert_event(e.clone(), &window) {
        //     ui.handle_event(event);
        // }

        // e.update(|_| {
        //     // println!("conrod update");
        //     use conrod::{color, widget, Colorable, Borderable, Labelable, Positionable, Sizeable, Widget};
        //     let mut ui = ui.set_widgets();
            
        //     widget::Canvas::new()
        //     .border(1.0)
        //     .pad(30.0)
        //     // .color(app.bg_color)
        //     .color(conrod::color::rgb(0.2, 0.35, 0.45))
        //     // .scroll_kids()
        //     .set(ids.canvas, &mut ui);
            
        //     // set_widgets(&mut ui, &mut app, &mut ids);
        //     if widget::Button::new()
        //             .w_h(200.0, 50.0)
        //             .mid_left_of(ids.canvas)
        //             .down_from(ids.title, 45.0)
        //             .rgb(0.4, 0.75, 0.6)
        //             .border(1.0)
        //             .label("PRESS")
        //             .set(ids.button, &mut ui)
        //             .was_clicked()
        //         {
        //             println!("clicked the button");
        //             show_gui = false;
        //         }
        // });
        
        // window.draw_2d(&e, |c, g| {
        //     if let Some(primitives) = ui.draw_if_changed() {
        //         fn texture_from_image<T>(img: &T) -> &T { img };
        //         conrod::backend::piston_window::draw(c, g, primitives,
        //                                              &mut text_texture_cache,
        //                                              &image_map,
        //                                              texture_from_image);
        //     }
        // });

        hum_players.push(HumanPlayer::new(0, InputTypes::MOUSE));
        hum_players.push(HumanPlayer::new(1, InputTypes::KEYBOARD));
        // cpu_players.push(CpuPlayer::new(1));
        //for players 
            // players.placetokens ( board )

        Game {
            rng: rng,
            timers: Timers::default(),
            board: board,
            hum_players: hum_players,
            cpu_players: cpu_players
        }
    }
    pub fn handle_mouse_click(&mut self, b: MouseButton, c: [f64; 2]){
        let cell_row: i32 = (c[0] / CELL_WIDTH) as i32;
        let cell_col: i32 = (c[1] / CELL_HEIGHT) as i32;
        for human in self.hum_players.iter_mut() {
            match human.input_type {
                InputTypes::MOUSE => human.handle_mouse_click((cell_row, cell_col)),
                InputTypes::KEYBOARD => {}
           }
        }
        // println!("Mouse cursor ({}, {}) clicked row '{}' col '{}'", c[0], c[1], cell_row, cell_col);
    }
    pub fn handle_key_press(&mut self, b: Key) {
        for human in self.hum_players.iter_mut() {
            match human.input_type {
                InputTypes::MOUSE => {},
                InputTypes::KEYBOARD => {human.handle_key_press(b)}
           }
        }
    }
    pub fn render(&mut self, c: graphics::context::Context, g: &mut GlGraphics) {
        // Clear everything
        graphics::clear(color::BLACK, g);
        self.board.draw(&c, g);
        for human in self.hum_players.iter() {
            human.draw_selection(&c, g);
        }
    }
    pub fn update(&mut self, dt: f64) {
        self.timers.current_time += dt;
        let player_count = self.board.players_remaining();
        if player_count == 1 {
            println!("VICTORY");
            return
        }
        for human in self.hum_players.iter_mut() {
            human.update(&mut self.board);
        }
        self.board.update(dt);
    }
}