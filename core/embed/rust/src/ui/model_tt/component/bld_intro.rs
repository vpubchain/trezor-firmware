use crate::ui::{component::{Child, Component, Event, EventCtx}, display, geometry::Rect};
use crate::ui::component::FormattedText;
use crate::ui::component::text::layout::DefaultTextTheme;
use crate::ui::display::{alpha, Color, Font};
use crate::ui::geometry::Point;
use crate::ui::model_tt::component::{BootloaderFrame, ButtonStyle, ButtonStyleSheet};
use crate::ui::model_tt::theme::{FONT_BOLD, FONT_MEDIUM, FONT_MONO, FONT_NORMAL, GREY_LIGHT, RED};
use crate::ui::model_tt::component::ButtonMsg::{Clicked};

use super::{Button, theme};
use super::super::constant::{HEIGHT, WIDTH};


pub const BG_COLOR: Color = Color::rgb(0x00, 0x17, 0xA3);
pub const FG_COLOR: Color = Color::rgb(0xFF, 0xFF, 0xFF);
pub const BTN_COLOR: Color =  Color::rgba(BG_COLOR, 0xFF, 0xFF, 0xFF, alpha!(0.22));
pub const BTN_COLOR_ACTIVE: Color =  Color::rgba(BG_COLOR, 0xFF, 0xFF, 0xFF, alpha!(0.11));
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


pub enum BldIntroMsg<M>  {
    Menu(M),
}

pub struct BldIntro {
    menu: Child<Button<&'static str>>,
    text1: Child<FormattedText<&'static str, &'static str>>,
}



pub fn button_menu() -> ButtonStyleSheet {
    ButtonStyleSheet {
        normal: &ButtonStyle {
            font: FONT_BOLD,
            text_color: FG_COLOR,
            button_color: BTN_COLOR,
            background_color: BG_COLOR,
            border_color: BG_COLOR,
            border_radius: 4,
            border_width: 0,
        },
        active: &ButtonStyle {
            font: FONT_BOLD,
            text_color: FG_COLOR,
            button_color: BTN_COLOR_ACTIVE,
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


impl BldIntro
{
    pub fn new() -> Self {

        const ICON: &'static [u8] = include_res!("model_tt/res/menu.toif");

        let text1 = FormattedText::new::<TTBootloaderText2>(
            "This is a bootloader. It does something.\n\nFollow instructions in ur PC to do stuff.",
        );

        Self {
            menu: Child::new(Button::with_icon(ICON).styled(button_menu())),
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
        display::rect_fill(Rect::new (Point::new(0,0), Point::new(WIDTH, HEIGHT)), BG_COLOR);
        display::text_top_left(Point::new(15,24), "BOOTLOADER", theme::FONT_BOLD, TITLE_COLOR, BG_COLOR);

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