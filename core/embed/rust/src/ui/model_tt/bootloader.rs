use core::slice;
use super::{
    theme, constant,
    component::Install,
    component::Dialog,
    component::Button,
};
use crate::{ui::component::text::formatted::FormattedText};
use crate::ui::component::{Component};
use cstr_core::CStr;
use crate::ui::model_tt::component::{BldIntro};
use crate::ui::model_tt::theme::{TTBootloaderText};

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


#[no_mangle]
extern "C" fn screen_menu() {
}


#[no_mangle]
extern "C" fn screen_intro() {
    let mut frame = BldIntro::new();
    frame.place(constant::screen());
    frame.paint();

}