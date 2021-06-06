use core::ops;
use vmx;


pub fn are_interrupts_enabled() -> bool {
    (vmx::read_flags() & 0x200) != 0
}


#[cfg(not(test))]
pub fn cli() {
    unsafe {
        asm!("cli");
    }
}


#[cfg(test)]
pub fn cli() {}


#[cfg(not(test))]
pub fn sti() {
    unsafe {
        asm!("sti");
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
    acquired: bool,
}


pub struct ClearLocalInterruptsGuardMut<'a, T: 'a> {
    guarded: &'a mut T,
    acquired: bool,
}



impl<T> ClearLocalInterrupts<T> {
    pub fn new(guarded: T) -> Self {
        ClearLocalInterrupts { guarded: guarded }
    }

    pub fn cli_mut(&mut self) -> ClearLocalInterruptsGuardMut<T> {
        cli();
        ClearLocalInterruptsGuardMut {
            guarded: &mut self.guarded,
            acquired: are_interrupts_enabled(),
        }
    }


    pub fn cli(&self) -> ClearLocalInterruptsGuard<T> {
        cli();
        ClearLocalInterruptsGuard {
            guarded: &self.guarded,
            acquired: are_interrupts_enabled(),
        }
    }
}


impl<'a, T> ops::Deref for ClearLocalInterruptsGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.guarded
    }
}


impl<'a, T> ops::Deref for ClearLocalInterruptsGuardMut<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.guarded
    }
}


impl<'a, T> ops::DerefMut for ClearLocalInterruptsGuardMut<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.guarded
    }
}


impl<'a, T> Drop for ClearLocalInterruptsGuard<'a, T> {
    fn drop(&mut self) {
        if self.acquired {
            sti();
        }
    }
}


impl<'a, T> Drop for ClearLocalInterruptsGuardMut<'a, T> {
    fn drop(&mut self) {
        if self.acquired {
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
