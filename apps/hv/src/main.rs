#![no_std]
#![no_main]

extern crate alloc;
#[macro_use]
extern crate libax;

use dtb::MachineMeta;
use libax::{
    hv::{
        self, GuestPageTable, GuestPageTableTrait, HyperCraftHalImpl, PerCpu,
        Result, VCpu, VmCpus, VM,
    },
    info,
};
#[cfg(target_arch = "riscv64")]
use libax::{
    hv::{HyperCallMsg,VmExitInfo}
};
use page_table_entry::MappingFlags;

mod dtb;

#[no_mangle]
fn main(hart_id: usize) {
    println!("Hello, hv!");

    #[cfg(target_arch = "riscv64")]
    {
        // boot cpu
        PerCpu::<HyperCraftHalImpl>::init(0, 0x4000);

        // get current percpu
        let pcpu = PerCpu::<HyperCraftHalImpl>::this_cpu();

        // create vcpu
        let gpt = setup_gpm(0x9000_0000).unwrap();
        let vcpu = pcpu.create_vcpu(0, 0x9020_0000).unwrap();
        let mut vcpus = VmCpus::new();

        // add vcpu into vm
        vcpus.add_vcpu(vcpu).unwrap();
        let mut vm: VM<HyperCraftHalImpl, GuestPageTable> = VM::new(vcpus, gpt).unwrap();
        vm.init_vcpu(0);

        // vm run
        info!("vm run cpu{}", hart_id);
        vm.run(0);
    }
    #[cfg(target_arch = "aarch64")]
    {
        // boot cpu
        PerCpu::<HyperCraftHalImpl>::init(0, 0x4000);   // change to pub const CPU_STACK_SIZE: usize = PAGE_SIZE * 128?

        // get current percpu
        let pcpu = PerCpu::<HyperCraftHalImpl>::this_cpu();

        // create vcpu, need to change addr for aarch64!
        let gpt = setup_gpm(0x9000_0000).unwrap();  
        let vcpu = pcpu.create_vcpu(0).unwrap();
        let mut vcpus = VmCpus::new();

        // add vcpu into vm
        vcpus.add_vcpu(vcpu).unwrap();
        let mut vm: VM<HyperCraftHalImpl, GuestPageTable> = VM::new(vcpus, gpt, 0).unwrap();
        vm.init_vm_vcpu(0x80080000, 0x80000000);

        info!("vm run cpu{}", hart_id);
        // suppose hart_id to be 0
        vm.run(0);
    }
    #[cfg(not(target_arch = "riscv64"))]
    {
        panic!("Other arch is not supported yet!")
    }
}

#[cfg(target_arch = "riscv64")]
pub fn setup_gpm(dtb: usize) -> Result<GuestPageTable> {
    let mut gpt = GuestPageTable::new()?;
    let meta = MachineMeta::parse(dtb);
    if let Some(test) = meta.test_finisher_address {
        gpt.map_region(
            test.base_address,
            test.base_address,
            test.size + 0x1000,
            MappingFlags::READ | MappingFlags::WRITE | MappingFlags::USER | MappingFlags::EXECUTE,
        )?;
    }
    for virtio in meta.virtio.iter() {
        gpt.map_region(
            virtio.base_address,
            virtio.base_address,
            virtio.size,
            MappingFlags::READ | MappingFlags::WRITE | MappingFlags::USER,
        )?;
    }

    if let Some(uart) = meta.uart {
        gpt.map_region(
            uart.base_address,
            uart.base_address,
            0x1000,
            MappingFlags::READ | MappingFlags::WRITE | MappingFlags::USER,
        )?;
    }

    if let Some(clint) = meta.clint {
        gpt.map_region(
            clint.base_address,
            clint.base_address,
            clint.size,
            MappingFlags::READ | MappingFlags::WRITE | MappingFlags::USER,
        )?;
    }

    if let Some(plic) = meta.plic {
        gpt.map_region(
            plic.base_address,
            plic.base_address,
            0x20_0000,
            MappingFlags::READ | MappingFlags::WRITE | MappingFlags::USER,
        )?;
    }

    if let Some(pci) = meta.pci {
        gpt.map_region(
            pci.base_address,
            pci.base_address,
            pci.size,
            MappingFlags::READ | MappingFlags::WRITE | MappingFlags::USER,
        )?;
    }

    info!(
        "physical memory: [{:#x}: {:#x})",
        meta.physical_memory_offset,
        meta.physical_memory_offset + meta.physical_memory_size
    );

    gpt.map_region(
        meta.physical_memory_offset,
        meta.physical_memory_offset,
        meta.physical_memory_size,
        MappingFlags::READ | MappingFlags::WRITE | MappingFlags::EXECUTE | MappingFlags::USER,
    )?;

    Ok(gpt)
}

#[cfg(target_arch = "aarch64")]
pub fn setup_gpm(dtb: usize) -> Result<AARCH64GuestPageTable> {
    let mut gpt = AARCH64GuestPageTable::new()?;
    let meta = MachineMeta::parse(dtb);
    if let Some(test) = meta.test_finisher_address {
        gpt.map_region(
            test.base_address,
            test.base_address,
            test.size + 0x1000,
            MappingFlags::READ | MappingFlags::WRITE | MappingFlags::USER | MappingFlags::EXECUTE,
        )?;
    }
    for virtio in meta.virtio.iter() {
        gpt.map_region(
            virtio.base_address,
            virtio.base_address,
            virtio.size,
            MappingFlags::READ | MappingFlags::WRITE | MappingFlags::USER,
        )?;
    }

    if let Some(uart) = meta.uart {
        gpt.map_region(
            uart.base_address,
            uart.base_address,
            0x1000,
            MappingFlags::READ | MappingFlags::WRITE | MappingFlags::USER,
        )?;
    }

    if let Some(clint) = meta.clint {
        gpt.map_region(
            clint.base_address,
            clint.base_address,
            clint.size,
            MappingFlags::READ | MappingFlags::WRITE | MappingFlags::USER,
        )?;
    }

    if let Some(plic) = meta.plic {
        gpt.map_region(
            plic.base_address,
            plic.base_address,
            0x20_0000,
            MappingFlags::READ | MappingFlags::WRITE | MappingFlags::USER,
        )?;
    }

    if let Some(pci) = meta.pci {
        gpt.map_region(
            pci.base_address,
            pci.base_address,
            pci.size,
            MappingFlags::READ | MappingFlags::WRITE | MappingFlags::USER,
        )?;
    }

    info!(
        "physical memory: [{:#x}: {:#x})",
        meta.physical_memory_offset,
        meta.physical_memory_offset + meta.physical_memory_size
    );

    gpt.map_region(
        meta.physical_memory_offset,
        meta.physical_memory_offset,
        meta.physical_memory_size,
        MappingFlags::READ | MappingFlags::WRITE | MappingFlags::EXECUTE | MappingFlags::USER,
    )?;

    Ok(gpt)
}
