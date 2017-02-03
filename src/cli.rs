use core::ops;
use vmx;
use cpu;
use core::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering};


lazy_static! {
    static ref CLI_COUNT: cpu::PerCoreVariable<AtomicUsize> = Default::default();
}


pub fn are_interrupts_enabled() -> bool {
    (vmx::read_flags() & 0x200) != 0
}


pub fn cli() {
    unsafe {
        asm!("cli" : : :);
    }
}


pub fn sti() {
    unsafe {
        asm!("sti" : : :);
    }
}


#[derive(Default)]
pub struct ClearLocalInterruptsGuard<T> {
    guarded: T,
    count: AtomicUsize,
}


impl<T> ClearLocalInterruptsGuard<T> {
    pub fn new(guarded: T) -> Self {
        ClearLocalInterruptsGuard { guarded: guarded, count: ATOMIC_USIZE_INIT }
    }
}


impl<T> ops::Deref for ClearLocalInterruptsGuard<T> {
    type Target = T;
    fn deref(&self) -> &T {
        cli();
        if self.count.fetch_add(1, Ordering::AcqRel) == 0 {
            CLI_COUNT.get().fetch_add(1, Ordering::AcqRel);
        }
        &self.guarded
    }
}


impl<T> ops::DerefMut for ClearLocalInterruptsGuard<T> {
    fn deref_mut(&mut self) -> &mut T {
        cli();
        if self.count.fetch_add(1, Ordering::AcqRel) == 0 {
            CLI_COUNT.get().fetch_add(1, Ordering::AcqRel);
        }
        &mut self.guarded
    }
}


impl<T> Drop for ClearLocalInterruptsGuard<T> {
    fn drop(&mut self) {
        if self.count.fetch_sub(1, Ordering::AcqRel) == 1 && CLI_COUNT.get().fetch_sub(1, Ordering::AcqRel) == 1 {
                sti();
        }
    }
}
