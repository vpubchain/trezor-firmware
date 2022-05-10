use core::slice;
use super::{
    theme, constant,
    component::Install,
    component::Dialog,
    component::Button,
    component::Icon,
};
use crate::{ui::component::text::formatted::FormattedText, trezorhal::display};
use crate::ui::component::{Component, ComponentExt};
use cstr_core::CStr;
use crate::ui::component::text::layout::DefaultTextTheme;
use crate::ui::display::{Color, Font};
use crate::ui::model_tt::theme::{BG, FG, FONT_BOLD, FONT_MEDIUM, FONT_MONO, FONT_NORMAL, GREY_LIGHT};

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


pub struct TTBootloaderText;

impl DefaultTextTheme for TTBootloaderText {
    const BACKGROUND_COLOR: Color = FG;
    const TEXT_FONT: Font = FONT_NORMAL;
    const TEXT_COLOR: Color = BG;
    const HYPHEN_FONT: Font = FONT_BOLD;
    const HYPHEN_COLOR: Color = GREY_LIGHT;
    const ELLIPSIS_FONT: Font = FONT_BOLD;
    const ELLIPSIS_COLOR: Color = GREY_LIGHT;

    const NORMAL_FONT: Font = FONT_NORMAL;
    const MEDIUM_FONT: Font = FONT_MEDIUM;
    const BOLD_FONT: Font = FONT_BOLD;
    const MONO_FONT: Font = FONT_MONO;
}

#[no_mangle]
extern "C" fn install_confirm_upgrade(vendor_str: *const cty::c_char, vendor_str_len: u8, version: *const cty::c_char) {
    let ptr = vendor_str as *const u8;
    let text = unsafe {CStr::from_bytes_with_nul_unchecked(slice::from_raw_parts(ptr, (vendor_str_len as usize)+1)).to_str().unwrap()};
    let version = unsafe { CStr::from_ptr(version).to_str().unwrap() };


    const ICON: &'static [u8] = include_res!("model_tt/res/info.toif");

    let mut frame = Install::new(
        "Firmware vupdate",
        ICON,
        FormattedText::new::<TTBootloaderText>(
            "{text}\n{msg}\n{version}",
        )   .with("text", "Update firmware by")
            .with("msg", text)
            .with("version", version),

    );
    frame.place(constant::screen());
    frame.paint();

    // display_bar(0, 0, DISPLAY_RESX, DISPLAY_RESY, COLOR_BL_BG);
    // display_text(16, 32, "Firmware update", -1, FONT_NORMAL, COLOR_BL_FG,
    //              COLOR_BL_BG);
    // display_bar(16, 44, DISPLAY_RESX - 14 * 2, 1, COLOR_BL_FG);
    // display_icon(16, 54, 32, 32, toi_icon_info + 12, sizeof(toi_icon_info) - 12,
    //              COLOR_BL_FG, COLOR_BL_BG);
    // display_text(55, 70, "Update firmware by", -1, FONT_NORMAL, COLOR_BL_FG,
    //              COLOR_BL_BG);

    // int split = display_text_split(text, textlen, FONT_NORMAL, DISPLAY_RESX - 55);
    // if (split >= textlen) {
    //     display_text(55, 95, text, textlen, FONT_NORMAL, fgcolor, COLOR_BL_BG);
    //     return 120;
    // } else {
    //     display_text(55, 95, text, split, FONT_NORMAL, fgcolor, COLOR_BL_BG);
    //     if (text[split] == ' ') {
    //         split++;
    //     }
    //     display_text(55, 120, text + split, textlen - split, FONT_NORMAL, fgcolor,
    //                  COLOR_BL_BG);
    //     return 145;
    // }

    // int next_y = display_vendor_string(vhdr->vstr, vhdr->vstr_len, COLOR_BL_FG);
    // const char *ver_str = format_ver("to version %d.%d.%d?", hdr->version);
    // display_text(55, next_y, ver_str, -1, FONT_NORMAL, COLOR_BL_FG, COLOR_BL_BG);
    // display_bar_radius(9, 184, 108, 50, COLOR_BL_FAIL, COLOR_BL_BG, 4);
    // display_icon(9 + (108 - 16) / 2, 184 + (50 - 16) / 2, 16, 16,
    //              toi_icon_cancel + 12, sizeof(toi_icon_cancel) - 12, COLOR_BL_BG,
    //              COLOR_BL_FAIL);
    // display_bar_radius(123, 184, 108, 50, COLOR_BL_DONE, COLOR_BL_BG, 4);
    // display_icon(123 + (108 - 19) / 2, 184 + (50 - 16) / 2, 20, 16,
    //              toi_icon_confirm + 12, sizeof(toi_icon_confirm) - 12,
    //              COLOR_BL_BG, COLOR_BL_DONE);
}