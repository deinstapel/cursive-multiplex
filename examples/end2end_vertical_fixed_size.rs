extern crate cursive;

use cursive::views::{Panel, ResizedView, TextView};
use cursive::Cursive;
use cursive_multiplex::Mux;

fn main() {
    let mut siv = cursive::default();
    let mut mux = Mux::new();
    let root = mux
        .add_right_of(TextView::new("Root"), mux.root().build().unwrap())
        .expect("Root failed");
    let view = ResizedView::with_fixed_size((42, 11), Panel::new(TextView::new("Fixed")));
    let _id = mux.add_below(view, root).expect("Fixed failed");

    siv.add_fullscreen_layer(mux);
    siv.run();
}
