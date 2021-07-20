//! Hypercall ABI
//! The hypervisor will handle hypercalls via the CPUID instruction.
//! Hypercalls must have a magic number HYPERCALL_MAGIC in RAX and a valid
//! hypercall reason in RBX.
//! Values will be returned in RAX, RBX, RCX, and RDX according to the
//! hypercall reason.

#![no_std]

use core::arch::x86_64::__cpuid_count;

/// Magic number which must be in RAX if this is a hypercall.
pub const HYPERCALL_MAGIC: u32 = 0x72737479;

/// Hypercall reasons must be in RBX. If RBX=1, the reason is version.
/// The major, minor, and patch version numbers from this crate's Cargo.toml
/// will be returned in rax, rbx, and rcx respectively. Rdx is reserved zero.
pub const HYPERCALL_REASON_VERSION: u32 = 0x1;

#[derive(Debug, Default, Clone, Copy)]
pub struct HyperCallResults {
    // The hypercall "reason" or discriminant. Similar to a syscall number.
    pub reason: u32,
    // Return values of the hypercall.
    pub results: [u32; 4],
}

pub fn invoke_hypercall(reason: u32) -> HyperCallResults {
    let results = unsafe { __cpuid_count(HYPERCALL_MAGIC, reason) };

    HyperCallResults {
        reason,
        results: [results.eax, results.ebx, results.ecx, results.edx],
    }
}
