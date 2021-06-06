use log::trace;

use crate::msr;
use crate::vmcs_fields::PinBasedControlsVmxPreemption;
use crate::{
    vcpu::get_current_vcpu,
    vmcs_fields::{CpuBasedControlsInterruptWindowExiting, VmcsField},
    vmx::{vmread, vmwrite},
};

const VM_PREEMPTION_TIMER_VALUE: u64 = 0xffff;
const INTERRUPT_COUNT: usize = 256;
pub struct VirtualLocalInterruptController {
    total_interrupts_count: usize,
    delayed_delivery_interrupts: [usize; INTERRUPT_COUNT],
    requested_poll_of_interrupts_on_next_preemption_timer: bool,
    we_should_disable_preemption_timer: bool,
}

fn get_local_interrupt_controller() -> &'static mut VirtualLocalInterruptController {
    unsafe { &mut *get_current_vcpu().virtual_local_interrupt_controller }
}

fn delay_interrupt_delivery(interrupt_number: u64) {
    let interrupt_controller = get_local_interrupt_controller();
    let count = &mut interrupt_controller.delayed_delivery_interrupts[interrupt_number as usize];
    *count += 1;
    interrupt_controller.total_interrupts_count = interrupt_controller
        .total_interrupts_count
        .checked_add(1)
        .unwrap();
}

fn check_for_highest_priority_interrupt() -> Option<u64> {
    let interrupt_controller = get_local_interrupt_controller();
    if interrupt_controller.total_interrupts_count == 0 {
        return None;
    }
    for (interrupt_number, count) in &mut interrupt_controller
        .delayed_delivery_interrupts
        .iter_mut()
        .enumerate()
    {
        if *count > 0 {
            *count -= 1;
            interrupt_controller.total_interrupts_count -= 1;
            return Some(interrupt_number as u64);
        }
    }
    unreachable!();
}

// See 33.3.3.4 Generation of Virtual Interrupt Events by VMM
fn vmx_is_guest_interruptable() -> bool {
    let guest_rflags = vmread(VmcsField::GuestRFlags).unwrap();
    // Is the guest interrupt flag clear?
    if (guest_rflags & 0x200) == 0 {
        return false;
    }
    // Are interrupts blocked by a hardware state like sti?
    // Remember, interrupts won't be injected until the instruction *after* sti.
    // There are other instructions which do this too, see vol 3 33.3.3.4
    let guest_interruptability_info = vmread(VmcsField::GuestInterruptibilityInfo).unwrap();
    // See Table 24-3. Format of Interruptibility State
    guest_interruptability_info == 0
}

fn vmx_inject_interrupt_into_guest(vector: u64) -> Result<(), x86::vmx::VmFail> {
    assert!(vector < INTERRUPT_COUNT as u64);
    let interrupt_info = vector | (1 << 31) | (0 << 8); // valid, external interrupts
    crate::debug::breakpoint();
    vmwrite(VmcsField::VmEntryIntrInfoField, interrupt_info)
}

fn vmx_configure_interrupts_wakeup() {
    let pair = msr::rdmsr(msr::Msr::Ia32VmxProcBasedControls);
    let disallowed_exit_values = pair.edx;

    // If we are allowed to set interrupt window exiting, set it.
    if u64::from(disallowed_exit_values) & CpuBasedControlsInterruptWindowExiting == 0 {
        trace!("CPU allows interrupt window exiting");
        let mut cpu_based_controls = vmread(VmcsField::CpuBasedVmExecControl).unwrap();
        cpu_based_controls |= CpuBasedControlsInterruptWindowExiting;
        vmwrite(VmcsField::CpuBasedVmExecControl, cpu_based_controls).unwrap();
    } else {
        trace!("CPU does not allow interrupt window exiting, use a preemption timer instead");
        let local_interrupt_controller = get_local_interrupt_controller();
        let mut pin_based_controls = vmread(VmcsField::PinBasedVmExecControl).unwrap();
        if (pin_based_controls & PinBasedControlsVmxPreemption) == 0 {
            pin_based_controls |= PinBasedControlsVmxPreemption;
            vmwrite(VmcsField::PinBasedVmExecControl, pin_based_controls).unwrap();
            local_interrupt_controller.we_should_disable_preemption_timer = true;
        } else {
            local_interrupt_controller.we_should_disable_preemption_timer = false;
        }
        vmwrite(
            VmcsField::VmxPreemptionTimerValue,
            VM_PREEMPTION_TIMER_VALUE,
        )
        .unwrap();
        local_interrupt_controller.requested_poll_of_interrupts_on_next_preemption_timer = true;
    }
}

fn vmx_deconfigure_timer_wakeup() -> Result<(), x86::vmx::VmFail> {
    let mut pin_based_controls = vmread(VmcsField::PinBasedVmExecControl)?;
    pin_based_controls &= !PinBasedControlsVmxPreemption;
    vmwrite(VmcsField::PinBasedVmExecControl, pin_based_controls)
}

fn vmx_deconfigure_interrupt_window_exiting() -> Result<(), x86::vmx::VmFail> {
    let mut cpu_based_controls = vmread(VmcsField::CpuBasedVmExecControl)?;
    cpu_based_controls &= !CpuBasedControlsInterruptWindowExiting;
    vmwrite(VmcsField::CpuBasedVmExecControl, cpu_based_controls)
}

pub fn received_external_interrupt() -> Result<(), x86::vmx::VmFail> {
    let interrupt_info = vmread(VmcsField::VmExitIntrInfo)?;
    let interrupt_number = interrupt_info & 0xff;
    trace!("Received external interrupt {:x}", interrupt_info);

    if vmx_is_guest_interruptable() {
        trace!("Guest is interruptable");
        vmx_inject_interrupt_into_guest(interrupt_number)
    } else {
        trace!("Guest is not interruptable, delay delivery");
        delay_interrupt_delivery(interrupt_number);
        vmx_configure_interrupts_wakeup();
        Ok(())
    }
}

pub fn received_preemption_timer() -> Result<(), x86::vmx::VmFail> {
    let local_interrupt_controller = get_local_interrupt_controller();
    if local_interrupt_controller.requested_poll_of_interrupts_on_next_preemption_timer
        && vmx_is_guest_interruptable()
    {
        if let Some(interrupt_number) = check_for_highest_priority_interrupt() {
            trace!("Delivering interrupt into guest");
            vmx_inject_interrupt_into_guest(interrupt_number as u64)?;
        }

        if local_interrupt_controller.total_interrupts_count == 0
            && local_interrupt_controller.we_should_disable_preemption_timer
        {
            trace!("No more external interrupts cached and we should disable the preemption timer, disabling interrupt window exiting");
            vmx_deconfigure_timer_wakeup()?;
            local_interrupt_controller.we_should_disable_preemption_timer = false;
            local_interrupt_controller.requested_poll_of_interrupts_on_next_preemption_timer =
                false;
        }
    }
    Ok(())
}

pub fn received_interrupt_window_exit() -> Result<(), x86::vmx::VmFail> {
    let local_interrupt_controller = get_local_interrupt_controller();
    assert!(vmx_is_guest_interruptable());
    if let Some(interrupt_number) = check_for_highest_priority_interrupt() {
        trace!("Delivering interrupt into guest");
        vmx_inject_interrupt_into_guest(interrupt_number)?;
    }
    if local_interrupt_controller.total_interrupts_count == 0 {
        trace!("No more external interrupts cached, disabling interrupt window exiting");
        vmx_deconfigure_interrupt_window_exiting()?;
    }
    Ok(())
}
