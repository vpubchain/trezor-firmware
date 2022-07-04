use super::{
    common::ButtonDetails, theme, Button, ButtonPos, ButtonStyleSheet, HoldToConfirm,
    HoldToConfirmMsg, LoaderStyle, LoaderStyleSheet,
};
use crate::{
    time::Duration,
    ui::{
        component::{base::Event, Child, Component, EventCtx},
        event::{ButtonEvent, PhysicalButton},
        geometry::Rect,
    },
};
use crate::ui::component::{ComponentExt, Pad};

#[derive(Copy, Clone, PartialEq, Eq)]
enum ButtonState {
    Nothing,
    OneDown(PhysicalButton),
    BothDown,
    OneReleased(PhysicalButton),
}

pub enum ButtonControllerMsg {
    Triggered(ButtonPos),
}

pub enum ButtonType {
    Nothing,
    NormalButton,
    HoldToConfirm,
}

/// Wrapping a button and its active state, so that it can be easily
/// shown/hidden according to `is_active`.
pub struct ButtonContainer<T> {
    button: Child<Button<T>>,
    hold_to_confirm: Child<HoldToConfirm<T>>,
    button_type: ButtonType,
}

impl<T: Clone + AsRef<str>> ButtonContainer<T> {
    pub fn new(pos: ButtonPos, text: T, styles: ButtonStyleSheet, button_type: ButtonType) -> Self {
        Self {
            button: Child::new(Button::with_text(pos, text.clone(), styles)),
            hold_to_confirm: Child::new(HoldToConfirm::new(
                pos,
                text,
                LoaderStyleSheet {
                    normal: &LoaderStyle {
                        font: theme::FONT_BOLD,
                        fg_color: theme::FG,
                        bg_color: theme::BG,
                    },
                },
                Duration::from_millis(1000),
            )),
            button_type,
        }
    }

    pub fn reacts_to_single_click(&self) -> bool {
        matches!(self.button_type, ButtonType::NormalButton)
    }

    /// Changing the state of the button.
    /// Passing `None` will mark the button as inactive.
    pub fn set(
        &mut self,
        ctx: &mut EventCtx,
        btn_details: Option<ButtonDetails<T>>,
        button_area: Rect,
    ) {
        if let Some(btn_details) = btn_details {
            if let Some(duration) = btn_details.duration {
                // self.hold_to_confirm
                //     .inner_mut()
                //     .set_text(btn_details.text, button_area);
                // self.hold_to_confirm.inner_mut().set_duration(duration);
                // self.hold_to_confirm.inner_mut().set_duration(duration);
                // self.hold_to_confirm.place(button_area);
                self.hold_to_confirm.mutate(ctx, |_ctx, btn| {
                    btn.set_text(btn_details.text, button_area);
                });
                self.hold_to_confirm.mutate(ctx, |_ctx, btn| {
                    btn.set_duration(duration);
                });
                self.hold_to_confirm.request_complete_repaint(ctx);
                // self.hold_to_confirm.place(button_area);
                // self.hold_to_confirm.inner_mut().paint();
                self.button_type = ButtonType::HoldToConfirm;
            } else {
                // self.button.set_text(btn_details.text, button_area);
                self.button.mutate(ctx, |_ctx, btn| {
                    btn.set_text(btn_details.text, button_area);
                });
                self.button.request_complete_repaint(ctx);
                self.button_type = ButtonType::NormalButton;
            }
        } else {
            self.button_type = ButtonType::Nothing;
        }
    }
}

/// Component responsible for handling buttons.
///
/// Acts as a state-machine of `ButtonState`.
///
/// Storing all three possible buttons - left, middle and right -
/// and handling their placement, painting and returning
/// appropriate events when they are triggered.
///
/// Buttons can be interactively changed by clients by appropriate
/// `set_XXX()` methods.
///
/// Only "final" button events are returned in `ButtonControllerMsg::Triggered`,
/// based upon the buttons being long-press or not.
pub struct ButtonController<T> {
    pad: Pad,
    left_btn: ButtonContainer<T>,
    middle_btn: ButtonContainer<T>,
    right_btn: ButtonContainer<T>,
    state: ButtonState,
    // Button area is needed so the buttons
    // can be "re-placed" after their text is changed
    // Will be set in `place`
    button_area: Rect,
}

