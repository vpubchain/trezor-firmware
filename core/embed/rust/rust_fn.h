//
// Created by mbruna on 3.5.22.
//

#ifndef _RUST_FN_H
#define _RUST_FN_H

uint32_t install_confirm_upgrade(const char * vendor_str, uint8_t vendor_str_len, const char * version_str);
uint32_t screen_wipe_confirm(void);
uint32_t screen_intro(void);
uint32_t screen_menu(void);


#endif //_RUST_FN_H
