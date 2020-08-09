use cursive::views::TextView;
use cursive_multiplex::Mux;

fn main() {
    let mut siv = cursive::default();
    let mut mux = Mux::new();
    let up = mux
        .add_right_of(TextView::new("Up"), mux.root().build().unwrap())
        .expect("Up failed");
    let down = mux
        .add_above(TextView::new("Down"), up)
        .expect("Left failed");
    mux.switch_views(up, down).expect("switch failed");

    siv.add_fullscreen_layer(mux);
    siv.run();
}
