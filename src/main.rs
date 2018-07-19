extern crate gtk;
#[macro_use]
extern crate clap;

use clap::App;
use gtk::prelude::*;
use gtk::{
    Window, WindowType,
};

mod font;
mod cpu;

type BYTE = u8;
type WORD = u16;

fn main() {
    let yaml = load_yaml!("../res/config.yml");
    let matches = App::from_yaml(yaml).get_matches();

    gtk::init().unwrap();
    let window = Window::new(WindowType::Toplevel);

    window.show_all();
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}
