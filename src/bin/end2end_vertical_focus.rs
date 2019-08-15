extern crate cursive;

use cursive::Cursive;
use cursive::views::TextArea;
use cursive_multiplex::Mux;

fn main() {
    let mut siv = Cursive::default();
    let (mut mux, upper) = Mux::new(TextArea::new());
    let lower = mux.add_vertical_id(TextArea::new(), upper).expect("lower left failed");
    let _id = mux.add_vertical_id(TextArea::new(), upper).expect("1st failed");
    mux.set_focus(lower);

    siv.add_fullscreen_layer(mux);
    siv.run();
}
