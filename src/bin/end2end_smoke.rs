extern crate cursive;

use cursive::views::{BoxView, TextView};
use cursive::Cursive;
use cursive_multiplex::Mux;

fn main() {
    let mut siv = Cursive::default();
    let mut mux = Mux::new();
    let _node1 = mux
        .add_right_of(
            TextView::new("Hello World".to_string()),
            mux.root().build().unwrap(),
        )
        .expect("hello failed");
    let boxview = BoxView::with_fixed_size((42, 11), mux);
    siv.add_layer(boxview);
    siv.run();
}