impl ButtonController<&'static str> {
    /// Supplying `None` marks the appropriate button inactive.
    pub fn new(
        left: Option<ButtonDetails<&'static str>>,
        mid: Option<ButtonDetails<&'static str>>,
        right: Option<ButtonDetails<&'static str>>,
    ) -> Self {
        let mut instance = Self {
            pad: Pad::with_background(theme::BG),
            left_btn: ButtonContainer::new(
                ButtonPos::Left,
                left.unwrap_or(ButtonDetails::new("LEFT")).text,
                theme::button_default(),
                ButtonController::get_button_type(left),
            ),
            middle_btn: ButtonContainer::new(
                ButtonPos::Middle,
                mid.unwrap_or(ButtonDetails::new("MID")).text,
                theme::button_default(),
                ButtonController::get_button_type(mid),
            ),
            right_btn: ButtonContainer::new(
                ButtonPos::Right,
                right.unwrap_or(ButtonDetails::new("RIGHT")).text,
                theme::button_default(),
                ButtonController::get_button_type(right),
            ),
            state: ButtonState::Nothing,
            button_area: Rect::zero(),
        };

        instance.pad.clear();

        instance
    }

    pub fn get_button_type(details: Option<ButtonDetails<&'static str>>) -> ButtonType{
        if details.is_none() {
            ButtonType::Nothing
        } else {
            if details.unwrap().duration.is_some() {
                ButtonType::HoldToConfirm
            } else {
                ButtonType::NormalButton
            }
        }
    }
}

impl<T: Clone + AsRef<str>> ButtonController<T> {
    pub fn set_left(&mut self, ctx: &mut EventCtx, btn_details: Option<ButtonDetails<T>>) {
        self.left_btn.set(ctx, btn_details, self.button_area);
        self.pad.clear();
    }

    pub fn set_right(&mut self, ctx: &mut EventCtx, btn_details: Option<ButtonDetails<T>>) {
        self.right_btn.set(ctx, btn_details, self.button_area);
        self.pad.clear();
    }

    pub fn set_middle(&mut self, ctx: &mut EventCtx, btn_details: Option<ButtonDetails<T>>) {
        self.middle_btn.set(ctx, btn_details, self.button_area);
        self.pad.clear();
    }
}

impl<T: Clone + AsRef<str>> Component for ButtonController<T> {
    type Msg = ButtonControllerMsg;

