extern crate cursive;

use cursive::views::TextView;
use cursive::Cursive;
use cursive_multiplex::Mux;

fn main() {
    let mut siv = cursive::default();
    let mut mux = Mux::new();
    let right = mux
        .add_right_of(
            TextView::new("Right".to_string()),
            mux.root().build().unwrap(),
        )
        .expect("right failed");
    let left = mux
        .add_right_of(TextView::new("Left"), right)
        .expect("Left failed");
    mux.switch_views(right, left).expect("switch failed");

    siv.add_fullscreen_layer(mux);
    siv.run();
}
