#[macro_use]
extern crate clap;
extern crate piston_window;

use clap::App;
use piston_window::*;

mod font;
mod cpu;

type BYTE = u8;
type WORD = u16;

fn main() {
    let yaml = load_yaml!("../res/config.yml");
    let matches = App::from_yaml(yaml).get_matches();
    
    // screen_scaling here means that every 'pixel' is screen_scaling x screen_scaling size.
    let screen_scaling = 10;
    let scr = [64 * screen_scaling, 32 * screen_scaling];

    let mut window: PistonWindow = 
        WindowSettings::new("rust-chip8", scr)
        .exit_on_esc(true).build().unwrap();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            clear([1.0; 4], graphics);
            rectangle([1.0, 0.0, 0.0, 1.0],
                      [0.0, 0.0, 100.0, 100.0],
                      context.transform,
                      graphics);
        });
    }
}
