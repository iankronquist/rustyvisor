use core::ops;
use vmx;
use cpu;
use core::sync::atomic::{AtomicUsize, AtomicBool, ATOMIC_BOOL_INIT, Ordering};

lazy_static! {
    static ref CLI_COUNT: cpu::PerCoreVariable<AtomicUsize> = Default::default();
}


pub fn are_interrupts_enabled() -> bool {
    (vmx::read_flags() & 0x200) != 0
}


#[cfg(not(test))]
pub fn cli() {
    unsafe {
        asm!("cli" : : :);
    }
}


#[cfg(test)]
pub fn cli() {}


#[cfg(not(test))]
pub fn sti() {
    unsafe {
        asm!("sti" : : :);
    }
}


#[cfg(test)]
pub fn sti() {}


#[derive(Default)]
pub struct ClearLocalInterrupts<T> {
    guarded: T,
}


pub struct ClearLocalInterruptsGuard<'a, T: 'a> {
    guarded: &'a T,
    acquired: AtomicBool,
}


pub struct ClearLocalInterruptsGuardMut<'a, T: 'a> {
    guarded: &'a mut T,
    acquired: AtomicBool,
}



impl<T> ClearLocalInterrupts<T> {
    pub fn new(guarded: T) -> Self {
        ClearLocalInterrupts { guarded: guarded }
    }

    pub fn cli_mut(&mut self) -> ClearLocalInterruptsGuardMut<T> {
        ClearLocalInterruptsGuardMut {
            guarded: &mut self.guarded,
            acquired: ATOMIC_BOOL_INIT,
        }
    }


    pub fn cli(&self) -> ClearLocalInterruptsGuard<T> {
        ClearLocalInterruptsGuard {
            guarded: &self.guarded,
            acquired: ATOMIC_BOOL_INIT,
        }
    }
}


impl<'a, T> ops::Deref for ClearLocalInterruptsGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        if !self.acquired.compare_and_swap(false, true, Ordering::AcqRel) &&
           CLI_COUNT.get().fetch_add(1, Ordering::AcqRel) == 0 {
            cli();
        }
        self.guarded
    }
}


impl<'a, T> ops::Deref for ClearLocalInterruptsGuardMut<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        if !self.acquired.compare_and_swap(false, true, Ordering::AcqRel) &&
           CLI_COUNT.get().fetch_add(1, Ordering::AcqRel) == 0 {
            cli();
        }
        self.guarded
    }
}


impl<'a, T> ops::DerefMut for ClearLocalInterruptsGuardMut<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        if !self.acquired.compare_and_swap(false, true, Ordering::AcqRel) &&
           CLI_COUNT.get().fetch_add(1, Ordering::AcqRel) == 0 {
            cli();
        }
        self.guarded
    }
}


impl<'a, T> Drop for ClearLocalInterruptsGuard<'a, T> {
    fn drop(&mut self) {
        if CLI_COUNT.get().fetch_sub(1, Ordering::AcqRel) == 1 {
            sti();
        }
    }
}


impl<'a, T> Drop for ClearLocalInterruptsGuardMut<'a, T> {
    fn drop(&mut self) {
        if CLI_COUNT.get().fetch_sub(1, Ordering::AcqRel) == 1 {
            sti();
        }
    }
}


#[cfg(feature = "runtime_tests")]
pub mod runtime_tests {
    use super::are_interrupts_enabled;
    use super::ClearLocalInterrupts;

    struct Guardee {
        data: u16,
    }

    lazy_static! {
        static ref STATIC_GUARDEE: ClearLocalInterrupts<Guardee> =
            ClearLocalInterrupts::new(Guardee { data: 0 });
    }

    impl Guardee {
        fn do_work(&self) {}
    }

    pub fn run() {
        info!("Running CLI tests...");
        test_clear_local_interrupts_guard_borrow();
        test_clear_local_interrupts_guard_mut_borrow();
        test_clear_local_interrupts_guard_static_borrow();
        info!("CLI tests succeeded");
    }


    fn test_clear_local_interrupts_guard_mut_borrow() {
        assert!(are_interrupts_enabled());
        {
            let mut g: ClearLocalInterrupts<Guardee> =
                ClearLocalInterrupts::new(Guardee { data: 0 });
            assert!(are_interrupts_enabled());
            g.cli_mut().data += 1;
            assert!(are_interrupts_enabled());
            assert_eq!(g.cli().data, 1);
            assert!(are_interrupts_enabled());
        }
        assert!(are_interrupts_enabled());
    }


    fn test_clear_local_interrupts_guard_borrow() {
        assert!(are_interrupts_enabled());
        {
            let g: ClearLocalInterrupts<Guardee> = ClearLocalInterrupts::new(Guardee { data: 0 });
            assert!(are_interrupts_enabled());
            let borrowed = g.cli();
            assert!(are_interrupts_enabled());
            borrowed.do_work();
            assert!(!are_interrupts_enabled());
        }
        assert!(are_interrupts_enabled());
    }


    fn test_clear_local_interrupts_guard_static_borrow() {
        assert!(are_interrupts_enabled());
        {
            assert!(are_interrupts_enabled());
            STATIC_GUARDEE.cli().do_work();
            assert!(are_interrupts_enabled());
            assert_eq!(STATIC_GUARDEE.cli().data, 0);
            assert!(are_interrupts_enabled());
        }
        assert!(are_interrupts_enabled());
    }
}
