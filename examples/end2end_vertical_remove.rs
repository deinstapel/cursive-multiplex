use cursive::views::TextView;
use cursive_multiplex::Mux;

fn main() {
    let mut siv = cursive::default();
    let mut mux = Mux::new();
    let root = mux
        .add_right_of(TextView::new("Center"), mux.root().build().unwrap())
        .expect("Center failed");
    let _id = mux
        .add_above(TextView::new("Up"), root)
        .expect("Upper failed");
    let _id = mux
        .add_below(TextView::new("Down"), root)
        .expect("Lower failed");
    mux.remove_id(root).expect("remove failed");

    siv.add_fullscreen_layer(mux);
    siv.run();
}
