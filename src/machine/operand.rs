use RegisterClass;
use util;

/// A machine operand.
#[derive(Clone,Debug,PartialEq,Eq)]
pub enum Operand
{
    /// An immediate value.
    Immediate { bit_width: u32, value: i64 },

    /// A register.
    Register(u32),

    /// A virtual register.
    VirtualRegister {
        id: util::Id,
        class: &'static RegisterClass,
    },
}

