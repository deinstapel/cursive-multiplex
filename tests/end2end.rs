use crossbeam::channel::{Receiver, Sender};
use cursive::backends::puppet::observed::ObservedScreen;
use cursive::backends::puppet::Backend;
use cursive::event::{Event, Key};
use cursive::views::{Panel, ResizedView, TextArea, TextView};
use cursive::Vec2;
use cursive_multiplex::Mux;
use insta::assert_display_snapshot;

fn setup_test_environment<F>(cb: F) -> (Receiver<ObservedScreen>, Sender<Option<Event>>)
where
    F: FnOnce(&mut cursive::Cursive),
{
    let backend = Backend::init(Some(Vec2::new(80, 24)));
    let frames = backend.stream();
    let input = backend.input();
    let mut siv = cursive::Cursive::new(|| backend);
    cb(&mut siv);
    input
        .send(Some(Event::Refresh))
        .expect("Refresh not accepted, backend not valid");
    siv.step();
    (frames, input)
}

struct TestCursive {
    siv: cursive::Cursive,
    frames: Receiver<ObservedScreen>,
    input: Sender<Option<Event>>,
}

impl TestCursive {
    fn new<F>(cb: F) -> Self
    where
        F: FnOnce(&mut cursive::Cursive),
    {
        let backend = Backend::init(Some(Vec2::new(80, 24)));
        let frames = backend.stream();
        let input = backend.input();
        let mut siv = cursive::Cursive::new(|| backend);
        cb(&mut siv);
        input
            .send(Some(Event::Refresh))
            .expect("Refresh not accepted, backend not valid");
        siv.step();
        Self { siv, frames, input }
    }
    fn _call_on<F>(&mut self, cb: F)
    where
        F: FnOnce(&mut cursive::Cursive),
    {
        cb(&mut self.siv);
    }

    fn input(&mut self, event: Event) {
        self.input
            .send(Some(event))
            .expect("Refresh not accepted, backend could not react");
        self.step();
    }

    fn step(&mut self) {
        self.input
            .send(Some(Event::Refresh))
            .expect("Refresh not accepted, backend could not react");
        self.siv.step();
    }

    fn last_screen(&mut self) -> ObservedScreen {
        self.frames.try_iter().last().unwrap()
    }
}

