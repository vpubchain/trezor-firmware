use super::{Button, ButtonPos};
use crate::ui::{
    component::{base::Event, Component, EventCtx, TimerToken},
    event::{ButtonEvent, PhysicalButton},
    geometry::Rect,
};

#[derive(Copy, Clone, PartialEq, Eq)]
enum ButtonState {
    Nothing,
    OneDown(PhysicalButton),
    BothDown,
    OneReleased(PhysicalButton),
}

enum ButtonControllerMsg {
    Clicked(ButtonPos),
    LongClicked(ButtonPos),
}

struct ButtonController<T> {
    left_btn: Button<T>,
    mid_btn: Button<T>,
    right_btn: Button<T>,
    state: ButtonState,
    long_timer: Option<TimerToken>,
}

impl<T: AsRef<str>> Component for ButtonController<T> {
    type Msg = ButtonControllerMsg;

    fn event(&mut self, ctx: &mut EventCtx, event: Event) -> Option<Self::Msg> {
        match event {
            Event::Button(button) => {
                let (new_state, event) = match self.state {
                    ButtonState::Nothing => match button {
                        ButtonEvent::ButtonPressed(which) => (ButtonState::OneDown(which), None),
                        _ => (self.state, None),
                    },
                    ButtonState::OneDown(which_down) => match button {
                        ButtonEvent::ButtonReleased(b) if b == which_down => match which_down {
                            PhysicalButton::Left => (
                                ButtonState::Nothing,
                                Some(ButtonControllerMsg::Clicked(ButtonPos::Left)),
                            ),
                            PhysicalButton::Right => (
                                ButtonState::Nothing,
                                Some(ButtonControllerMsg::Clicked(ButtonPos::Right)),
                            ),
                            _ => (ButtonState::Nothing, None),
                        },

                        ButtonEvent::ButtonPressed(b) if b != which_down => {
                            (ButtonState::BothDown, None)
                        }
                        _ => (self.state, None),
                    },
                    ButtonState::BothDown => match button {
                        ButtonEvent::ButtonReleased(b) => (ButtonState::OneReleased(b), None),
                        _ => (self.state, None),
                    },
                    ButtonState::OneReleased(which_up) => match button {
                        ButtonEvent::ButtonPressed(b) if b == which_up => {
                            (ButtonState::BothDown, None)
                        }
                        ButtonEvent::ButtonReleased(b) if b != which_up => (
                            ButtonState::Nothing,
                            Some(ButtonControllerMsg::Clicked(ButtonPos::Middle)),
                        ),
                        _ => (self.state, None),
                    },
                };
                self.state = new_state;
                event
            }
            Event::Timer(token) if self.long_timer == Some(token) => {
                self.long_timer = None;
                self.state = ButtonState::Nothing;
                let which_button = match self.state {
                    ButtonState::OneDown(PhysicalButton::Left) => Some(ButtonPos::Left),
                    ButtonState::OneDown(PhysicalButton::Right) => Some(ButtonPos::Right),
                    ButtonState::BothDown | ButtonState::OneReleased(_) => Some(ButtonPos::Middle),
                    _ => None,
                };
                which_button.map(|pos| ButtonControllerMsg::LongClicked(pos))
            }
            _ => None,
        }
    }

    fn paint(&mut self) {
        let highlight = match self.state {
            ButtonState::Nothing => None,
            ButtonState::OneDown(down_button) => match down_button {
                PhysicalButton::Left => Some(ButtonPos::Left),
                PhysicalButton::Right => Some(ButtonPos::Right),
                _ => None,
            },
            ButtonState::BothDown | ButtonState::OneReleased(_) => Some(ButtonPos::Middle),
        };
        self.left_btn
            .paint_pressed(matches!(highlight, Some(ButtonPos::Left)));
        self.mid_btn
            .paint_pressed(matches!(highlight, Some(ButtonPos::Middle)));
        self.right_btn
            .paint_pressed(matches!(highlight, Some(ButtonPos::Right)));
    }

    fn place(&mut self, bounds: Rect) -> Rect {
        self.left_btn.place(bounds);
        self.mid_btn.place(bounds);
        self.right_btn.place(bounds);
        bounds
    }
}
