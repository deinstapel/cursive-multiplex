use cursive::views::TextArea;
use cursive_multiplex::Mux;

fn main() {
    let mut siv = cursive::default();
    let mut mux = Mux::new();
    let left = mux
        .add_right_of(TextArea::new(), mux.root().build().unwrap())
        .expect("left failed");
    let _right = mux
        .add_right_of(TextArea::new(), left)
        .expect("right failed");
    let _id = mux.add_right_of(TextArea::new(), left).expect("1st failed");
    mux.set_focus(left);

    siv.add_fullscreen_layer(mux);
    siv.run();
}
