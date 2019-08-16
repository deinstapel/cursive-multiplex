extern crate cursive;

use cursive::views::TextView;
use cursive::Cursive;
use cursive_multiplex::Mux;

fn main() {
    let mut siv = Cursive::default();
    let (mut mux, right) = Mux::new(TextView::new("Right".to_string()));
    let left = mux
        .add_right_of(TextView::new("Left"), right)
        .expect("Left failed");
    mux.switch_views(right, left).expect("switch failed");

    siv.add_fullscreen_layer(mux);
    siv.run();
}
