extern crate cursive;

use cursive::Cursive;
use cursive_multiplex::{Mux, Path};

fn main() {
    let mut siv = Cursive::default();
    siv.show_debug_console();

    cursive::logger::init();

    let mut mux = Mux::new();

    match mux.add_horizontal(cursive::views::TextView::new("It works!".to_string()), None, Some("foo".to_string()), "bar".to_string()) {
        Ok(_) => {
            print!("Woohoo");
        },
        Err(_) => {
            print!("Ohhnoo");
        },
    }
    match mux.add_horizontal(cursive::views::TextView::new("It works!".to_string()), None, Some("foo".to_string()), "goo".to_string()) {
        Ok(_) => {
            print!("Woohoo");
        },
        Err(_) => {
            print!("Ohhnoo");
        },
    }
    match mux.add_horizontal(cursive::views::TextView::new("More text".to_string()), Some(Path::LeftOrUp(Box::new(None))), None, "loo".to_string()) {
        Ok(_) => {
            print!("Woohoo");
        },
        Err(_) => {
            print!("Ohhnoo");
        },
    }

    siv.add_fullscreen_layer(mux);
    siv.add_global_callback('q', Cursive::quit);
    siv.run();
}
