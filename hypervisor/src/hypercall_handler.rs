use crate::hypercall;
use crate::register_state::GeneralPurposeRegisterState;

const HYPERVISOR_VERSION: &str = env!("CARGO_PKG_VERSION");

fn parse_version() -> [u32; 3] {
    let mut index = 0;
    let mut version: [u32; 3] = [0, 0, 0];
    for c in HYPERVISOR_VERSION.chars() {
        if index > version.len() {
            break;
        }
        if c == '.' {
            index += 1;
            continue;
        }

        if let Some(digit) = c.to_digit(10) {
            version[index] *= 10;
            version[index] += digit;
        }
    }

    version
}

pub fn handle_hypercall(gprs: &mut GeneralPurposeRegisterState) -> Result<(), x86::vmx::VmFail> {
    assert_eq!(hypercall::HYPERCALL_MAGIC, gprs.rax as u32);

    let reason = gprs.rbx as u32;
    match reason {
        hypercall::HYPERCALL_REASON_VERSION => {
            let version = parse_version();
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
/*
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_version() {
        let version = parse_version();
        assert_eq!(version, [0, 1, 0]);
    }
}
*/
