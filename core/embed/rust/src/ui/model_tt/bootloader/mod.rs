use core::slice;

use crate::ui::component::text::formatted::FormattedText;
use crate::ui::component::{Component, Event, EventCtx};
use cstr_core::CStr;
use crate::ui::model_tt::theme::TTBootloaderTextTemp;
use crate::ui::event::TouchEvent;
use crate::trezorhal::io::{io_touch_read, io_touch_unpack_x, io_touch_unpack_y};
use crate::ui::display;
use crate::ui::model_tt::constant;

pub mod confirm;
pub mod menu;
pub mod intro;
pub mod progress;

use confirm::Install;
use progress::BldProgress;
use menu::BldMenu;
use intro::BldIntro;


pub trait ReturnToC {
    fn return_to_c(&self) -> u32;
}
pub struct BootloaderLayout<F> {
    frame: F,
}

impl<F> BootloaderLayout<F>
where F: Component,
      F::Msg: ReturnToC{
    pub fn new(frame: F) -> BootloaderLayout<F> {
        Self {
            frame,
        }
    }

    pub fn process(&mut self) -> u32 {
        self.frame.place(constant::screen());
        self.frame.paint();
        display::fadein();

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
        return None
    }
    let event_type = event >> 24;
    let x = io_touch_unpack_x(event) as u32;
    let y = io_touch_unpack_y(event) as u32;
    let event = TouchEvent::new(event_type, x, y);

    if let Ok(event) = event {
        return Some(event)
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
) -> u32{
    let ptr = vendor_str as *const u8;
    let text = unsafe {CStr::from_bytes_with_nul_unchecked(slice::from_raw_parts(ptr, (vendor_str_len as usize)+1)).to_str().unwrap()};
    let version = unsafe { CStr::from_ptr(version).to_str().unwrap() };

    const ICON: Option<&'static [u8]> = Some(include_res!("model_tt/res/info.toif"));
    //const ICON: Option<&'static [u8]> = None;

    let title =
        if downgrade {"Downgrade firmware"}
        else if vendor {"Vendor change"}
        else {"Update firmware"};

    let mut frame = Install::new(
        title,
        ICON,
        FormattedText::new::<TTBootloaderTextTemp>(
            "{text}\n{msg}\n{version}",
        )   .with("text", "Install firmware by")
            .with("msg", text)
            .with("version", version),

    );

    if vendor || downgrade {
        frame.add_warning("Seed will be erased!");
    }

    let mut layout = BootloaderLayout::new(frame);
    return layout.process();

}

#[no_mangle]
extern "C" fn screen_wipe_confirm() -> u32 {

    const ICON: Option<&'static [u8]> = Some(include_res!("model_tt/res/info.toif"));

    let mut frame = Install::new(
        "Wipe device",
        ICON,
        FormattedText::new::<TTBootloaderTextTemp>(
            "{text}",
        ).with("text", "Do you want to wipe the device?")
    );
    frame.add_warning("Seed will be erased!");

    let mut layout = BootloaderLayout::new(frame);
    return layout.process();
}



#[no_mangle]
extern "C" fn screen_menu() -> u32 {
    let mut layout = BootloaderLayout::new(BldMenu::new());
    return layout.process()
}


#[no_mangle]
extern "C" fn screen_intro() -> u32 {
    let mut layout = BootloaderLayout::new(BldIntro::new());
    return layout.process()
}

#[no_mangle]
extern "C" fn screen_progress(text: *const cty::c_char, progress: u16, initialize: bool) -> u32 {
    let text = unsafe { CStr::from_ptr(text).to_str().unwrap() };
    let mut frame = BldProgress::new(text, initialize);

    frame.place(constant::screen());
    frame.set_progress(progress);
    frame.paint();
    0
}
