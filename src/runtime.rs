#![cfg(not(test))]

use ::log::{error, log};

use core::panic::PanicInfo;

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {
    error!("PANIC: eh_personality\n");
}

#[lang = "eh_unwind_resume"]
#[no_mangle]
pub extern "C" fn rust_eh_unwind_resume() {
    error!("PANIC: rust_eh_unwind_resume\n");
}

#[no_mangle]
pub extern "C" fn __udivti3() {
    error!("ERROR: Unimplemented intrinsic __udivti3\n");
}

#[no_mangle]
pub extern "C" fn __umodti3() {
    error!("ERROR: Unimplemented intrinsic __umodti3\n");
}

#[no_mangle]
pub extern "C" fn __muloti4() {
    error!("ERROR: Unimplemented intrinsic __muloti4\n");
}

#[no_mangle]
pub extern "C" fn __floatundisf() {
    error!("ERROR: Unimplemented intrinsic __floatundisf\n");
}

#[no_mangle]
pub extern "C" fn __floatundidf() {
    error!("ERROR: Unimplemented intrinsic __floatundidf\n");
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() {
    error!("PANIC: _Unwind_Resume\n");
}

#[no_mangle]
#[panic_handler]
pub extern "C" fn panic_fmt(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        error!(
            "PANIC: {} {} {}",
            info.payload().downcast_ref::<&str>().unwrap(),
            location.file(),
            location.line()
        )
    }

    loop {
        unsafe {
            asm!("cli; hlt;");
        }
    }
}
