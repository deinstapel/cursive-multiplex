use cursive::event::{Event, Key};
use cursive::traits::View;
use cursive::views::{NamedView, TextArea};
use cursive::Cursive;
use cursive_multiplex::Mux;

#[test]
fn test_line_vertical() {
    // Vertical test

    let mut siv = Cursive::dummy();

    println!("Vertical Test");
    let mut test_mux = Mux::new();
    let node1 = test_mux
        .add_right_of(TextArea::new(), test_mux.root().build().unwrap())
        .expect("first failed");
    let node2 = test_mux.add_below(TextArea::new(), node1).unwrap();
    let node3 = test_mux.add_below(TextArea::new(), node2).unwrap();

    let id = NamedView::new("mux".to_string(), test_mux);

    siv.add_fullscreen_layer(id);
    siv.run();

    let mut mux: cursive::views::ViewRef<Mux> = siv.find_name("mux").unwrap();
    assert_eq!(node3, mux.focus());
    mux.on_event(Event::Alt(Key::Up));
    assert_eq!(node2, mux.focus());
    mux.on_event(Event::Alt(Key::Down));
    assert_eq!(node3, mux.focus());
    match mux.on_event(Event::Alt(Key::Left)) {
        cursive::event::EventResult::Ignored => {}
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_triangle() {
    let mut mux = Mux::new();
    let node1 = mux
        .add_right_of(TextArea::new(), mux.root().build().unwrap())
        .expect("first failed");
    let mut siv = Cursive::dummy();

    let node2 = mux.add_right_of(TextArea::new(), node1).unwrap();
    let node3 = mux.add_below(TextArea::new(), node2).unwrap();

    let id = NamedView::new("mux".to_string(), mux);
    siv.add_fullscreen_layer(id);
    siv.run();
    let mut mux: cursive::views::ViewRef<Mux> = siv.find_name("mux").unwrap();

    assert_eq!(mux.focus(), node3);
    mux.on_event(Event::Alt(Key::Up));
    assert_eq!(mux.focus(), node2);
    match mux.on_event(Event::Alt(Key::Left)) {
        cursive::event::EventResult::Consumed(_) => {
            assert_eq!(mux.focus(), node1);
        }
        cursive::event::EventResult::Ignored => {
            println!(
                "Not to be ignored Event ignored, Focus was at: {}",
                mux.focus()
            );
            assert!(false);
        }
    }
}

#[test]
fn test_diagonal() {
    let mut mux = Mux::new();
    let node1 = mux
        .add_right_of(TextArea::new(), mux.root().build().unwrap())
        .expect("first failed");
    let mut siv = Cursive::dummy();

    let node2 = mux.add_right_of(TextArea::new(), node1).unwrap();
    let _ = mux.add_below(TextArea::new(), node2).unwrap();
    let upper_right_corner = mux.add_right_of(TextArea::new(), node2).unwrap();
    let bottom_left_corner = mux.add_below(TextArea::new(), node1).unwrap();
    let bottom_left_middle = mux
        .add_right_of(TextArea::new(), bottom_left_corner)
        .unwrap();

    let id = NamedView::new("mux".to_string(), mux);
    siv.add_fullscreen_layer(id);
    siv.run();
    let mut mux: cursive::views::ViewRef<Mux> = siv.find_name("mux").unwrap();

    println!("Moving left...");
    mux.on_event(Event::Alt(Key::Left));
    assert_eq!(mux.focus(), bottom_left_corner);
    println!("Moving right...");
    mux.on_event(Event::Alt(Key::Right));
    assert_eq!(mux.focus(), bottom_left_middle);
    println!("Moving up...");
    mux.on_event(Event::Alt(Key::Up));
    assert_eq!(mux.focus(), node1);
    println!("Moving right...");
    mux.on_event(Event::Alt(Key::Right));
    assert_eq!(mux.focus(), node2);
    println!("Moving right...");
    mux.on_event(Event::Alt(Key::Right));
    assert_eq!(mux.focus(), upper_right_corner);
}

#[test]
fn test_quadratic() {
    // Quadratic test

    let mut siv = Cursive::dummy();
    let mut mux = Mux::new();
    let top_left_corner = mux
        .add_right_of(TextArea::new(), mux.root().build().unwrap())
        .expect("top left corner failed");

    let top_right_mid = mux.add_right_of(TextArea::new(), top_left_corner).unwrap();
    let bottom_right_mid = mux.add_below(TextArea::new(), top_right_mid).unwrap();
    let bottom_right_corner = mux.add_right_of(TextArea::new(), bottom_right_mid).unwrap();
    let bottom_left_corner = mux.add_below(TextArea::new(), top_left_corner).unwrap();
    let top_left_mid = mux.add_right_of(TextArea::new(), top_left_corner).unwrap();
    let bottom_left_mid = mux
        .add_right_of(TextArea::new(), bottom_left_corner)
        .unwrap();
    let top_right_corner = mux.add_right_of(TextArea::new(), top_right_mid).unwrap();

    let id = NamedView::new("mux".to_string(), mux);

    siv.add_fullscreen_layer(id);
    siv.run();

    let mut mux: cursive::views::ViewRef<Mux> = siv.find_name("mux").unwrap();

    println!("Moving left...");
    mux.on_event(Event::Alt(Key::Left));
    println!("Moving left...");
    mux.on_event(Event::Alt(Key::Left));
    assert_eq!(mux.focus(), top_left_mid);
    println!("Moving left...");
    mux.on_event(Event::Alt(Key::Left));
    assert_eq!(mux.focus(), top_left_corner);
    println!("Moving down");
    mux.on_event(Event::Alt(Key::Down));
    assert_eq!(mux.focus(), bottom_left_corner);
    println!("Moving right...");
    mux.on_event(Event::Alt(Key::Right));
    assert_eq!(mux.focus(), bottom_left_mid);
    println!("Moving right...");
    mux.on_event(Event::Alt(Key::Right));
    assert_eq!(mux.focus(), bottom_right_mid);
    println!("Moving right...");
    mux.on_event(Event::Alt(Key::Right));
    assert_eq!(mux.focus(), bottom_right_corner);
    println!("Moving up...");
    mux.on_event(Event::Alt(Key::Up));
    assert_eq!(mux.focus(), top_right_corner);

    println!("Circle completed");
}
