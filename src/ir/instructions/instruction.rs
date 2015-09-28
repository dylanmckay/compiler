
use std::fmt;
use ir::{self, instructions, Value, Type, TypeTrait};
use lang;
use util::Upcast;

pub trait InstructionTrait : fmt::Debug + fmt::Display + Upcast<Instruction> 
{
}

#[derive(Clone,Debug)]
pub enum Instruction
{
    Add(instructions::Add),
    Sub(instructions::Sub),
    Mul(instructions::Mul),
    Div(instructions::Div),
    Shl(instructions::Shl),
    Shr(instructions::Shr),

    Call(instructions::Call),
    Return(instructions::Return),
}

impl Instruction
{
    pub fn add(ty: ir::Type, lhs: ir::Value, rhs: ir::Value) -> Instruction {
        instructions::Add::new(ty, lhs, rhs).upcast()
    }

    pub fn sub(ty: ir::Type, lhs: ir::Value, rhs: ir::Value) -> Instruction {
        instructions::Sub::new(ty, lhs, rhs).upcast()
    }

    pub fn mul(ty: ir::Type, lhs: ir::Value, rhs: ir::Value) -> Instruction {
        instructions::Mul::new(ty, lhs, rhs).upcast()
    }

    pub fn div(ty: ir::Type, lhs: ir::Value, rhs: ir::Value) -> Instruction {
        instructions::Div::new(ty, lhs, rhs).upcast()
    }

    pub fn shl(ty: ir::Type, val: ir::Value, amount: ir::Value) -> Instruction {
        instructions::Shl::new(ty, val, amount).upcast()
    }

    pub fn shr(ty: ir::Type, val: ir::Value, amount: ir::Value) -> Instruction {
        instructions::Shr::new(ty, val, amount).upcast()
    }

    pub fn ret(value: Option<ir::Value>) -> Instruction {
        instructions::Return::new(value).upcast()
    }
}

impl InstructionTrait for Instruction { }

impl fmt::Display for Instruction
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        match self {
            &Instruction::Add(ref instr) => instr.fmt(fmt),
            &Instruction::Sub(ref instr) => instr.fmt(fmt),
            &Instruction::Mul(ref instr) => instr.fmt(fmt),
            &Instruction::Div(ref instr) => instr.fmt(fmt),
            &Instruction::Shl(ref instr) => instr.fmt(fmt),
            &Instruction::Shr(ref instr) => instr.fmt(fmt),
            &Instruction::Call(ref instr) => instr.fmt(fmt),
            &Instruction::Return(ref instr) => instr.fmt(fmt),
        }
    }
}

impl lang::Instruction for Instruction
{

}

impl_upcast!(Instruction);
