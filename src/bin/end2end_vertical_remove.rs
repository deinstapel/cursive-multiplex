extern crate cursive;

use cursive::Cursive;
use cursive::views::TextView;
use cursive_multiplex::Mux;

fn main() {
    let mut siv = Cursive::default();
    let (mut mux, root) = Mux::new(TextView::new("Center".to_string()));
    let _id = mux.add_above(TextView::new("Up"), root).expect("Upper failed");
    let _id = mux.add_below(TextView::new("Down"), root).expect("Lower failed");
    mux.remove_id(root).expect("remove failed");

    siv.add_fullscreen_layer(mux);
    siv.run();
}
