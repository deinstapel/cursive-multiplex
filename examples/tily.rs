use cursive_core::views::{ResizedView, TextArea, TextView};
use cursive_core::Cursive;
use cursive_multiplex::{Id, Mux};

fn main() {
    let mut siv = cursive::default();
    siv.show_debug_console();
    let mut mux = Mux::new();
    let top_left_corner = mux
        .add_right_of(
            ResizedView::with_full_screen(TextArea::new()),
            mux.root().build().unwrap(),
        )
        .expect("first failed");

    let top_right_mid = mux
        .add_right_of(
            ResizedView::with_full_screen(TextArea::new()),
            top_left_corner,
        )
        .unwrap();
    let bottom_right_mid = mux
        .add_below(
            ResizedView::with_full_screen(TextView::new("I will not be focused!")),
            top_right_mid,
        )
        .unwrap();
    let _ = mux
        .add_right_of(
            cursive_core::views::Panel::new(ResizedView::with_full_screen(TextArea::new())),
            top_right_mid,
        )
        .unwrap();
    let _ = mux
        .add_right_of(
            ResizedView::with_full_screen(TextArea::new()),
            bottom_right_mid,
        )
        .unwrap();
    let bottom_left_corner = mux
        .add_below(
            ResizedView::with_full_screen(TextArea::new()),
            top_left_corner,
        )
        .unwrap();
    let top_left_mid = mux
        .add_right_of(
            ResizedView::with_full_screen(TextArea::new()),
            top_left_corner,
        )
        .unwrap();
    let _ = mux
        .add_right_of(
            cursive_core::views::Panel::new(ResizedView::with_full_screen(TextArea::new())),
            bottom_left_corner,
        )
        .unwrap();

    let idlayer = cursive_core::views::NamedView::new("Steven", mux);

    let boxes = cursive_core::views::ResizedView::new(
        cursive_core::view::SizeConstraint::Full,
        cursive_core::view::SizeConstraint::Full,
        idlayer,
    );

    siv.add_fullscreen_layer(boxes);
    siv.add_global_callback('q', Cursive::quit);
    siv.add_global_callback('e', move |ref mut siv| {
        add_plane(siv, top_left_mid);
    });
    cursive_core::logger::init();
    siv.run();
}

fn add_plane(siv: &mut Cursive, node: Id) {
    let mut foo: cursive_core::views::ViewRef<Mux> = siv.find_name("Steven").unwrap();
    foo.add_below(
        cursive_core::views::TextView::new("Dynamic!".to_string()),
        node,
    )
    .unwrap();
}
