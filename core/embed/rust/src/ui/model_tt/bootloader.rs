use core::slice;
use super::{
    theme, constant,
    component::Install,
    component::Dialog,
    component::Button,
};
use crate::{ui::component::text::formatted::FormattedText};
use crate::ui::component::{Component, Event, EventCtx};
use cstr_core::CStr;
use crate::error::Error;
use crate::ui::model_tt::component::{BldIntro, BldIntroMsg, BldMenu, BldMenuMsg};
use crate::ui::model_tt::theme::{TTBootloaderText};
use crate::ui::event::TouchEvent;
use crate::ui::model_tt::component::ButtonMsg::{Clicked};
use crate::trezorhal::io::{io_touch_read, io_touch_unpack_x, io_touch_unpack_y, io_usb_process};
use crate::ui::display;

#[no_mangle]
extern "C" fn hello_world(text: *const cty::c_char) {
    let text = unsafe { CStr::from_ptr(text).to_str().unwrap() };
    let mut frame = Dialog::new(
        FormattedText::new::<theme::TTDefaultText>(
            "Testing text layout, with some text, and some more text. And {param}",
        )
            .with("param", text),
        Button::with_text("Left"),
        Button::with_text("Right"),
    );
    frame.place(constant::screen());
    frame.paint();
}



#[no_mangle]
extern "C" fn install_confirm_upgrade(vendor_str: *const cty::c_char, vendor_str_len: u8, version: *const cty::c_char) {
    let ptr = vendor_str as *const u8;
    let text = unsafe {CStr::from_bytes_with_nul_unchecked(slice::from_raw_parts(ptr, (vendor_str_len as usize)+1)).to_str().unwrap()};
    let version = unsafe { CStr::from_ptr(version).to_str().unwrap() };


    const ICON: Option<&'static [u8]> = Some(include_res!("model_tt/res/info.toif"));
    //const ICON: Option<&'static [u8]> = None;

    let mut frame = Install::new(
        "Firmware update",
        ICON,
        FormattedText::new::<TTBootloaderText>(
            "{text}\n{msg}\n{version}",
        )   .with("text", "Update firmware by")
            .with("msg", text)
            .with("version", version),

    );
    frame.place(constant::screen());
    frame.paint();

}

#[no_mangle]
extern "C" fn screen_wipe_confirm() {

    const ICON: Option<&'static [u8]> = Some(include_res!("model_tt/res/info.toif"));
    //const ICON: Option<&'static [u8]> = None;

    let mut frame = Install::new(
        "Wipe device",
        ICON,
        FormattedText::new::<TTBootloaderText>(
            "{text}",
        ).with("text", "Do you want to wipe the device?")
    );
    frame.add_warning("Seed will be erased!");
    frame.place(constant::screen());
    frame.paint();

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

fn usb_eval() -> u32 {
    let usb_result = io_usb_process();

    if usb_result == 0 {
        return 0xBBBB_BBBB_u32;
    }
    if usb_result == 0xAAAAAAAA_u32 {
        return 0xAAAA_AAAA_u32;
    }
    return 0;
}


#[no_mangle]
extern "C" fn screen_menu() -> u32 {
    let mut frame = BldMenu::new();
    frame.place(constant::screen());
    frame.paint();
    display::fadein();
    loop {
        let msg = touch_eval();
        if let Some(e) = event {
            let mut ctx = EventCtx::new();
            let msg = frame.event(&mut ctx, Event::Touch(e));
            frame.repaint();
            match msg {
                Some(BldMenuMsg::Close(Clicked)) => return 1,
                Some(BldMenuMsg::Reboot(Clicked)) => return 2,
                _ => (),
            };
        }
        let usb = usb_eval();
        if usb != 0 {return usb};
    }
}


#[no_mangle]
extern "C" fn screen_intro() -> u32 {
    let mut frame = BldIntro::new();
    frame.place(constant::screen());
    frame.paint();
    display::fadein();

    loop {
        let event = touch_eval();

        if let Some(e) = event {
            let mut ctx = EventCtx::new();
            let msg = frame.event(&mut ctx, Event::Touch(e));
            frame.repaint();
            match msg {
                Some(BldIntroMsg::Menu(Clicked)) => return 1,
                _ => (),
            };
        }

        let usb = usb_eval();
        if usb != 0 {return usb};
    }
}