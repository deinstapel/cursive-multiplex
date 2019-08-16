extern crate cursive;

use cursive::views::TextArea;
use cursive::Cursive;
use cursive_multiplex::Mux;

fn main() {
    let mut siv = Cursive::default();
    let (mut mux, left) = Mux::new(TextArea::new());
    let right = mux
        .add_right_of(TextArea::new(), left)
        .expect("right failed");
    let _id = mux.add_right_of(TextArea::new(), left).expect("1st failed");
    mux.set_focus(right);

    siv.add_fullscreen_layer(mux);
    siv.run();
}
