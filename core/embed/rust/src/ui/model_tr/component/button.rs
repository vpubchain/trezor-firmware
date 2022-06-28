use crate::{
    time::Duration,
    ui::{
        component::{Component, Event, EventCtx, TimerToken},
        display::{self, Color, Font},
        event::{ButtonEvent, PhysicalButton},
        geometry::{Offset, Point, Rect},
    },
};

use super::theme;

#[derive(PartialEq)]
pub enum ButtonMsg {
    Clicked,
    LongPressed,
}

#[derive(Copy, Clone)]
pub enum ButtonPos {
    Left,
    Middle,
    Right,
}

impl ButtonPos {
    fn hit(&self, b: &PhysicalButton) -> bool {
        matches!(
            (self, b),
            (Self::Left, PhysicalButton::Left)
                | (Self::Middle, PhysicalButton::Both)
                | (Self::Right, PhysicalButton::Right)
        )
    }
}

pub struct Button<T> {
    area: Rect,
    pos: ButtonPos,
    baseline: Point,
    content: ButtonContent<T>,
    styles: ButtonStyleSheet,
    state: State,
    long_press: Option<Duration>,
    long_timer: Option<TimerToken>,
}

impl<T: AsRef<str>> Button<T> {
    pub fn new(pos: ButtonPos, content: ButtonContent<T>, styles: ButtonStyleSheet) -> Self {
        Self {
            pos,
            content,
            styles,
            baseline: Point::zero(),
            area: Rect::zero(),
            state: State::Released,
            long_press: None,
            long_timer: None,
        }
    }

    pub fn with_text(pos: ButtonPos, text: T, styles: ButtonStyleSheet) -> Self {
        Self::new(pos, ButtonContent::Text(text), styles)
    }

    pub fn with_icon(pos: ButtonPos, image: &'static [u8], styles: ButtonStyleSheet) -> Self {
        Self::new(pos, ButtonContent::Icon(image), styles)
    }

    pub fn with_long_press(mut self, duration: Duration) -> Self {
        self.long_press = Some(duration);
        self
    }

    pub fn content(&self) -> &ButtonContent<T> {
        &self.content
    }

    pub fn is_longpress(&self) -> bool {
        self.long_press.is_some()
    }

    pub fn get_longpress(&self) -> Option<Duration> {
        self.long_press
    }

    fn style(&self) -> &ButtonStyle {
        match self.state {
            State::Released => self.styles.normal,
            State::Pressed => self.styles.active,
        }
    }

    /// Changing the text content of the button.
    ///
    /// NOTE: the button must/should be placed again
    /// after this, to update the button boundaries.
    pub fn set_text(&mut self, text: T, button_area: Rect) {
        self.content = ButtonContent::Text(text);
        self.place(button_area);
    }

    /// Allows for toggling the long press feature.
    ///
    /// Supplying `None` will disable it, `Some(Duration)` will set it.
    pub fn set_long_press(&mut self, duration: Option<Duration>) {
        self.long_press = duration;
    }

    fn set(&mut self, ctx: &mut EventCtx, state: State) {
        if self.state != state {
            self.state = state;
            ctx.request_paint();
        }
    }

    pub fn set_pressed(&mut self, is_pressed: bool) {
        if is_pressed {
            self.state = State::Pressed;
        } else {
            self.state = State::Released;
        }
    }

    pub fn paint_pressed(&mut self, is_pressed: bool) {
        self.set_pressed(is_pressed);
        self.paint();
    }

    fn placement(
        area: Rect,
        pos: ButtonPos,
        content: &ButtonContent<T>,
        styles: &ButtonStyleSheet,
    ) -> (Rect, Point) {
        let border_width = if styles.normal.border_horiz { 2 } else { 0 };
        let content_width = match content {
            ButtonContent::Text(text) => styles.normal.font.text_width(text.as_ref()) - 1,
            ButtonContent::Icon(_icon) => todo!(),
        };
        let button_width = content_width + 2 * border_width;
        let area = match pos {
            ButtonPos::Left => area.split_left(button_width).0,
            ButtonPos::Right => area.split_right(button_width).1,
            ButtonPos::Middle => area.split_center(button_width),
        };

        let start_of_baseline = area.bottom_left() + Offset::new(border_width, -2);

        (area, start_of_baseline)
    }
}

impl<T> Component for Button<T>
where
    T: AsRef<str>,
{
    type Msg = ButtonMsg;

    fn place(&mut self, bounds: Rect) -> Rect {
        let (area, baseline) = Self::placement(bounds, self.pos, &self.content, &self.styles);
        self.area = area;
        self.baseline = baseline;
        self.area
    }

    fn event(&mut self, ctx: &mut EventCtx, event: Event) -> Option<Self::Msg> {
        match event {
            Event::Button(ButtonEvent::ButtonPressed(which)) if self.pos.hit(&which) => {
                self.set(ctx, State::Pressed);
                if let Some(duration) = self.long_press {
                    self.long_timer = Some(ctx.request_timer(duration));
                }
            }
            Event::Button(ButtonEvent::ButtonReleased(which)) if self.pos.hit(&which) => {
                if matches!(self.state, State::Pressed) {
                    self.set(ctx, State::Released);
                    return Some(ButtonMsg::Clicked);
                }
            }
            Event::Timer(token) => {
                if self.long_timer == Some(token) {
                    self.long_timer = None;
                    if matches!(self.state, State::Pressed) {
                        self.set(ctx, State::Released);
                        return Some(ButtonMsg::LongPressed);
                    }
                }
            }
            _ => {}
        };
        None
    }

    fn paint(&mut self) {
        let style = self.style();

        match &self.content {
            ButtonContent::Text(text) => {
                let background_color = style.text_color.negate();
                if style.border_horiz {
                    display::rect_fill_rounded1(self.area, background_color, theme::BG);
                } else {
                    display::rect_fill(self.area, background_color)
                }

                display::text(
                    self.baseline,
                    text.as_ref(),
                    style.font,
                    style.text_color,
                    background_color,
                );
            }
            ButtonContent::Icon(_image) => {
                todo!();
            }
        }
    }
}

#[cfg(feature = "ui_debug")]
impl<T> crate::trace::Trace for Button<T>
where
    T: AsRef<str> + crate::trace::Trace,
{
    fn trace(&self, t: &mut dyn crate::trace::Tracer) {
        t.open("Button");
        match &self.content {
            ButtonContent::Text(text) => t.field("text", text),
            ButtonContent::Icon(_) => t.symbol("icon"),
        }
        t.close();
    }
}

#[derive(PartialEq, Eq)]
enum State {
    Released,
    Pressed,
}

pub enum ButtonContent<T> {
    Text(T),
    Icon(&'static [u8]),
}

pub struct ButtonStyleSheet {
    pub normal: &'static ButtonStyle,
    pub active: &'static ButtonStyle,
}

pub struct ButtonStyle {
    pub font: Font,
    pub text_color: Color,
    pub border_horiz: bool,
}
