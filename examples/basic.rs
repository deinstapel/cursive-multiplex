extern crate cursive;

use cursive::Cursive;
use cursive_multiplex::{Mux, Path, Id};

fn main() {
    let mut siv = Cursive::default();
    siv.show_debug_console();


    let mut mux = Mux::new();

    let node1 = mux.add_horizontal_id(cursive::views::TextView::new("Foo".to_string()), mux.get_root()).unwrap();
    let node2 = mux.add_vertical_id(cursive::views::TextView::new("Bar".to_string()), node1).unwrap();
    let node3 = mux.add_horizontal_id(cursive::views::TextView::new("Fin".to_string()), node2).unwrap();

    let idlayer = cursive::views::IdView::new("Steven", mux);

    let boxes = cursive::views::BoxView::new(cursive::view::SizeConstraint::Full, cursive::view::SizeConstraint::Fixed(20), idlayer, );

    siv.add_fullscreen_layer(boxes);
    siv.add_global_callback('q', Cursive::quit);
    siv.add_global_callback('e', move |ref mut siv| {
        add_plane(siv, node3);
    });
    cursive::logger::init();
    siv.run();
}

fn add_plane(siv: &mut Cursive, node: Id) {
    let mut foo: cursive::views::ViewRef<Mux> = siv.find_id("Steven").unwrap();
    foo.add_vertical_id(cursive::views::TextView::new("Dynamic!".to_string()), node);
}
