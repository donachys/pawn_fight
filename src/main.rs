// extern crate find_folder;
extern crate graphics;

extern crate itertools;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;
#[macro_use]
extern crate conrod;
extern crate rand;
extern crate sdl2_window;

use piston_window::*;
use opengl_graphics::GlGraphics;
use sdl2_window::Sdl2Window;


mod game;
mod drawing;
mod game_objects;
mod traits;

use game::Game;
use drawing::screen;
const SCREEN_WIDTH: u32 = screen::WIDTH as u32;
const SCREEN_HEIGHT: u32 = screen::HEIGHT as u32;
// Use this typedef to make type of window prettier.

// pub type SDL2GameWindow = PistonWindow<Sdl2Window>;

widget_ids! {
    struct Ids {
        canvas,
        button,
        title
    }
}

#[derive(Debug)]
struct CommandLineArgs {
    is_server: bool,
    is_client: bool,
    remoteip : String,
}

impl CommandLineArgs {
    pub fn new() -> Self {
        CommandLineArgs {
            is_server: false,
            is_client: false,
            remoteip : String::from(""),
        }
    }
}

fn parse_args() -> Result<CommandLineArgs, String> {
    let mut cli_args = CommandLineArgs::new();
    let mut args = std::env::args();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--server" => cli_args.is_server = true,
            "--client" => {
                cli_args.is_client = true;
                if let Some(ip) = args.next() {
                    cli_args.remoteip = ip;
                } else {
                    return Err(String::from("--client requires <remote_ip>"));
                }
            },
            _ => (),
        }
    }

    Ok(cli_args)
}

fn main() {
    let args = parse_args();
    if args.is_err() {
        println!("{}", args.unwrap_err());
        std::process::exit(1);
    }

    let opengl = OpenGL::V3_2;

    let mut game = Game::new();

    // let mut window: SDL2GameWindow = WindowSettings::new("Pawn_Fight!", [SCREEN_WIDTH, SCREEN_HEIGHT])
    //     .opengl(opengl).samples(64).exit_on_esc(true).build().unwrap();

    // Construct the window.
    let mut window: PistonWindow = WindowSettings::new("Pawn_Fight!!",
                                                       [SCREEN_WIDTH, SCREEN_HEIGHT])
        .opengl(opengl)
        .samples(64)
        .exit_on_esc(true)
        .vsync(true)
        .build()
        .unwrap();

    window.set_ups(60);
    window.set_max_fps(60);

    let mut gl = GlGraphics::new(opengl);

    let mut events = window.events();

    let mut cursor = [0.0, 0.0];
    // Create a texture to use for efficiently caching text on the GPU.
    let mut text_texture_cache =
        conrod::backend::piston_window::GlyphCache::new(&mut window, SCREEN_WIDTH, SCREEN_HEIGHT);
    // The image map describing each of our widget->image mappings (in our case, none).
    let image_map = conrod::image::Map::new();
    // construct our `Ui`.
    let mut ui = conrod::UiBuilder::new().build();

    let mut ids = Ids::new(ui.widget_id_generator());
    let mut show_gui: bool = true;

    while let Some(e) = events.next(&mut window) {
        // Event handling
        // Convert the piston event to a conrod event.
        if show_gui {
            if let Some(event) = conrod::backend::piston_window::convert_event(e.clone(), &window) {
                ui.handle_event(event);
            }
            println!("{:?}", e);
            e.update(|_| {
                // println!("conrod update");
                use conrod::{color, widget, Colorable, Borderable, Labelable, Positionable,
                             Sizeable, Widget};
                let mut ui = ui.set_widgets();

                widget::Canvas::new()
                .border(1.0)
                .pad(30.0)
                // .color(app.bg_color)
                .color(conrod::color::rgb(0.2, 0.35, 0.45))
                // .scroll_kids()
                .set(ids.canvas, &mut ui);

                // set_widgets(&mut ui, &mut app, &mut ids);
                if widget::Button::new()
                    .w_h(200.0, 50.0)
                    .mid_left_of(ids.canvas)
                    .down_from(ids.title, 45.0)
                    .rgb(0.4, 0.75, 0.6)
                    .border(1.0)
                    .label("PRESS")
                    .set(ids.button, &mut ui)
                    .was_clicked() {
                    println!("clicked the button");
                    show_gui = false;
                }
            });

            window.draw_2d(&e, |c, g| {
                if let Some(primitives) = ui.draw_if_changed() {
                    fn texture_from_image<T>(img: &T) -> &T {
                        img
                    };
                    conrod::backend::piston_window::draw(c,
                                                         g,
                                                         primitives,
                                                         &mut text_texture_cache,
                                                         &image_map,
                                                         texture_from_image);
                }
            });
        } else {
            match e {
                Event::Render(args) => {
                    gl.draw(args.viewport(), |c, g| game.render(c, g));
                }
                Event::Update(args) => {
                    game.update(args.dt);
                }
                Event::Input(Input::Press(Button::Mouse(button))) => {
                    // println!("Pressed mouse button '{:?}'", button);
                    game.handle_mouse_click(button, cursor);
                }
                Event::Input(Input::Press(Button::Keyboard(key))) => {
                    // handle the keyboard press
                    // println!("Pressed keyboard button '{:?}'", key);
                    game.handle_key_press(key);
                }
                Event::Input(Input::Release(Button::Keyboard(key))) => {
                    // handle the keyboard release?
                    // println!("Released keyboard button '{:?}'", key);
                }
                Event::Input(Input::Move(Motion::MouseCursor(x, y))) => {
                    cursor = [x, y];
                    println!("Mouse moved '{} {}'", x, y);
                }
                _ => {}
            }
        }

    }
}
