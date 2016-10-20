use {TargetRegister, TargetRegisterClass};

use util;
use std;

#[derive(Debug)]
pub enum Instruction<TI: TargetInstruction>
{
    Target(TI),
}

pub trait TargetInstruction : std::fmt::Debug
{
    type TargetOperand: TargetOperand;

    fn operands_mut(&mut self) -> Vec<&mut Self::TargetOperand>;
}

pub trait TargetOperand : Clone + std::fmt::Debug
{
    type Register: TargetRegister;
    type RegisterClass: TargetRegisterClass;

    fn is_virtual(&self) -> bool;
    fn virtual_register_id(&self) -> util::Id;

    fn register_class(&self) -> Self::RegisterClass;

    fn allocate(&mut self, register: Register<Self>);
}

#[derive(Clone,PartialEq,Eq)]
pub enum Register<TO: TargetOperand+'static>
{
    /// An allocated register.
    Physical(TO::Register),

    /// A virtual register.
    Virtual {
        id: util::Id,
        class: TO::RegisterClass,
    },
}

impl<TI: TargetInstruction> TargetInstruction for Instruction<TI>
{
    type TargetOperand = TI::TargetOperand;

    fn operands_mut(&mut self) -> Vec<&mut Self::TargetOperand> {
        match *self {
            Instruction::Target(ref mut i) => i.operands_mut(),
        }
    }
}

