use crate::ui::{component::{Child, Component, Event, EventCtx}, display, geometry::Rect};
use crate::ui::component::Pad;
use crate::ui::geometry::Point;
use crate::ui::model_tt::bootloader::ReturnToC;
use crate::ui::model_tt::theme::{FONT_BOLD};
use crate::ui::model_tt::bootloader::theme::{BLD_BG, button_bld_menu, button_bld_menu_item, BLD_TITLE_COLOR, REBOOT, FWINFO, RESET, CLOSE};
use crate::ui::model_tt::component::ButtonMsg::{Clicked};
use crate::ui::model_tt::component::{Button, IconText};
use crate::ui::model_tt::constant::{HEIGHT, WIDTH};


#[repr(u32)]
#[derive(Copy, Clone)]
pub enum MenuMsg {
    Close = 1,
    Reboot = 2,
    FactoryReset = 3,
    FwInfo = 4,
}
impl ReturnToC for MenuMsg {
    fn return_to_c(&self) -> u32 { *self as u32 }
}

pub struct Menu {
    bg: Pad,
    close: Child<Button<&'static str>>,
    reboot: Child<Button<&'static str>>,
    fwinfo: Child<Button<&'static str>>,
    reset: Child<Button<&'static str>>,
}


impl Menu
{
    pub fn new() -> Self {

        let content_reboot = IconText::new("REBOOT", REBOOT, 46, 25);
        let content_fwinfo = IconText::new("FW INFO", FWINFO, 46, 25);
        let content_reset = IconText::new("FACTORY RESET", RESET, 46, 25);

        let mut instance = Self {
            bg: Pad::with_background(BLD_BG),
            close: Child::new(Button::with_icon(CLOSE).styled(button_bld_menu())),
            reboot: Child::new(Button::with_icon_and_text(content_reboot).styled(button_bld_menu_item())),
            fwinfo: Child::new(Button::with_icon_and_text(content_fwinfo).styled(button_bld_menu_item())),
            reset: Child::new(Button::with_icon_and_text(content_reset).styled(button_bld_menu_item())),
        };
        instance.bg.clear();
        instance
    }
}


impl Component for Menu
{

    type Msg = MenuMsg;

    fn place(&mut self, bounds: Rect) -> Rect {
        self.bg.place(Rect::new (Point::new(0,0), Point::new(WIDTH, HEIGHT)));
        self.close.place(Rect::new (Point::new(187,15), Point::new( 187+38,15+38)));
        self.reboot.place(Rect::new (Point::new(16,66), Point::new( 16+209,66+48)));
        self.fwinfo.place(Rect::new (Point::new(16,122), Point::new( 16+209,122+48)));
        self.reset.place(Rect::new (Point::new(16,178), Point::new( 16+209,178+48)));
        bounds
    }

    fn event(&mut self, ctx: &mut EventCtx, event: Event) -> Option<Self::Msg> {
        if let Some(Clicked) = self.close.event(ctx, event) { return Some(Self::Msg::Close) }
        if let Some(Clicked) = self.reboot.event(ctx, event) { return Some(Self::Msg::Reboot) }
        if let Some(Clicked) = self.fwinfo.event(ctx, event) { return Some(Self::Msg::FwInfo) }
        if let Some(Clicked) = self.reset.event(ctx, event) { return Some(Self::Msg::FactoryReset) }

        None
    }

    fn paint(&mut self) {
        self.bg.paint();
        display::text_top_left(Point::new(15,24), "BOOTLOADER", FONT_BOLD, BLD_TITLE_COLOR, BLD_BG);
        self.close.paint();
        self.reboot.paint();
        self.fwinfo.paint();
        self.reset.paint();
    }

    fn bounds(&self, sink: &mut dyn FnMut(Rect)) {
        self.close.bounds(sink);
        self.reboot.bounds(sink);
        self.fwinfo.bounds(sink);
        self.reset.bounds(sink);
    }
}
