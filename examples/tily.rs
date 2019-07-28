extern crate cursive;

use cursive::Cursive;
use cursive::views::TextArea;
use cursive_multiplex::{Mux, Path, Id};

fn main() {
    let mut siv = Cursive::default();
    siv.show_debug_console();
    println!("Vertical Test");
    let mut mux = Mux::new();

    let node1 = mux.add_vertical_id(TextArea::new(), mux.get_root()).unwrap();
    let node2 = mux.add_horizontal_id(TextArea::new(), node1).unwrap();
    let _ = mux.add_vertical_id(TextArea::new(), node2).unwrap();
    let bottom_left_corner = mux.add_vertical_id(TextArea::new(), node1).unwrap();
    let bottom_left_middle = mux.add_horizontal_id(TextArea::new(), bottom_left_corner).unwrap();
    let upper_right_corner = mux.add_horizontal_id(TextArea::new(), node2).unwrap();

    let idlayer = cursive::views::IdView::new("Steven", mux);

    let boxes = cursive::views::BoxView::new(cursive::view::SizeConstraint::Full, cursive::view::SizeConstraint::Full, idlayer, );

    siv.add_fullscreen_layer(boxes);
    siv.add_global_callback('q', Cursive::quit);
    siv.add_global_callback('e', move |ref mut siv| {
        add_plane(siv, node2);
    });
    cursive::logger::init();
    siv.run();
}

fn add_plane(siv: &mut Cursive, node: Id) {
    let mut foo: cursive::views::ViewRef<Mux> = siv.find_id("Steven").unwrap();
    foo.add_vertical_id(cursive::views::TextView::new("Dynamic!".to_string()), node);
}
