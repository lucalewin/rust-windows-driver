#![no_std]
#![cfg(not(test))] // removes `found duplicate lang item `panic_impl`` error

use core::panic::PanicInfo;
use win_sys::base::{
    DRIVER_OBJECT as DriverObject, NTSTATUS as Status, STATUS_SUCCESS,
    UNICODE_STRING as UnicodeString,
};
use win_sys::ntoskrnl::DbgPrint as debug_print;

#[used]
#[no_mangle]
pub static _fltused: i32 = 0;

#[no_mangle]
pub extern "system" fn __CxxFrameHandler3() -> i32 {
    0
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "system" fn driver_entry(driver: &mut DriverObject, _: &UnicodeString) -> Status {
    unsafe {
        debug_print("Hello, World!\0".as_ptr() as _);
    }

    driver.DriverUnload = Some(driver_exit);

    STATUS_SUCCESS
}

pub extern "C" fn driver_exit(_driver: *mut DriverObject) {
    unsafe {
        debug_print("Goodbye, World!\0".as_ptr() as _);
    }
}
