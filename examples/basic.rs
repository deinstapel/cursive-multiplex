extern crate cursive;

use cursive::Cursive;
use cursive::traits::Scrollable;
use cursive_multiplex::{Mux, Path, Id, MuxBuilder};

fn main() {
    let mut siv = Cursive::default();
    // siv.show_debug_console();


    let text = "
Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec congue porttitor pellentesque. Vestibulum a tellus sagittis, blandit erat ac, finibus eros. Praesent cursus at ligula laoreet congue. Proin vehicula diam mattis metus aliquet aliquam. Nullam finibus tellus id dolor porta venenatis. Cras vestibulum leo sit amet congue ultrices. Phasellus convallis ut enim tincidunt interdum.

In velit felis, consectetur quis fringilla id, interdum congue metus. Mauris tincidunt, nibh a fermentum posuere, nibh elit auctor lacus, sollicitudin lobortis nisi arcu quis massa. Ut id augue malesuada justo venenatis pellentesque. Donec egestas nec purus sit amet euismod. Integer aliquet sollicitudin ex id viverra. Vivamus porta odio ac volutpat vehicula. Nullam et nunc in erat imperdiet aliquet vel vel sapien. Nulla viverra porttitor nulla, ut efficitur arcu pharetra sit amet. Nunc aliquet, elit non elementum commodo, augue libero pellentesque lacus, ut iaculis nulla ipsum eu turpis. Ut gravida lacus a nunc dictum maximus. Nulla sollicitudin lobortis malesuada. Praesent fermentum eros ac nisl facilisis, non tincidunt ligula pulvinar.

Cras elementum hendrerit interdum. Proin in diam elit. Maecenas mollis eros id tristique dictum. Nullam euismod scelerisque nibh, et vulputate ipsum consequat vitae. Nunc tempus lacus diam, non fermentum ligula vehicula vel. Nam commodo sodales purus, eu imperdiet orci vulputate eget. Fusce ac quam leo.

Morbi id velit a nisi convallis malesuada eget a lorem. Integer gravida varius varius. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Maecenas pulvinar est diam, sed egestas mauris congue non. Maecenas malesuada augue urna, et efficitur neque pellentesque eu. Donec turpis magna, feugiat non consectetur eget, luctus a metus. Maecenas gravida luctus tempor.

Integer sit amet eleifend ex. Vivamus aliquam eros et massa pellentesque gravida. Nam ullamcorper in urna eget condimentum. Integer tincidunt cursus purus, non egestas erat ultrices a. Pellentesque id leo tristique, tincidunt nunc nec, iaculis nisl. Etiam sit amet ex vitae nunc facilisis auctor. Mauris ultrices lobortis purus, eget venenatis odio. Donec vulputate arcu nunc, quis posuere eros vestibulum non. Nullam aliquam ex ac mi varius, non sodales enim ultricies. Phasellus nec feugiat enim, at vestibulum enim. Nulla fermentum velit sem, ac dapibus nisi lobortis eu. Nulla eget consectetur massa, sed eleifend lorem. Ut convallis erat nec sapien facilisis posuere. Nam sit amet mollis tortor. Donec posuere neque eu risus sodales, vitae maximus erat sagittis. ";

    let (mut mux, node1) = MuxBuilder::new().build(cursive::views::TextView::new(text).scrollable());

    let node2 = mux.add_horizontal_id(cursive::views::TextArea::new(), node1).unwrap();
    let node3 = mux.add_vertical_id(cursive::views::TextArea::new(), node2).unwrap();

    let idlayer = cursive::views::IdView::new("Steven", mux);

    let boxes = cursive::views::BoxView::new(cursive::view::SizeConstraint::Full, cursive::view::SizeConstraint::Full, idlayer, );

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
