
extern "C" {
    fn touch_read() -> cty::c_uint;
    fn touch_unpack_x(event: cty::c_uint) -> cty::c_ushort;
    fn touch_unpack_y(event: cty::c_uint) -> cty::c_ushort;
    fn bootloader_usb_process() -> cty::c_uint;
}


pub fn io_touch_read() -> u32 {
    unsafe { touch_read() }
}
pub fn io_touch_unpack_x(event: u32) -> u16 { unsafe { touch_unpack_x(event) } }
pub fn io_touch_unpack_y(event: u32) -> u16 { unsafe { touch_unpack_y(event) } }
pub fn io_usb_process() -> u32 { unsafe { bootloader_usb_process() } }

