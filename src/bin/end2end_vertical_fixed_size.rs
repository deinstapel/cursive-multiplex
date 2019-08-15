extern crate cursive;

use cursive::Cursive;
use cursive::views::{Panel, BoxView, TextView};
use cursive_multiplex::Mux;

fn main() {
    let mut siv = Cursive::default();
    let (mut mux, root) = Mux::new(TextView::new("Root".to_string()));
    let mut view = BoxView::with_fixed_size((42, 11), Panel::new(TextView::new("Fixed")));
    view.set_squishable(false);
    let _id = mux.add_below(view, root).expect("Fixed failed");

    siv.add_fullscreen_layer(mux);
    siv.run();
}
