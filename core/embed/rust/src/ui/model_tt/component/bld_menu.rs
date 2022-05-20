use crate::ui::{component::{Child, Component, Event, EventCtx}, display, geometry::Rect};
use crate::ui::geometry::Point;
use crate::ui::model_tt::component::{BootloaderFrame};
use crate::ui::model_tt::component::button::IconText;
use crate::ui::model_tt::theme::{BLD_BG, button_bld_menu, button_bld_menu_item, BLD_TITLE_COLOR, REBOOT, FWINFO, RESET, CLOSE};
use crate::ui::model_tt::component::ButtonMsg::{Clicked};
use super::{Button, theme};
use super::super::constant::{HEIGHT, WIDTH};



pub struct TTBootloaderText2;


pub enum BldMenuMsg<M>  {
    Close(M),
    Reboot(M),
    FwInfo(M),
    FactoryReset(M),
}

pub struct BldMenu {
    close: Child<Button<&'static str>>,
    reboot: Child<Button<&'static str>>,
    fwinfo: Child<Button<&'static str>>,
    reset: Child<Button<&'static str>>,
}



impl BldMenu
{
    pub fn new() -> Self {

        let content_reboot = IconText::new("REBOOT", REBOOT, 46, 25);
        let content_fwinfo = IconText::new("FW INFO", FWINFO, 46, 25);
        let content_reset = IconText::new("FACTORY RESET", RESET, 46, 25);

        Self {
            close: Child::new(Button::with_icon(CLOSE).styled(button_bld_menu())),
            reboot: Child::new(Button::with_icon_and_text(content_reboot).styled(button_bld_menu_item())),
            fwinfo: Child::new(Button::with_icon_and_text(content_fwinfo).styled(button_bld_menu_item())),
            reset: Child::new(Button::with_icon_and_text(content_reset).styled(button_bld_menu_item())),
        }
    }
}



impl Component for BldMenu
{

    type Msg = BldMenuMsg<
        <Button<&'static str> as Component>::Msg>;

    fn place(&mut self, bounds: Rect) -> Rect {
        self.close.place(Rect::new (Point::new(187,15), Point::new( 187+38,15+38)));
        self.reboot.place(Rect::new (Point::new(16,66), Point::new( 16+209,66+48)));
        self.fwinfo.place(Rect::new (Point::new(16,122), Point::new( 16+209,122+48)));
        self.reset.place(Rect::new (Point::new(16,178), Point::new( 16+209,178+48)));
        bounds
    }

    fn event(&mut self, ctx: &mut EventCtx, event: Event) -> Option<Self::Msg> {
        self.close.event(ctx, event).map(Self::Msg::Close)
            .or_else(|| self.reboot.event(ctx, event).map(Self::Msg::Reboot))
            .or_else(|| self.fwinfo.event(ctx, event).map(Self::Msg::FwInfo))
            .or_else(|| self.reset.event(ctx, event).map(Self::Msg::FactoryReset))
    }

    fn paint(&mut self) {
        display::rect_fill(Rect::new (Point::new(0,0), Point::new(WIDTH, HEIGHT)), BLD_BG);
        display::text_top_left(Point::new(15,24), "BOOTLOADER", theme::FONT_BOLD, BLD_TITLE_COLOR, BLD_BG);
        self.repaint();
    }

    fn bounds(&self, sink: &mut dyn FnMut(Rect)) {
        self.close.bounds(sink);
        self.reboot.bounds(sink);
        self.fwinfo.bounds(sink);
        self.reset.bounds(sink);
    }
}

impl BootloaderFrame for BldMenu {

    fn repaint(&mut self) {
        self.close.paint();
        self.reboot.paint();
        self.fwinfo.paint();
        self.reset.paint();
    }
    fn messages(&mut self, msg: <Self as Component>::Msg) -> Option<u32> where Self: Component{
        let result = match msg {
            BldMenuMsg::Close(Clicked) => {return Some(1)},
            BldMenuMsg::Reboot(Clicked) => {return Some(2)},
            BldMenuMsg::FactoryReset(Clicked) => {return Some(3)},
            _ => {None}
        };
        result
    }
}