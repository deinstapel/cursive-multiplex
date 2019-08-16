extern crate cursive;

use cursive::views::TextView;
use cursive::Cursive;
use cursive_multiplex::Mux;

fn main() {
    let mut siv = Cursive::default();
    let (mut mux, up) = Mux::new(TextView::new("Up".to_string()));
    let down = mux
        .add_above(TextView::new("Down"), up)
        .expect("Left failed");
    mux.switch_views(up, down).expect("switch failed");

    siv.add_fullscreen_layer(mux);
    siv.run();
}
