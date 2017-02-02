use vmx;
use collections::vec::Vec;
use core::sync::atomic::{ATOMIC_U16_INIT, AtomicU16, Ordering};
use spin;


static CPU_COUNT: spin::Once<u16> = spin::Once::new();
static CPU_ASSIGNMENT: AtomicU16 = ATOMIC_U16_INIT;


pub fn init(count: u16) {
    CPU_COUNT.call_once(||{ count });
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
}


impl<T: Clone> PerCoreVariable<T> {
    pub fn new(item: T) -> Self {
        PerCoreVariable { vars: vec![item; get_cpu_count() as usize] }
    }
}