#[test]
fn end2end_complex() {
    let (frames, _) = setup_test_environment(|siv: &mut cursive::Cursive| {
        let mut mux = Mux::new();
        let left1 = mux
            .add_right_of(
                TextView::new("left 1".to_string()),
                mux.root().build().unwrap(),
            )
            .expect("left 1 failed");
        let right1 = mux
            .add_right_of(TextView::new("right 1"), left1)
            .expect("right 1 failed");
        let _right3 = mux
            .add_below(TextView::new("right 3"), right1)
            .expect("right 3 failed");
        let _right2 = mux
            .add_right_of(TextView::new("right 2"), right1)
            .expect("right 2 failed");

        let _left2 = mux
            .add_below(TextView::new("left 2"), left1)
            .expect("left 2 failed");
        let _left3 = mux
            .add_below(TextView::new("left 3"), left1)
            .expect("left 3 failed");

        siv.add_fullscreen_layer(mux);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap());
}

#[test]
fn end2end_complex_focus() {
    let (frames, _) = setup_test_environment(|siv: &mut cursive::Cursive| {
        let mut mux = Mux::new();
        let left1 = mux
            .add_right_of(TextArea::new(), mux.root().build().unwrap())
            .expect("left 1 failed");

        let right1 = mux
            .add_right_of(TextArea::new(), left1)
            .expect("right 1 failed");
        let _right3 = mux
            .add_below(TextArea::new(), right1)
            .expect("right 3 failed");
        let right2 = mux
            .add_right_of(TextArea::new(), right1)
            .expect("right 2 failed");

        let _left2 = mux
            .add_below(TextArea::new(), left1)
            .expect("left 2 failed");
        let _left3 = mux
            .add_below(TextArea::new(), left1)
            .expect("left 3 failed");
        mux.set_focus(right2);

        siv.add_fullscreen_layer(mux);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap());
}

#[test]
fn end2end_complex_remove() {
    let (frames, _) = setup_test_environment(|siv: &mut cursive::Cursive| {
        let mut mux = Mux::new();
        let left1 = mux
            .add_right_of(TextView::new("left 1"), mux.root().build().unwrap())
            .unwrap();

        let right1 = mux
            .add_right_of(TextView::new("right 1"), left1)
            .expect("right 1 failed");
        let right3 = mux
            .add_below(TextView::new("right 3"), right1)
            .expect("right 3 failed");
        let _right2 = mux
            .add_right_of(TextView::new("right 2"), right1)
            .expect("right 2 failed");

        let _left2 = mux
            .add_below(TextView::new("left 2"), left1)
            .expect("left 2 failed");
        let _left3 = mux
            .add_below(TextView::new("left 3"), left1)
            .expect("left 3 failed");
        mux.remove_id(right3).expect("remove failed");

        siv.add_fullscreen_layer(mux);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap());
}

#[test]
fn end2end_complex_resize() {
    let mut tsiv = TestCursive::new(|siv: &mut cursive::Cursive| {
        let mut mux = Mux::new();
        let left1 = mux
            .add_right_of(TextArea::new(), mux.root().build().unwrap())
            .expect("left 1 failed");

        let right1 = mux
            .add_right_of(TextArea::new(), left1)
            .expect("right 1 failed");
        let _right3 = mux
            .add_below(TextArea::new(), right1)
            .expect("right 3 failed");
        let _right2 = mux
            .add_right_of(TextArea::new(), right1)
            .expect("right 2 failed");

        let _left2 = mux
            .add_below(TextArea::new(), left1)
            .expect("left 2 failed");
        let _left3 = mux
            .add_below(TextArea::new(), left1)
            .expect("left 3 failed");
        mux.set_focus(right1);

        siv.add_fullscreen_layer(mux);
    });
    tsiv.input(Event::Ctrl(Key::Down));
    tsiv.input(Event::Ctrl(Key::Right));
    tsiv.input(Event::Alt(Key::Left));
    tsiv.input(Event::Alt(Key::Down));
    tsiv.input(Event::Ctrl(Key::Up));
    tsiv.input(Event::Ctrl(Key::Left));
    assert_display_snapshot!(tsiv.last_screen());
}

#[test]
fn end2end_complex_switch() {
    let (frames, _) = setup_test_environment(|siv: &mut cursive::Cursive| {
        let mut mux = Mux::new();
        let left1 = mux
            .add_right_of(TextView::new("left 1"), mux.root().build().unwrap())
            .expect("left 1 failed");

        let right1 = mux
            .add_right_of(TextView::new("right 1"), left1)
            .expect("right 1 failed");
        let _right3 = mux
            .add_below(TextView::new("right 3"), right1)
            .expect("right 3 failed");
        let _right2 = mux
            .add_right_of(TextView::new("right 2"), right1)
            .expect("right 2 failed");

        let _left2 = mux
            .add_below(TextView::new("left 2"), left1)
            .expect("left 2 failed");
        let left3 = mux
            .add_below(TextView::new("left 3"), left1)
            .expect("left 3 failed");
        mux.switch_views(right1, left3).expect("switch failed");
        siv.add_fullscreen_layer(mux);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap());
}

#[test]
fn end2end_down_focus() {
    let mut tsiv = TestCursive::new(|siv: &mut cursive::Cursive| {
        let mut mux = Mux::new();
        let upper = mux
            .add_right_of(TextArea::new(), mux.root().build().unwrap())
            .expect("upper failed");
        let _lower = mux.add_below(TextArea::new(), upper).expect("lower failed");
        let _id = mux.add_below(TextArea::new(), upper).expect("1st failed");
        mux.set_focus(upper);
        siv.add_fullscreen_layer(mux);
    });
    tsiv.input(Event::Alt(Key::Down));
    assert_display_snapshot!("down once", tsiv.last_screen());
    tsiv.input(Event::Alt(Key::Down));
    assert_display_snapshot!("down twice", tsiv.last_screen());
    tsiv.input(Event::Alt(Key::Down));
    assert_display_snapshot!("down thrice", tsiv.last_screen());
}

#[test]
fn end2end_horizontal() {
    let (frames, _) = setup_test_environment(|siv: &mut cursive::Cursive| {
        let mut mux = Mux::new();
        let root = mux
            .add_right_of(TextView::new("Center"), mux.root().build().unwrap())
            .expect("Center failed");
        let _id = mux
            .add_right_of(TextView::new("Right"), root)
            .expect("Right failed");
        let _id = mux
            .add_left_of(TextView::new("Left"), root)
            .expect("Left failed");
        siv.add_fullscreen_layer(mux);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap());
}

#[test]
fn end2end_horizontal_fixed_size() {
    let (frames, _) = setup_test_environment(|siv: &mut cursive::Cursive| {
        let mut mux = Mux::new();
        let root = mux
            .add_right_of(TextView::new("Root"), mux.root().build().unwrap())
            .expect("Center failed");
        let view = ResizedView::with_fixed_size((42, 11), Panel::new(TextView::new("Fixed")));
        let _id = mux.add_right_of(view, root).expect("Fixed failed");

        siv.add_fullscreen_layer(mux);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap());
}

#[test]
fn end2end_horizontal_remove() {
    let (frames, _) = setup_test_environment(|siv: &mut cursive::Cursive| {
        let mut mux = Mux::new();
        let root = mux
            .add_right_of(TextView::new("Center"), mux.root().build().unwrap())
            .expect("Center failed");
        let _id = mux
            .add_left_of(TextView::new("Left"), root)
            .expect("Left failed");
        let _id = mux
            .add_right_of(TextView::new("Right"), root)
            .expect("Right failed");
        mux.remove_id(root).expect("remove failed");
        siv.add_fullscreen_layer(mux);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap());
}

#[test]
fn end2end_down_resize() {
    let mut tsiv = TestCursive::new(|siv: &mut cursive::Cursive| {
        let mut mux = Mux::new();
        let left = mux
            .add_right_of(TextArea::new(), mux.root().build().unwrap())
            .expect("left failed");
        let center = mux
            .add_right_of(TextArea::new(), left)
            .expect("Center failed");
        let _right = mux
            .add_right_of(TextArea::new(), center)
            .expect("Right failed");
        mux.set_focus(left);
        siv.add_fullscreen_layer(mux);
    });
    tsiv.input(Event::Ctrl(Key::Left));
    tsiv.input(Event::Ctrl(Key::Left));
    tsiv.input(Event::Ctrl(Key::Right));
    assert_display_snapshot!(tsiv.last_screen());
}

#[test]
fn end2end_horizontal_switch() {
    let (frames, _) = setup_test_environment(|siv: &mut cursive::Cursive| {
        let mut mux = Mux::new();
        let right = mux
            .add_right_of(
                TextView::new("Right".to_string()),
                mux.root().build().unwrap(),
            )
            .expect("right failed");
        let left = mux
            .add_right_of(TextView::new("Left"), right)
            .expect("Left failed");
        mux.switch_views(right, left).expect("switch failed");
        siv.add_fullscreen_layer(mux);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap());
}

#[test]
fn end2end_left_focus() {
    let mut tsiv = TestCursive::new(|siv: &mut cursive::Cursive| {
        let mut mux = Mux::new();
        let left = mux
            .add_right_of(TextArea::new(), mux.root().build().unwrap())
            .expect("left failed");
        let right = mux
            .add_right_of(TextArea::new(), left)
            .expect("right failed");
        let _id = mux.add_right_of(TextArea::new(), left).expect("1st failed");
        mux.set_focus(right);
        siv.add_fullscreen_layer(mux);
    });
    tsiv.input(Event::Alt(Key::Left));
    assert_display_snapshot!("left once", tsiv.last_screen());
    tsiv.input(Event::Alt(Key::Left));
    assert_display_snapshot!("left twice", tsiv.last_screen());
    tsiv.input(Event::Alt(Key::Left));
    assert_display_snapshot!("left thrice", tsiv.last_screen());
}

#[test]
fn end2end_right_focus() {
    let mut tsiv = TestCursive::new(|siv: &mut cursive::Cursive| {
        let mut mux = Mux::new();
        let left = mux
            .add_right_of(TextArea::new(), mux.root().build().unwrap())
            .expect("left failed");
        let _right = mux
            .add_right_of(TextArea::new(), left)
            .expect("right failed");
        let _id = mux.add_right_of(TextArea::new(), left).expect("1st failed");
        mux.set_focus(left);
        siv.add_fullscreen_layer(mux);
    });
    tsiv.input(Event::Alt(Key::Right));
    assert_display_snapshot!("right once", tsiv.last_screen());
    tsiv.input(Event::Alt(Key::Right));
    assert_display_snapshot!("right twice", tsiv.last_screen());
    tsiv.input(Event::Alt(Key::Right));
    assert_display_snapshot!("right thrice", tsiv.last_screen());
}

#[test]
fn end2end_smoke() {
    let (frames, _) = setup_test_environment(|siv: &mut cursive::Cursive| {
        let mut mux = Mux::new();
        let _node1 = mux
            .add_right_of(
                TextView::new("Hello World".to_string()),
                mux.root().build().unwrap(),
            )
            .expect("hello failed");
        let boxview = ResizedView::with_fixed_size((42, 11), mux);
        siv.add_layer(boxview);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap());
}

#[test]
fn end2end_up_focus() {
    let mut tsiv = TestCursive::new(|siv: &mut cursive::Cursive| {
        let mut mux = Mux::new();
        let upper = mux
            .add_right_of(TextArea::new(), mux.root().build().unwrap())
            .expect("upper failed");
        let lower = mux.add_below(TextArea::new(), upper).expect("lower failed");
        let _id = mux.add_below(TextArea::new(), upper).expect("1st failed");
        mux.set_focus(lower);

        siv.add_fullscreen_layer(mux);
    });
    tsiv.input(Event::Alt(Key::Up));
    assert_display_snapshot!("up once", tsiv.last_screen());
    tsiv.input(Event::Alt(Key::Up));
    assert_display_snapshot!("up twice", tsiv.last_screen());
    tsiv.input(Event::Alt(Key::Up));
    assert_display_snapshot!("up thrice", tsiv.last_screen());
}

#[test]
fn end2end_zoom() {
    let mut tsiv = TestCursive::new(|siv: &mut cursive::Cursive| {
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
        let id = mux
            .add_above(
                TextView::new(
                    "Ups asd
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
    D",
                ),
                root,
            )
            .expect("Up failed");
        mux.set_focus(id);
        siv.add_fullscreen_layer(mux);
    });
    tsiv.input(Event::CtrlChar('x'));
    assert_display_snapshot!(tsiv.last_screen());
}

#[test]
fn end2end_vertical() {
    let (frames, _) = setup_test_environment(|siv: &mut cursive::Cursive| {
        let mut mux = Mux::new();
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
        siv.add_fullscreen_layer(mux);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap());
}

#[test]
fn end2end_vertical_fixed_size() {
    let (frames, _) = setup_test_environment(|siv: &mut cursive::Cursive| {
        let mut mux = Mux::new();
        let root = mux
            .add_right_of(TextView::new("Root"), mux.root().build().unwrap())
            .expect("Root failed");
        let view = ResizedView::with_fixed_size((42, 11), Panel::new(TextView::new("Fixed")));
        let _id = mux.add_below(view, root).expect("Fixed failed");

        siv.add_fullscreen_layer(mux);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap());
}

#[test]
fn end2end_vertical_remove() {
    let (frames, _) = setup_test_environment(|siv: &mut cursive::Cursive| {
        let mut mux = Mux::new();
        let root = mux
            .add_right_of(TextView::new("Center"), mux.root().build().unwrap())
            .expect("Center failed");
        let _id = mux
            .add_above(TextView::new("Up"), root)
            .expect("Upper failed");
        let _id = mux
            .add_below(TextView::new("Down"), root)
            .expect("Lower failed");
        mux.remove_id(root).expect("remove failed");
        siv.add_fullscreen_layer(mux);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap());
}

#[test]
fn end2end_vertical_resize() {
    let mut tsiv = TestCursive::new(|siv: &mut cursive::Cursive| {
        let mut mux = Mux::new();
        let up = mux
            .add_right_of(TextArea::new(), mux.root().build().unwrap())
            .expect("Up failed");
        let center = mux.add_below(TextArea::new(), up).expect("Center failed");
        let _down = mux.add_below(TextArea::new(), center).expect("Down failed");
        mux.set_focus(up);
        siv.add_fullscreen_layer(mux);
    });
    tsiv.input(Event::Ctrl(Key::Up));
    tsiv.input(Event::Ctrl(Key::Up));
    tsiv.input(Event::Ctrl(Key::Down));
    assert_display_snapshot!(tsiv.last_screen());
}

#[test]
fn end2end_switch_views() {
    let (frames, _) = setup_test_environment(|siv: &mut cursive::Cursive| {
        let mut mux = Mux::new();
        let up = mux
            .add_right_of(TextView::new("Up"), mux.root().build().unwrap())
            .expect("Up failed");
        let down = mux
            .add_above(TextView::new("Down"), up)
            .expect("Left failed");
        mux.switch_views(up, down).expect("switch failed");
        siv.add_fullscreen_layer(mux);
    });
    assert_display_snapshot!(frames.try_iter().last().unwrap());
}

#[test]
fn end2end_up_down_focus_history() {
    let mut tsiv = TestCursive::new(|siv: &mut cursive::Cursive| {
        let mut mux = Mux::new();
        let up = mux
            .add_right_of(TextArea::new(), mux.root().build().unwrap())
            .expect("Up failed");
        let down_left = mux.add_below(TextArea::new(), up).expect("Left failed");
        let _down_right = mux
            .add_right_of(TextArea::new(), down_left)
            .expect("Right Failed");
        siv.add_fullscreen_layer(mux);
    });
    tsiv.input(Event::Alt(Key::Up));
    tsiv.input(Event::Alt(Key::Down));
    assert_display_snapshot!(tsiv.last_screen());
}

#[test]
fn end2end_left_right_focus_history() {
    let mut tsiv = TestCursive::new(|siv: &mut cursive::Cursive| {
        let mut mux = Mux::new();
        let up = mux
            .add_right_of(TextArea::new(), mux.root().build().unwrap())
            .expect("Up failed");
        let up_right = mux.add_right_of(TextArea::new(), up).expect("Left failed");
        let _down_right = mux
            .add_below(TextArea::new(), up_right)
            .expect("Right Failed");
        siv.add_fullscreen_layer(mux);
    });
    tsiv.input(Event::Alt(Key::Left));
    tsiv.input(Event::Alt(Key::Right));
    assert_display_snapshot!(tsiv.last_screen());
}
