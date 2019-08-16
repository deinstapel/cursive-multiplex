extern crate cursive;

use cursive::views::TextView;
use cursive::Cursive;
use cursive_multiplex::Mux;

fn main() {
    let mut siv = Cursive::default();
    let (mut mux, left1) = Mux::new(TextView::new("left 1".to_string()));

    let right1 = mux
        .add_right_of(TextView::new("right 1"), left1)
        .expect("right 1 failed");
    let _right3 = mux
        .add_below(TextView::new("right 3"), right1)
        .expect("right 3 failed");
    let _right2 = mux
        .add_right_of(TextView::new("right 2"), right1)
        .expect("right 2 failed");

    let _left2 = mux
        .add_below(TextView::new("left 2"), left1)
        .expect("left 2 failed");
    let left3 = mux
        .add_below(TextView::new("left 3"), left1)
        .expect("left 3 failed");

    mux.switch_views(right1, left3).expect("switch failed");

    siv.add_fullscreen_layer(mux);
    siv.run();
}
