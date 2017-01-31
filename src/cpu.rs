use vmx;
use collections::vec::Vec;
use core::sync::atomic::{ATOMIC_U16_INIT, AtomicU16, Ordering};


static CPU_COUNT: AtomicU16 = ATOMIC_U16_INIT;
static CPU_ASSIGNMENT: AtomicU16 = ATOMIC_U16_INIT;


pub fn init(count: u16) {
    CPU_COUNT.store(count, Ordering::Relaxed);
}


pub fn bring_core_online() {
    let cpu_num = CPU_ASSIGNMENT.fetch_add(1, Ordering::Relaxed);
    set_number(cpu_num);
}


fn set_number(num: u16) {
    vmx::write_gs(num);
}


pub fn get_number() -> u16 {
    vmx::read_gs()
}


#[derive(Default)]
pub struct PerCoreVariable<T> {
    vars: Vec<T>,
}


impl<T> PerCoreVariable<T> {
    pub fn get(&self) -> &T {
        &self.vars[get_number() as usize]
    }
}
