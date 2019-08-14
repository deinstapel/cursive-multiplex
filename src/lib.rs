//! # cursive view multiplexer
//!
//! This crate provides a view for the [cursive tui crate](https://github.com/gyscos/cursive).
//! It provides an easier way to display nesting view structures as for example in tmux in cursive.
//! All that has to be done is to insert the view into cursive and later to operate on the reference of it, to add, remove, switch views.
//!
//! Similar to tmux the user is able to resize, and switch between the current views, given they are focusable.
//!
//! # Usage example
//! ```rust
//! extern crate cursive;
//! extern crate cursive_multiplex;
//!
//! use cursive_multiplex::Mux;
//! use cursive::views::TextView;
//! use cursive::Cursive;
//!
//! fn main() {
//!     let (mut mux, node1) = Mux::new(TextView::new("Hello World".to_string()));
//!     let mut siv = Cursive::default();
//!     mux.add_horizontal_id(TextView::new("Hello from me too!".to_string()), node1);
//!     siv.add_fullscreen_layer(mux);
//!
//!     // When your finished setting up
//!     // siv.run();
//! }
//! ```
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate log;

mod actions;
mod error;
mod id;
mod node;
mod path;

use cursive::direction::{Absolute, Direction};
use cursive::event::{Event, EventResult, Key};
use cursive::view::{Selector, View};
use cursive::{Printer, Vec2};
pub use id::Id;
use node::Node;
pub use path::Path;
use std::convert::TryFrom;

#[derive(Debug, PartialEq)]
enum Orientation {
    Vertical,
    Horizontal,
}

#[derive(Debug, PartialEq)]
enum SearchPath {
    Left,
    Right,
    Up,
    Down,
}

/// View holding information and managing multiplexer.
pub struct Mux {
    tree: indextree::Arena<Node>,
    root: indextree::NodeId,
    focus: indextree::NodeId,
    focus_up: Event,
    focus_down: Event,
    focus_left: Event,
    focus_right: Event,
    resize_left: Event,
    resize_right: Event,
    resize_up: Event,
    resize_down: Event,
}

impl View for Mux {
    fn draw(&self, printer: &Printer) {
        debug!("Current Focus: {}", self.focus);
        // println!("Mux currently focused: {}", printer.focused);
        self.rec_draw(printer, self.root)
    }

    fn needs_relayout(&self) -> bool {
        true
    }

    fn required_size(&mut self, constraint: Vec2) -> Vec2 {
        constraint
    }

    fn layout(&mut self, constraint: Vec2) {
        self.rec_layout(self.root, constraint);
    }

    fn take_focus(&mut self, _source: Direction) -> bool {
        true
    }

    fn focus_view(&mut self, _: &Selector) -> Result<(), ()> {
        Ok(())
    }

    fn on_event(&mut self, evt: Event) -> EventResult {
        let result = self
            .tree
            .get_mut(self.focus)
            .unwrap()
            .get_mut()
            .on_event(evt.relativized(Vec2::new(0, 0)));
        match result {
            EventResult::Ignored => match evt {
                _ if self.focus_left == evt => self.move_focus(Absolute::Left),
                _ if self.focus_right == evt => self.move_focus(Absolute::Right),
                _ if self.focus_up == evt => self.move_focus(Absolute::Up),
                _ if self.focus_down == evt => self.move_focus(Absolute::Down),
                _ if self.resize_left == evt => self.resize(Absolute::Left),
                _ if self.resize_right == evt => self.resize(Absolute::Right),
                _ if self.resize_up == evt => self.resize(Absolute::Up),
                _ if self.resize_down == evt => self.resize(Absolute::Down),
                _ => EventResult::Ignored,
            },
            result => result,
        }
    }
}

