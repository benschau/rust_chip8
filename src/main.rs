#[macro_use]
extern crate clap;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

mod font;
mod cpu;
mod screen;

type BYTE = u8;
type WORD = u16;

use screen::GraphicsConfig;
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use std::path::Path;
use std::path::PathBuf;

fn main() {
    // Get project directory path to locate project resources consistently:
    let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    dir.push("res/");
    
    let yaml = load_yaml!("../res/config.yml");
    let matches = clap::App::from_yaml(yaml).get_matches();
    // TODO: do things with matches/yaml

    // TODO: filepath should be optional; otherwise, we should just
    // open up a empty window with a menu bar at the top.
    let test_file = Path::new("res/games/breakout.ch8");
    let display = test_file.display();
    let mut cpu = match cpu::Cpu::new(test_file) {
        Err(why) => panic!("Couldn't successfully initialize CPU with \"{}\".", display),
        Ok(cpu) => cpu,
    };

    cpu.run();

    let mut app = screen::App::new(GraphicsConfig::new("rust-chip8", cpu::SCREEN_DIM));

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut app.window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
