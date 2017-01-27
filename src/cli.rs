use vmx;

pub struct ClearLocalInterruptsGuard;

impl ClearLocalInterruptsGuard {
    pub fn new() -> Self {
        vmx::cli();
        ClearLocalInterruptsGuard {}
    }
}

impl Drop for ClearLocalInterruptsGuard {
    fn drop(&mut self) {
        vmx::sti();
    }
}
