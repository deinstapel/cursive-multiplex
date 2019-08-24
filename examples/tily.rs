extern crate cursive;

use cursive::views::{BoxView, TextArea, TextView};
use cursive::Cursive;
use cursive_multiplex::{Id, Mux};

fn main() {
    let mut siv = Cursive::default();
    siv.show_debug_console();
    let mut mux = Mux::new();
    let top_left_corner = mux
        .add_right_of(
            BoxView::with_full_screen(TextArea::new()),
            mux.root().build().unwrap(),
        )
        .expect("first failed");

    let top_right_mid = mux
        .add_right_of(BoxView::with_full_screen(TextArea::new()), top_left_corner)
        .unwrap();
    let bottom_right_mid = mux
        .add_below(BoxView::with_full_screen(TextView::new("I will not be focused!")), top_right_mid)
        .unwrap();
    let _ = mux
        .add_right_of(
            cursive::views::Panel::new(BoxView::with_full_screen(TextArea::new())),
            top_right_mid,
        )
        .unwrap();
    let _ = mux
        .add_right_of(BoxView::with_full_screen(TextArea::new()), bottom_right_mid)
        .unwrap();
    let bottom_left_corner = mux
        .add_below(BoxView::with_full_screen(TextArea::new()), top_left_corner)
        .unwrap();
    let top_left_mid = mux
        .add_right_of(BoxView::with_full_screen(TextArea::new()), top_left_corner)
        .unwrap();
    let _ = mux
        .add_right_of(
            cursive::views::Panel::new(BoxView::with_full_screen(TextArea::new())),
            bottom_left_corner,
        )
        .unwrap();

    let idlayer = cursive::views::IdView::new("Steven", mux);

    let boxes = cursive::views::BoxView::new(
        cursive::view::SizeConstraint::Full,
        cursive::view::SizeConstraint::Full,
        idlayer,
    );

    siv.add_fullscreen_layer(boxes);
    siv.add_global_callback('q', Cursive::quit);
    siv.add_global_callback('e', move |ref mut siv| {
        add_plane(siv, top_left_mid);
    });
    cursive::logger::init();
    siv.run();
}

fn add_plane(siv: &mut Cursive, node: Id) {
    let mut foo: cursive::views::ViewRef<Mux> = siv.find_id("Steven").unwrap();
    foo.add_below(cursive::views::TextView::new("Dynamic!".to_string()), node)
        .unwrap();
}
