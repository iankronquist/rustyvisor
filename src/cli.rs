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
        ClearLocalInterruptsGuard {
            guarded: guarded,
            count: ATOMIC_USIZE_INIT,
        }
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
        if CLI_COUNT.get().fetch_sub(1, Ordering::AcqRel) == 1 {
            sti();
        }
    }
}

#[cfg(feature = "runtime_tests")]
pub mod runtime_tests {
    use super::are_interrupts_enabled;
    use super::ClearLocalInterruptsGuard;

    struct Guardee {
        data: u16,
    }

    lazy_static! {
        static ref STATIC_GUARDEE: ClearLocalInterruptsGuard<Guardee> = ClearLocalInterruptsGuard::new(Guardee { data: 0 });
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
            let mut g: ClearLocalInterruptsGuard<Guardee> = ClearLocalInterruptsGuard::new(Guardee { data: 0 });
            assert!(are_interrupts_enabled());
            (*g).data += 1;
            assert!(!are_interrupts_enabled());
            assert_eq!((*g).data, 1);
            assert!(!are_interrupts_enabled());
        }
        assert!(are_interrupts_enabled());
    }


    fn test_clear_local_interrupts_guard_borrow() {
        assert!(are_interrupts_enabled());
        {
            let g: ClearLocalInterruptsGuard<Guardee> = ClearLocalInterruptsGuard::new(Guardee { data: 0 });
            assert!(are_interrupts_enabled());
            (*g).do_work();
            assert!(!are_interrupts_enabled());
        }
        assert!(are_interrupts_enabled());
    }


    fn test_clear_local_interrupts_guard_static_borrow() {
        assert!(are_interrupts_enabled());
        {
            assert!(are_interrupts_enabled());
            (*STATIC_GUARDEE).do_work();
            assert!(!are_interrupts_enabled());
            assert_eq!((*STATIC_GUARDEE).data, 0);
            assert!(!are_interrupts_enabled());
        }
        assert!(are_interrupts_enabled());
    }


}
