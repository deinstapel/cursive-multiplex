extern crate cursive;
extern crate indextree;

use cursive::views::{IdView, Canvas, LinearLayout};
use cursive::view::View;
use cursive::Printer;
use std::fmt;

struct AddViewError;

impl fmt::Display for AddViewError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Adding not possible with given parameters")
    }
}

enum Path {
    LeftOrUp(Box<Path>),
    RightOrDown(Box<Path>),
}

type Id = String;

struct Mux {
    tree: indextree::Arena<Node>,
    v: Box<dyn View>,
}

impl View for Mux {
    fn draw(&self, printer: &Printer<'_, '_>) {
        self.v.draw(printer);
    }
}

struct Node {
    view: Option<Box<dyn View>>,
    id: Id,
}

impl Node {
    fn new<T>(v: T, id: Id) -> Self
    where
        T: View
    {
        Self {
            view: Some(Box::new(v)),
            id: "bar".to_string(),
        }
    }
}

impl Mux {
    pub fn new() -> Mux {

        let layout = LinearLayout::vertical();
        let id_view = IdView::new("root", layout);
        let canvas = Canvas::wrap(id_view);
        let mut new = Mux{
            tree: indextree::Arena::new(),
            v: Box::new(canvas),
        };
        new.tree.new_node(Node::new(canvas, "foo".to_string()));
        new
    }

    pub fn add_horizontal<T>(&self, v: T, path: Option<Path>, id: Option<Id>) -> Result<&Self, AddViewError>
    where
        T: View
    {
        match path {
            Some(path) => self.add_horizontal_path(v, Some(path)),
            None => {
                match id {
                    Some(id) => self.add_horizontal_id(v, Some(id)),
                    None => Err(AddViewError{})
                }
            }
        }
    }

    pub fn add_horizontal_path<T>(&self, v: T, path: Option<Path>) -> Result<&Self, AddViewError>
    where
        T: View
    {
        while path.is_some() {
            match path {
                Some(LeftOrUp) => {},
                Some(RightOrDown) => {},
                None => break,
            }
        }
        Ok(self)
    }

    pub fn add_horizontal_id<T>(&self, v: T, id: Option<Id>) -> Result<&Self, AddViewError>
    where
        T: View
    {
        match id {
            Some(id) => {
                for node in self.tree.iter() {
                    if node.data.id == id {
                        // Add View to node
                        // Split up into two nodes while keeping the parent node but emptying
                    }
                }
            },
            None => {},
        }

        Ok(self)
    }
}
