extern crate cursive;

use cursive::Cursive;
use cursive::views::TextView;
use cursive_multiplex::Mux;

fn main() {
    let mut siv = Cursive::default();
    let (mut mux, root) = Mux::new(TextView::new("Center".to_string()));
    let _id = mux.add_horizontal_id(TextView::new("Left"), root).expect("Left failed");
    let _id = mux.add_horizontal_id(TextView::new("Right"), root).expect("Right failed");
    mux.remove_id(root).expect("remove failed");

    siv.add_fullscreen_layer(mux);
    siv.run();
}
