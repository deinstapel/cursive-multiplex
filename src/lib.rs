extern crate cursive;
extern crate indextree;
extern crate failure;
#[macro_use] extern crate failure_derive;
#[macro_use] extern crate log;

mod error;

use cursive::view::{View, Selector};
use cursive::event::{Event, EventResult, Key};
use cursive::direction::{Absolute, Direction};
use cursive::Vec2;
use cursive::Printer;
use error::AddViewError;

#[derive(Debug)]
pub enum Path {
    LeftOrUp(Box<Option<Path>>),
    RightOrDown(Box<Option<Path>>),
}

#[derive(PartialEq)]
enum Orientation {
    Vertical,
    Horizontal,
}

#[derive(Debug,PartialEq)]
enum SearchPath {
    Left,
    Right,
    Up,
    Down,
}

pub type Id = indextree::NodeId;

pub struct Mux {
    tree: indextree::Arena<Node>,
    root: indextree::NodeId,
    focus: indextree::NodeId,
}

impl View for Mux {
    fn draw(&self, printer: &Printer) {
        debug!("Current Focus: {}", self.focus);
        self.rec_draw(printer, self.root)
    }


    fn needs_relayout(&self) -> bool {
        true
    }

    fn required_size(&mut self, constraint: Vec2) -> Vec2 {
        constraint
    }

    fn layout(&mut self, constraint: Vec2) {
        // We need mutables for layouting so lets take another route
        let mut ids = Vec::new();
        for node_id in self.root.descendants(&self.tree) {
            ids.push(node_id);
        }

        // And now read them out, but mutable
        for node_id in ids {
            self.tree.get_mut(node_id).unwrap().data.layout_view(constraint);
        }
    }

    fn take_focus(&mut self, _source: Direction) -> bool {
        true
    }

    fn focus_view(&mut self, _: &Selector) -> Result<(), ()> {
        Ok(())
    }

    fn on_event(&mut self, evt: Event) -> EventResult {
        match evt {
            Event::Key(Key::Left) => {
                self.move_focus(Absolute::Left)
            },
            Event::Key(Key::Right) => {
                self.move_focus(Absolute::Right)
            },
            Event::Key(Key::Up) => {
                self.move_focus(Absolute::Up)
            },
            Event::Key(Key::Down) => {
                self.move_focus(Absolute::Down)
            },
            _ => EventResult::Ignored,
        }
    }
}

struct Node {
    view: Option<Box<dyn View>>,
    orientation: Orientation,
}

impl Node {
    fn new<T>(v: T, orit: Orientation) -> Self
    where
        T: View
    {
        Self {
            view: Some(Box::new(v)),
            orientation: orit
        }
    }

    fn has_view(&self) -> bool {
        match self.view {
            Some(_) => true,
            None => false,
        }
    }

    fn layout_view(&mut self, vec: Vec2) {
        if let Some(x) = self.view.as_mut() {
            x.layout(vec);
        }
    }

    fn draw(&self, printer: &Printer) {
        match self.view {
            Some(ref view) => {
                view.draw(printer);
            },
            None => {},
        }
    }

    fn take_focus(&mut self) -> bool {
        if let Some(view) = self.view.as_mut() {
            view.take_focus(Direction::none())
        } else {
            false
        }
    }

}

impl Mux {
    pub fn new() -> Mux {
        let root_node = Node {
            view: None,
            orientation: Orientation::Horizontal,
        };
        let mut new_tree = indextree::Arena::new();
        let new_root = new_tree.new_node(root_node);
        let new_mux = Mux{
            tree: new_tree,
            root: new_root,
            focus: new_root,
        };
        new_mux
    }

    pub fn get_root(&self) -> Id {
        self.root
    }

