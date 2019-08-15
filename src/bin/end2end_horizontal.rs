extern crate cursive;

use cursive::Cursive;
use cursive::views::TextView;
use cursive_multiplex::Mux;

fn main() {
    let mut siv = Cursive::default();
    let (mut mux, root) = Mux::new(TextView::new("Center".to_string()));
    let _id = mux.add_right_of(TextView::new("Right"), root).expect("Right failed");
    let _id = mux.add_left_of(TextView::new("Left"), root).expect("Left failed");

    siv.add_fullscreen_layer(mux);
    siv.run();
}
