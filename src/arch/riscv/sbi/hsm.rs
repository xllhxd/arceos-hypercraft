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
