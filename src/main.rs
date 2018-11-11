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

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use std::path::Path;

fn main() {
    let yaml = load_yaml!("../res/config.yml");
    let matches = clap::App::from_yaml(yaml).get_matches();
    let test_file = Path::new("../res/games/breakout.ch8");
    let display = test_file.display();

    // TODO: do things with matches/yaml
    
    // TODO: filepath should be optional; otherwise, we should just
    // open up a empty window with a menu bar at the top.
    let mut cpu = match cpu::Cpu::new(test_file) {
        Err(why) => panic!("Couldn't read {}.", display),
        Ok(cpu) => cpu,
    };
    // cpu.run();
    
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "spinning-square",
            [200, 200]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = screen::App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
