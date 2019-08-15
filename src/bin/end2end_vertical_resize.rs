extern crate cursive;

use cursive::Cursive;
use cursive::views::TextArea;
use cursive_multiplex::Mux;

fn main() {
    let mut siv = Cursive::default();
    let (mut mux, up) = Mux::new(TextArea::new());
    let center = mux.add_vertical_id(TextArea::new(), up).expect("Center failed");
    let _down = mux.add_vertical_id(TextArea::new(), center).expect("Down failed");
    mux.set_focus(up);

    siv.add_fullscreen_layer(mux);
    siv.run();
}
