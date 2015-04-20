
use std::fmt;
use ir::{self, instructions, Value, Type, TypeTrait};
use lang;
use util;

pub trait InstructionTrait : fmt::Debug + fmt::Display + util::Upcast<Instruction> 
{
}

#[derive(Clone,Debug)]
pub enum Instruction
{
    Add(instructions::Add),
    Return(instructions::Return),
}

impl Instruction
{
    pub fn add(ty: ir::Type, lhs: ir::Value, rhs: ir::Value) -> Instruction {
        use util::Upcast;
        instructions::Add::new(ty, lhs, rhs).upcast()
    }

    pub fn ret(value: Option<ir::Value>) -> Instruction {
        use util::Upcast;
        instructions::Return::new(value).upcast()
    }
}

impl InstructionTrait for Instruction { }

impl fmt::Display for Instruction
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        match self {
            &Instruction::Add(ref instr) => instr.fmt(fmt),
            &Instruction::Return(ref instr) => instr.fmt(fmt),
        }
    }
}

impl lang::Instruction for Instruction
{

}

impl_upcast!(Instruction);