    fn rec_draw(&self, printer: &Printer, root: indextree::NodeId) {
        self.tree.get(root).unwrap().data.draw(printer);
        let printer1;
        let printer2;
        match root.children(&self.tree).count() {
            1 => self.rec_draw(printer, root.children(&self.tree).next().unwrap()),
            2 => {
                debug!("Print Children Nodes");
                let left = root.children(&self.tree).next().unwrap();
                let right = root.children(&self.tree).last().unwrap();
                match self.tree.get(root).unwrap().data.orientation {
                    Orientation::Horizontal => {
                        printer1 = printer.cropped(Vec2::new(printer.size.x/2, printer.size.y));
                        printer2 = printer.offset(Vec2::new(printer.size.x/2, 0)).cropped(Vec2::new(printer.size.x/2, printer.size.y));
                    },
                    Orientation::Vertical => {
                        printer1 = printer.cropped(Vec2::new(printer.size.x, printer.size.y/2)).focused(self.focus == left);
                        printer2 = printer.offset(Vec2::new(0,printer.size.y/2)).cropped(Vec2::new(printer.size.x,printer.size.y/2));
                    },
                }
                self.rec_draw(&printer1, left);
                match self.tree.get(root).unwrap().data.orientation {
                    Orientation::Vertical => {
                        printer1.print_hline(Vec2::new(0, printer.size.y/2-1), printer.size.x, "─");
                    },
                    Orientation::Horizontal => {
                        printer1.print_vline(Vec2::new(printer.size.x/2-1, 0), printer.size.y, "|");
                    },
                }
                debug!("Print Delimiter");
                self.rec_draw(&printer2, right);
            },
            0 => {},
            _ => {debug!("Illegal Number of Child Nodes")},
        }
    }

    pub fn add_horizontal_path<T>(&mut self, v: T, path: Option<Path>) -> Result<Id, AddViewError>
    where
        T: View
    {
        self.add_node_path(v, path, Orientation::Horizontal, self.root)
    }

    pub fn add_vertical_path<T>(&mut self, v: T, path: Option<Path>) -> Result<Id, AddViewError>
    where
        T: View
    {
        self.add_node_path(v, path, Orientation::Vertical, self.root)
    }

    fn add_node_path<T>(&mut self, v: T, path: Option<Path>, orientation: Orientation, cur_node: indextree::NodeId) -> Result<Id, AddViewError>
    where
        T: View
    {
            match path {
                Some(path_val) => {
                    match path_val {
                        Path::LeftOrUp(ch)=> {
                            match cur_node.children(&self.tree).nth(0) {
                                Some(node) => {
                                    self.add_node_path(v, *ch, orientation, node)
                                },
                                None => {
                                    // Truncate
                                    self.add_node_path(v, None, orientation, cur_node)
                                },
                            }
                        },
                        Path::RightOrDown(ch) => {
                            if cur_node.children(&self.tree).count() < 2 {
                                match cur_node.children(&self.tree).last() {
                                    Some(node) => {
                                        self.add_node_path(v, *ch, orientation, node)
                                        // Ok(self)
                                    },
                                    None => {
                                        // Truncate, if too specific
                                        self.add_node_path(v, None, orientation, cur_node)
                                    },
                                }
                            } else {
                                Err(AddViewError::InvalidPath{path: ch.unwrap()})
                            }
                        },
                    }
                },
                None if cur_node.following_siblings(&self.tree, ).count() + cur_node.preceding_siblings(&self.tree, ).count() < 2 => {
                    let new_node = self.tree.new_node(Node::new(v, Orientation::Horizontal));
                    cur_node.insert_after(new_node, &mut self.tree, )?;
                    self.focus = new_node;
                    debug!("Changed Focus: {}", new_node);
                    Ok(new_node)
                },
                None => {
                    // First element is node itself, second direct parent
                    let parent = cur_node.ancestors(&self.tree).nth(1).unwrap();
                    cur_node.detach(&mut self.tree);

                    let new_intermediate = self.tree.new_node(Node{
                        view: None,
                        orientation: Orientation::Horizontal,
                    });

                    parent.append(new_intermediate, &mut self.tree)?;
                    new_intermediate.append(cur_node, &mut self.tree, )?;
                    let new_node = self.tree.new_node(Node::new(v, Orientation::Horizontal));
                    new_intermediate.append(new_node, &mut self.tree, )?;
                    self.focus = new_node;
                    debug!("Changed Focus: {}", new_node);
                    Ok(new_node)
                },
            }
    }

