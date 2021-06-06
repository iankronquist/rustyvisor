#![cfg(not(test))]

use core::sync::atomic;

use core::panic::PanicInfo;

use crate::UNSYNCHRONIZED_LOGGER;

// Prevent recursive panicking.
static HAVE_PANICKED: atomic::AtomicBool = atomic::AtomicBool::new(false);

#[no_mangle]
#[panic_handler]
pub extern "C" fn panic_fmt(info: &PanicInfo) -> ! {
    if HAVE_PANICKED
        .compare_exchange(
            false,
            true,
            atomic::Ordering::SeqCst,
            atomic::Ordering::SeqCst,
        )
        .is_ok()
    {
        write!(UNSYNCHRONIZED_LOGGER, "PANIC: {}", info);
    }

    loop {
        unsafe {
            asm!("cli; hlt;");
        }
    }
}
