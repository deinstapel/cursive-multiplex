extern crate cursive;

use cursive::views::{BoxView, TextView};
use cursive::Cursive;
use cursive_multiplex::Mux;

fn main() {
    let mut siv = Cursive::default();
    let (mux, _node1) = Mux::new(TextView::new("Hello World".to_string()));
    let boxview = BoxView::with_fixed_size((42, 11), mux);
    siv.add_layer(boxview);
    siv.run();
}
