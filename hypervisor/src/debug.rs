//! This module defines functions useful for debugging the hypervisor.

/// A breakpoint function for debuggers to hook.
///
/// This does NOT generate an int 3 breakpoint instruction.
/// Instead it does the bochs magic breakpoint instruction to cause bochs to break if we are running under bochs and that feature is enabled.
/// This function is never inlined so a debugger like GDB can hook it more easily.
#[inline(never)]
pub fn breakpoint() {
    // If feature inline asm...
    // Bochs magic breakpoint.
    unsafe {
        asm!("xchg bx, bx");
    }
}
