use axalloc::global_allocator;
use axhal::mem::PAGE_SIZE_4K;
use hypercraft::{HostPhysAddr, HyperCraftHal};

#[cfg(target_arch = "aarch64")]
pub use hypercraft::arch::{current_cpu, ipi_irq_handler, timer_irq_handler, timer_irq_handler, interrupt_init};
#[cfg(target_arch = "aarch64")]
pub use axhal::platform::aarch64_common::{GICH, GICC, GICD};


/// An empty struct to implementate of `HyperCraftHal`
pub struct HyperCraftHalImpl;

impl HyperCraftHal for HyperCraftHalImpl {
    fn alloc_pages(num_pages: usize) -> Option<hypercraft::HostPhysAddr> {
        global_allocator()
            .alloc_pages(num_pages, PAGE_SIZE_4K)
            .map(|pa| pa as HostPhysAddr)
            .ok()
    }

    fn dealloc_pages(pa: HostPhysAddr, num_pages: usize) {
        global_allocator().dealloc_pages(pa as usize, num_pages);
    }
}

pub fn interrupt_register_for_aarch64_hv() {
    if current_cpu().cpu_id == 0 {
        axhal::irq::register_handler(IPI_IRQ_NUM, ipi_irq_handler());
        interrupt_init();
    }
    axhal::irq::register_handler(MAINTENANCE_IRQ_NUM, maintenance_irq_handler());
    axhal::irq::register_handler(HYPERVISOR_TIMER_IRQ_NUM, timer_irq_handler());
}