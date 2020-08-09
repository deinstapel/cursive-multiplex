use cursive::event::Event;
use cursive::traits::View;
use cursive::views::{TextView};
use cursive_multiplex::Mux;

fn main() {
    let mut siv = cursive::default();
    let mut mux = Mux::new().with_zoom(Event::CtrlChar('x'));
    let root = mux
        .add_right_of(
            TextView::new("Center".to_string()),
            mux.root().build().unwrap(),
        )
        .expect("Center failed");
    let _id = mux
        .add_below(TextView::new("Down"), root)
        .expect("Down failed");
    let id = mux.add_above(TextView::new("Ups asd
    asd 
    asd asd
    as a
    s da
    s a
    sdasdasdasdasdfasfgarhbah
    ga
    fa
    sdf
    asf




    a
    sdfa
    sdf
    ad
    fas
    f



    asdf

    a
    as
    DAS
    D"), root).expect("Up failed");
    mux.set_focus(id);
    mux.on_event(Event::CtrlChar('x'));
    siv.add_fullscreen_layer(mux);
    siv.run();
}