    pub fn add_horizontal_id<T>(&mut self, v: T, id: Id) -> Result<Id, AddViewError>
    where
        T: View
    {
        self.add_node_id(v, id, Orientation::Horizontal)
    }

    fn add_node_id<T>(&mut self, v: T, id: Id, orientation: Orientation) -> Result<Id, AddViewError>
    where
        T: View
    {
        let new_node = self.tree.new_node(Node::new(v, Orientation::Horizontal));

        let mut node_id;
        if let Some(parent) = id.ancestors(&self.tree).nth(1) {
            node_id = parent;
        } else {
            node_id = id;
        }

        if node_id.children(&self.tree).count() < 2 && !self.tree.get(node_id).unwrap().data.has_view() {
            node_id.append(new_node, &mut self.tree)?;
            self.tree.get_mut(node_id).unwrap().data.orientation = orientation;
        } else {
            // First element is node itself, second direct parent
            let parent = node_id;
            node_id = id;

            let position: Path;
            if parent.children(&self.tree).next().unwrap() == node_id {
                position = Path::LeftOrUp(Box::new(None));
            } else {
                position = Path::RightOrDown(Box::new(None));
            }

            node_id.detach(&mut self.tree);

            let new_intermediate = self.tree.new_node(Node{
                view: None,
                orientation: orientation,
            });
            match position {
                Path::RightOrDown(_) => {
                    parent.append(new_intermediate, &mut self.tree)?;
                },
                Path::LeftOrUp(_) => {
                    parent.prepend(new_intermediate, &mut self.tree)?;
                }
            }
            new_intermediate.append(node_id, &mut self.tree, )?;
            new_intermediate.append(new_node, &mut self.tree, )?;
            debug!("Changed order");
        }

        self.focus = new_node;
        debug!("Changed Focus: {}", new_node);
        Ok(new_node)
    }

    pub fn add_vertical_id<T>(&mut self, v: T, id: Id) -> Result<Id, AddViewError>
    where
        T: View
    {
        self.add_node_id(v, id, Orientation::Vertical)
    }

    fn move_focus(&mut self, direction: Absolute) -> EventResult {
        match self.search_focus_path(direction, self.focus.ancestors(&self.tree).nth(1).unwrap(), self.focus) {
            Ok((path, turn_point)) => {
                // Traverse the path down again
                if let Some(focus) = self.traverse_search_path(path, turn_point) {
                    if self.tree.get_mut(focus).unwrap().data.take_focus() {
                        self.focus = focus;
                    } else {
                        println!("Focus rejected by {}", focus);
                    }
                    EventResult::Consumed(None)
                } else {
                    EventResult::Ignored
                }
            },
            Err(_) => EventResult::Ignored,
        }
    }

    fn traverse_search_path(&self, mut path: Vec<SearchPath>, turn_point: Id) -> Option<Id> {
        let mut cur_node = turn_point;

        println!("Path Begin: {:?}", path);
        while let Some(step) = path.pop() {
            match self.traverse_single_node(step, turn_point, cur_node) {
                Some(node) => {
                    cur_node = node;
                },
                None => {
                    println!("Remaining Path: {:?}", path);
                    println!("Current node: {}", cur_node);
                    // cur_node = cur_node.children(&self.tree).next().unwrap();
                    break
                },
            }
        }
        Some(cur_node)
    }


