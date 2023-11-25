// crates/hypercraft/src/arch/riscv/devices/cpu.rs
use arrayvec::{ArrayString, ArrayVec};
use spin::Once;
use core::fmt;
use fdt::Fdt;
/// const
const MAX_ISA_STRING_LEN: usize = 256;

/// const
pub const MAX_CPUS_COUNT: usize = 128;

/// Logical CPU number. Not necessarily the same as hart ID; see `CpuInfo` for translating between
/// hart ID and logical CPU ID.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct CpuId(usize);

impl CpuId {
    /// function
    pub fn new(raw: usize) -> Self {
        CpuId(raw)
    }
    /// function
    pub fn raw(&self) -> usize {
        self.0
    }
}

/// Holds static global information about CPU features and topology.
#[derive(Debug)]
pub struct CpuInfo {
    timer_frequency: usize,
    /// hart_id
    pub hart_ids: ArrayVec<usize, MAX_CPUS_COUNT>,
    intc_phandles: ArrayVec<usize, MAX_CPUS_COUNT>,
}

/// Error for CpuInfo creation
#[derive(Debug)]
pub enum Error {
    /// Child node missing from parent
    MissingChildNode(&'static str, &'static str),
    /// Property missing on a node
    MissingFdtProperty(&'static str, &'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        match self {
            MissingChildNode(child, parent) => {
                write!(f, "Child node {} missing on {} parent node", child, parent)
            }
            MissingFdtProperty(property, node) => {
                write!(f, "Property {} missing on {} node", property, node)
            }
        }
    }
}

static CPU_INFO: Once<CpuInfo> = Once::new(); // 

impl CpuInfo {
    /// function
    pub fn parse_from(dtb: usize) -> Result<(), Error> {
        let fdt = unsafe { Fdt::from_ptr(dtb as *const u8)}.unwrap();
        let cpus_node = fdt.find_node("/cpus").unwrap();
        // timer_frequency
        let tf = cpus_node.property("timebase-frequency").unwrap().as_usize().unwrap();
        // hard_ids and intc_phandles
        let mut hart_ids: ArrayVec<usize, MAX_CPUS_COUNT> = ArrayVec::new();
        let mut intc_phandles: ArrayVec<usize, MAX_CPUS_COUNT> = ArrayVec::new();
        for cpu in cpus_node.children() {
            if cpu.name == "cpu-map" {
                break;
            }
            hart_ids.push(cpu.property("reg").unwrap().as_usize().unwrap());
            for cpu_int in cpu.children() {
                intc_phandles.push(cpu_int.property("phandle").unwrap().as_usize().unwrap());
            }
        }
        let cpu_info = CpuInfo {
            timer_frequency: tf,
            hart_ids: hart_ids,
            intc_phandles: intc_phandles,
        };
        info!("{}", tf);
        CPU_INFO.call_once(||cpu_info);
        info!("{:?}", CPU_INFO);
        Ok(())
    }

    /// function
    pub fn get() -> &'static CpuInfo {
        CPU_INFO.get().unwrap()
    }

    /// function
    pub fn num_cpus(&self) -> usize {
        self.hart_ids.len()
    }
}

