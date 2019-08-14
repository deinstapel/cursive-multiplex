use crate::{Direction, Event, EventResult, Orientation, Printer, Vec2, View};

pub(crate) struct Node {
    pub(crate) view: Option<Box<dyn View>>,
    pub(crate) orientation: Orientation,
    pub(crate) split_ratio_offset: i16,
    size: Option<Vec2>,
}

impl Node {
    pub(crate) fn new<T>(v: T, orit: Orientation) -> Self
    where
        T: View,
    {
        Self {
            view: Some(Box::new(v)),
            orientation: orit,
            split_ratio_offset: 0,
            size: None,
        }
    }

    pub(crate) fn new_empty(orit: Orientation) -> Self {
        Self {
            view: None,
            orientation: orit,
            split_ratio_offset: 0,
            size: None,
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
            let size = Vec2::min(vec, x.required_size(vec));
            self.size = Some(size);
            x.layout(size);
        }
    }

    pub(crate) fn on_event(&mut self, evt: Event) -> EventResult {
        if let Some(view) = self.view.as_mut() {
            view.on_event(evt)
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
}
