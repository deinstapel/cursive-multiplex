use crate::{Direction, Event, EventResult, Orientation, Printer, Vec2, View, AnyCb, Selector};

pub(crate) struct Node {
    pub(crate) view: Option<Box<dyn View + Send>>,
    pub(crate) orientation: Orientation,
    pub(crate) split_ratio_offset: i16,
    total_position: Option<Vec2>,
    size: Option<Vec2>,
    total_size: Option<Vec2>,
}

impl Node {
    pub(crate) fn new<T>(v: T, orit: Orientation) -> Self
    where
        T: View + Send,
    {
        Self {
            view: Some(Box::new(v)),
            orientation: orit,
            split_ratio_offset: 0,
            total_position: None,
            size: None,
            total_size: None,
        }
    }

    pub(crate) fn click(&self, mp: Vec2) -> bool {
        if let Some(pos) = self.total_position {
            if let Some(total_size) = self.total_size {
                let end_pos = pos + total_size;
                if !pos.fits(mp) && end_pos.fits(mp) {
                    return true;
                }
            }
        }
        false
    }

    pub(crate) fn new_empty(orit: Orientation) -> Self {
        Self {
            view: None,
            orientation: orit,
            split_ratio_offset: 0,
            total_position: None,
            size: None,
            total_size: None,
        }
    }

    pub(crate) fn set_pos(&mut self, pos: Vec2) {
        if let Some(_) = self.view {
            self.total_position = Some(pos);
        }
    }

    pub(crate) fn has_view(&self) -> bool {
        match self.view {
            Some(_) => true,
            None => false,
        }
    }

    pub(crate) fn layout_view(&mut self, vec: Vec2) {
        if let Some(x) = self.view.as_mut() {
            self.total_size = Some(vec);
            let size = Vec2::min(vec, x.required_size(vec));
            self.size = Some(x.required_size(vec));
            x.layout(size);
        }
    }

    pub(crate) fn on_event(&mut self, evt: Event) -> EventResult {
        if let Some(view) = self.view.as_mut() {
            view.on_event(evt.relativized(match self.total_position {
                Some(vec) => vec,
                None => Vec2::new(0, 0),
            }))
        } else {
            EventResult::Ignored
        }
    }

    pub(crate) fn draw(&self, printer: &Printer) {
        match self.view {
            Some(ref view) => {
                let printer_crop = {
                    if let Some(size) = self.size {
                        // cropped_centered is bugged here, panics on valid values
                        printer.cropped(size)
                    } else {
                        printer.clone()
                    }
                };
                view.draw(&printer_crop);
            }
            None => {}
        }
    }

    pub(crate) fn take_focus(&mut self) -> bool {
        if let Some(view) = self.view.as_mut() {
            view.take_focus(Direction::none())
        } else {
            false
        }
    }

    pub(crate) fn call_on_any<'a>(&mut self, slct: &Selector, cb: AnyCb<'a>) {
        if let Some(view) = self.view.as_mut() {
            view.call_on_any(slct, cb);
        }
    }
}
