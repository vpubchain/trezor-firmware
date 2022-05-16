use crate::ui::{component::{Child, Component, Event, EventCtx}, display, geometry::{Rect}};
use crate::ui::display::Color;
use crate::ui::geometry::Point;
use crate::ui::model_tt::component::{ButtonStyle, ButtonStyleSheet};
use crate::ui::model_tt::theme::{BG, FG, FONT_BOLD, GREEN, GREEN_DARK, GREY_LIGHT, RADIUS, RED, RED_DARK};
use super::{theme, Button};
use super::super::constant::{HEIGHT, WIDTH};

pub enum WipeMsg<L, R>  {
    Left(L),
    Right(R),
}

pub struct Wipe< M> {
    label: &'static str,
    icon: Option<&'static [u8]>,
    message: Child<M>,
    left: Child<Button<&'static str>>,
    right: Child<Button<&'static str>>,
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



impl<M> Wipe<M>
    where
        M: Component,
{
    pub fn new(label: &'static str, icon: Option<&'static [u8]> , message: M) -> Self {

        Self {
            label,
            icon,
            message: Child::new(message),
            left: Child::new(Button::with_icon(theme::ICON_CANCEL).styled(button_cancel())),
            right: Child::new(Button::with_icon(theme::ICON_CONFIRM).styled(button_confirm())),

        }
    }

    pub fn inner(&self) -> &M {
        self.message.inner()
    }
}



impl<M> Component for Wipe<M>
    where
        M: Component,
{

    type Msg = WipeMsg<
        <Button<&'static str> as Component>::Msg,
        <Button<&'static str> as Component>::Msg>;

    fn place(&mut self, bounds: Rect) -> Rect {
        self.message.place(Rect::new (Point::new(55,52), Point::new(WIDTH-12, HEIGHT-80)));
        self.left.place(Rect::new (Point::new(9,184), Point::new(117, 234)));
        self.right.place(Rect::new (Point::new(123,184), Point::new(231, 234)));
        bounds
    }

    fn event(&mut self, ctx: &mut EventCtx, event: Event) -> Option<Self::Msg> {
        self.left.event(ctx, event).map(Self::Msg::Left)
            .or_else(|| self.right.event(ctx, event).map(Self::Msg::Right))
    }

    fn paint(&mut self) {
        display::rect_fill(Rect::new (Point::new(0,0), Point::new(WIDTH, HEIGHT)), theme::FG);
        display::rect_fill(Rect::new (Point::new(16,44), Point::new(WIDTH-12, 45)), theme::BG);
        display::text(Point::new(16,32), self.label, theme::FONT_NORMAL, theme::BG, theme::FG);
        display::text_center(Point::new(120,170), "Seed will be erased!", theme::FONT_NORMAL, Color::rgb(0xFF, 0x00, 0x00), theme::FG);

        match self.icon {
            Some(icon) => {display::icon(
                Point::new(32, 70),
                icon,
                Color::rgb(0x99, 0x99, 0x99),
                FG,
            );}
            None => ()
        }

        // self.label.paint();
        self.message.paint();
        self.left.paint();
        self.right.paint();

    }

    fn bounds(&self, sink: &mut dyn FnMut(Rect)) {
        self.left.bounds(sink);
        self.right.bounds(sink);
    }
}