    fn event(&mut self, ctx: &mut EventCtx, event: Event) -> Option<Self::Msg> {
        // Handling the hold_to_confirm elements
        if matches!(self.left_btn.button_type, ButtonType::HoldToConfirm) {
            let msg = self.left_btn.hold_to_confirm.event(ctx, event);
            if matches!(msg, Some(HoldToConfirmMsg::Confirmed)) {
                self.state = ButtonState::Nothing;
                self.left_btn.hold_to_confirm.inner_mut().reset();
                self.left_btn.hold_to_confirm.request_complete_repaint(ctx);
                return Some(ButtonControllerMsg::Triggered(ButtonPos::Left));
            }
        }
        if matches!(self.right_btn.button_type, ButtonType::HoldToConfirm) {
            let msg = self.right_btn.hold_to_confirm.event(ctx, event);
            if matches!(msg, Some(HoldToConfirmMsg::Confirmed)) {
                self.state = ButtonState::Nothing;
                self.right_btn.hold_to_confirm.inner_mut().reset();
                self.right_btn.hold_to_confirm.request_complete_repaint(ctx);
                return Some(ButtonControllerMsg::Triggered(ButtonPos::Right));
            }
        }
        if matches!(self.middle_btn.button_type, ButtonType::HoldToConfirm) {
            let msg = self.middle_btn.hold_to_confirm.event(ctx, event);
            if matches!(msg, Some(HoldToConfirmMsg::Confirmed)) {
                self.state = ButtonState::Nothing;
                self.middle_btn.hold_to_confirm.inner_mut().reset();
                self.middle_btn.hold_to_confirm.request_complete_repaint(ctx);
                return Some(ButtonControllerMsg::Triggered(ButtonPos::Middle));
            }
        }


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
                                if self.left_btn.reacts_to_single_click() {
                                    Some(ButtonControllerMsg::Triggered(ButtonPos::Left))
                                } else {
                                    None
                                },
                            ),
                            PhysicalButton::Right => (
                                ButtonState::Nothing,
                                if self.right_btn.reacts_to_single_click() {
                                    Some(ButtonControllerMsg::Triggered(ButtonPos::Right))
                                } else {
                                    None
                                },
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
                            if self.middle_btn.reacts_to_single_click() {
                                Some(ButtonControllerMsg::Triggered(ButtonPos::Middle))
                            } else {
                                None
                            },
                        ),
                        _ => (self.state, None),
                    },
                };

                match new_state {
                    ButtonState::Nothing => {
                        self.left_btn.button.mutate(ctx, |ctx, btn| {
                            btn.set_pressed(ctx, false);
                        });
                        self.right_btn.button.mutate(ctx, |ctx, btn| {
                            btn.set_pressed(ctx, false);
                        });
                        self.middle_btn.button.mutate(ctx, |ctx, btn| {
                            btn.set_pressed(ctx, false);
                        });
                    },
                    ButtonState::OneDown(down_button) => match down_button {
                        PhysicalButton::Left => {
                            self.left_btn.button.mutate(ctx, |ctx, btn| {
                                btn.set_pressed(ctx, true);
                            });
                            self.right_btn.button.mutate(ctx, |ctx, btn| {
                                btn.set_pressed(ctx, false);
                            });
                            self.middle_btn.button.mutate(ctx, |ctx, btn| {
                                btn.set_pressed(ctx, false);
                            });
                        },
                        PhysicalButton::Right => {
                            self.left_btn.button.mutate(ctx, |ctx, btn| {
                                btn.set_pressed(ctx, false);
                            });
                            self.right_btn.button.mutate(ctx, |ctx, btn| {
                                btn.set_pressed(ctx, true);
                            });
                            self.middle_btn.button.mutate(ctx, |ctx, btn| {
                                btn.set_pressed(ctx, false);
                            });
                        },
                        _ => {},
                    },
                    ButtonState::BothDown | ButtonState::OneReleased(_) => {
                        self.left_btn.button.mutate(ctx, |ctx, btn| {
                            btn.set_pressed(ctx, false);
                        });
                        self.right_btn.button.mutate(ctx, |ctx, btn| {
                            btn.set_pressed(ctx, false);
                        });
                        self.middle_btn.button.mutate(ctx, |ctx, btn| {
                            btn.set_pressed(ctx, true);
                        });
                    },
                };

                self.state = new_state;
                event
            }
            _ => None,
        }
    }

    fn paint(&mut self) {
        self.pad.paint();


        if matches!(self.left_btn.button_type, ButtonType::NormalButton) {
            self.left_btn.button.paint();
        } else if matches!(self.left_btn.button_type, ButtonType::HoldToConfirm) {
            self.left_btn.hold_to_confirm.paint();
        }

        if matches!(self.middle_btn.button_type, ButtonType::NormalButton) {
            self.middle_btn.button.paint();
        } else if matches!(self.middle_btn.button_type, ButtonType::HoldToConfirm) {
            self.middle_btn.hold_to_confirm.paint();
        }

        if matches!(self.right_btn.button_type, ButtonType::NormalButton) {
            self.right_btn.button.paint();
        } else if matches!(self.right_btn.button_type, ButtonType::HoldToConfirm) {
            self.right_btn.hold_to_confirm.paint();
        }
    }

    fn place(&mut self, bounds: Rect) -> Rect {
        // Saving button area so that we can re-place the buttons
        // when when they get updated
        self.button_area = bounds;

        self.pad.place(bounds);

        self.left_btn.button.place(bounds);
        self.left_btn.hold_to_confirm.place(bounds);

        self.middle_btn.button.place(bounds);
        self.middle_btn.hold_to_confirm.place(bounds);

        self.right_btn.button.place(bounds);
        self.right_btn.hold_to_confirm.place(bounds);

        bounds
    }
}
