# 修改 linux.dts 并编译为 linux.dtb

```dts
// apps/hv/guest/linux/linux.dts 39-142
cpus {
	#address-cells = <0x01>;
	#size-cells = <0x00>;
	timebase-frequency = <0x989680>;

	cpu@0 {
		phandle = <0x01>;
		device_type = "cpu";
		reg = <0x00>;
		status = "okay";
		compatible = "riscv";
		riscv,isa = "rv64ima";
		mmu-type = "riscv,sv39";

		interrupt-controller {
			#interrupt-cells = <0x01>;
			interrupt-controller;
			compatible = "riscv,cpu-intc";
			phandle = <0x02>;
		};
	};

	cpu@1 {
		phandle = <0x05>;
		device_type = "cpu";
		reg = <0x01>;
		status = "okay";
		compatible = "riscv";
		riscv,isa = "rv64ima";
		mmu-type = "riscv,sv39";
	
		interrupt-controller {
			#interrupt-cells = <0x01>;
			interrupt-controller;
			compatible = "riscv,cpu-intc";
			phandle = <0x06>;
		};
	};
	
	cpu@2 {
		phandle = <0x07>;
		device_type = "cpu";
		reg = <0x02>;
		status = "okay";
		compatible = "riscv";
		riscv,isa = "rv64ima";
		mmu-type = "riscv,sv39";
	
		interrupt-controller {
			#interrupt-cells = <0x01>;
			interrupt-controller;
			compatible = "riscv,cpu-intc";
			phandle = <0x08>;
		};
	};
	
	cpu@3 {
		phandle = <0x09>;
		device_type = "cpu";
		reg = <0x03>;
		status = "okay";
		compatible = "riscv";
		riscv,isa = "rv64ima";
		mmu-type = "riscv,sv39";
	
		interrupt-controller {
			#interrupt-cells = <0x01>;
			interrupt-controller;
			compatible = "riscv,cpu-intc";
			phandle = <0x0A>;
		};
	};
	
	cpu-map {
	
		cluster0 {
	
			core0 {
				cpu = <0x01>;
			};
		};
	
		cluster1 {
	
			core0 {
				cpu = <0x05>;
			};
		};
	
		cluster2 {
	
			core0 {
				cpu = <0x07>;
			};
		};
	
		cluster3 {
	
			core0 {
				cpu = <0x09>;
			};
		};
	};
};
```

```shell
xuzx@ubuntu:~/arceos/apps/hv/guest/linux$ dtc -O dtb -b 0 -o linux.dtb linux.dts
linux.dts:165.12-170.5: Warning (simple_bus_reg): /soc/poweroff: missing or empty reg/ranges property
linux.dts:172.10-177.5: Warning (simple_bus_reg): /soc/reboot: missing or empty reg/ranges property
linux.dts:260.4-48: Warning (interrupts_extended_property): /soc/plic@c000000:interrupts-extended: cell 0 is not a phandle reference
linux.dts:260.4-48: Warning (interrupts_extended_property): /soc/plic@c000000:interrupts-extended: cell 2 is not a phandle reference
linux.dts:267.4-48: Warning (interrupts_extended_property): /soc/clint@2000000:interrupts-extended: cell 0 is not a phandle reference
linux.dts:267.4-48: Warning (interrupts_extended_property): /soc/clint@2000000:interrupts-extended: cell 2 is not a phandle reference
linux.dts:53.25-58.6: Warning (interrupt_provider): /cpus/cpu@0/interrupt-controller: Missing #address-cells in interrupt provider
linux.dts:70.25-75.6: Warning (interrupt_provider): /cpus/cpu@1/interrupt-controller: Missing #address-cells in interrupt provider
linux.dts:87.25-92.6: Warning (interrupt_provider): /cpus/cpu@2/interrupt-controller: Missing #address-cells in interrupt provider
linux.dts:104.25-109.6: Warning (interrupt_provider): /cpus/cpu@3/interrupt-controller: Missing #address-cells in interrupt provider
linux.dts:256.16-264.5: Warning (interrupt_provider): /soc/plic@c000000: Missing #address-cells in interrupt provider
```

# 创建 cpu.rs 并解析 linux.dtb

```Rust
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

```

这里由于我对 Rust 的模块系统不太了解，在让 **apps/hv/src/main.rs* 可见的时候基本编译器说哪里错哪里改的。

# 解析 SbiMessage 使其支持 HSM 扩展

新增 **crates/hypercraft/src/arch/riscv/sbi/hsm.rs**

```Rust
// from new bing
use crate::HyperResult;

/// Functions defined for the Hart State Management extension
#[derive(Clone, Copy, Debug)]
pub enum HartStateManagementFunction {
    /// Start the hart with the given ID and address
    HartStart {
        hartid: usize,
        start_addr: usize,
        opaque: usize,
    },
    /// Stop the hart with the given ID
    HartStop {
        hartid: usize,
    },
    /// Get the status of the hart with the given ID
    GetHartStatus {
        hartid: usize,
    },
    /// Suspend the hart with the given ID and type
    HartSuspend {
        hartid: usize,
        suspend_type: usize,
        resume_addr: usize,
        opaque: usize,
    },
}

impl HartStateManagementFunction {
    pub(crate) fn from_regs(args: &[usize]) -> HyperResult<Self> {
        match args[6] {
            0 => Ok(Self::HartStart {
                hartid: args[0],
                start_addr: args[1],
                opaque: args[2],
            }),
            1 => Ok(Self::HartStop {
                hartid: args[0],
            }),
            2 => Ok(Self::GetHartStatus {
                hartid: args[0],
            }),
            3 => Ok(Self::HartSuspend {
                hartid: args[0],
                suspend_type: args[1],
                resume_addr: args[2],
                opaque: args[3],
            }),
            _ => Err(crate::HyperError::NotFound),
        }
    }
}

```

修改 **crates/hypercraft/src/arch/riscv/sbi/mod.rs**

```Rust
sbi_spec::hsm::EID_HSM=>HartStateManagementFunction::from_regs(args).map(SbiMessage::Hsm)
```

修改 **crates/hypercraft/src/arch/riscv/vm.rs**

```Rust
fn handle_hsm_function(

        &self,

        hsm: HartStateManagementFunction,

        gprs: &mut GeneralPurposeRegisters,

    ) -> HyperResult<()> {

        gprs.set_reg(GprIndex::A0, 0);

        match hsm {

            HartStateManagementFunction::HartStart {

                hartid,

                start_addr,

                opaque,

            } => {

                let sbi_ret = sbi_rt::hart_start(hartid, start_addr, opaque);

                gprs.set_reg(GprIndex::A0, sbi_ret.error);

            }

            HartStateManagementFunction::HartStop { hartid: _ } => {

                let sbi_ret = sbi_rt::hart_stop();

                gprs.set_reg(GprIndex::A0, sbi_ret.error);

            }

            HartStateManagementFunction::GetHartStatus { hartid } => {

                let sbi_ret = sbi_rt::hart_get_status(hartid) ;

                gprs.set_reg(GprIndex::A0, sbi_ret.error);

            }

            HartStateManagementFunction::HartSuspend { hartid: _, suspend_type, resume_addr, opaque } => {

                let sbi_ret = sbi_rt::hart_suspend(suspend_type as u32, resume_addr, opaque);

                gprs.set_reg(GprIndex::A0, sbi_ret.error);

            }

        }

        Ok(())

    }
```