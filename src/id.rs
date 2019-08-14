use crate::error::{AddViewError, RemoveViewError, SwitchError};
use crate::node::Node;
pub use crate::path::Path;
use crate::{Mux, Orientation, View};

/// Identifier for views in binary tree of mux, typically returned after adding a new view to the multiplexer.
pub type Id = indextree::NodeId;

impl Mux {
    /// Removes the given id from the multiplexer, returns an error if not a valid id contained in the tree or the lone root of the tree.
    /// When successful the Id of the removed Node is returned.
    /// # Example
    /// ```
    /// # extern crate cursive;
    /// # use cursive_multiplex::{Path};
    /// # fn main () {
    /// # let (mut mux, node1) = cursive_multiplex::Mux::new(cursive::views::DummyView);
    /// let new_node = mux.add_vertical_id(cursive::views::DummyView, node1).unwrap();
    /// mux.remove_id(new_node);
    /// # }
    /// ```
    pub fn remove_id(&mut self, id: Id) -> Result<Id, RemoveViewError> {
        let desc: Vec<Id> = self.root.descendants(&self.tree).collect();
        if desc.contains(&id) {
            let sib_id: Id;
            if id.preceding_siblings(&self.tree).count() > 1 {
                sib_id = id.preceding_siblings(&self.tree).nth(1).unwrap();
            } else if id.following_siblings(&self.tree).count() > 1 {
                sib_id = id.following_siblings(&self.tree).nth(1).unwrap();
            } else {
                return Err(RemoveViewError::Generic {});
            }
            let parent = id.ancestors(&self.tree).nth(1).unwrap();
            id.detach(&mut self.tree);
            if let Some(anker) = parent.ancestors(&self.tree).nth(1) {
                if anker.children(&self.tree).next().unwrap() == parent {
                    parent.detach(&mut self.tree);
                    anker.prepend(sib_id, &mut self.tree);
                    self.focus = sib_id;
                    Ok(id)
                } else {
                    parent.detach(&mut self.tree);
                    anker.append(sib_id, &mut self.tree);
                    self.focus = sib_id;
                    Ok(id)
                }
            } else {
                self.root = sib_id;
                self.focus = sib_id;
                Ok(id)
            }
        } else {
            Err(RemoveViewError::InvalidId { id: id })
        }
    }

    /// Add the given view to the tree based on the path, if the path is too specific it will be truncated, if not specific enough an error will be returned.
    /// When successful `Ok()` will contain the assigned `Id`
    /// # Example
    /// ```
    /// # extern crate cursive;
    /// # use cursive_multiplex::{Path};
    /// # fn main () {
    /// let (mut mux, node1) = cursive_multiplex::Mux::new(cursive::views::DummyView);
    /// let new_node = mux.add_vertical_id(cursive::views::DummyView, node1).unwrap();
    /// # }
    /// ```
    pub fn add_vertical_id<T>(&mut self, v: T, id: Id) -> Result<Id, AddViewError>
    where
        T: View,
    {
        self.add_node_id(v, id, Orientation::Vertical)
    }

    /// Add the given view to the tree based on the path, if the path is too specific it will be truncated, if not specific enough an error will be returned.
    /// When successful `Ok()` will contain the assigned `Id`
    /// # Example
    /// ```
    /// # extern crate cursive;
    /// # use cursive_multiplex::{Path};
    /// # fn main () {
    /// let (mut mux, node1) = cursive_multiplex::Mux::new(cursive::views::DummyView);
    /// let new_node = mux.add_horizontal_id(cursive::views::DummyView, node1).unwrap();
    /// # }
    /// ```
    pub fn add_horizontal_id<T>(&mut self, v: T, id: Id) -> Result<Id, AddViewError>
    where
        T: View,
    {
        self.add_node_id(v, id, Orientation::Horizontal)
    }

    fn add_node_id<T>(&mut self, v: T, id: Id, orientation: Orientation) -> Result<Id, AddViewError>
    where
        T: View,
    {
        let new_node = self.tree.new_node(Node::new(v, Orientation::Horizontal));

        let mut node_id;
        if let Some(parent) = id.ancestors(&self.tree).nth(1) {
            node_id = parent;
        } else {
            node_id = id;
        }

        if node_id.children(&self.tree).count() < 2
            && !self.tree.get(node_id).unwrap().get().has_view()
        {
            node_id.append(new_node, &mut self.tree);
            self.tree.get_mut(node_id).unwrap().get_mut().orientation = orientation;
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

            let new_intermediate = self.tree.new_node(Node::new_empty(orientation));
            match position {
                Path::RightOrDown(_) => {
                    parent.append(new_intermediate, &mut self.tree);
                }
                Path::LeftOrUp(_) => {
                    parent.prepend(new_intermediate, &mut self.tree);
                }
            }
            new_intermediate.append(node_id, &mut self.tree);
            new_intermediate.append(new_node, &mut self.tree);
            debug!("Changed order");
        }

        self.focus = new_node;
        debug!("Changed Focus: {}", new_node);
        Ok(new_node)
    }

    /// Allows for position switching of two views, returns error if ids not in multiplexer.
    /// When successful empty `Ok(())`
    /// # Example
    /// ```
    /// # extern crate cursive;
    /// # use cursive_multiplex::{Path};
    /// # fn main () {
    /// # let (mut mux, node1) = cursive_multiplex::Mux::new(cursive::views::DummyView);
    /// let daniel = mux.add_vertical_id(cursive::views::DummyView, node1).unwrap();
    /// let the_cooler_daniel = mux.add_vertical_id(cursive::views::DummyView, node1).unwrap();
    /// // Oops I wanted the cooler daniel in another spot
    /// mux.switch_views(daniel, the_cooler_daniel);
    /// # }
    /// ```
    pub fn switch_views(&mut self, fst: Id, snd: Id) -> Result<(), SwitchError> {
        if let Some(parent1) = fst.ancestors(&self.tree).nth(1) {
            if let Some(parent2) = snd.ancestors(&self.tree).nth(1) {
                if parent1.children(&self.tree).next().unwrap() == fst {
                    fst.detach(&mut self.tree);
                    if parent2.children(&self.tree).next().unwrap() == snd {
                        snd.detach(&mut self.tree);
                        parent1.prepend(snd, &mut self.tree);
                        parent2.prepend(fst, &mut self.tree);
                        Ok(())
                    } else {
                        snd.detach(&mut self.tree);
                        parent1.prepend(snd, &mut self.tree);
                        parent2.append(fst, &mut self.tree);
                        Ok(())
                    }
                } else {
                    fst.detach(&mut self.tree);
                    if parent2.children(&self.tree).next().unwrap() == snd {
                        snd.detach(&mut self.tree);
                        parent1.append(snd, &mut self.tree);
                        parent2.prepend(fst, &mut self.tree);
                        Ok(())
                    } else {
                        snd.detach(&mut self.tree);
                        parent1.append(snd, &mut self.tree);
                        parent2.append(fst, &mut self.tree);
                        Ok(())
                    }
                }
            } else {
                Err(SwitchError::NoParent { from: snd, to: fst })
            }
        } else {
            Err(SwitchError::NoParent { from: fst, to: snd })
        }
    }
}
