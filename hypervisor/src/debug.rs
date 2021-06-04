#[inline(never)]
pub fn breakpoint() {
    // If feature inline asm...
    // Bochs magic breakpoint.
    asm!("xchg bx, bx");
}