use core::ops;
use vmx;
use cpu;
use core::sync::atomic::{AtomicUsize, Ordering};


lazy_static! {
    static ref CLI_COUNT: cpu::PerCoreVariable<AtomicUsize> = Default::default();
}


pub struct ClearLocalInterruptsGuard<T>{
    guarded: T,
}


impl<T> ClearLocalInterruptsGuard<T> {
    pub fn new(guarded: T) -> Self {
        ClearLocalInterruptsGuard { guarded: guarded }
    }
}


impl<T> ops::Deref for ClearLocalInterruptsGuard<T> {
    type Target = T;
    fn deref(&self) -> &T {
        vmx::cli();
        CLI_COUNT.get().fetch_add(1, Ordering::AcqRel);
        &self.guarded
    }
}


impl<T> ops::DerefMut for ClearLocalInterruptsGuard<T> {
    fn deref_mut(&mut self) -> &mut T {
        vmx::cli();
        &mut self.guarded
    }
}


impl<T: Default> Default for ClearLocalInterruptsGuard<T> {
    fn default() -> Self {
        ClearLocalInterruptsGuard{ guarded: Default::default() }
    }
}


impl<T> Drop for ClearLocalInterruptsGuard<T> {
    fn drop(&mut self) {
        if CLI_COUNT.get().fetch_sub(1, Ordering::AcqRel) == 1 {
            vmx::sti();
        }
    }
}
