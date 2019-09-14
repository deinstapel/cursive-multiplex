use crate::error::{AddViewError, RemoveViewError, SwitchError};
use crate::node::Node;
use crate::path::SearchPath;
use crate::{Mux, Orientation, View};

/// Identifier for views in binary tree of mux, typically returned after adding a new view to the multiplexer.
pub type Id = indextree::NodeId;

impl Mux {
    /// Removes the given id from the multiplexer, returns an error if not a valid id contained in the tree or the lone root of the tree.
    /// When successful the Id of the removed Node is returned.
    /// # Example
    /// ```
    /// # fn main () {
    /// # let mut mux = cursive_multiplex::Mux::new();
    /// # let node1 = mux.add_right_of(cursive::views::DummyView, mux.root().build().unwrap()).unwrap();
    /// let new_node = mux.add_below(cursive::views::DummyView, node1).unwrap();
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
            self.invalidated = true;
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

    /// Add the given view, below the given Id.
    /// The new view and the indexed one will share the space previously given to the give Id.
    /// When successful `Ok()` will contain the assigned `Id`
    /// # Example
    /// ```
    /// # extern crate cursive;
    /// # fn main () {
    /// let mut mux = cursive_multiplex::Mux::new();
    /// let node1 = mux.add_right_of(cursive::views::DummyView, mux.root().build().unwrap()).unwrap();
    /// let new_node = mux.add_below(cursive::views::DummyView, node1).unwrap();
    /// # }
    /// ```
    pub fn add_below<T>(&mut self, v: T, id: Id) -> Result<Id, AddViewError>
    where
        T: View,
    {
        self.add_node_id(v, id, Orientation::Vertical, SearchPath::Down)
    }

    /// Add the given view, above the given Id.
    /// The new view and the indexed one will share the space previously given to the give Id.
    /// When successful `Ok()` will contain the assigned `Id`
    /// # Example
    /// ```
    /// # extern crate cursive;
    /// # fn main () {
    /// let mut mux = cursive_multiplex::Mux::new();
    /// let node1 = mux.add_right_of(cursive::views::DummyView, mux.root().build().unwrap()).unwrap();
    /// let new_node = mux.add_above(cursive::views::DummyView, node1).unwrap();
    /// # }
    /// ```
    pub fn add_above<T>(&mut self, v: T, id: Id) -> Result<Id, AddViewError>
    where
        T: View,
    {
        self.add_node_id(v, id, Orientation::Vertical, SearchPath::Up)
    }

    /// Add the given view, left of the given Id.
    /// The new view and the indexed one will share the space previously given to the give Id.
    /// When successful `Ok()` will contain the assigned `Id`
    /// # Example
    /// ```
    /// # extern crate cursive;
    /// # fn main () {
    /// let mut mux = cursive_multiplex::Mux::new();
    /// let node1 = mux.add_right_of(cursive::views::DummyView, mux.root().build().unwrap()).unwrap();
    /// let new_node = mux.add_left_of(cursive::views::DummyView, node1).unwrap();
    /// # }
    /// ```
    pub fn add_left_of<T>(&mut self, v: T, id: Id) -> Result<Id, AddViewError>
    where
        T: View,
    {
        self.add_node_id(v, id, Orientation::Horizontal, SearchPath::Left)
    }

    /// Add the given view, right of the given Id.
    /// The new view and the indexed one will share the space previously given to the give Id.
    /// When successful `Ok()` will contain the assigned `Id`
    /// # Example
    /// ```
    /// # extern crate cursive;
    /// # fn main () {
    /// let mut mux = cursive_multiplex::Mux::new();
    /// let node1 = mux.add_right_of(cursive::views::DummyView, mux.root().build().unwrap()).unwrap();
    /// let new_node = mux.add_right_of(cursive::views::DummyView, node1).unwrap();
    /// # }
    /// ```
    pub fn add_right_of<T>(&mut self, v: T, id: Id) -> Result<Id, AddViewError>
    where
        T: View,
    {
        self.add_node_id(v, id, Orientation::Horizontal, SearchPath::Right)
    }

