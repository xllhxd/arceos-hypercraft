use axalloc::global_allocator;
use axhal::mem::PAGE_SIZE_4K;
use hypercraft::{HostPhysAddr, HyperCraftHal};

#[cfg(target_arch = "aarch64")]
pub use hypercraft::{current_cpu, CPU_INTERFACE_LIST, active_vm, IPI_HANDLER_LIST};
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

pub fn ipi_irq_handler() {
    let cpu_id = current_cpu().cpu_id;
    let mut cpu_if_list = CPU_INTERFACE_LIST.lock();
    let mut msg: Option<IpiMessage> = cpu_if_list[cpu_id].pop();
    drop(cpu_if_list);

    while !msg.is_none() {
        let ipi_msg = msg.unwrap();
        let ipi_type = ipi_msg.ipi_type as usize;

        let ipi_handler_list = IPI_HANDLER_LIST.lock();
        let len = ipi_handler_list.len();
        let handler = ipi_handler_list[ipi_type].handler.clone();
        drop(ipi_handler_list);

        if len <= ipi_type {
            info!("illegal ipi type {}", ipi_type)
        } else {
            // info!("ipi type is {:#?}", ipi_msg.ipi_type);
            handler(&ipi_msg);
        }
        let mut cpu_if_list = CPU_INTERFACE_LIST.lock();
        msg = cpu_if_list[cpu_id].pop();
    }
}

pub fn maintenance_irq_handler() {
    let misr = GICH.misr();
    let vm = match active_vm() {
        Some(vm) => vm,
        None => {
            panic!("maintenance_irq_handler: current vcpu.vm is None");
        }
    };
    let vgic = vm.vgic();

    if misr & 1 != 0 {
        // vgic.handle_trapped_eoir(current_cpu().active_vcpu.clone().unwrap());
        info!("misr 1")
    }

    if misr & (1 << 3) != 0 {
        // vgic.refill_lrs(current_cpu().active_vcpu.clone().unwrap());
        info!("misr 3")
    }

    if misr & (1 << 2) != 0 {
        /* 
        let mut hcr = GICH.hcr();
        while hcr & (0b11111 << 27) != 0 {
            vgic.eoir_highest_spilled_active(current_cpu().active_vcpu.clone().unwrap());
            hcr -= 1 << 27;
            GICH.set_hcr(hcr);
            hcr = GICH.hcr();
        }
        */
        info!("misr 2")
    }
}

pub fn timer_irq_handler() {
    /* 
    use crate::arch::timer_arch_disable_irq;

    timer_arch_disable_irq();
    current_cpu().scheduler().do_schedule();

    timer_notify_after(1);
    */
    info!("timer_irq_handler")
}