#![warn(rust_2018_idioms)]

use conrod_core::{widget_ids, Borderable, Colorable, Labelable, Positionable, Sizeable, Widget};
use piston_window::Button;
use piston_window::MouseCursorEvent;
use piston_window::OpenGL;
use piston_window::{clear, texture::UpdateTexture};
use piston_window::{G2d, G2dTexture, TextureSettings};
use piston_window::{PistonWindow, PressEvent, UpdateEvent, Window, WindowSettings};

mod drawing;
mod game;
mod game_objects;

use crate::drawing::color;
use crate::drawing::screen;
use crate::game::Game;
const SCREEN_WIDTH: u32 = screen::WIDTH as u32;
const SCREEN_HEIGHT: u32 = screen::HEIGHT as u32;

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
    remoteip: String,
}

impl CommandLineArgs {
    pub fn new() -> Self {
        CommandLineArgs {
            is_server: false,
            is_client: false,
            remoteip: String::from(""),
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
            }
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

    let mut game = Game::new();

    // Construct the window.
    let mut window: PistonWindow =
        WindowSettings::new("Pawn_Fight!!", [SCREEN_WIDTH, SCREEN_HEIGHT])
            .graphics_api(OpenGL::V3_2)
            .samples(64)
            .exit_on_esc(true)
            .vsync(true)
            .build()
            .unwrap();
    // window.set_ups(60);
    // window.set_max_fps(60);

    // construct our `Ui`.
    let mut ui = conrod_core::UiBuilder::new([SCREEN_WIDTH as f64, SCREEN_HEIGHT as f64]).build();

    let mut cursor = [0.0, 0.0]; // Create texture context to perform operations on textures.

    let mut texture_context = window.create_texture_context();

    // Create a texture to use for efficiently caching text on the GPU.
    let mut text_vertex_data = Vec::new();
    let (mut glyph_cache, mut text_texture_cache) = {
        const SCALE_TOLERANCE: f32 = 0.1;
        const POSITION_TOLERANCE: f32 = 0.1;
        let cache = conrod_core::text::GlyphCache::builder()
            .dimensions(SCREEN_WIDTH, SCREEN_HEIGHT)
            .scale_tolerance(SCALE_TOLERANCE)
            .position_tolerance(POSITION_TOLERANCE)
            .build();
        let buffer_len = SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize;
        let init = vec![128; buffer_len];
        let settings = TextureSettings::new();
        let texture = G2dTexture::from_memory_alpha(
            &mut texture_context,
            &init,
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            &settings,
        )
        .unwrap();

        (cache, texture)
    };

    // The image map describing each of our widget->image mappings (in our case, none).
    let image_map = conrod_core::image::Map::new();

    let ids = Ids::new(ui.widget_id_generator());
    let mut show_gui: bool = true;
    let mut clearnow: bool = false;

    // Poll events from the window
    while let Some(e) = window.next() {
        e.update(|args| game.update(args.dt));

        e.press(|b| {
            match b {
                Button::Mouse(button) => {
                    // println!("Pressed mouse button '{:?}'", button);
                    game.handle_mouse_click(button, cursor);
                }
                Button::Keyboard(key) => {
                    // println!("Pressed keyboard button '{:?}'", key);
                    game.handle_key_press(key);
                }
                _ => {}
            }
        });

        e.mouse_cursor(|[x, y]| {
            // println!("Mouse moved '{} {}'", x, y);
            cursor = [x, y];
        });

        let size = window.size();
        let (win_w, win_h) = (
            size.width as conrod_core::Scalar,
            size.height as conrod_core::Scalar,
        );
        if let Some(event) = conrod_piston::event::convert(e.clone(), win_w, win_h) {
            ui.handle_event(event);
        }

        e.update(|_| {
            if show_gui {
                let mut ui = ui.set_widgets();

                conrod_core::widget::Canvas::new()
                    .border(1.0)
                    .pad(30.0)
                    // .color(app.bg_color)
                    .color(conrod_core::color::rgb(0.2, 0.35, 0.45))
                    // .scroll_kids()
                    .set(ids.canvas, &mut ui);

                // set_widgets(&mut ui, &mut app, &mut ids);
                if conrod_core::widget::Button::new()
                    .w_h(200.0, 50.0)
                    .mid_left_of(ids.canvas)
                    .down_from(ids.title, 45.0)
                    .rgb(0.4, 0.75, 0.6)
                    .border(1.0)
                    .label("PRESS")
                    .set(ids.button, &mut ui)
                    .was_clicked()
                {
                    println!("clicked the button");
                    show_gui = false;
                    clearnow = true;
                }
            }
        });

        window.draw_2d(&e, |context, graphics, device| {
            if show_gui {
                if let Some(primitives) = ui.draw_if_changed() {
                    // A function used for caching glyphs to the texture cache.
                    let cache_queued_glyphs =
                        |_graphics: &mut G2d<'_>,
                         cache: &mut G2dTexture,
                         rect: conrod_core::text::rt::Rect<u32>,
                         data: &[u8]| {
                            let offset = [rect.min.x, rect.min.y];
                            let size = [rect.width(), rect.height()];
                            let format = piston_window::texture::Format::Rgba8;
                            text_vertex_data.clear();
                            text_vertex_data
                                .extend(data.iter().flat_map(|&b| vec![255, 255, 255, b]));
                            UpdateTexture::update(
                                cache,
                                &mut texture_context,
                                format,
                                &text_vertex_data[..],
                                offset,
                                size,
                            )
                            .expect("failed to update texture")
                        };

                    // Specify how to get the drawable texture from the image. In this case, the image
                    // *is* the texture.
                    fn texture_from_image<T>(img: &T) -> &T {
                        img
                    }

                    conrod_piston::draw::primitives(
                        primitives,
                        context,
                        graphics,
                        &mut text_texture_cache,
                        &mut glyph_cache,
                        &image_map,
                        cache_queued_glyphs,
                        texture_from_image,
                    );
                }

                texture_context.encoder.flush(device);
            } else {
                game.render(&context, graphics);
                if clearnow {
                    clear(color::BLACK, graphics);
                    clearnow = false;
                }
            }
        });
    }
}
