use crate::{
    trezorhal::io::{io_touch_read, io_touch_unpack_x, io_touch_unpack_y},
    ui::{
        component::{Component, Event, EventCtx},
        display,
        event::TouchEvent,
        model_tt::{bootloader::theme::TTBootloaderTextTemp, constant},
    },
};
use cstr_core::CStr;

pub mod confirm;
mod connect;
mod fwinfo;
pub mod intro;
pub mod menu;
pub mod progress;
mod theme;
mod title;

use crate::ui::{
    component::text::paragraphs::Paragraphs,
    geometry::LinearPlacement,
    model_tt::{
        bootloader::connect::Connect,
        theme::{BACKLIGHT_DIM, BACKLIGHT_NORMAL, FONT_NORMAL},
    },
};
use confirm::Confirm;
use fwinfo::FwInfo;
use intro::Intro;
use menu::Menu;
use progress::Progress;

pub trait ReturnToC {
    fn return_to_c(&self) -> u32;
}
pub struct BootloaderLayout<F> {
    frame: F,
}

fn fadein() {
    display::fade_backlight_duration(BACKLIGHT_NORMAL, 1000);
}

fn fadeout() {
    display::fade_backlight_duration(BACKLIGHT_DIM, 1000);
}

unsafe fn from_c_str(c_str: *const cty::c_char) -> Option<&'static str> {
    unsafe {
        let bytes = CStr::from_ptr(c_str).to_bytes();
        if bytes.is_ascii() {
            Some(core::str::from_utf8_unchecked(bytes))
        } else {
            None
        }
    }
}

unsafe fn from_c_array(c_str: *const cty::c_char, len: usize) -> Option<&'static str> {
    unsafe {
        let slice = core::slice::from_raw_parts(c_str as *const u8, len);
        core::str::from_utf8(slice).ok()
    }
}

impl<F> BootloaderLayout<F>
where
    F: Component,
    F::Msg: ReturnToC,
{
    pub fn new(frame: F) -> BootloaderLayout<F> {
        Self { frame }
    }

    pub fn process(&mut self) -> u32 {
        self.frame.place(constant::screen());
        self.frame.paint();
        fadein();

        loop {
            let event = touch_eval();
            if let Some(e) = event {
                let mut ctx = EventCtx::new();
                let msg = self.frame.event(&mut ctx, Event::Touch(e));

                self.frame.paint();
                if let Some(message) = msg {
                    return message.return_to_c();
                }
            }
        }
    }
}

fn touch_eval() -> Option<TouchEvent> {
    let event = io_touch_read();
    if event == 0 {
        return None;
    }
    let event_type = event >> 24;
    let x = io_touch_unpack_x(event) as u32;
    let y = io_touch_unpack_y(event) as u32;
    let event = TouchEvent::new(event_type, x, y);

    if let Ok(event) = event {
        return Some(event);
    }
    None
}

#[no_mangle]
extern "C" fn screen_install_confirm(
    vendor_str: *const cty::c_char,
    vendor_str_len: u8,
    version: *const cty::c_char,
    downgrade: bool,
    vendor: bool,
) -> u32 {
    let text = unwrap!(
        unsafe { from_c_array(vendor_str, vendor_str_len as usize) },
        "Invalid vendor string"
    );
    let version = unwrap!(unsafe { from_c_str(version) }, "Invalid version string");

    const ICON: Option<&'static [u8]> = Some(include_res!("model_tt/res/info.toif"));
    //const ICON: Option<&'static [u8]> = None;

    let title = if downgrade {
        "Downgrade firmware"
    } else if vendor {
        "Vendor change"
    } else {
        "Update firmware"
    };

    let message = Paragraphs::new()
        .add::<TTBootloaderTextTemp>(FONT_NORMAL, "Install firmware by")
        .add::<TTBootloaderTextTemp>(FONT_NORMAL, text)
        .add::<TTBootloaderTextTemp>(FONT_NORMAL, version)
        .with_placement(LinearPlacement::vertical().align_at_start());

    let mut frame = Confirm::new(title, ICON, message);

    if vendor || downgrade {
        frame.add_warning("Seed will be erased!");
    }

    let mut layout = BootloaderLayout::new(frame);
    return layout.process();
}

#[no_mangle]
extern "C" fn screen_wipe_confirm() -> u32 {
    const ICON: Option<&'static [u8]> = Some(include_res!("model_tt/res/info.toif"));

    let message = Paragraphs::new()
        .add::<TTBootloaderTextTemp>(FONT_NORMAL, "Do you want to wipe the device?")
        .with_placement(LinearPlacement::vertical().align_at_start());

    let mut frame = Confirm::new("Wipe device", ICON, message);
    frame.add_warning("Seed will be erased!");

    let mut layout = BootloaderLayout::new(frame);
    return layout.process();
}

#[no_mangle]
extern "C" fn screen_menu(bld_version: *const cty::c_char) -> u32 {
    let bld_version = unwrap!(unsafe { from_c_str(bld_version) });

    let mut layout = BootloaderLayout::new(Menu::new(bld_version));
    return layout.process();
}

#[no_mangle]
extern "C" fn screen_intro(
    bld_version: *const cty::c_char,
    vendor_str: *const cty::c_char,
    vendor_str_len: u8,
    version: *const cty::c_char,
) -> u32 {
    let vendor = unwrap!(unsafe { from_c_array(vendor_str, vendor_str_len as usize) });
    let version = unwrap!(unsafe { from_c_str(version) });
    let bld_version = unwrap!(unsafe { from_c_str(bld_version) });

    let mut layout = BootloaderLayout::new(Intro::new(bld_version, vendor, version));
    return layout.process();
}

#[no_mangle]
extern "C" fn screen_progress(text: *const cty::c_char, progress: u16, initialize: bool) -> u32 {
    let text = unwrap!(unsafe { from_c_str(text) });
    let mut frame = Progress::new(text, initialize);

    frame.place(constant::screen());
    frame.set_progress(progress);
    frame.paint();
    0
}

#[no_mangle]
extern "C" fn screen_connect() -> u32 {
    let mut frame = Connect::new("Waiting for host");

    frame.place(constant::screen());
    frame.paint();
    fadein();
    0
}

#[no_mangle]
extern "C" fn screen_fwinfo(fingerprint: *const cty::c_char) -> u32 {
    let fingerprint = unwrap!(unsafe { from_c_str(fingerprint) });

    let mut layout = BootloaderLayout::new(FwInfo::new(fingerprint));
    return layout.process();
}
