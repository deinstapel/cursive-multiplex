extern crate cursive;

use cursive::views::TextArea;
use cursive::Cursive;
use cursive_multiplex::Mux;

fn main() {
    let mut siv = Cursive::default();
    let (mut mux, left1) = Mux::new(TextArea::new());

    let right1 = mux
        .add_right_of(TextArea::new(), left1)
        .expect("right 1 failed");
    let _right3 = mux
        .add_below(TextArea::new(), right1)
        .expect("right 3 failed");
    let right2 = mux
        .add_right_of(TextArea::new(), right1)
        .expect("right 2 failed");

    let _left2 = mux
        .add_below(TextArea::new(), left1)
        .expect("left 2 failed");
    let _left3 = mux
        .add_below(TextArea::new(), left1)
        .expect("left 3 failed");
    mux.set_focus(right2);

    siv.add_fullscreen_layer(mux);
    siv.run();
}
