extern crate cursive;

use cursive::Cursive;
use cursive_multiplex::{Mux, Path};

fn main() {
    let mut siv = Cursive::default();
    siv.show_debug_console();

    cursive::logger::init();

    let mux = Mux::new();

    siv.add_fullscreen_layer(mux);
    siv.add_global_callback('q', Cursive::quit);
    siv.run();
}
