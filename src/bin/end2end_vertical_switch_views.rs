extern crate cursive;

use cursive::Cursive;
use cursive::views::TextView;
use cursive_multiplex::Mux;

fn main() {
    let mut siv = Cursive::default();
    let (mut mux, down) = Mux::new(TextView::new("Down".to_string()));
    let up = mux.add_vertical_id(TextView::new("Up"), down).expect("Left failed");
    mux.switch_views(down, up).expect("switch failed");

    siv.add_fullscreen_layer(mux);
    siv.run();
}
