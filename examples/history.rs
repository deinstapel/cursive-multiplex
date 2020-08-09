use cursive::Cursive;
use cursive_multiplex::Mux;

fn main() {
    let mut siv = cursive::default();
    // siv.show_debug_console();
    let mut mux = Mux::new();
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
    let mut linear = cursive_core::views::LinearLayout::new(cursive_core::direction::Orientation::Vertical);

    linear.add_child(idlayer);
    linear.add_child(menubar);
    siv.add_fullscreen_layer(linear);
    siv.add_global_callback('q', Cursive::quit);
    siv.add_global_callback(
        cursive_core::event::Event::Alt(cursive_core::event::Key::Ins),
        move |ref mut siv| {
            add_pane(siv);
        },
    );
    siv.add_global_callback(
        cursive_core::event::Event::Alt(cursive_core::event::Key::Del),
        move |ref mut siv| {
            remove_pane(siv);
        },
    );
    cursive_core::logger::init();
    siv.run();
}

fn add_pane(siv: &mut Cursive) {
    let mut mux: cursive_core::views::ViewRef<Mux> = siv.find_name("Mux").unwrap();
    let surprise = "⢀⣠⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⣠⣤⣶⣶
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⢰⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣧⣀⣀⣾⣿⣿⣿⣿
⣿⣿⣿⣿⣿⡏⠉⠛⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⣿
⣿⣿⣿⣿⣿⣿⠀⠀⠀⠈⠛⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠿⠛⠉⠁⠀⣿
⣿⣿⣿⣿⣿⣿⣧⡀⠀⠀⠀⠀⠙⠿⠿⠿⠻⠿⠿⠟⠿⠛⠉⠀⠀⠀⠀⠀⣸⣿
⣿⣿⣿⣿⣿⣿⣿⣷⣄⠀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⠏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠠⣴⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⡟⠀⠀⢰⣹⡆⠀⠀⠀⠀⠀⠀⣭⣷⠀⠀⠀⠸⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⠃⠀⠀⠈⠉⠀⠀⠤⠄⠀⠀⠀⠉⠁⠀⠀⠀⠀⢿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⢾⣿⣷⠀⠀⠀⠀⡠⠤⢄⠀⠀⠀⠠⣿⣿⣷⠀⢸⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⡀⠉⠀⠀⠀⠀⠀⢄⠀⢀⠀⠀⠀⠀⠉⠉⠁⠀⠀⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣧⠀⠀⠀⠀⠀⠀⠀⠈⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢹⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿
";
    let id = mux.focus();
    mux.add_below(cursive_core::views::TextView::new(surprise), id)
        .unwrap();
}

fn remove_pane(siv: &mut Cursive) {
    let mut mux: cursive_core::views::ViewRef<Mux> = siv.find_name("Mux").unwrap();
    let id = mux.focus();
    mux.remove_id(id).unwrap();
}
