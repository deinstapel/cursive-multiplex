extern crate cursive;

use cursive::views::TextArea;
use cursive::Cursive;
use cursive_multiplex::Mux;

fn main() {
    let mut siv = cursive::default();
    let mut mux = Mux::new();
    let up = mux
        .add_right_of(TextArea::new(), mux.root().build().unwrap())
        .expect("Up failed");
    let down_left = mux
        .add_below(TextArea::new(), up)
        .expect("Left failed");
    let _down_right = mux
        .add_right_of(TextArea::new(), down_left)
        .expect("Right Failed");

    siv.add_fullscreen_layer(mux);
    siv.run();
}
