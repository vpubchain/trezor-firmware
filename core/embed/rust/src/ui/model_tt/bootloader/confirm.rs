use crate::ui::{component::{Child, Component, Event, EventCtx}, display, geometry::{Rect}};
use crate::ui::component::{FormattedText, Pad};
use crate::ui::display::Color;
use crate::ui::geometry::Point;
use crate::ui::model_tt::bootloader::ReturnToC;
use crate::ui::model_tt::component::{Button, ButtonStyle, ButtonStyleSheet};
use crate::ui::model_tt::theme::{BG, FG, FONT_BOLD, GREEN, GREEN_DARK, GREY_LIGHT, RADIUS, RED, RED_DARK, ICON_CANCEL, ICON_CONFIRM, FONT_NORMAL};
use crate::ui::model_tt::constant::{HEIGHT, WIDTH};
use crate::ui::model_tt::component::ButtonMsg::{Clicked};


#[repr(u32)]
#[derive(Copy, Clone)]
pub enum InstallMsg  {
    Cancel = 1,
    Confirm = 2,
}
impl ReturnToC for InstallMsg{
    fn return_to_c(&self) -> u32 { *self as u32 }
}

pub struct Install {
    bg: Pad,
    label: &'static str,
    icon: Option<&'static [u8]>,
    message: Child<FormattedText<&'static str, &'static str>>,
    warning: Option<&'static str>,
    cancel: Child<Button<&'static str>>,
    confirm: Child<Button<&'static str>>,
}


pub fn button_cancel() -> ButtonStyleSheet {
    ButtonStyleSheet {
        normal: &ButtonStyle {
            font: FONT_BOLD,
            text_color: FG,
            button_color: RED,
            background_color: FG,
            border_color: FG,
            border_radius: RADIUS,
            border_width: 0,
        },
        active: &ButtonStyle {
            font: FONT_BOLD,
            text_color: FG,
            button_color: RED_DARK,
            background_color: FG,
            border_color: BG,
            border_radius: RADIUS,
            border_width: 0,
        },
        disabled: &ButtonStyle {
            font: FONT_BOLD,
            text_color: GREY_LIGHT,
            button_color: RED,
            background_color: FG,
            border_color: FG,
            border_radius: RADIUS,
            border_width: 0,
        },
    }
}


pub fn button_confirm() -> ButtonStyleSheet {
    ButtonStyleSheet {
        normal: &ButtonStyle {
            font: FONT_BOLD,
            text_color: FG,
            button_color: GREEN,
            background_color: FG,
            border_color: FG,
            border_radius: RADIUS,
            border_width: 0,
        },
        active: &ButtonStyle {
            font: FONT_BOLD,
            text_color: FG,
            button_color: GREEN_DARK,
            background_color: FG,
            border_color: FG,
            border_radius: RADIUS,
            border_width: 0,
        },
        disabled: &ButtonStyle {
            font: FONT_BOLD,
            text_color: FG,
            button_color: GREEN,
            background_color: FG,
            border_color: FG,
            border_radius: RADIUS,
            border_width: 0,
        },
    }
}



impl Install
{
    pub fn new(label: &'static str, icon: Option<&'static [u8]> , message: FormattedText<&'static str, &'static str>) -> Self {

        let mut instance = Self {
            bg: Pad::with_background(FG),
            label,
            icon,
            warning: None,
            message: Child::new(message),
            cancel: Child::new(Button::with_icon(ICON_CANCEL).styled(button_cancel())),
            confirm: Child::new(Button::with_icon(ICON_CONFIRM).styled(button_confirm())),

        };
        instance.bg.clear();
        instance
    }

    pub fn add_warning(&mut self, warning: &'static str) {
        self.warning = Option::from(warning);
    }
}



impl Component for Install
{
    type Msg = InstallMsg;

    fn place(&mut self, bounds: Rect) -> Rect {
        self.bg.place(Rect::new (Point::new(0,0), Point::new(WIDTH, HEIGHT)));
        self.message.place(Rect::new (Point::new(55,52), Point::new(WIDTH-12, HEIGHT-80)));
        self.cancel.place(Rect::new (Point::new(9,184), Point::new(117, 234)));
        self.confirm.place(Rect::new (Point::new(123,184), Point::new(231, 234)));
        bounds
    }

    fn event(&mut self, ctx: &mut EventCtx, event: Event) -> Option<Self::Msg> {
        if let Some(Clicked) = self.cancel.event(ctx, event) { return Some(Self::Msg::Cancel) };
        if let Some(Clicked) = self.confirm.event(ctx, event) { return Some(Self::Msg::Confirm) };
        None
    }

    fn paint(&mut self) {
        self.bg.paint();
        display::rect_fill(Rect::new (Point::new(16,44), Point::new(WIDTH-12, 45)), BG);
        display::text(Point::new(16,32), self.label, FONT_NORMAL, BG, FG);


        match self.icon {
            Some(icon) => {display::icon(
                Point::new(32, 70),
                icon,
                Color::rgb(0x99, 0x99, 0x99),
                FG,
            );}
            None => ()
        }

        match self.warning {
            Some(warning) => {
                display::text_center(Point::new(120,170), warning, FONT_NORMAL, Color::rgb(0xFF, 0x00, 0x00), FG);
            }
            None => ()
        }

        self.message.paint();
        self.cancel.paint();
        self.confirm.paint();

    }

    fn bounds(&self, sink: &mut dyn FnMut(Rect)) {
        self.cancel.bounds(sink);
        self.confirm.bounds(sink);
    }
}
