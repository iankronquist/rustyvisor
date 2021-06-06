use vmx;
use core::sync::atomic::{ATOMIC_U16_INIT, AtomicU16, Ordering};

const CPU_COUNT_MAX: usize = 32;


static CPU_ASSIGNMENT: AtomicU16 = ATOMIC_U16_INIT;


pub fn bring_core_online() {
    let cpu_num = CPU_ASSIGNMENT.fetch_add(1, Ordering::Relaxed);
    set_number(cpu_num);
}


fn set_number(num: u16) {
    vmx::write_fs(num);
}


pub fn get_number() -> u16 {
    vmx::read_fs()
}


#[derive(Default)]
pub struct PerCoreVariable<T> {
    vars: [T; CPU_COUNT_MAX],
}



impl<T> PerCoreVariable<T> {
    pub fn get(&self) -> &T {
        &self.vars[get_number() as usize]
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.vars[get_number() as usize]
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_default() {
        let pcv: PerCoreVariable<u32> = Default::default();
        let def: u32 = Default::default();
        for variable in &pcv.vars {
            assert_eq!(*variable, def);
        }
    }

    #[test]
    fn test_get_mut() {
        let mut pcv: PerCoreVariable<u32> = Default::default();
        assert_eq!(*pcv.get(), 0);
        {
            let b = pcv.get_mut();
            *b = 42;
        }
        assert_eq!(*pcv.get(), 42);
    }


    #[test]
    fn test_get() {
        let mut pcv: PerCoreVariable<u32> = Default::default();
        assert_eq!(*pcv.get(), 0);
        pcv.vars[0] = 42;
        assert_eq!(*pcv.get(), 42);
    }

    #[test]
    fn test_atomic() {
        use core::sync::atomic::AtomicUsize;
        let mut pcv: PerCoreVariable<AtomicUsize> = Default::default();
        assert_eq!(pcv.get().load(Ordering::Relaxed), 0);
        {
            let b = pcv.get_mut();
            b.store(42, Ordering::Relaxed);
        }
        assert_eq!(pcv.get().load(Ordering::Relaxed), 42);

    }
}

#[cfg(feature = "runtime_tests")]
pub mod runtime_tests {
    use super::*;

    pub fn run() {
        info!("Running CPU tests...");
        test_atomic();
        info!("CPU succeeded.");
    }

    fn test_atomic() {
        use core::sync::atomic::AtomicUsize;
        let mut pcv: PerCoreVariable<AtomicUsize> = Default::default();
        assert_eq!(pcv.get().load(Ordering::Relaxed), 0);
        {
            let b = pcv.get_mut();
            b.store(42, Ordering::Relaxed);
        }
        assert_eq!(pcv.get().load(Ordering::Relaxed), 42);

    }
}
