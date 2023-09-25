use axhal::mem::{PhysAddr, VirtAddr};
// #[cfg(target_arch = "riscv64")]
// use hypercraft::{GuestPageTableTrait, GuestPhysAddr, HyperError, HyperResult, NestedPageTable};
// #[cfg(target_arch = "aarch64")]
use hypercraft::{GuestPageTableTrait, GuestPhysAddr, HyperError, HyperResult, A64HVPageTable};

use page_table_entry::MappingFlags;

pub type GuestPagingIfImpl = axhal::paging::PagingIfImpl;

/// Guest Page Table struct
// #[cfg(target_arch = "riscv64")]
pub struct GuestPageTable(NestedPageTable<GuestPagingIfImpl>);

impl GuestPageTableTrait for GuestPageTable {
    fn new() -> HyperResult<Self> {
        #[cfg(target_arch = "riscv64")]
        {
            let npt = NestedPageTable::<GuestPagingIfImpl>::try_new_gpt()
                .map_err(|_| HyperError::NoMemory)?;
            Ok(GuestPageTable(npt))
        }
        #[cfg(not(target_arch = "riscv64"))]
        {
            todo!()
        }
    }

    fn map(
        &mut self,
        gpa: GuestPhysAddr,
        hpa: hypercraft::HostPhysAddr,
        flags: MappingFlags,
    ) -> HyperResult<()> {
        #[cfg(target_arch = "riscv64")]
        {
            self.0
                .map(
                    VirtAddr::from(gpa),
                    PhysAddr::from(hpa),
                    page_table::PageSize::Size4K,
                    flags,
                )
                .map_err(|paging_err| {
                    error!("paging error: {:?}", paging_err);
                    HyperError::Internal
                })?;
            Ok(())
        }
        #[cfg(not(target_arch = "riscv64"))]
        {
            todo!()
        }
    }

    fn map_region(
        &mut self,
        gpa: GuestPhysAddr,
        hpa: hypercraft::HostPhysAddr,
        size: usize,
        flags: MappingFlags,
    ) -> HyperResult<()> {
        #[cfg(target_arch = "riscv64")]
        {
            self.0
                .map_region(VirtAddr::from(gpa), PhysAddr::from(hpa), size, flags, true)
                .map_err(|err| {
                    error!("paging error: {:?}", err);
                    HyperError::Internal
                })?;
            Ok(())
        }
        #[cfg(not(target_arch = "riscv64"))]
        {
            todo!()
        }
    }

    fn unmap(&mut self, gpa: GuestPhysAddr) -> HyperResult<()> {
        #[cfg(target_arch = "riscv64")]
        {
            let (_, _) = self.0.unmap(VirtAddr::from(gpa)).map_err(|paging_err| {
                error!("paging error: {:?}", paging_err);
                return HyperError::Internal;
            })?;
            Ok(())
        }
        #[cfg(not(target_arch = "riscv64"))]
        {
            todo!()
        }
    }

    fn translate(&self, gpa: GuestPhysAddr) -> HyperResult<hypercraft::HostPhysAddr> {
        #[cfg(target_arch = "riscv64")]
        {
            let (addr, _, _) = self.0.query(VirtAddr::from(gpa)).map_err(|paging_err| {
                error!("paging error: {:?}", paging_err);
                HyperError::Internal
            })?;
            Ok(addr.into())
        }
        
        {
            todo!()
        }
    }

    fn token(&self) -> usize {
        #[cfg(target_arch = "riscv64")]
        {
            8usize << 60 | usize::from(self.0.root_paddr()) >> 12
        }
        #[cfg(not(target_arch = "riscv64"))]
        {
            todo!()
        }
    }
}

// #[cfg(target_arch = "aarch64")]
pub struct AARCH64GuestPageTable(A64HVPageTable<GuestPagingIfImpl>);
impl GuestPageTableTrait for AARCH64GuestPageTable {
    fn new() -> HyperResult<Self> {
        #[cfg(target_arch = "aarch64")]
        {
            let agpt = A64HVPageTable::<GuestPagingIfImpl>::try_new()
            .map_err(|_| HyperError::NoMemory)?;
            Ok(GuestPageTable(agpt))
        }
        #[cfg(not(target_arch = "aarch64"))]
        {
            todo!()
        }
    }

    fn map(
        &mut self,
        gpa: GuestPhysAddr,
        hpa: hypercraft::HostPhysAddr,
        flags: MappingFlags,
    ) -> HyperResult<()> {
        #[cfg(target_arch = "aarch64")]
        {
            self.0
                .map(
                    VirtAddr::from(gpa),
                    PhysAddr::from(hpa),
                    page_table::PageSize::Size4K,
                    flags,
                )
                .map_err(|paging_err| {
                    error!("paging error: {:?}", paging_err);
                    HyperError::Internal
                })?;
            Ok(())
        }
        #[cfg(not(target_arch = "aarch64"))]
        {
            todo!()
        }
    }

    fn map_region(
        &mut self,
        gpa: GuestPhysAddr,
        hpa: hypercraft::HostPhysAddr,
        size: usize,
        flags: MappingFlags,
    ) -> HyperResult<()> {
        #[cfg( target_arch = "aarch64")]
        {
            self.0
                .map_region(VirtAddr::from(gpa), PhysAddr::from(hpa), size, flags, true)
                .map_err(|err| {
                    error!("paging error: {:?}", err);
                    HyperError::Internal
                })?;
            Ok(())
        }
        #[cfgnot((target_arch = "aarch64"))]
        {
            todo!()
        }
    }

    fn unmap(&mut self, gpa: GuestPhysAddr) -> HyperResult<()> {
        #[cfg(target_arch = "aarch64")]
        {
            let (_, _) = self.0.unmap(VirtAddr::from(gpa)).map_err(|paging_err| {
                error!("paging error: {:?}", paging_err);
                return HyperError::Internal;
            })?;
            Ok(())
        }
        #[cfg(not(target_arch = "aarch64"))]
        {
            todo!()
        }
    }

    fn translate(&self, gpa: GuestPhysAddr) -> HyperResult<hypercraft::HostPhysAddr> {
        #[cfg(target_arch = "aarch64")]
        {
            let (addr, _, _) = self.0.query(VirtAddr::from(gpa)).map_err(|paging_err| {
                error!("paging error: {:?}", paging_err);
                HyperError::Internal
            })?;
            Ok(addr.into())
        }
        #[cfg(not(target_arch = "aarch64"))]
        {
            todo!()
        }
    }

    fn token(&self) -> usize {
        #[cfg(target_arch = "aarch64")]
        {
            usize::from(self.0.root_paddr()) << 1   // need to lrs 1 bit for CnP??
        }
        #[cfg(not(target_arch = "aarch64"))]
        {
            todo!()
        }
    }
}