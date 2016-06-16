use RegisterClass;
use util;
use std;

/// A machine operand.
#[derive(Clone,PartialEq,Eq)]
pub enum Operand
{
    /// An immediate value.
    Immediate { bit_width: u32, value: i64 },

    /// A virtual register.
    VirtualRegister {
        id: util::Id,
        class: &'static RegisterClass,
    },
}

impl std::fmt::Debug for Operand
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Operand::Immediate { bit_width, value } => {
                write!(fmt, "i{}:{}", bit_width, value)
            },
            Operand::VirtualRegister { id, class } => {
                write!(fmt, "<{}:#{}>", class.name, id)
            },
        }
    }
}

