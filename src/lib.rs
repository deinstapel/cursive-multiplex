extern crate cursive;
extern crate indextree;

use cursive::views::*;

enum Path {
    LeftOrUp(Box<Path>),
    RightOrDown(Box<Path>),
}

struct Mux {
    tree: indextree::Arena<Node>,
}

struct Node {
    v: Box<dyn cursive::view::View>,
}

impl Mux {

    pub fn new() -> Self {
        let new = Mux{
            tree: indextree::Arena::new(),
        };

        let layout = LinearLayout::vertical();
        let id_view = IdView::new("root", layout);
        let canvas = Canvas::wrap(id_view);
        new
    }
}
