extern crate cursive;

use cursive::views::{BoxView, Panel, TextView};
use cursive::Cursive;
use cursive_multiplex::Mux;

fn main() {
    let mut siv = Cursive::default();
    let (mut mux, root) = Mux::new(TextView::new("Root".to_string()));
    let view = BoxView::with_fixed_size((42, 11), Panel::new(TextView::new("Fixed")));
    let _id = mux.add_right_of(view, root).expect("Fixed failed");

    siv.add_fullscreen_layer(mux);
    siv.run();
}
