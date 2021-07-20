use crate::register_state::GeneralPurposeRegisterState;

const HYPERVISOR_VERSION: &str = env!("CARGO_PKG_VERSION");

fn parse_version(version_string: &str) -> [u32; 3] {
    let mut index = 0;
    let mut version: [u32; 3] = [0, 0, 0];
    for c in version_string.chars() {
        if index > version.len() {
            break;
        }

        if let Some(digit) = c.to_digit(10) {
            version[index] *= 10;
            version[index] += digit;
        } else {
            index += 1;
            continue;
        }
    }

    version
}

/// Handle a hypercall.
/// Expects gprs.rax to hold hypercall::HYPERCALL_MAGIC and gprs.rcx to hold a
/// valid hypercall reason.
pub fn handle_hypercall(gprs: &mut GeneralPurposeRegisterState) -> Result<(), x86::vmx::VmFail> {
    assert_eq!(hypervisor_abi::HYPERCALL_MAGIC, gprs.rax as u32);

    let reason = gprs.rcx as u32;
    match reason {
        hypervisor_abi::HYPERCALL_REASON_VERSION => {
            let version = parse_version(&HYPERVISOR_VERSION);
            gprs.rax = u64::from(version[0]);
            gprs.rbx = u64::from(version[1]);
            gprs.rcx = u64::from(version[2]);
            gprs.rdx = 0; // Reserved 0
        }
        _ => {
            gprs.rax = 0;
            gprs.rbx = 0;
            gprs.rcx = 0;
            gprs.rdx = 0;
        }
    }
    Ok(())
}
