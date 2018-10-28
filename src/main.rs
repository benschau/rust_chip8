#[macro_use]
extern crate clap;

use clap::App;
//use piston_window::*;

mod font;
mod cpu;

type BYTE = u8;
type WORD = u16;

fn main() {
    let yaml = load_yaml!("../res/config.yml");
    let matches = App::from_yaml(yaml).get_matches();
    
    let mut cpu: cpu::Cpu = cpu::Cpu::new("../res/games/breakout.ch8");
    cpu.run();
}
