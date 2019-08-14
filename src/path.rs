use crate::error::AddViewError;
use crate::node::Node;
use crate::{Id, Mux, Orientation, View};

/// Path is a recursive enum made to be able to identify a pane by it's actual location in the multiplexer. An upper Pane on the left side for example would have the path `Path::LeftOrUp(Box::new(Some(Path::LeftOrUp(Box::new(None)))))`.
#[derive(Debug)]
pub enum Path {
    LeftOrUp(Box<Option<Path>>),
    RightOrDown(Box<Option<Path>>),
}

impl Mux {
    /// Add the given view to the tree based on the path, if the path is too specific it will be truncated, if not specific enough an error will be returned.
    /// When successful `Ok()` will contain the assigned `Id`
    /// # Example
    /// ```
    /// # extern crate cursive;
    /// # use cursive_multiplex::{Path};
    /// # fn main () {
    /// # let (mut mux, node1) = cursive_multiplex::Mux::new(cursive::views::DummyView);
    /// # let current_focus = mux.get_focus();
    /// # assert_eq!(current_focus, node1);
    /// let new_node = mux.add_horizontal_path(cursive::views::DummyView, Path::RightOrDown(Box::new(None))).unwrap();
    /// # }
    /// ```
    pub fn add_horizontal_path<T>(&mut self, v: T, path: Path) -> Result<Id, AddViewError>
    where
        T: View,
    {
        self.add_node_path(v, Some(path), Orientation::Horizontal, self.root)
    }

    /// Add the given view to the tree based on the path, if the path is too specific it will be truncated, if not specific enough an error will be returned.
    /// When successful `Ok()` will contain the assigned `Id`
    /// # Example
    /// ```
    /// # extern crate cursive;
    /// # use cursive_multiplex::{Path};
    /// # fn main () {
    /// # let (mut mux, node1) = cursive_multiplex::Mux::new(cursive::views::DummyView);
    /// # let current_focus = mux.get_focus();
    /// # assert_eq!(current_focus, node1);
    /// let new_node = mux.add_vertical_path(cursive::views::DummyView, Path::RightOrDown(Box::new(None))).unwrap();
    /// # }
    /// ```
    pub fn add_vertical_path<T>(&mut self, v: T, path: Path) -> Result<Id, AddViewError>
    where
        T: View,
    {
        self.add_node_path(v, Some(path), Orientation::Vertical, self.root)
    }

    fn add_node_path<T>(
        &mut self,
        v: T,
        path: Option<Path>,
        orientation: Orientation,
        cur_node: indextree::NodeId,
    ) -> Result<Id, AddViewError>
    where
        T: View,
    {
        match path {
            Some(path_val) => {
                match path_val {
                    Path::LeftOrUp(ch) => {
                        match cur_node.children(&self.tree).nth(0) {
                            Some(node) => self.add_node_path(v, *ch, orientation, node),
                            None => {
                                // Truncate
                                self.add_node_path(v, None, orientation, cur_node)
                            }
                        }
                    }
                    Path::RightOrDown(ch) => {
                        if cur_node.children(&self.tree).count() < 2 {
                            match cur_node.children(&self.tree).last() {
                                Some(node) => {
                                    self.add_node_path(v, *ch, orientation, node)
                                    // Ok(self)
                                }
                                None => {
                                    // Truncate, if too specific
                                    self.add_node_path(v, None, orientation, cur_node)
                                }
                            }
                        } else {
                            Err(AddViewError::InvalidPath { path: ch.unwrap() })
                        }
                    }
                }
            }
            None if cur_node.following_siblings(&self.tree).count()
                + cur_node.preceding_siblings(&self.tree).count()
                < 2 =>
            {
                let new_node = self.tree.new_node(Node::new(v, Orientation::Horizontal));
                cur_node.insert_after(new_node, &mut self.tree);
                self.focus = new_node;
                debug!("Changed Focus: {}", new_node);
                Ok(new_node)
            }
            None => {
                // First element is node itself, second direct parent
                let parent = cur_node.ancestors(&self.tree).nth(1).unwrap();
                cur_node.detach(&mut self.tree);
                let new_intermediate = self.tree.new_node(Node {
                    view: None,
                    split_ratio_offset: 0,
                    orientation: Orientation::Horizontal,
                });
                parent.append(new_intermediate, &mut self.tree);
                new_intermediate.append(cur_node, &mut self.tree);
                let new_node = self.tree.new_node(Node::new(v, Orientation::Horizontal));
                new_intermediate.append(new_node, &mut self.tree);
                self.focus = new_node;
                debug!("Changed Focus: {}", new_node);
                Ok(new_node)
            }
        }
    }
}
