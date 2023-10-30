//! Hypervisor related functions

use axhal::mem::{phys_to_virt, virt_to_phys, PhysAddr};
pub use axruntime::{GuestPageTable, HyperCraftHalImpl};
pub use hypercraft::GuestPageTableTrait;

pub use hypercraft::HyperError as Error;
pub use hypercraft::HyperResult as Result;
pub use hypercraft::{PerCpu, VCpu, VmCpus, VM};
#[cfg(target_arch = "riscv64")]
pub use hypercraft::{HyperCallMsg, VmExitInfo};
