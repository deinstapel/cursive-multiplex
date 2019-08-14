use cursive::event::{Event, Key};
use cursive::traits::View;
use cursive::views::{IdView, TextArea};
use cursive::Cursive;
use cursive_multiplex::Mux;

#[test]
fn test_line_vertical() {
    // Vertical test

    let mut siv = Cursive::dummy();

    println!("Vertical Test");
    let (mut test_mux, node1) = Mux::new(TextArea::new());
    let node2 = test_mux.add_vertical_id(TextArea::new(), node1).unwrap();
    let node3 = test_mux.add_vertical_id(TextArea::new(), node2).unwrap();

    let id = IdView::new("mux".to_string(), test_mux);

    siv.add_fullscreen_layer(id);
    siv.run();

    let mut mux: cursive::views::ViewRef<Mux> = siv.find_id("mux").unwrap();
    assert_eq!(node3, mux.get_focus());
    mux.on_event(Event::Key(Key::Up));
    assert_eq!(node2, mux.get_focus());
    mux.on_event(Event::Key(Key::Down));
    assert_eq!(node3, mux.get_focus());
    match mux.on_event(Event::Key(Key::Left)) {
        cursive::event::EventResult::Ignored => {}
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_triangle() {
    let (mut mux, node1) = Mux::new(TextArea::new());
    let mut siv = Cursive::dummy();

    let node2 = mux.add_horizontal_id(TextArea::new(), node1).unwrap();
    let node3 = mux.add_vertical_id(TextArea::new(), node2).unwrap();

    let id = IdView::new("mux".to_string(), mux);
    siv.add_fullscreen_layer(id);
    siv.run();
    let mut mux: cursive::views::ViewRef<Mux> = siv.find_id("mux").unwrap();

    assert_eq!(mux.get_focus(), node3);
    mux.on_event(Event::Key(Key::Up));
    assert_eq!(mux.get_focus(), node2);
    match mux.on_event(Event::Key(Key::Left)) {
        cursive::event::EventResult::Consumed(_) => {
            assert_eq!(mux.get_focus(), node1);
        }
        cursive::event::EventResult::Ignored => {
            println!(
                "Not to be ignored Event ignored, Focus was at: {}",
                mux.get_focus()
            );
            assert!(false);
        }
    }
}

#[test]
fn test_diagonal() {
    let (mut mux, node1) = Mux::new(TextArea::new());
    let mut siv = Cursive::dummy();

    let node2 = mux.add_horizontal_id(TextArea::new(), node1).unwrap();
    let _ = mux.add_vertical_id(TextArea::new(), node2).unwrap();
    let upper_right_corner = mux.add_horizontal_id(TextArea::new(), node2).unwrap();
    let bottom_left_corner = mux.add_vertical_id(TextArea::new(), node1).unwrap();
    let bottom_left_middle = mux
        .add_horizontal_id(TextArea::new(), bottom_left_corner)
        .unwrap();

    let id = IdView::new("mux".to_string(), mux);
    siv.add_fullscreen_layer(id);
    siv.run();
    let mut mux: cursive::views::ViewRef<Mux> = siv.find_id("mux").unwrap();

    println!("Moving left...");
    mux.on_event(Event::Key(Key::Left));
    assert_eq!(mux.get_focus(), bottom_left_corner);
    println!("Moving right...");
    mux.on_event(Event::Key(Key::Right));
    assert_eq!(mux.get_focus(), bottom_left_middle);
    println!("Moving up...");
    mux.on_event(Event::Key(Key::Up));
    assert_eq!(mux.get_focus(), node1);
    println!("Moving right...");
    mux.on_event(Event::Key(Key::Right));
    assert_eq!(mux.get_focus(), node2);
    println!("Moving right...");
    mux.on_event(Event::Key(Key::Right));
    assert_eq!(mux.get_focus(), upper_right_corner);
}

#[test]
fn test_quadratic() {
    // Quadratic test

    let mut siv = Cursive::dummy();
    let (mut mux, top_left_corner) = Mux::new(TextArea::new());

    let top_right_mid = mux
        .add_horizontal_id(TextArea::new(), top_left_corner)
        .unwrap();
    let bottom_right_mid = mux.add_vertical_id(TextArea::new(), top_right_mid).unwrap();
    let bottom_right_corner = mux
        .add_horizontal_id(TextArea::new(), bottom_right_mid)
        .unwrap();
    let bottom_left_corner = mux
        .add_vertical_id(TextArea::new(), top_left_corner)
        .unwrap();
    let top_left_mid = mux
        .add_horizontal_id(TextArea::new(), top_left_corner)
        .unwrap();
    let bottom_left_mid = mux
        .add_horizontal_id(TextArea::new(), bottom_left_corner)
        .unwrap();
    let top_right_corner = mux
        .add_horizontal_id(TextArea::new(), top_right_mid)
        .unwrap();

    let id = IdView::new("mux".to_string(), mux);

    siv.add_fullscreen_layer(id);
    siv.run();

    let mut mux: cursive::views::ViewRef<Mux> = siv.find_id("mux").unwrap();

    println!("Moving left...");
    mux.on_event(Event::Key(Key::Left));
    println!("Moving left...");
    mux.on_event(Event::Key(Key::Left));
    assert_eq!(mux.get_focus(), top_left_mid);
    println!("Moving left...");
    mux.on_event(Event::Key(Key::Left));
    assert_eq!(mux.get_focus(), top_left_corner);
    println!("Moving down");
    mux.on_event(Event::Key(Key::Down));
    assert_eq!(mux.get_focus(), bottom_left_corner);
    println!("Moving right...");
    mux.on_event(Event::Key(Key::Right));
    assert_eq!(mux.get_focus(), bottom_left_mid);
    println!("Moving right...");
    mux.on_event(Event::Key(Key::Right));
    assert_eq!(mux.get_focus(), bottom_right_mid);
    println!("Moving right...");
    mux.on_event(Event::Key(Key::Right));
    assert_eq!(mux.get_focus(), bottom_right_corner);
    println!("Moving up...");
    mux.on_event(Event::Key(Key::Up));
    assert_eq!(mux.get_focus(), top_right_corner);

    println!("Circle completed");
}
