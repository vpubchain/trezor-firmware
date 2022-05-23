use crate::ui::{component::{Child, Component, Event, EventCtx}, display, geometry::{Rect}};
use crate::ui::display::Color;
use crate::ui::geometry::Point;
use crate::ui::model_tt::component::{BootloaderFrame, ButtonStyle, ButtonStyleSheet};
use crate::ui::model_tt::theme::{BG, FG, FONT_BOLD, GREEN, GREEN_DARK, GREY_LIGHT, RADIUS, RED, RED_DARK};
use super::{theme, Button};
use super::super::constant::{HEIGHT, WIDTH};
use crate::ui::model_tt::component::ButtonMsg::{Clicked};


pub enum InstallMsg<M>  {
    Cancel(M),
    Confirm(M),
}

pub struct Install<T> {
    label: &'static str,
    icon: Option<&'static [u8]>,
    message: Child<T>,
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



impl<T> Install<T>
where T: Component
{
    pub fn new(label: &'static str, icon: Option<&'static [u8]> , message: T) -> Self {

        Self {
            label,
            icon,
            warning: None,
            message: Child::new(message),
            cancel: Child::new(Button::with_icon(theme::ICON_CANCEL).styled(button_cancel())),
            confirm: Child::new(Button::with_icon(theme::ICON_CONFIRM).styled(button_confirm())),

        }
    }

    pub fn add_warning(&mut self, warning: &'static str) {
        self.warning = Option::from(warning);
    }
}



impl<T> Component for Install<T>
    where T: Component
{

    type Msg = InstallMsg<<Button<&'static str> as Component>::Msg>;

    fn place(&mut self, bounds: Rect) -> Rect {
        self.message.place(Rect::new (Point::new(55,52), Point::new(WIDTH-12, HEIGHT-80)));
        self.cancel.place(Rect::new (Point::new(9,184), Point::new(117, 234)));
        self.confirm.place(Rect::new (Point::new(123,184), Point::new(231, 234)));
        bounds
    }

    fn event(&mut self, ctx: &mut EventCtx, event: Event) -> Option<Self::Msg> {
        self.cancel.event(ctx, event).map(Self::Msg::Cancel)
            .or_else(|| self.confirm.event(ctx, event).map(Self::Msg::Confirm))
    }

    fn paint(&mut self) {
        display::rect_fill(Rect::new (Point::new(0,0), Point::new(WIDTH, HEIGHT)), theme::FG);
        display::rect_fill(Rect::new (Point::new(16,44), Point::new(WIDTH-12, 45)), theme::BG);
        display::text(Point::new(16,32), self.label, theme::FONT_NORMAL, theme::BG, theme::FG);


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
                display::text_center(Point::new(120,170), warning, theme::FONT_NORMAL, Color::rgb(0xFF, 0x00, 0x00), theme::FG);
            }
            None => ()
        }

        // self.label.paint();
        self.message.paint();
        self.repaint();

    }

    fn bounds(&self, sink: &mut dyn FnMut(Rect)) {
        self.cancel.bounds(sink);
        self.confirm.bounds(sink);
    }
}
impl<T> BootloaderFrame for Install<T>
where T: Component
{

    fn repaint(&mut self) {
        self.cancel.paint();
        self.confirm.paint();
    }
    fn messages(&mut self, msg: <Self as Component>::Msg) -> Option<u32>
        where
            Self: Component,
    {

        let result = match msg {
            InstallMsg::Cancel(Clicked) => {return Some(1)},
            InstallMsg::Confirm(Clicked) => {return Some(2)},
            _ => {None}
        };
        result
    }
}