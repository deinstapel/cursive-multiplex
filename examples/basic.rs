use cursive::{
    views::{LinearLayout, Menubar, NamedView, ResizedView, ScrollView, TextArea},
    Cursive,
};
use cursive_core::{direction::Orientation, views::TextView};
use cursive_multiplex::Mux;

fn main() {
    let mut siv = cursive::default();
    // siv.show_debug_console();

    let text = "
Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec congue porttitor pellentesque. Vestibulum a tellus sagittis, blandit erat ac, finibus eros. Praesent cursus at ligula laoreet congue. Proin vehicula diam mattis metus aliquet aliquam. Nullam finibus tellus id dolor porta venenatis. Cras vestibulum leo sit amet congue ultrices. Phasellus convallis ut enim tincidunt interdum.

In velit felis, consectetur quis fringilla id, interdum congue metus. Mauris tincidunt, nibh a fermentum posuere, nibh elit auctor lacus, sollicitudin lobortis nisi arcu quis massa. Ut id augue malesuada justo venenatis pellentesque. Donec egestas nec purus sit amet euismod. Integer aliquet sollicitudin ex id viverra. Vivamus porta odio ac volutpat vehicula. Nullam et nunc in erat imperdiet aliquet vel vel sapien. Nulla viverra porttitor nulla, ut efficitur arcu pharetra sit amet. Nunc aliquet, elit non elementum commodo, augue libero pellentesque lacus, ut iaculis nulla ipsum eu turpis. Ut gravida lacus a nunc dictum maximus. Nulla sollicitudin lobortis malesuada. Praesent fermentum eros ac nisl facilisis, non tincidunt ligula pulvinar.

Cras elementum hendrerit interdum. Proin in diam elit. Maecenas mollis eros id tristique dictum. Nullam euismod scelerisque nibh, et vulputate ipsum consequat vitae. Nunc tempus lacus diam, non fermentum ligula vehicula vel. Nam commodo sodales purus, eu imperdiet orci vulputate eget. Fusce ac quam leo.

Morbi id velit a nisi convallis malesuada eget a lorem. Integer gravida varius varius. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Maecenas pulvinar est diam, sed egestas mauris congue non. Maecenas malesuada augue urna, et efficitur neque pellentesque eu. Donec turpis magna, feugiat non consectetur eget, luctus a metus. Maecenas gravida luctus tempor.

Integer sit amet eleifend ex. Vivamus aliquam eros et massa pellentesque gravida. Nam ullamcorper in urna eget condimentum. Integer tincidunt cursus purus, non egestas erat ultrices a. Pellentesque id leo tristique, tincidunt nunc nec, iaculis nisl. Etiam sit amet ex vitae nunc facilisis auctor. Mauris ultrices lobortis purus, eget venenatis odio. Donec vulputate arcu nunc, quis posuere eros vestibulum non. Nullam aliquam ex ac mi varius, non sodales enim ultricies. Phasellus nec feugiat enim, at vestibulum enim. Nulla fermentum velit sem, ac dapibus nisi lobortis eu. Nulla eget consectetur massa, sed eleifend lorem. Ut convallis erat nec sapien facilisis posuere. Nam sit amet mollis tortor. Donec posuere neque eu risus sodales, vitae maximus erat sagittis. ";

    let mut mux = Mux::new();
    let node1 = mux
        .add_right_of(
            ScrollView::new(TextView::new(text)),
            mux.root().build().unwrap(),
        )
        .expect("text view failed");

    let mut menubar = Menubar::new();
    menubar.add_leaf("Hello from cursive_multiplex", |_| {});
    menubar.add_leaf("Feel free to try out the examples simply with `cargo run --example=basic` or `cargo run --example=tily`", |_|{});

    let node2 = mux
        .add_right_of(ResizedView::with_full_screen(TextArea::new()), node1)
        .unwrap();
    if let Some(textview) = mux.active_view_mut() {
        let valid_view = textview
            .downcast_mut::<ResizedView<TextArea>>()
            .unwrap()
            .get_inner_mut();
        valid_view.set_content(
            "This text is added by later modification! Check out the `basic` example to see how.",
        );
    }
    let _ = mux
        .add_below(ResizedView::with_full_screen(TextArea::new()), node2)
        .unwrap();

    mux.set_container_split_ratio(node2, 0.7).unwrap();

    let idlayer = NamedView::new("Mux", mux);
    let mut linear = LinearLayout::new(Orientation::Vertical);

    linear.add_child(idlayer);
    linear.add_child(menubar);
    siv.add_fullscreen_layer(linear);
    siv.add_global_callback('q', Cursive::quit);
    siv.add_global_callback(
        cursive::event::Event::Alt(cursive_core::event::Key::Ins),
        move |ref mut siv| {
            add_pane(siv);
        },
    );
    siv.add_global_callback(
        cursive::event::Event::Alt(cursive_core::event::Key::Del),
        move |ref mut siv| {
            remove_pane(siv);
        },
    );
    cursive::logger::init();
    siv.run();
}

fn add_pane(siv: &mut Cursive) {
    let mut mux: cursive::views::ViewRef<Mux> = siv.find_name("Mux").unwrap();
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
    mux.add_below(cursive::views::TextView::new(surprise), id)
        .unwrap();
}

fn remove_pane(siv: &mut Cursive) {
    let mut mux: cursive::views::ViewRef<Mux> = siv.find_name("Mux").unwrap();
    let id = mux.focus();
    mux.remove_id(id).unwrap();
}
