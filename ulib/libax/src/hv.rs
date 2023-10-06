//! Hypervisor related functions

pub use axhal::mem::{phys_to_virt, virt_to_phys, PhysAddr};
pub use axruntime::GuestPageTable;
pub use axruntime::HyperCraftHalImpl;
pub use hypercraft::GuestPageTableTrait;

pub use hypercraft::HyperError as Error;
pub use hypercraft::HyperResult as Result;
pub use hypercraft::{HyperCallMsg, PerCpu, VCpu, VmCpus, VmExitInfo, VM};
pub use hypercraft::{GuestPhysAddr, GuestVirtAddr, HostPhysAddr, HostVirtAddr};
pub use hypercraft::HyperCraftHal;
