extern crate cursive;

use cursive::views::TextArea;
use cursive::Cursive;
use cursive_multiplex::Mux;

fn main() {
    let mut siv = Cursive::default();
    let mut mux = Mux::new();
    let up = mux
        .add_right_of(TextArea::new(), mux.root().build().unwrap())
        .expect("Up failed");
    let up_right = mux
        .add_right_of(TextArea::new(), up)
        .expect("Left failed");
    let _down_right = mux
        .add_below(TextArea::new(), up_right)
        .expect("Right Failed");

    siv.add_fullscreen_layer(mux);
    siv.run();
}
