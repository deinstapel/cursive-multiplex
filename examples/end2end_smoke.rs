use cursive::views::{ResizedView, TextView};
use cursive_multiplex::Mux;

fn main() {
    let mut siv = cursive::default();
    let mut mux = Mux::new();
    let _node1 = mux
        .add_right_of(
            TextView::new("Hello World".to_string()),
            mux.root().build().unwrap(),
        )
        .expect("hello failed");
    let boxview = ResizedView::with_fixed_size((42, 11), mux);
    siv.add_layer(boxview);
    siv.run();
}
