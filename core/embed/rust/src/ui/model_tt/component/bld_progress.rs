use crate::ui::{component::{Component, Event, EventCtx}, display, geometry::Rect};
use crate::ui::geometry::Point;
use crate::ui::model_tt::component::{BootloaderFrame};
use crate::ui::model_tt::theme::{BLD_BG, BLD_FG, RECEIVE};
use super::{theme};
use super::super::constant::{HEIGHT, WIDTH};


pub enum BldProgressMsg {
    None
}

pub struct BldProgress {
    text: &'static str,
    icon: &'static [u8],
    progress: u16,
}



impl BldProgress
{
    pub fn new(text: &'static str) -> Self {
        Self {
            text,
            icon: RECEIVE,
            progress: 200
        }
    }

    pub fn set_progres(&mut self, progress: u16) {
        self.progress = progress;
    }
}



impl Component for BldProgress
{

    type Msg = BldProgressMsg;

    fn place(&mut self, bounds: Rect) -> Rect {
        bounds
    }

    fn event(&mut self, _ctx: &mut EventCtx, _event: Event) -> Option<Self::Msg> {
        None
    }

    fn paint(&mut self) {
        display::rect_fill(Rect::new (Point::new(0,0), Point::new(WIDTH, HEIGHT)), BLD_BG);
        display::text_center(Point::new(WIDTH/2,214), self.text, theme::FONT_MEDIUM, BLD_FG, BLD_BG);
        self.repaint();
    }

    fn bounds(&self, _sink: &mut dyn FnMut(Rect)) {

    }
}

impl BootloaderFrame for BldProgress {

    fn repaint(&mut self) {
        display::loader(
            self.progress,
            0,
            BLD_FG,
            BLD_BG,
            Some((self.icon, BLD_FG)),
        );
    }
    fn messages(&mut self, _msg: <Self as Component>::Msg) -> Option<u32> where Self: Component{
        None
    }
}