impl Mux {
    /// Chainable setter for action
    pub fn with_move_focus_up(mut self, evt: Event) -> Self {
        self.focus_up = evt;
        self
    }
    /// Chainable setter for action
    pub fn with_move_focus_down(mut self, evt: Event) -> Self {
        self.focus_down = evt;
        self
    }
    /// Chainable setter for action
    pub fn with_move_focus_left(mut self, evt: Event) -> Self {
        self.focus_left = evt;
        self
    }
    /// Chainable setter for action
    pub fn with_move_focus_right(mut self, evt: Event) -> Self {
        self.focus_right = evt;
        self
    }
    /// Chainable setter for action
    pub fn with_resize_up(mut self, evt: Event) -> Self {
        self.resize_up = evt;
        self
    }
    /// Chainable setter for action
    pub fn with_resize_down(mut self, evt: Event) -> Self {
        self.resize_down = evt;
        self
    }
    /// Chainable setter for action
    pub fn with_resize_left(mut self, evt: Event) -> Self {
        self.resize_left = evt;
        self
    }
    /// Chainable setter for action
    pub fn with_resize_right(mut self, evt: Event) -> Self {
        self.resize_right = evt;
        self
    }

    /// Setter for action
    pub fn set_move_focus_up(&mut self, evt: Event) {
        self.focus_up = evt;
    }
    /// Setter for action
    pub fn set_move_focus_down(&mut self, evt: Event) {
        self.focus_down = evt;
    }
    /// Setter for action
    pub fn set_move_focus_left(&mut self, evt: Event) {
        self.focus_left = evt;
    }
    /// Setter for action
    pub fn set_move_focus_right(&mut self, evt: Event) {
        self.focus_right = evt;
    }
    /// Setter for action
    pub fn set_resize_up(&mut self, evt: Event) {
        self.resize_up = evt;
    }
    /// Setter for action
    pub fn set_resize_down(&mut self, evt: Event) {
        self.resize_down = evt;
    }
    /// Setter for action
    pub fn set_resize_left(&mut self, evt: Event) {
        self.resize_left = evt;
    }
    /// Setter for action
    pub fn set_resize_right(&mut self, evt: Event) {
        self.resize_right = evt;
    }

    /// Chainable setter for the focus the mux should have
    pub fn with_focus(mut self, id: Id) -> Self {
        let nodes: Vec<Id> = self.root.descendants(&self.tree).collect();
        if nodes.contains(&id) {
            self.focus = id;
        }
        self
    }

    /// Setter for the focus the mux should have
    pub fn set_focus(&mut self, id: Id) {
        let nodes: Vec<Id> = self.root.descendants(&self.tree).collect();
        if nodes.contains(&id) {
            self.focus = id;
        }
    }

    /// # Example
    /// ```
    /// # extern crate cursive;
    /// # fn main () {
    /// let (mut mux, node1) = cursive_multiplex::Mux::new(cursive::views::DummyView);
    /// # }
    /// ```
    pub fn new<T>(v: T) -> (Self, Id)
    where
        T: View,
    {
        let mut new_tree = indextree::Arena::new();
        let new_root = new_tree.new_node(Node::new_empty(Orientation::Horizontal));
        let mut new_mux = Mux {
            tree: new_tree,
            root: new_root,
            focus: new_root,
            focus_up: Event::Key(Key::Up),
            focus_down: Event::Key(Key::Down),
            focus_left: Event::Key(Key::Left),
            focus_right: Event::Key(Key::Right),
            resize_left: Event::Ctrl(Key::Left),
            resize_right: Event::Ctrl(Key::Right),
            resize_up: Event::Ctrl(Key::Up),
            resize_down: Event::Ctrl(Key::Down),
        };
        // borked if not succeeding
        let fst_view = new_mux.add_horizontal_id(v, new_root).unwrap();
        (new_mux, fst_view)
    }

