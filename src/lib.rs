#![crate_type = "staticlib"]
#![feature(lang_items)]
#![no_std]

#[no_mangle]
pub extern "C" fn rust_hello() -> *const u8 {
        "Hello, world!".as_ptr()
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }