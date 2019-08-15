use crate::{Id, Mux, Orientation};

/// Path used to get the id of a specific pane in the mux.
/// self can be directed by calling `.up()`, `.down()`, `.left()` and `.right()` on the instance.
/// To get the final id of a pane `.build()`.
pub struct AwesomePath<'a> {
    mux: &'a Mux,
    cur_id: Option<Id>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum SearchPath {
    Left,
    Right,
    Up,
    Down,
}

impl<'a> AwesomePath<'a> {
    fn new(mux: &'a Mux,id: Id) -> Self {
        AwesomePath {
            mux,
            cur_id: Some(id),
        }
    }

    /// Finsihing of the path, Option contains the target Id
    /// If Option None no Id could be found fitting to the path
    /// Consumes the path
    /// # Example
    /// ```
    /// # use cursive::views::DummyView;
    /// # use cursive_multiplex::Mux;
    /// # fn main() {
    /// let (mut mux, node1) = Mux::new(DummyView);
    /// mux.add_below(DummyView, node1);
    /// let path = mux.root().up().build();
    /// assert_eq!(node1, path.unwrap());
    /// # }
    /// ```
    pub fn build(self) -> Option<Id> {
        if let Some(node) = self.cur_id {
            if self.mux.tree.get(node).unwrap().get().has_view() {
                self.cur_id
            } else {
                None
            }
        } else {
            self.cur_id
        }
    }

    /// Going up from the current position in the mux
    /// Target can be get by calling `.build()`
    pub fn up(self) -> Self {
        self.next_node(SearchPath::Up, Orientation::Vertical)
    }

    /// Going down from the current position in the mux
    /// Target can be get by calling `.build()`
    pub fn down(self) -> Self {
        self.next_node(SearchPath::Down, Orientation::Vertical)
    }

    /// Going left from the current position in the mux
    /// Target can be get by calling `.build()`
    pub fn left(self) -> Self {
        self.next_node(SearchPath::Left, Orientation::Horizontal)
    }

    /// Going right from the current position in the mux
    /// Target can be get by calling `.build()`
    pub fn right(self) -> Self {
        self.next_node(SearchPath::Right, Orientation::Horizontal)
    }

    fn next_node(mut self, direction: SearchPath, orit: Orientation) -> Self {
        if let Some(node) = self.cur_id {
            // Node can be passed
            if node.children(&self.mux.tree).count() > 0 {
                if let Some(node_content) = self.mux.tree.get(node) {
                    match node_content.get().orientation {
                        _ if node_content.get().orientation == orit => {
                            if let Some(new) = node.children(&self.mux.tree).nth(
                                match direction {
                                    SearchPath::Up | SearchPath::Left => 0,
                                    SearchPath::Right | SearchPath::Down => 1,
                                }
                            ) {
                                self.cur_id = Some(new);
                            } else {
                                // Invalid Path
                                self.cur_id = None;
                            }
                        }
                        _ => {
                            // Invalid Path
                            println!("ello");
                            self.cur_id = None;
                        },
                    }
                } else {
                    // State corrupted, should not occur
                    self.cur_id = None;
                }
            }
        }
        self
    }

}

impl Mux {

    /// Getter for the initial path to traverse the tree and find a specific Id.
    /// Returns a Path which can be traversed.
    pub fn root(&self) -> AwesomePath {
        AwesomePath::new(self,self.root)
    }

}

#[cfg(test)]
mod test {
    use super::Mux;
    use cursive::views::DummyView;

    #[test]
    fn path_up() {
        let (mut mux, node1) = Mux::new(DummyView);
        mux.add_below(DummyView, node1).unwrap();
        let upper_pane = mux.root().up().build();
        assert!(upper_pane.is_some());
        assert_eq!(node1, upper_pane.unwrap());
    }

    #[test]
    fn path_down() {
        let (mut mux, node1) = Mux::new(DummyView);
        let node2 = mux.add_below(DummyView, node1).unwrap();
        let lower_pane = mux.root().down().build();
        assert!(lower_pane.is_some());
        assert_eq!(node2, lower_pane.unwrap());
    }

    #[test]
    fn path_left() {
        let (mut mux, node1) = Mux::new(DummyView);
        mux.add_right_of(DummyView, node1).unwrap();
        let left_pane = mux.root().left().build();
        assert!(left_pane.is_some());
        assert_eq!(node1, left_pane.unwrap());
    }

    #[test]
    fn path_right() {
        let (mut mux, node1) = Mux::new(DummyView);
        let node2 = mux.add_right_of(DummyView, node1).unwrap();
        let right_pane = mux.root().right().build();
        assert!(right_pane.is_some());
        assert_eq!(node2, right_pane.unwrap());
    }

    #[test]
    fn path_invalid() {
        let (mut mux, node1) = Mux::new(DummyView);
        let _ = mux.add_right_of(DummyView, node1).unwrap();
        let root_pane = mux.root().build();
        assert!(root_pane.is_none());
    }

}