    /// Returns the current focused view id.
    /// By default the newest node added to the multiplexer gets focused.
    /// Focus can also be changed by the user.
    /// # Example
    /// ```
    /// # extern crate cursive;
    /// # fn main () {
    /// let (mut mux, node1) = cursive_multiplex::Mux::new(cursive::views::DummyView);
    /// let current_focus = mux.get_focus();
    /// assert_eq!(current_focus, node1);
    /// # }
    /// ```
    pub fn get_focus(&self) -> Id {
        self.focus
    }

    fn rec_layout(&mut self, root: Id, constraint: Vec2) {
        match root.children(&self.tree).count() {
            1 => self.rec_layout(root.children(&self.tree).next().unwrap(), constraint),
            2 => {
                let left = root.children(&self.tree).next().unwrap();
                let right = root.children(&self.tree).last().unwrap();
                let const1;
                let const2;
                let root_data = &self.tree.get(root).unwrap().get();
                match root_data.orientation {
                    Orientation::Horizontal => {
                        const1 = Vec2::new(
                            Mux::add_offset(constraint.x / 2, root_data.split_ratio_offset),
                            constraint.y,
                        );
                        const2 = Vec2::new(
                            Mux::add_offset(constraint.x / 2, -root_data.split_ratio_offset) + 1,
                            constraint.y,
                        );
                        // Precautions have to be taken here as modification of the split is not possible elsewhere
                        if const1.x <= 3 {
                            self.tree
                                .get_mut(root)
                                .unwrap()
                                .get_mut()
                                .split_ratio_offset += 1;
                        } else if const1.x >= constraint.x - 3 {
                            self.tree
                                .get_mut(root)
                                .unwrap()
                                .get_mut()
                                .split_ratio_offset -= 1;
                        }
                    }
                    Orientation::Vertical => {
                        const1 = Vec2::new(
                            constraint.x,
                            Mux::add_offset(constraint.y / 2, root_data.split_ratio_offset),
                        );
                        const2 = Vec2::new(
                            constraint.x,
                            Mux::add_offset(constraint.y / 2, -root_data.split_ratio_offset) + 1,
                        );
                        // Precautions have to be taken here as modification of the split is not possible elsewhere
                        if const1.y <= 3 {
                            self.tree
                                .get_mut(root)
                                .unwrap()
                                .get_mut()
                                .split_ratio_offset += 1;
                        } else if const1.y >= constraint.y - 3 {
                            self.tree
                                .get_mut(root)
                                .unwrap()
                                .get_mut()
                                .split_ratio_offset -= 1;
                        }
                    }
                }
                self.rec_layout(left, const1);
                self.rec_layout(right, const2);
            }
            0 => {
                self.tree
                    .get_mut(root)
                    .unwrap()
                    .get_mut()
                    .layout_view(constraint);
            }
            _ => debug!("Illegal Number of Child Nodes"),
        }
    }

    fn add_offset(split: usize, offset: i16) -> usize {
        if offset < 0 {
            match usize::try_from(offset.abs()) {
                Ok(u) => {
                    if split < u {
                        split
                    } else {
                        split - u
                    }
                }
                Err(_) => split,
            }
        } else {
            match usize::try_from(offset) {
                Ok(u) => split + u,
                Err(_) => split,
            }
        }
    }

