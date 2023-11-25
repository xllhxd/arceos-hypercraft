use crate::HyperResult;

#[derive(Clone, Copy, Debug)]

pub enum IPIFunction {
    SendIPI {
        hart_mask: usize,
        hart_mask_base: usize,
    },
}
impl IPIFunction {
    pub(crate) fn from_regs(args: &[usize]) -> HyperResult<Self> {
        match args[6] {
            0 => Ok(Self::SendIPI {
                hart_mask: args[0],
                hart_mask_base: args[1],
            }),
            _ => panic!("Unsupported yet!"),
        }
    }
}
