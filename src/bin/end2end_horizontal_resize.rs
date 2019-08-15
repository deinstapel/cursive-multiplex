extern crate cursive;

use cursive::Cursive;
use cursive::views::TextArea;
use cursive_multiplex::Mux;

fn main() {
    let mut siv = Cursive::default();
    let (mut mux, left) = Mux::new(TextArea::new());
    let center = mux.add_horizontal_id(TextArea::new(), left).expect("Center failed");
    let _right = mux.add_horizontal_id(TextArea::new(), center).expect("Right failed");
    mux.set_focus(left);

    siv.add_fullscreen_layer(mux);
    siv.run();
}
