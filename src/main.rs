extern crate gtk;
#[macro_use]
extern crate clap;

use gtk::prelude::*;
use clap::App;

mod cpu;

type BYTE = u8;
type WORD = u16;

fn main() {
    let yaml = load_yaml!("../res/config.yml");
    let matches = App::from_yaml(yaml).get_matches();


}
