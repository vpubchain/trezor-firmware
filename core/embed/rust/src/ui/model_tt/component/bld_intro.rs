use crate::ui::{component::{Child, Component, Event, EventCtx}, display, geometry::Rect};
use crate::ui::component::FormattedText;
use crate::ui::geometry::Point;
use crate::ui::model_tt::component::{BootloaderFrame};
use crate::ui::model_tt::theme::{BLD_BG, BLD_TITLE_COLOR, button_bld_menu, MENU, TTBootloaderText};
use crate::ui::model_tt::component::ButtonMsg::{Clicked};

use super::{Button, theme};
use super::super::constant::{HEIGHT, WIDTH};


pub enum BldIntroMsg<M>  {
    Menu(M),
}


pub struct BldIntro {
    menu: Child<Button<&'static str>>,
    text1: Child<FormattedText<&'static str, &'static str>>,
}


impl BldIntro
{
    pub fn new() -> Self {

        let text1 = FormattedText::new::<TTBootloaderText>(
            "This is a bootloader. It does something.\n\nFollow instructions in ur PC to do stuff.",
        );

        Self {
            menu: Child::new(Button::with_icon(MENU).styled(button_bld_menu())),
            text1: Child::new(text1),
        }
    }
}


impl Component for BldIntro
{

    type Msg = BldIntroMsg<
        <Button<&'static str> as Component>::Msg>;

    fn place(&mut self, bounds: Rect) -> Rect {
        self.menu.place(Rect::new (Point::new(187,15), Point::new(187+38, 15+38)));
        self.text1.place(Rect::new (Point::new(15,75), Point::new(225, 200)));
        bounds
    }

    fn event(&mut self, ctx: &mut EventCtx, event: Event) -> Option<Self::Msg> {
        self.text1.event(ctx, event);
        self.menu.event(ctx, event).map(Self::Msg::Menu)
    }

    fn paint(&mut self) {
        display::rect_fill(Rect::new (Point::new(0,0), Point::new(WIDTH, HEIGHT)), BLD_BG);
        display::text_top_left(Point::new(15,24), "BOOTLOADER", theme::FONT_BOLD, BLD_TITLE_COLOR, BLD_BG);

        self.repaint()

    }

    fn bounds(&self, sink: &mut dyn FnMut(Rect)) {
        self.menu.bounds(sink);
    }
}


impl BootloaderFrame for BldIntro {

    fn repaint(&mut self) {
        self.text1.paint();
        self.menu.paint();
    }
    fn messages(&mut self, msg: <Self as Component>::Msg) -> Option<u32> where Self: Component{
        let result = match msg {
            BldIntroMsg::Menu(Clicked) => {Some(1_u32)}
            _ => {None}
        };
        result
    }
}