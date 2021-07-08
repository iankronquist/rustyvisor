//! Hypercall ABI
//! Th

/// Magic number which must be in RAX if this is a hypercall.
pub const HYPERCALL_MAGIC: u32 = 0x72737479;

/// Hypercall reasons must be in RBX. If RBX=1, the reason is version.
/// The major, minor, and patch version numbers from this crate's Cargo.toml
/// will be returned in rax, rbx, and rcx respectively. Rdx is reserved zero.
pub const HYPERCALL_REASON_VERSION: u32 = 0x1;
