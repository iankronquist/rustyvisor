#![cfg(not(test))]
use ::log::{error};

use core::sync::atomic;

use core::panic::PanicInfo;

// Prevent recursive panicking.
static HAVE_PANICKED: atomic::AtomicBool = atomic::AtomicBool::new(false);

#[no_mangle]
#[panic_handler]
pub extern "C" fn panic_fmt(info: &PanicInfo) -> ! {
    if HAVE_PANICKED.compare_exchange(false, true, atomic::Ordering::SeqCst, atomic::Ordering::SeqCst).is_ok() {
        unsafe {
            crate::LOGGER.bust_locks();
        }

        error!("PANIC: {}", info);
    }

    loop {
        unsafe {
            asm!("cli; hlt;");
        }
    }
}