    fn traverse_single_node(&self, action: SearchPath, turn_point: Id, cur_node: Id) -> Option<Id> {
        let left = || -> Option<Id> {
            if let Some(left) = cur_node.children(&self.tree).next() {
                Some(left)
            } else {
                None
            }
        };

        let right = || -> Option<Id> {
            if let Some(right) = cur_node.children(&self.tree).last() {
                Some(right)
            } else {
                None
            }
        };
        let up = left;
        let down = right;

        match self.tree.get(turn_point).unwrap().data.orientation {
            Orientation::Horizontal => {
                match action {
                    // Switching Sides for Left & Right
                    SearchPath::Right if self.tree.get(cur_node).unwrap().data.orientation == Orientation::Horizontal => {
                        left()
                    },
                    SearchPath::Left if self.tree.get(cur_node).unwrap().data.orientation == Orientation::Horizontal => {
                        right()
                    },
                    // Remain for Up & Down
                    SearchPath::Up if self.tree.get(cur_node).unwrap().data.orientation == Orientation::Vertical => {
                        up()
                    },
                    SearchPath::Down if self.tree.get(cur_node).unwrap().data.orientation == Orientation::Vertical => {
                        down()
                    },
                    _ => None,
                }
            },
            Orientation::Vertical => {
                match action {
                    // Remain for Left & Right
                    SearchPath::Right if self.tree.get(cur_node).unwrap().data.orientation == Orientation::Horizontal => {
                        right()
                    },
                    SearchPath::Left if self.tree.get(cur_node).unwrap().data.orientation == Orientation::Horizontal => {
                        left()
                    },
                    // Switch for Up & Down
                    SearchPath::Up if self.tree.get(cur_node).unwrap().data.orientation == Orientation::Vertical => {
                        down()
                    },
                    SearchPath::Down if self.tree.get(cur_node).unwrap().data.orientation == Orientation::Vertical => {
                        up()
                    },
                    _ => None,
                }
            },
        }
    }

    fn search_focus_path(&self, direction: Absolute, nodeid: Id, fromid: Id) -> Result<(Vec<SearchPath>, Id), ()>  {

        let mut cur_node = Some(nodeid);
        let mut from_node = fromid;

        let mut path = Vec::new();

        while cur_node.is_some() {
            println!("Current node in search path: {}", cur_node.unwrap());
            match self.tree.get(cur_node.unwrap()).unwrap().data.orientation {
                Orientation::Horizontal if direction == Absolute::Left || direction == Absolute::Right => {
                    if nodeid.children(&self.tree).next().unwrap() == from_node {
                        path.push(SearchPath::Left);
                        from_node = cur_node.unwrap();
                        cur_node = None;
                    } else {
                        path.push(SearchPath::Right);
                        from_node = cur_node.unwrap();
                        cur_node = None;
                    }
                },
                Orientation::Vertical if direction == Absolute::Up || direction == Absolute::Down => {
                    if nodeid.children(&self.tree).next().unwrap() == from_node {
                        path.push(SearchPath::Up);
                        from_node = cur_node.unwrap();
                        cur_node = None;
                    } else {
                        path.push(SearchPath::Down);
                        from_node = cur_node.unwrap();
                        cur_node = None;
                    }
                },
                Orientation::Horizontal => {
                    if nodeid.children(&self.tree).next().unwrap() == from_node {
                        path.push(SearchPath::Left);
                        from_node = cur_node.unwrap();
                        cur_node = cur_node.unwrap().ancestors(&self.tree).nth(1);
                    } else {
                        path.push(SearchPath::Right);
                        from_node = cur_node.unwrap();
                        cur_node = cur_node.unwrap().ancestors(&self.tree).nth(1);
                    }
                },
                Orientation::Vertical => {
                    if nodeid.children(&self.tree).next().unwrap() == from_node {
                        path.push(SearchPath::Up);
                        from_node = cur_node.unwrap();
                        cur_node = cur_node.unwrap().ancestors(&self.tree).nth(1);
                    } else {
                        path.push(SearchPath::Down);
                        from_node = cur_node.unwrap();
                        cur_node = cur_node.unwrap().ancestors(&self.tree).nth(1);
                    }
                }
            }
        }

        match self.tree.get(from_node).unwrap().data.orientation {
            Orientation::Horizontal if *path.last().unwrap() == SearchPath::Down || *path.last().unwrap() == SearchPath::Up => {
                Err(())
            },
            Orientation::Vertical if *path.last().unwrap() == SearchPath::Left || *path.last().unwrap() == SearchPath::Right => {
                Err(())
            },
            _ => {
                Ok((path, from_node))
            }
        }
    }
}