    fn add_node_id<T>(
        &mut self,
        v: T,
        id: Id,
        orientation: Orientation,
        direction: SearchPath,
    ) -> Result<Id, AddViewError>
    where
        T: View,
    {
        self.invalidated = true;
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
            match direction {
                SearchPath::Up | SearchPath::Left => node_id.prepend(new_node, &mut self.tree),
                SearchPath::Down | SearchPath::Right => node_id.append(new_node, &mut self.tree),
            }
            self.tree.get_mut(node_id).unwrap().get_mut().orientation = orientation;
        } else {
            // First element is node itself, second direct parent
            let parent = node_id;
            node_id = id;

            let position: SearchPath;
            if parent.children(&self.tree).next().unwrap() == node_id {
                position = SearchPath::Left;
            } else {
                position = SearchPath::Right;
            }

            node_id.detach(&mut self.tree);

            let new_intermediate = self.tree.new_node(Node::new_empty(orientation));
            match position {
                SearchPath::Right | SearchPath::Down => {
                    parent.append(new_intermediate, &mut self.tree);
                }
                SearchPath::Left | SearchPath::Up => {
                    parent.prepend(new_intermediate, &mut self.tree);
                }
            }
            match direction {
                SearchPath::Up | SearchPath::Left => {
                    new_intermediate.append(new_node, &mut self.tree);
                    new_intermediate.append(node_id, &mut self.tree);
                }
                SearchPath::Down | SearchPath::Right => {
                    new_intermediate.append(node_id, &mut self.tree);
                    new_intermediate.append(new_node, &mut self.tree);
                }
            }
            debug!("Changed order");
        }

        if self.tree.get_mut(new_node).unwrap().get_mut().take_focus() {
            self.focus = new_node;
            debug!("Changed Focus: {}", new_node);
        }
        Ok(new_node)
    }

    /// Allows for position switching of two views, returns error if ids not in multiplexer.
    /// When successful empty `Ok(())`
    /// # Example
    /// ```
    /// # extern crate cursive;
    /// # fn main () {
    /// # let mut mux = cursive_multiplex::Mux::new();
    /// # let node1 = mux.add_right_of(cursive::views::DummyView, mux.root().build().unwrap()).unwrap();
    /// let daniel = mux.add_below(cursive::views::DummyView, node1).unwrap();
    /// let the_cooler_daniel = mux.add_below(cursive::views::DummyView, node1).unwrap();
    /// // Oops I wanted the cooler daniel in another spot
    /// mux.switch_views(daniel, the_cooler_daniel);
    /// # }
    /// ```
    pub fn switch_views(&mut self, fst: Id, snd: Id) -> Result<(), SwitchError> {
        if let Some(parent1) = fst.ancestors(&self.tree).nth(1) {
            if let Some(parent2) = snd.ancestors(&self.tree).nth(1) {
                self.invalidated = true;
                if parent1.children(&self.tree).next().unwrap() == fst {
                    if parent2.children(&self.tree).next().unwrap() == snd {
                        fst.detach(&mut self.tree);
                        snd.detach(&mut self.tree);
                        parent1.checked_prepend(snd, &mut self.tree)?;
                        parent2.checked_prepend(fst, &mut self.tree)?;
                        Ok(())
                    } else {
                        fst.detach(&mut self.tree);
                        snd.detach(&mut self.tree);
                        parent1.checked_prepend(snd, &mut self.tree)?;
                        parent2.checked_append(fst, &mut self.tree)?;
                        Ok(())
                    }
                } else {
                    if parent2.children(&self.tree).next().unwrap() == snd {
                        fst.detach(&mut self.tree);
                        snd.detach(&mut self.tree);
                        parent1.checked_append(snd, &mut self.tree)?;
                        parent2.checked_prepend(fst, &mut self.tree)?;
                        Ok(())
                    } else {
                        fst.detach(&mut self.tree);
                        snd.detach(&mut self.tree);
                        parent1.checked_append(snd, &mut self.tree)?;
                        parent2.checked_append(fst, &mut self.tree)?;
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

#[cfg(test)]
mod test {
    use super::Mux;
    use cursive::views::DummyView;

    #[test]
    fn left_to_right() {
        let mut mux = Mux::new();
        let node1 = mux.add_left_of(DummyView, mux.root).unwrap();
        let node2 = mux.add_left_of(DummyView, node1).unwrap();
        assert!(mux.switch_views(node1, node2).is_ok());
    }

    #[test]
    fn right_to_left() {
        let mut mux = Mux::new();
        let node1 = mux.add_right_of(DummyView, mux.root).unwrap();
        let node2 = mux.add_left_of(DummyView, node1).unwrap();
        assert!(mux.switch_views(node2, node1).is_ok());
    }
}