    fn rec_draw(&self, printer: &Printer, root: Id) {
        match root.children(&self.tree).count() {
            1 => self.rec_draw(printer, root.children(&self.tree).next().unwrap()),
            2 => {
                debug!("Print Children Nodes");
                let left = root.children(&self.tree).next().unwrap();
                let right = root.children(&self.tree).last().unwrap();
                let printer1;
                let printer2;
                let root_data = &self.tree.get(root).unwrap().get();
                match root_data.orientation {
                    Orientation::Horizontal => {
                        printer1 = printer.cropped(Vec2::new(
                            Mux::add_offset(printer.size.x / 2, root_data.split_ratio_offset),
                            printer.size.y,
                        ));
                        printer2 = printer
                            .offset(Vec2::new(
                                Mux::add_offset(printer.size.x / 2, root_data.split_ratio_offset)
                                    + 1,
                                0,
                            ))
                            .cropped(Vec2::new(
                                Mux::add_offset(printer.size.x / 2, -root_data.split_ratio_offset),
                                printer.size.y,
                            ));
                    }
                    Orientation::Vertical => {
                        printer1 = printer.cropped(Vec2::new(
                            printer.size.x,
                            Mux::add_offset(printer.size.y / 2, root_data.split_ratio_offset),
                        ));
                        printer2 = printer
                            .offset(Vec2::new(
                                0,
                                Mux::add_offset(printer.size.y / 2, root_data.split_ratio_offset)
                                    + 1,
                            ))
                            .cropped(Vec2::new(
                                printer.size.x,
                                Mux::add_offset(printer.size.y / 2, -root_data.split_ratio_offset),
                            ));
                    }
                }
                self.rec_draw(&printer1, left);
                match self.tree.get(root).unwrap().get().orientation {
                    Orientation::Vertical => {
                        if printer.size.y > 1 {
                            printer.print_hline(
                                Vec2::new(
                                    0,
                                    Mux::add_offset(
                                        printer.size.y / 2,
                                        root_data.split_ratio_offset,
                                    ),
                                ),
                                printer.size.x,
                                "─",
                            );
                        }
                    }
                    Orientation::Horizontal => {
                        if printer.size.x > 1 {
                            printer.print_vline(
                                Vec2::new(
                                    Mux::add_offset(
                                        printer.size.x / 2,
                                        root_data.split_ratio_offset,
                                    ),
                                    0,
                                ),
                                printer.size.y,
                                "│",
                            );
                        }
                    }
                }
                self.rec_draw(&printer2, right);
            }
            0 => {
                self.tree
                    .get(root)
                    .unwrap()
                    .get()
                    .draw(&printer.focused(self.focus == root));
            }
            _ => debug!("Illegal Number of Child Nodes"),
        }
    }
}

#[cfg(test)]
mod tree {
    use super::Mux;
    use cursive::event::{Event, Key};
    use cursive::traits::View;
    use cursive::views::DummyView;

    #[test]
    fn test_remove() {
        // General Remove test
        let (mut test_mux, node1) = Mux::new(DummyView);
        let node2 = test_mux.add_vertical_id(DummyView, node1).unwrap();
        let node3 = test_mux.add_vertical_id(DummyView, node2).unwrap();

        print_tree(&test_mux);
        test_mux.remove_id(node3).unwrap();
        print_tree(&test_mux);
        match test_mux.remove_id(node3) {
            Ok(_) => {
                print_tree(&test_mux);
                println!("Delete should have removed: {}", node3);
                assert!(false);
            }
            Err(_) => {}
        }
    }

    #[test]
    fn test_switch() {
        let (mut mux, node1) = Mux::new(DummyView);
        let node2 = mux.add_horizontal_id(DummyView, node1).unwrap();
        let node3 = mux.add_vertical_id(DummyView, node2).unwrap();

        mux.switch_views(node1, node3).unwrap();
    }

    #[test]
    fn test_nesting() {
        println!("Nesting Test");

        let (mut mux, _) = Mux::new(DummyView);

        let mut nodes = Vec::new();

        for _ in 0..10 {
            print_tree(&mux);
            match mux.add_horizontal_id(
                DummyView,
                if let Some(x) = nodes.last() {
                    *x
                } else {
                    mux.root
                },
            ) {
                Ok(node) => {
                    nodes.push(node);
                }
                Err(_) => {
                    assert!(false);
                }
            }
            match mux.add_vertical_id(DummyView, *nodes.last().unwrap()) {
                Ok(node) => {
                    nodes.push(node);
                }
                Err(_) => {
                    assert!(false);
                }
            }
        }

        for node in nodes.iter() {
            mux.focus = *node;
            direction_test(&mut mux);
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
