extern crate cursive;

use cursive::event::Event;
use cursive::traits::View;
use cursive::views::TextView;
use cursive::Cursive;
use cursive_multiplex::Mux;

fn main() {
    let mut siv = Cursive::default();
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
    let _id = mux.add_above(TextView::new("Up"), root).expect("Up failed");
    mux.on_event(Event::CtrlChar('x'));
    siv.add_fullscreen_layer(mux);
    siv.run();
}
