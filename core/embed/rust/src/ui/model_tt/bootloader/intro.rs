use crate::ui::{component::{Child, Component, Event, EventCtx}, display, geometry::Rect};
use crate::ui::component::{Pad, TextLayout};
use crate::ui::component::text::paragraphs::{Paragraphs};
use crate::ui::geometry::{LinearPlacement, Point};
use crate::ui::model_tt::bootloader::ReturnToC;
use crate::ui::model_tt::theme::{FONT_BOLD, FONT_MEDIUM};
use crate::ui::model_tt::bootloader::theme::{BLD_BG, BLD_TITLE_COLOR, button_bld_menu, MENU, TTBootloaderText, button_bld_menu_item};
use crate::ui::model_tt::component::{ButtonMsg::{Clicked}};

use crate::ui::model_tt::component::{Button};
use crate::ui::model_tt::constant::{HEIGHT, WIDTH};


#[repr(u32)]
#[derive(Copy, Clone)]
pub enum BldIntroMsg  {
    Menu = 1,
    Host = 2,
}
impl ReturnToC for BldIntroMsg {
    fn return_to_c(&self) -> u32 { *self as u32 }
}

pub struct BldIntro {
    bg: Pad,
    menu: Child<Button<&'static str>>,
    host: Child<Button<&'static str>>,
    text1: Child<Paragraphs<&'static str>>,
}


impl BldIntro
{
    pub fn new() -> Self {

        let p1 = Paragraphs::new(
        ).add::<TTBootloaderText>(FONT_MEDIUM,
                                  "This is a bootloader. It does something.")
            .with_placement(LinearPlacement::vertical().align_at_start());

        let mut instance = Self {            
            bg: Pad::with_background(BLD_BG),
            menu: Child::new(Button::with_icon(MENU).styled(button_bld_menu())),
            host: Child::new(Button::with_text("Connect to host").styled(button_bld_menu_item())),
            text1: Child::new(p1),
        };
        
        instance.bg.clear();
        instance
    }
}


impl Component for BldIntro
{

    type Msg = BldIntroMsg;

    fn place(&mut self, bounds: Rect) -> Rect {
        self.bg.place(Rect::new (Point::new(0,0), Point::new(WIDTH, HEIGHT)));
        self.menu.place(Rect::new (Point::new(187,15), Point::new(187+38, 15+38)));
        self.host.place(Rect::new (Point::new(16,178), Point::new( 16+209,178+48)));
        self.text1.place(Rect::new (Point::new(15,75), Point::new(225, 200)));
        bounds
    }

    fn event(&mut self, ctx: &mut EventCtx, event: Event) -> Option<Self::Msg> {
        if let Some(Clicked) = self.menu.event(ctx, event) { return Some(Self::Msg::Menu)};
        if let Some(Clicked) = self.host.event(ctx, event) { return Some(Self::Msg::Host)};
        None
    }

    fn paint(&mut self) {
        self.bg.paint();            
        display::text_top_left(Point::new(15,24), "BOOTLOADER", FONT_BOLD, BLD_TITLE_COLOR, BLD_BG);
        self.text1.paint();
        self.host.paint();
        self.menu.paint();
    }

    fn bounds(&self, sink: &mut dyn FnMut(Rect)) {
        self.menu.bounds(sink);
    }
}
