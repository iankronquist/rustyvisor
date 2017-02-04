use vmx;
use collections::vec::Vec;
use core::sync::atomic::{ATOMIC_U16_INIT, AtomicU16, Ordering};
use spin;


static CPU_COUNT: spin::Once<u16> = spin::Once::new();
static CPU_ASSIGNMENT: AtomicU16 = ATOMIC_U16_INIT;


pub fn init(count: u16) {
    CPU_COUNT.call_once(|| count);
}

fn get_cpu_count() -> u16 {
    *CPU_COUNT.call_once(|| {
        panic!("Must initialize CPU count before requesting it");
    })
}


pub fn bring_core_online() {
    let cpu_num = CPU_ASSIGNMENT.fetch_add(1, Ordering::Relaxed);
    set_number(cpu_num);
}


fn set_number(num: u16) {
    vmx::write_es(num);
}


pub fn get_number() -> u16 {
    vmx::read_es()
}


pub struct PerCoreVariable<T> {
    vars: Vec<T>,
}


impl<T: Default> Default for PerCoreVariable<T> {
    fn default() -> Self {
        let mut vars = vec![];
        for _ in 0..get_cpu_count() {
            vars.push(Default::default());
        }
        PerCoreVariable { vars: vars }
    }
}


impl<T> PerCoreVariable<T> {
    pub fn get(&self) -> &T {
        &self.vars[get_number() as usize]
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.vars[get_number() as usize]
    }

}


impl<T: Clone> PerCoreVariable<T> {
    pub fn new(item: T) -> Self {
        PerCoreVariable { vars: vec![item; get_cpu_count() as usize] }
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_new() {
        init(4);
        let pcv: PerCoreVariable<u32> = PerCoreVariable::new(42);
        assert_eq!(pcv.vars.len(), 4);
        for variable in &pcv.vars {
            assert_eq!(*variable, 42);
        }
    }


    #[test]
    fn test_default() {
        init(4);
        let pcv: PerCoreVariable<u32> = Default::default();
        let def: u32 = Default::default();
        assert_eq!(pcv.vars.len(), 4);
        for variable in &pcv.vars {
            assert_eq!(*variable, def);
        }
    }

    #[test]
    fn test_get_mut() {
        init(4);
        let mut pcv: PerCoreVariable<u32> = Default::default();
        assert_eq!(*pcv.get(), 0);
        {
            let mut b = pcv.get_mut();
            *b = 42;
        }
        assert_eq!(*pcv.get(), 42);
    }


    #[test]
    fn test_get() {
        init(4);
        let mut pcv: PerCoreVariable<u32> = Default::default();
        assert_eq!(*pcv.get(), 0);
        pcv.vars[0] = 42;
        assert_eq!(*pcv.get(), 42);
    }
}
