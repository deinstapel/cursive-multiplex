use cursive::Cursive;
use cursive_multiplex::Mux;

fn main() {
    let mut siv = cursive::default();
    // siv.show_debug_console();
    let mut mux = Mux::new().with_default_split_ratio(0.7);
    let node1 = mux
        .add_right_of(
            cursive_core::views::ResizedView::with_full_screen(cursive_core::views::TextArea::new()),
            mux.root().build().unwrap(),
        )
        .expect("text view failed");

    let mut menubar = cursive_core::views::Menubar::new();
    menubar.add_leaf("Hello from cursive_multiplex", |_| {});
    menubar.add_leaf("Feel free to try out the examples simply with `cargo run --example=basic` or `cargo run --example=tily`", |_|{});

    let node2 = mux
        .add_right_of(
            cursive_core::views::ResizedView::with_full_screen(cursive_core::views::TextArea::new()),
            node1,
        )
        .unwrap();
    let _ = mux
        .add_below(
            cursive_core::views::ResizedView::with_full_screen(cursive_core::views::TextArea::new()),
            node2,
        )
        .unwrap();

    let idlayer = cursive_core::views::NamedView::new("Mux", mux);
    let mut linear =
        cursive_core::views::LinearLayout::new(cursive_core::direction::Orientation::Vertical);

    linear.add_child(idlayer);
    linear.add_child(menubar);
    siv.add_fullscreen_layer(linear);
    siv.add_global_callback('q', Cursive::quit);
    cursive_core::logger::init();
    siv.run();
}
