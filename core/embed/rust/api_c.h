#ifndef _API_C_H
#define _API_C_H

uint32_t screen_install_confirm(
        const char * vendor_str,
        uint8_t vendor_str_len,
        const char * version_str,
        bool downgrade,
        bool vendor
        );
uint32_t screen_wipe_confirm(void);
uint32_t screen_progress(const char * text, uint16_t progress, bool initialize);
uint32_t screen_intro(void);
uint32_t screen_menu(void);
uint32_t screen_connect(void);


#endif //_API_C_H
