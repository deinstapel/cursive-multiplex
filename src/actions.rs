use crate::id::Id;
use crate::path::SearchPath;
use crate::{Absolute, EventResult, Mux, Orientation, Vec2};

impl Mux {
    // Handler for mouse events
    pub(crate) fn clicked_pane(&self, mp: Vec2) -> Option<Id> {
        if self.zoomed {
            return None;
        }
        for node in self.root.descendants(&self.tree) {
            if self.tree.get(node).unwrap().get().click(mp) {
                return Some(node);
            }
        }
        None
    }

    pub(crate) fn zoom_focus(&mut self) -> EventResult {
        self.zoomed = !self.zoomed;
        self.invalidated = true;
        EventResult::Consumed(None)
    }

    pub(crate) fn move_focus(&mut self, direction: Absolute) -> EventResult {
        if self.zoomed {
            return EventResult::Ignored;
        }
        let prev_move = self.focus;
        match self.move_focus_relative(direction, self.focus, self.focus) {
            EventResult::Consumed(any) => {
                self.history.push_back((prev_move, self.focus, direction));
                if self.history.len() > self.history_length {
                    self.history.pop_front();
                }
                EventResult::Consumed(any)
            }
            EventResult::Ignored => EventResult::Ignored,
        }
    }

    fn move_focus_relative(&mut self, direction: Absolute, node: Id, origin: Id) -> EventResult {
        match self.search_focus_path(direction, node.ancestors(&self.tree).nth(1).unwrap(), node) {
            Ok((path, turn_point)) => {
                // Traverse the path down again
                if let Some(focus) = self.traverse_search_path(path, turn_point, direction, origin)
                {
                    if self.tree.get_mut(focus).unwrap().get_mut().take_focus() {
                        self.focus = focus;
                        EventResult::Consumed(None)
                    } else {
                        // rejected
                        self.move_focus_relative(direction, focus, origin)
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
            cur_node.children(&self.tree).next()
        };
        let right = || -> Option<Id> {
            cur_node.children(&self.tree).last()
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

    fn traverse_search_path(
        &self,
        mut path: Vec<SearchPath>,
        turn_point: Id,
        direction: Absolute,
        origin: Id,
    ) -> Option<Id> {
        let mut cur_node = turn_point;
        while let Some(step) = path.pop() {
            match self.traverse_single_node(step, turn_point, cur_node) {
                Some(node) => {
                    cur_node = node;
                }
                None => {
                    // Truncate remaining path
                    // cur_node = cur_node.children(&self.tree).next().unwrap();
                    break;
                }
            }
        }

        let check = |comp: Absolute, cur_node: &mut Id| -> Result<(), ()> {
            if direction == comp {
                match cur_node.children(&self.tree).last() {
                    Some(node) => {
                        *cur_node = node;
                        Ok(())
                    }
                    None => Err(()),
                }
            } else {
                match cur_node.children(&self.tree).next() {
                    Some(node) => {
                        *cur_node = node;
                        Ok(())
                    }
                    None => Err(()),
                }
            }
        };

        // Check if values exist in the history that specify this path
        let goal_opt = {
            let history = self.history.iter().rev();
            for entry in history {
                match entry {
                    (goal, past_origin, past_direction)
                        if *past_direction == direction.invert() && origin == *past_origin =>
                    {
                        return Some(*goal);
                    }
                    _ => {}
                }
            }
            None
        };

        if let Some(goal) = goal_opt {
            return Some(goal);
        }

        // Have to find nearest child here in case path is too short
        while !self.tree.get(cur_node).unwrap().get().has_view() {
            match self.tree.get(cur_node).unwrap().get().orientation {
                Orientation::Horizontal
                    if direction == Absolute::Left || direction == Absolute::Right =>
                {
                    if check(Absolute::Left, &mut cur_node).is_err() {
                        return None;
                    }
                }
                Orientation::Vertical
                    if direction == Absolute::Up || direction == Absolute::Down =>
                {
                    if check(Absolute::Up, &mut cur_node).is_err() {
                        return None;
                    }
                }
                _ => match cur_node.children(&self.tree).next() {
                    Some(node) => cur_node = node,
                    None => return None,
                },
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
                            if cur_node.is_none() {
                                return Err(());
                            }
                        } else {
                            cur_node = None;
                        }
                    } else {
                        // Originated from right
                        path.push(SearchPath::Right);
                        from_node = cur_node.unwrap();
                        if direction == Absolute::Right {
                            cur_node = cur_node.unwrap().ancestors(&self.tree).nth(1);
                            if cur_node.is_none() {
                                return Err(());
                            }
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
                            if cur_node.is_none() {
                                return Err(());
                            }
                        } else {
                            cur_node = None;
                        }
                    } else {
                        // Originated from down
                        path.push(SearchPath::Down);
                        from_node = cur_node.unwrap();
                        if direction == Absolute::Down {
                            cur_node = cur_node.unwrap().ancestors(&self.tree).nth(1);
                            if cur_node.is_none() {
                                return Err(());
                            }
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
        // TODO: Do not let children be resized to a lower amount then they said they could be
        if self.zoomed {
            return EventResult::Ignored;
        }
        let mut parent = self.focus.ancestors(&self.tree).nth(1);
        while parent.is_some() {
            if let Some(view) = self.tree.get_mut(parent.unwrap()) {
                if view.get().orientation == direction.into() {
                    match view.get_mut().move_offset(direction) {
                        Ok(()) => {
                            self.invalidated = true;
                            return EventResult::Consumed(None);
                        }
                        Err(_) => break,
                    }
                } else {
                    parent = parent.unwrap().ancestors(&self.tree).nth(1);
                }
            }
        }
        EventResult::Ignored
    }
}

impl std::convert::From<Absolute> for Orientation {
    fn from(direction: Absolute) -> Orientation {
        match direction {
            Absolute::Up | Absolute::Down => Orientation::Vertical,
            Absolute::Left | Absolute::Right => Orientation::Horizontal,
            // If no direction default to Horizontal
            Absolute::None => Orientation::Horizontal,
        }
    }
}

trait Invertable {
    fn invert(&self) -> Self;
}

impl Invertable for Absolute {
    fn invert(&self) -> Absolute {
        match self {
            Absolute::Right => Absolute::Left,
            Absolute::Left => Absolute::Right,
            Absolute::Up => Absolute::Down,
            Absolute::Down => Absolute::Up,
            Absolute::None => Absolute::None,
        }
    }
}
