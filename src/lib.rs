extern crate cursive;
extern crate indextree;
extern crate failure;
#[macro_use] extern crate failure_derive;

mod error;

use cursive::views::{IdView, Canvas, LinearLayout};
use cursive::view::View;
use cursive::Printer;
use failure::Error;
use error::AddViewError;

#[derive(Debug)]
enum Path {
    LeftOrUp(Box<Path>),
    RightOrDown(Box<Path>),
}

type Id = String;

struct Mux {
    tree: indextree::Arena<Node>,
    root: indextree::NodeId,
    v: Box<dyn View>,
}

impl View for Mux {
    fn draw(&self, printer: &Printer<'_, '_>) {
        self.v.draw(printer);
        for node in self.tree.iter() {
            match node.data.view {
                Some(ref view) => {
                    view.draw(printer);
                },
                None => {}
            }
        }
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
            id: id,
        }
    }
}

impl Mux {
    pub fn new() -> Mux {

        let layout = LinearLayout::vertical();
        let id_view = IdView::new("root", layout);
        let canvas = Canvas::wrap(id_view);
        let mut new_tree = indextree::Arena::new();
        let new_root = new_tree.new_node(Node::new(canvas, "foo".to_string()));
        let new_mux = Mux{
            tree: new_tree,
            root: new_root,
            v: Box::new(cursive::views::DummyView),
        };
        new_mux
    }

    // Might remove this
    pub fn add_horizontal<T>(&mut self, v: T, path: Option<Path>, id: Option<Id>, new_id: Id) -> Result<&Self, AddViewError>
    where
        T: View
    {
        match path {
            Some(path) => self.add_horizontal_path(v, self.root, Some(path), new_id),
            None => {
                match id {
                    Some(id) => self.add_horizontal_id(v, id, new_id),
                    None => Err(AddViewError::GenericError{})
                }
            }
        }
    }

    pub fn add_horizontal_path<T>(&mut self, v: T, cur_node: indextree::NodeId, path: Option<Path>, new_id: Id) -> Result<&Self, AddViewError>
    where
        T: View
    {
            match path {
                Some(path_val) => {
                    match path_val {
                        Path::LeftOrUp(ch)=> {
                            match cur_node.children(&self.tree).nth(0) {
                                Some(node) => {
                                    self.add_horizontal_path(v, node, Some(*ch), new_id)
                                    // Ok (self)
                                },
                                None => {
                                    // Truncate
                                    self.add_horizontal_path(v, cur_node, None, new_id)
                                },
                            }
                        },
                        Path::RightOrDown(ch) => {
                            if cur_node.children(&self.tree).count() < 2 {
                                match cur_node.children(&self.tree).last() {
                                    Some(node) => {
                                        self.add_horizontal_path(v, node, Some(*ch), new_id)
                                        // Ok(self)
                                    },
                                    None => {
                                        // Truncate, if too specific
                                        self.add_horizontal_path(v, cur_node, None, new_id)
                                    },
                                }
                            } else {
                                Err(AddViewError::InvalidPath{path: *ch})
                            }
                        },
                    }
                },
                None if cur_node.following_siblings(&self.tree, ).count() + cur_node.preceding_siblings(&self.tree, ).count() < 2 => {
                    let new_node = self.tree.new_node(Node::new(v, new_id));
                    cur_node.insert_after(new_node, &mut self.tree, )?;
                    Ok(self)
                },
                None => {
                    // First element is node itself, second direct parent
                    let parent = cur_node.ancestors(&self.tree).nth(1).unwrap();
                    cur_node.detach(&mut self.tree);

                    let new_intermediate = self.tree.new_node(Node{
                        view: None,
                        id: "intermediate".to_string(),
                    });

                    parent.append(new_intermediate, &mut self.tree)?;
                    new_intermediate.append(cur_node, &mut self.tree, )?;
                    new_intermediate.append(self.tree.new_node(Node::new(v, new_id)), &mut self.tree, )?;
                    Ok(self)
                },
            }
    }

    pub fn add_horizontal_id<T>(&mut self, v: T, id: Id, new_id: Id) -> Result<&Self, AddViewError>
    where
        T: View
    {
                let new_node = self.tree.new_node(Node::new(v, new_id));

                // Copy index here to extra vector so self is not bound to the iterator
                // self.tree is here not clonable, bc no cursive implements the clone trait
                let mut descendants = Vec::new();
                self.root.descendants(&self.tree).for_each(|node_id| {
                    descendants.push(node_id);
                });

                for node_id in descendants.iter() {
                    if self.tree.get(*node_id).unwrap().data.id == id {
                        if node_id.children(&self.tree).count() < 2 {
                            node_id.append(new_node, &mut self.tree)?;
                        } else {
                            // First element is node itself, second direct parent
                            let parent = node_id.ancestors(&self.tree).nth(1).unwrap();
                            node_id.detach(&mut self.tree);

                            let new_intermediate = self.tree.new_node(Node{
                                view: None,
                                id: "intermediate".to_string(),
                            });

                            parent.append(new_intermediate, &mut self.tree)?;
                            new_intermediate.append(*node_id, &mut self.tree, )?;
                            new_intermediate.append(new_node, &mut self.tree, )?;
                        }
                    }
                }
                Ok(self)
    }
}
