use crate::ui::{component::{Child, Component, Event, EventCtx}, display, geometry::Rect};
// use crate::ui::component::FormattedText;
use crate::ui::component::text::layout::DefaultTextTheme;
use crate::ui::display::{alpha, Color, Font};
use crate::ui::geometry::Point;
use crate::ui::model_tt::component::{BootloaderFrame, ButtonStyle, ButtonStyleSheet};
use crate::ui::model_tt::component::button::IconText;
use crate::ui::model_tt::theme::{FONT_BOLD, FONT_MEDIUM, FONT_MONO, FONT_NORMAL, GREY_LIGHT, RED};
use crate::ui::model_tt::component::ButtonMsg::{Clicked};
use super::{Button, theme};
use super::super::constant::{HEIGHT, WIDTH};


pub const BG_COLOR: Color = Color::rgb(0x00, 0x17, 0xA3);
pub const FG_COLOR: Color = Color::rgb(0xFF, 0xFF, 0xFF);
pub const BTN_CLOSE_COLOR: Color =  Color::rgba(BG_COLOR, 0xFF, 0xFF, 0xFF, alpha!(0.22));
pub const BTN_CLOSE_COLOR_ACTIVE: Color =  Color::rgba(BG_COLOR, 0xFF, 0xFF, 0xFF, alpha!(0.11));
pub const BTN_MENU_COLOR: Color =  Color::rgba(BG_COLOR, 0xFF, 0xFF, 0xFF, alpha!(0.33));
pub const BTN_MENU_COLOR_ACTIVE: Color =  Color::rgba(BG_COLOR, 0xFF, 0xFF, 0xFF, alpha!(0.11));
pub const TITLE_COLOR: Color =  Color::rgba(BG_COLOR, 0xFF, 0xFF, 0xFF, alpha!(0.75));



pub struct TTBootloaderText2;

impl DefaultTextTheme for TTBootloaderText2 {
    const BACKGROUND_COLOR: Color = BG_COLOR;
    const TEXT_FONT: Font = FONT_MEDIUM;
    const TEXT_COLOR: Color = FG_COLOR;
    const HYPHEN_FONT: Font = FONT_BOLD;
    const HYPHEN_COLOR: Color = GREY_LIGHT;
    const ELLIPSIS_FONT: Font = FONT_BOLD;
    const ELLIPSIS_COLOR: Color = GREY_LIGHT;

    const NORMAL_FONT: Font = FONT_NORMAL;
    const MEDIUM_FONT: Font = FONT_MEDIUM;
    const BOLD_FONT: Font = FONT_BOLD;
    const MONO_FONT: Font = FONT_MONO;
}


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



pub fn button_cancel() -> ButtonStyleSheet {
    ButtonStyleSheet {
        normal: &ButtonStyle {
            font: FONT_BOLD,
            text_color: FG_COLOR,
            button_color: BTN_CLOSE_COLOR,
            background_color: BG_COLOR,
            border_color: BG_COLOR,
            border_radius: 4,
            border_width: 0,
        },
        active: &ButtonStyle {
            font: FONT_BOLD,
            text_color: FG_COLOR,
            button_color: BTN_CLOSE_COLOR_ACTIVE,
            background_color: BG_COLOR,
            border_color: BG_COLOR,
            border_radius: 4,
            border_width: 0,
        },
        disabled: &ButtonStyle {
            font: FONT_BOLD,
            text_color: GREY_LIGHT,
            button_color: RED,
            background_color: BG_COLOR,
            border_color: FG_COLOR,
            border_radius: 4,
            border_width: 0,
        },
    }
}


pub fn button_menu() -> ButtonStyleSheet {
    ButtonStyleSheet {
        normal: &ButtonStyle {
            font: FONT_BOLD,
            text_color: FG_COLOR,
            button_color: BTN_MENU_COLOR,
            background_color: BG_COLOR,
            border_color: BG_COLOR,
            border_radius: 4,
            border_width: 0,
        },
        active: &ButtonStyle {
            font: FONT_BOLD,
            text_color: FG_COLOR,
            button_color: BTN_MENU_COLOR_ACTIVE,
            background_color: BG_COLOR,
            border_color: BG_COLOR,
            border_radius: 4,
            border_width: 0,
        },
        disabled: &ButtonStyle {
            font: FONT_BOLD,
            text_color: GREY_LIGHT,
            button_color: RED,
            background_color: BG_COLOR,
            border_color: FG_COLOR,
            border_radius: 4,
            border_width: 0,
        },
    }
}

impl BldMenu
{
    pub fn new() -> Self {

        const CLOSE: &'static [u8] = include_res!("model_tt/res/close.toif");
        const RESET: &'static [u8] = include_res!("model_tt/res/reset.toif");
        const FWINFO: &'static [u8] = include_res!("model_tt/res/fwinfo.toif");
        const REBOOT: &'static [u8] = include_res!("model_tt/res/reboot.toif");


        let content_reboot = IconText::new("REBOOT", REBOOT, 46, 25);
        let content_fwinfo = IconText::new("FW INFO", FWINFO, 46, 25);
        let content_reset = IconText::new("FACTORY RESET", RESET, 46, 25);

        Self {
            close: Child::new(Button::with_icon(CLOSE).styled(button_cancel())),
            reboot: Child::new(Button::with_icon_and_text(content_reboot).styled(button_menu())),
            fwinfo: Child::new(Button::with_icon_and_text(content_fwinfo).styled(button_menu())),
            reset: Child::new(Button::with_icon_and_text(content_reset).styled(button_menu())),
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
        display::rect_fill(Rect::new (Point::new(0,0), Point::new(WIDTH, HEIGHT)), BG_COLOR);
        display::text_top_left(Point::new(15,24), "BOOTLOADER", theme::FONT_BOLD, TITLE_COLOR, BG_COLOR);

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