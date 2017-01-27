use vmx;

pub struct ClearLocalInterruptsGuard(());

impl ClearLocalInterruptsGuard {
    pub fn new() -> Self {
        vmx::cli();
        ClearLocalInterruptsGuard(())
    }
}

impl Default for ClearLocalInterruptsGuard {
    fn default() -> Self {
        ClearLocalInterruptsGuard::new()
    }
}

impl Drop for ClearLocalInterruptsGuard {
    fn drop(&mut self) {
        vmx::sti();
    }
}
