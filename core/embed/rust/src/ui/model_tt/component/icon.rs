use crate::ui::{
    component::{Component, ComponentExt, Event, EventCtx, GridPlaced, Map},
    display::{self, Color, Font},
    event::TouchEvent,
    geometry::{Insets, Offset, Rect},
};
use crate::ui::model_tt::theme::{BG, FG};

use super::theme;

pub enum IconMsg {
    Touched,
}

pub struct Icon {
    area: Rect,
    content: &'static [u8],
    background: Color,
    foreground: Color,
}

impl Icon{

    pub fn new(content: &'static [u8], ) -> Self {
        Self {
            area: Rect::zero(),
            content,
            background: FG,
            foreground: Color::rgb(0x99, 0x99, 0x99),
        }
    }


    pub fn area(&self) -> Rect {
        self.area
    }

    pub fn paint_background(&self) {
        // Paint the border and a smaller background on top of it.
        display::rect_fill(
            self.area,
            self.background,
        );

    }

    pub fn paint_content(&self)
    {
        display::icon(
            self.area.center(),
            self.content,
            self.foreground,
            self.background,
        );

    }
}

impl Component for Icon
{
    type Msg = IconMsg;

    fn place(&mut self, bounds: Rect) -> Rect {
        self.area = bounds;
        self.area
    }

    fn event(&mut self, ctx: &mut EventCtx, event: Event) -> Option<Self::Msg> {
        match event {
            Event::Touch(TouchEvent::TouchStart(pos)) => {
                // Touch started in our area, transform to `Pressed` state.
                if self.area.contains(pos) {
                    return Some(IconMsg::Touched);
                }
            },
            _ => {}
        };
        None
    }

    fn paint(&mut self) {
        self.paint_background();
        self.paint_content();
    }

    fn bounds(&self, sink: &mut dyn FnMut(Rect)) {
        sink(self.area);
    }
}
