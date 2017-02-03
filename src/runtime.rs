#![cfg(not(test))]

use core::fmt;
use core::fmt::Write;

use serial_logger;
use serial_logger::write_static;


#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {
    write_static("PANIC: eh_personality\n");
}

#[lang = "eh_unwind_resume"]
#[no_mangle]
pub extern "C" fn rust_eh_unwind_resume() {
    write_static("PANIC: rust_eh_unwind_resume\n");
}

#[no_mangle]
pub extern "C" fn __udivti3() {
    write_static("ERROR: Unimplemented intrinsic __udivti3\n");
}

#[no_mangle]
pub extern "C" fn __umodti3() {
    write_static("ERROR: Unimplemented intrinsic __umodti3\n");
}

#[no_mangle]
pub extern "C" fn __muloti4() {
    write_static("ERROR: Unimplemented intrinsic __muloti4\n");
}

#[no_mangle]
pub extern "C" fn __floatundisf() {
    write_static("ERROR: Unimplemented intrinsic __floatundisf\n");
}

#[no_mangle]
pub extern "C" fn __floatundidf() {
    write_static("ERROR: Unimplemented intrinsic __floatundidf\n");
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() {
    write_static("PANIC: _Unwind_Resume\n");
}


#[allow(empty_loop)]
#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn panic_fmt(fmt: fmt::Arguments, _file_line: &(&'static str, u32)) -> ! {

    write_static("PANIC: \n");
    let mut logger: serial_logger::SerialLogger = Default::default();
    let _ = write!(logger, "{}\n", fmt);

    loop {
        unsafe {
            asm!("cli; hlt;");
        }
    }
}
