extern crate cursive;

use cursive_core::views::TextArea;
use cursive_core::Cursive;
use cursive_multiplex::Mux;

fn main() {
    let mut siv = cursive::default();
    let mut mux = Mux::new();
    let upper = mux
        .add_right_of(TextArea::new(), mux.root().build().unwrap())
        .expect("upper failed");
    let _lower = mux.add_below(TextArea::new(), upper).expect("lower failed");
    let _id = mux.add_below(TextArea::new(), upper).expect("1st failed");
    mux.set_focus(upper);

    siv.add_fullscreen_layer(mux);
    siv.run();
}
