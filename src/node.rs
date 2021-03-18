use crate::{AnyCb, Direction, Event, EventResult, Orientation, Printer, Selector, Vec2, View};
use cursive_core::direction::Absolute;
use std::convert::TryFrom;
use crate::error::RenderError;

pub(crate) struct Node {
    pub(crate) view: Option<Box<dyn View>>,
    pub(crate) orientation: Orientation,
    pub(crate) split_ratio_offset: i16,
    pub(crate) split_ratio: f32,
    total_position: Option<Vec2>,
    size: Option<Vec2>,
    pub(crate) total_size: Option<Vec2>,
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
            split_ratio: 0.5,
            total_position: None,
            size: None,
            total_size: None

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

    pub(crate) fn move_offset(&mut self, direction: Absolute) -> Result<(), RenderError> {
        if let Some(total_size) = self.total_size {
            match direction {
                Absolute::Left | Absolute::Up => match direction.into() {
                    Orientation::Horizontal => {
                        if (total_size.x as f32 * self.split_ratio) as i16 - self.split_ratio_offset.abs()
                            > 1
                            || self.split_ratio_offset > 0
                        {
                            self.split_ratio_offset -= 1;
                            Ok(())
                        } else {
                            Err(RenderError::Arithmetic{})
                        }
                    }
                    Orientation::Vertical => {
                        if (total_size.y as f32 * self.split_ratio) as i16 - self.split_ratio_offset.abs()
                            > 1
                            || self.split_ratio_offset > 0
                        {
                            self.split_ratio_offset -= 1;
                            Ok(())
                        } else {
                            Err(RenderError::Arithmetic{})
                        }
                    }
                },
                Absolute::Right | Absolute::Down => match direction.into() {
                    Orientation::Horizontal => {
                        if (total_size.x as f32 * (1.0 - self.split_ratio)) as i16 - self.split_ratio_offset.abs()
                            > 1
                            || self.split_ratio_offset < 0
                        {
                            self.split_ratio_offset += 1;
                            Ok(())
                        } else {
                            Err(RenderError::Arithmetic{})
                        }
                    }
                    Orientation::Vertical => {
                        if (total_size.y as f32 * (1.0 - self.split_ratio)) as i16 - self.split_ratio_offset.abs()
                            > 1
                            || self.split_ratio_offset < 0
                        {
                            self.split_ratio_offset += 1;
                            Ok(())
                        } else {
                            Err(RenderError::Arithmetic{})
                        }
                    }
                },
                _ => Err(RenderError::Arithmetic{}),
            }
        } else {
            Err(RenderError::Arithmetic{})
        }
    }

    pub(crate) fn new_empty(orit: Orientation, split: f32) -> Self {
        Self {
            view: None,
            orientation: orit,
            split_ratio_offset: 0,
            split_ratio: split,
            total_position: None,
            size: None,
            total_size: None,
        }
    }

    pub(crate) fn set_pos(&mut self, pos: Vec2) {
        if self.view.is_some() {
            self.total_position = Some(pos);
        }
    }

    pub(crate) fn has_view(&self) -> bool {
        self.view.is_some()
    }

    pub(crate) fn layout_view(&mut self, vec: Vec2) {
        if let Some(x) = self.view.as_mut() {
            let size = Vec2::min(vec, x.required_size(vec));
            self.size = Some(x.required_size(vec));
            x.layout(size);
        }
        self.total_size = Some(vec);
    }

    pub(crate) fn on_event(&mut self, evt: Event, zoomed: bool) -> EventResult {
        if let Some(view) = self.view.as_mut() {
            view.on_event(evt.relativized(if zoomed {
                Vec2::new(0, 0)
            } else {
                self.total_position.unwrap_or_else(|| Vec2::new(0, 0))
            }))
        } else {
            EventResult::Ignored
        }
    }

    pub(crate) fn draw(&self, printer: &Printer) {
        if let Some(ref view) = self.view {
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
