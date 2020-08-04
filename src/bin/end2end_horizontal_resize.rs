extern crate cursive;

use cursive::views::TextArea;
use cursive::Cursive;
use cursive_multiplex::Mux;

fn main() {
    let mut siv = cursive::default();
    let mut mux = Mux::new();
    let left = mux
        .add_right_of(TextArea::new(), mux.root().build().unwrap())
        .expect("left failed");
    let center = mux
        .add_right_of(TextArea::new(), left)
        .expect("Center failed");
    let _right = mux
        .add_right_of(TextArea::new(), center)
        .expect("Right failed");
    mux.set_focus(left);

    siv.add_fullscreen_layer(mux);
    siv.run();
}