#[cfg(test)]
mod tree {
    use cursive::views::{DummyView, TextArea};
    use cursive::event::{Key, Event};
    use cursive::traits::View;
    use super::Mux;

    #[test]
    fn test_vertical() {
        // Vertical test
        println!("Vertical Test");
        let mut test_mux = Mux::new();
        let node1 = test_mux.add_vertical_id(TextArea::new(), test_mux.get_root()).unwrap();
        let node2 = test_mux.add_vertical_id(TextArea::new(), node1).unwrap();
        let node3 = test_mux.add_vertical_id(TextArea::new(), node2).unwrap();

        assert_eq!(node3, test_mux.focus);
        println!("Up Movement");
        test_mux.on_event(Event::Key(Key::Up));
        print_tree(&test_mux);
        assert_eq!(node2, test_mux.focus);
        println!("Down Movement");
        test_mux.on_event(Event::Key(Key::Down));
        assert_eq!(node3, test_mux.focus);

        // direction_test(&mut test_mux);
    }

    #[test]
    fn test_nesting() {
        println!("Nesting Test");

        let mut mux = Mux::new();

        let mut nodes = Vec::new();

        for _ in 0..10 {
            print_tree(&mux);
            match mux.add_horizontal_id(DummyView, if let Some(x) = nodes.last() {*x} else {mux.get_root()}) {
                Ok(node) => {
                    nodes.push(node);
                },
                Err(_) => {
                    assert!(false);
                },
            }
            match mux.add_vertical_id(DummyView, *nodes.last().unwrap()) {
                Ok(node) => {
                    nodes.push(node);
                },
                Err(_) => {
                    assert!(false);
                },
            }
        }

        for node in nodes.iter() {
            mux.focus = *node;
            direction_test(&mut mux);
        }
    }

    #[test]
    fn expected_vertical_horizontal() {
        let mut mux = Mux::new();
        let node1 = mux.add_horizontal_id(TextArea::new(), mux.root).unwrap();
        let node2 = mux.add_horizontal_id(TextArea::new(), node1).unwrap();
        let node3 = mux.add_vertical_id(TextArea::new(), node2).unwrap();
        print_tree(&mux);

        assert_eq!(mux.focus, node3);
        mux.on_event(Event::Key(Key::Up));
        assert_eq!(mux.focus, node2);
        match mux.on_event(Event::Key(Key::Left)) {
            cursive::event::EventResult::Consumed(_) => {
                assert_eq!(mux.focus, node1);
            },
            cursive::event::EventResult::Ignored => {
                println!("Not to be ignored Event ignored, Focus was at: {}", mux.focus);
                assert!(false);
            },
        }
    }

    fn print_tree(mux: &Mux) {
        print!("Current Tree: ");
        for node in mux.root.descendants(&mux.tree) {
            print!("{},", node);
        }
        println!("");
    }

    fn direction_test(mux: &mut Mux) {
        // This is a shotgun approach to have a look if any unforeseen focus moves could happen, resulting in a uncertain state
        mux.on_event(Event::Key(Key::Up));
        mux.on_event(Event::Key(Key::Left));
        mux.on_event(Event::Key(Key::Down));
        mux.on_event(Event::Key(Key::Right));
        mux.on_event(Event::Key(Key::Up));
        mux.on_event(Event::Key(Key::Left));
        mux.on_event(Event::Key(Key::Left));
        mux.on_event(Event::Key(Key::Down));
        mux.on_event(Event::Key(Key::Right));
        mux.on_event(Event::Key(Key::Up));
        mux.on_event(Event::Key(Key::Left));
    }
}
