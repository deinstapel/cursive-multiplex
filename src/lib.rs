extern crate cursive;
extern crate indextree;
extern crate failure;
#[macro_use] extern crate failure_derive;
#[macro_use] extern crate log;

mod error;

use cursive::view::View;
use cursive::Vec2;
use cursive::Printer;
use error::AddViewError;

#[derive(Debug)]
pub enum Path {
    LeftOrUp(Box<Option<Path>>),
    RightOrDown(Box<Option<Path>>),
}

enum Orientation {
    Vertical,
    Horizontal,
}

type Id = String;

pub struct Mux {
    tree: indextree::Arena<Node>,
    root: indextree::NodeId,
}

impl View for Mux {
    fn draw(&self, printer: &Printer) {
        self.rec_draw(printer, self.root)
    }


    fn needs_relayout(&self) -> bool {
        true
    }

    fn required_size(&mut self, constraint: Vec2) -> Vec2 {
        Vec2::new(constraint.x, constraint.y/3)
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
}

struct Node {
    view: Option<Box<dyn View>>,
    id: Id,
    orientation: Orientation,
}

impl Node {
    fn new<T>(v: T, id: Id, orit: Orientation) -> Self
    where
        T: View
    {
        Self {
            view: Some(Box::new(v)),
            id: id,
            orientation: orit
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
}

impl Mux {
    pub fn new() -> Mux {
        let root_node = Node {
            view: None,
            id: "foo".to_string(),
            orientation: Orientation::Horizontal,
        };
        let mut new_tree = indextree::Arena::new();
        let new_root = new_tree.new_node(root_node);
        let new_mux = Mux{
            tree: new_tree,
            root: new_root,
        };
        new_mux
    }

    fn rec_draw(&self, printer: &Printer, root: indextree::NodeId) {
        self.tree.get(root).unwrap().data.draw(printer);
        let printer1;
        let printer2;
        match root.children(&self.tree).count() {
            1 => self.rec_draw(printer, root.children(&self.tree).next().unwrap()),
            2 => {
                debug!("Print Children Nodes");
                match self.tree.get(root).unwrap().data.orientation {
                    Orientation::Horizontal => {
                        printer1 = printer.cropped(Vec2::new(printer.size.x/2, printer.size.y));
                        printer2 = printer.offset(Vec2::new(printer.size.x/2, 0)).cropped(Vec2::new(printer.size.x/2, printer.size.y));
                    },
                    Orientation::Vertical => {
                        printer1 = printer.cropped(Vec2::new(printer.size.x, printer.size.y/2));
                        printer2 = printer.offset(Vec2::new(0,printer.size.y/2)).cropped(Vec2::new(printer.size.x,printer.size.y/2));
                    },
                }
                self.rec_draw(&printer1, root.children(&self.tree).next().unwrap());
                match self.tree.get(root).unwrap().data.orientation {
                    Orientation::Vertical => {
                        printer1.print_hline(Vec2::new(0, printer.size.y/2-1), printer.size.x, "─");
                    },
                    Orientation::Horizontal => {
                        printer1.print_vline(Vec2::new(printer.size.x/2-1, 0), printer.size.y, "|");
                    },
                }
                debug!("Print Delimiter");
                self.rec_draw(&printer2, root.children(&self.tree).last().unwrap());
            },
            0 => {},
            _ => {debug!("Illegal Number of Child Nodes")},
        }
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
                                    self.add_horizontal_path(v, node, *ch, new_id)
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
                                        self.add_horizontal_path(v, node, *ch, new_id)
                                        // Ok(self)
                                    },
                                    None => {
                                        // Truncate, if too specific
                                        self.add_horizontal_path(v, cur_node, None, new_id)
                                    },
                                }
                            } else {
                                Err(AddViewError::InvalidPath{path: ch.unwrap()})
                            }
                        },
                    }
                },
                None if cur_node.following_siblings(&self.tree, ).count() + cur_node.preceding_siblings(&self.tree, ).count() < 2 => {
                    let new_node = self.tree.new_node(Node::new(v, new_id, Orientation::Horizontal));
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
                        orientation: Orientation::Horizontal,
                    });

                    parent.append(new_intermediate, &mut self.tree)?;
                    new_intermediate.append(cur_node, &mut self.tree, )?;
                    new_intermediate.append(self.tree.new_node(Node::new(v, new_id, Orientation::Horizontal)), &mut self.tree, )?;
                    Ok(self)
                },
            }
    }

    pub fn add_horizontal_id<T>(&mut self, v: T, id: Id, new_id: Id) -> Result<&Self, AddViewError>
    where
        T: View
    {
        self.add_node_id(v, id, new_id,Orientation::Horizontal)
    }

    fn add_node_id<T>(&mut self, v: T, id: Id, new_id: Id, orientation: Orientation) -> Result<&Self, AddViewError>
    where
        T: View
    {
        let new_node = self.tree.new_node(Node::new(v, new_id, Orientation::Horizontal));

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
                    self.tree.get_mut(*node_id).unwrap().data.orientation = orientation;
                } else {
                    // First element is node itself, second direct parent
                    let parent = node_id.ancestors(&self.tree).nth(1).unwrap();
                    node_id.detach(&mut self.tree);

                    let new_intermediate = self.tree.new_node(Node{
                        view: None,
                        id: "intermediate".to_string(),
                        orientation: orientation,
                    });

                    parent.append(new_intermediate, &mut self.tree)?;
                    new_intermediate.append(*node_id, &mut self.tree, )?;
                    new_intermediate.append(new_node, &mut self.tree, )?;
                }
                break
            }
        }
        Ok(self)
    }

    pub fn add_vertical_id<T>(&mut self, v: T, id: Id, new_id: Id) -> Result<&Self, AddViewError>
    where
        T: View
    {
        self.add_node_id(v, id, new_id, Orientation::Vertical)
    }
}
