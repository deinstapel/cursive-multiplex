use crate::id::Id;
use crate::path::SearchPath;
use crate::{Absolute, EventResult, Mux, Orientation};

impl Mux {
    pub(crate) fn move_focus(&mut self, direction: Absolute) -> EventResult {
        match self.search_focus_path(
            direction,
            self.focus.ancestors(&self.tree).nth(1).unwrap(),
            self.focus,
        ) {
            Ok((path, turn_point)) => {
                // Traverse the path down again
                if let Some(focus) = self.traverse_search_path(path, turn_point) {
                    if self.tree.get_mut(focus).unwrap().get_mut().take_focus() {
                        self.focus = focus;
                        EventResult::Consumed(None)
                    } else {
                        debug!("Focus rejected by {}", focus);
                        EventResult::Ignored
                    }
                } else {
                    EventResult::Ignored
                }
            }
            Err(_) => EventResult::Ignored,
        }
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
        match self.tree.get(turn_point).unwrap().get().orientation {
            Orientation::Horizontal => {
                match action {
                    // Switching Sides for Left & Right
                    SearchPath::Right
                        if self.tree.get(cur_node).unwrap().get().orientation
                            == Orientation::Horizontal =>
                    {
                        left()
                    }
                    SearchPath::Left
                        if self.tree.get(cur_node).unwrap().get().orientation
                            == Orientation::Horizontal =>
                    {
                        right()
                    }
                    // Remain for Up & Down
                    SearchPath::Up
                        if self.tree.get(cur_node).unwrap().get().orientation
                            == Orientation::Vertical =>
                    {
                        up()
                    }
                    SearchPath::Down
                        if self.tree.get(cur_node).unwrap().get().orientation
                            == Orientation::Vertical =>
                    {
                        down()
                    }
                    _ => None,
                }
            }
            Orientation::Vertical => {
                match action {
                    // Remain for Left & Right
                    SearchPath::Right
                        if self.tree.get(cur_node).unwrap().get().orientation
                            == Orientation::Horizontal =>
                    {
                        right()
                    }
                    SearchPath::Left
                        if self.tree.get(cur_node).unwrap().get().orientation
                            == Orientation::Horizontal =>
                    {
                        left()
                    }
                    // Switch for Up & Down
                    SearchPath::Up
                        if self.tree.get(cur_node).unwrap().get().orientation
                            == Orientation::Vertical =>
                    {
                        down()
                    }
                    SearchPath::Down
                        if self.tree.get(cur_node).unwrap().get().orientation
                            == Orientation::Vertical =>
                    {
                        up()
                    }
                    _ => None,
                }
            }
        }
    }

    fn traverse_search_path(&self, mut path: Vec<SearchPath>, turn_point: Id) -> Option<Id> {
        let mut cur_node = turn_point;
        // println!("Path Begin: {:?}", path);
        while let Some(step) = path.pop() {
            // println!("Next Step: {:?}", step);
            match self.traverse_single_node(step, turn_point, cur_node) {
                Some(node) => {
                    // println!("{}", node);
                    cur_node = node;
                }
                None => {
                    // Truncate remaining path
                    // cur_node = cur_node.children(&self.tree).next().unwrap();
                    break;
                }
            }
        }
        while !self.tree.get(cur_node).unwrap().get().has_view() {
            match cur_node.children(&self.tree).next() {
                Some(node) => cur_node = node,
                None => return None,
            }
        }
        Some(cur_node)
    }

    fn search_focus_path(
        &self,
        direction: Absolute,
        nodeid: Id,
        fromid: Id,
    ) -> Result<(Vec<SearchPath>, Id), ()> {
        let mut cur_node = Some(nodeid);
        let mut from_node = fromid;
        let mut path = Vec::new();
        while cur_node.is_some() {
            // println!("Current node in search path: {}", cur_node.unwrap());
            // println!("Originating from node: {}", from_node);
            match self.tree.get(cur_node.unwrap()).unwrap().get().orientation {
                Orientation::Horizontal
                    if direction == Absolute::Left || direction == Absolute::Right =>
                {
                    if cur_node.unwrap().children(&self.tree).next().unwrap() == from_node {
                        // Originated from left
                        path.push(SearchPath::Left);
                        from_node = cur_node.unwrap();
                        if direction == Absolute::Left {
                            cur_node = cur_node.unwrap().ancestors(&self.tree).nth(1);
                        } else {
                            cur_node = None;
                        }
                    } else {
                        // Originated from right
                        path.push(SearchPath::Right);
                        from_node = cur_node.unwrap();
                        if direction == Absolute::Right {
                            cur_node = cur_node.unwrap().ancestors(&self.tree).nth(1);
                        } else {
                            cur_node = None;
                        }
                    }
                }
                Orientation::Vertical
                    if direction == Absolute::Up || direction == Absolute::Down =>
                {
                    if cur_node.unwrap().children(&self.tree).next().unwrap() == from_node {
                        // Originated from up
                        path.push(SearchPath::Up);
                        from_node = cur_node.unwrap();
                        if direction == Absolute::Up {
                            cur_node = cur_node.unwrap().ancestors(&self.tree).nth(1);
                        } else {
                            cur_node = None;
                        }
                    } else {
                        // Originated from down
                        path.push(SearchPath::Down);
                        from_node = cur_node.unwrap();
                        if direction == Absolute::Down {
                            cur_node = cur_node.unwrap().ancestors(&self.tree).nth(1);
                        } else {
                            cur_node = None;
                        }
                    }
                }
                Orientation::Horizontal => {
                    if cur_node.unwrap().children(&self.tree).next().unwrap() == from_node {
                        path.push(SearchPath::Left);
                        from_node = cur_node.unwrap();
                        cur_node = cur_node.unwrap().ancestors(&self.tree).nth(1);
                    } else {
                        path.push(SearchPath::Right);
                        from_node = cur_node.unwrap();
                        cur_node = cur_node.unwrap().ancestors(&self.tree).nth(1);
                    }
                }
                Orientation::Vertical => {
                    if cur_node.unwrap().children(&self.tree).next().unwrap() == from_node {
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
        match self.tree.get(from_node).unwrap().get().orientation {
            Orientation::Horizontal if direction == Absolute::Up || direction == Absolute::Down => {
                Err(())
            }
            Orientation::Vertical
                if direction == Absolute::Left || direction == Absolute::Right =>
            {
                Err(())
            }
            _ => Ok((path, from_node)),
        }
    }

    pub(crate) fn resize(&mut self, direction: Absolute) -> EventResult {
        let orit = {
            match direction {
                Absolute::Left | Absolute::Right => Orientation::Horizontal,
                Absolute::Up | Absolute::Down => Orientation::Vertical,
                _ => Orientation::Horizontal,
            }
        };
        let mut parent = self.focus.ancestors(&self.tree).nth(1);
        while parent.is_some() {
            if let Some(view) = self.tree.get_mut(parent.unwrap()) {
                if view.get().orientation == orit {
                    match direction {
                        Absolute::Left | Absolute::Up => {
                            view.get_mut().split_ratio_offset -= 1;
                            return EventResult::Consumed(None);
                        }
                        Absolute::Right | Absolute::Down => {
                            view.get_mut().split_ratio_offset += 1;
                            return EventResult::Consumed(None);
                        }
                        _ => {}
                    }
                } else {
                    parent = parent.unwrap().ancestors(&self.tree).nth(1);
                }
            }
        }
        EventResult::Ignored
    }
}