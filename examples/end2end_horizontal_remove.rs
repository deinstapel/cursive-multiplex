use cursive::views::TextView;
use cursive_multiplex::Mux;

fn main() {
    let mut siv = cursive::default();
    let mut mux = Mux::new();
    let root = mux
        .add_right_of(TextView::new("Center"), mux.root().build().unwrap())
        .expect("Center failed");
    let _id = mux
        .add_left_of(TextView::new("Left"), root)
        .expect("Left failed");
    let _id = mux
        .add_right_of(TextView::new("Right"), root)
        .expect("Right failed");
    mux.remove_id(root).expect("remove failed");

    siv.add_fullscreen_layer(mux);
    siv.run();
}
