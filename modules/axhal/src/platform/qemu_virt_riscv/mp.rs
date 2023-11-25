use crate::mem::{virt_to_phys, PhysAddr, VirtAddr};

/// Starts the given secondary CPU with its boot stack.
pub fn start_secondary_cpu(hartid: usize, stack_top: PhysAddr) {
    info!{"start_sccondary_cpu"};
    extern "C" {
        fn _start_secondary();
    }
    info!{"_start_secondary"};
    if sbi_rt::probe_extension(sbi_rt::Hsm).is_unavailable() {
        warn!("HSM SBI extension is not supported for current SEE.");
        return;
    }
    let entry = virt_to_phys(VirtAddr::from(_start_secondary as usize));
    info!{"phys:{:?}", virt_to_phys(VirtAddr::from(_start_secondary as usize))};
    sbi_rt::hart_start(hartid, entry.as_usize(), stack_top.as_usize());
}
