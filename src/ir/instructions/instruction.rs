
use std::fmt;
use ir::{self,instructions,Value,Type};

pub trait InstructionTrait : fmt::Debug + fmt::Display +
                             Into<Value> +
                             ir::ValueTrait
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
        instructions::Add::new(ty, lhs, rhs).into()
    }

    pub fn sub(ty: ir::Type, lhs: ir::Value, rhs: ir::Value) -> Instruction {
        instructions::Sub::new(ty, lhs, rhs).into()
    }

    pub fn mul(ty: ir::Type, lhs: ir::Value, rhs: ir::Value) -> Instruction {
        instructions::Mul::new(ty, lhs, rhs).into()
    }

    pub fn div(ty: ir::Type, lhs: ir::Value, rhs: ir::Value) -> Instruction {
        instructions::Div::new(ty, lhs, rhs).into()
    }

    pub fn shl(ty: ir::Type, val: ir::Value, amount: ir::Value) -> Instruction {
        instructions::Shl::new(ty, val, amount).into()
    }

    pub fn shr(ty: ir::Type, val: ir::Value, amount: ir::Value) -> Instruction {
        instructions::Shr::new(ty, val, amount).into()
    }

    pub fn ret(value: Option<ir::Value>) -> Instruction {
        instructions::Return::new(value).into()
    }
}

impl InstructionTrait for Instruction { }

impl Into<Value> for Instruction
{
    fn into(self) -> Value {
        Value::Instruction(self)
    }
}

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

impl Instruction
{
    pub fn subvalues(&self) -> Vec<Value> {
         match self {
            &Instruction::Add(ref instr) => instr.subvalues(),
            &Instruction::Sub(ref instr) => instr.subvalues(),
            &Instruction::Mul(ref instr) => instr.subvalues(),
            &Instruction::Div(ref instr) => instr.subvalues(),
            &Instruction::Shl(ref instr) => instr.subvalues(),
            &Instruction::Shr(ref instr) => instr.subvalues(),
            &Instruction::Call(ref instr) => instr.subvalues(),
            &Instruction::Return(ref instr) => instr.subvalues(),
         }
    }
    
    pub fn map_subvalues<F>(self, f: F) -> Value
        where F: FnMut(Value) -> Value {
        use lang::Value;

        match self {
           ir::Instruction::Add(instr) => instr.map_subvalues(f).into(),
           ir::Instruction::Sub(instr) => instr.map_subvalues(f).into(),
           ir::Instruction::Mul(instr) => instr.map_subvalues(f).into(),
           ir::Instruction::Div(instr) => instr.map_subvalues(f).into(),
           ir::Instruction::Shl(instr) => instr.map_subvalues(f).into(),
           ir::Instruction::Shr(instr) => instr.map_subvalues(f).into(),
           ir::Instruction::Call(instr) => instr.map_subvalues(f).into(),
           ir::Instruction::Return(instr) => instr.map_subvalues(f).into(),
        }
    }
}

impl ir::ValueTrait for Instruction
{
    fn ty(&self) -> ir::Type {
        match self {
            &Instruction::Add(ref instr) => instr.ty(),
            &Instruction::Sub(ref instr) => instr.ty(),
            &Instruction::Mul(ref instr) => instr.ty(),
            &Instruction::Div(ref instr) => instr.ty(),
            &Instruction::Shl(ref instr) => instr.ty(),
            &Instruction::Shr(ref instr) => instr.ty(),
            &Instruction::Call(ref instr) => instr.ty(),
            &Instruction::Return(ref instr) => instr.ty(),
         }

    }
}

/// Implements `lang::Instruction` for an instruction.
// TODO: s/impl_lang_instruction/impl_instruction
macro_rules! impl_lang_instruction {
    ($inst:ident) => {
        impl_lang_instruction!($inst: );
    };
    (
        $inst:ident: $($val_name:ident),*
    ) => {
        impl $inst
        {
            pub fn subvalues(&self) -> Vec<::ir::Value> {
                vec![$(*self.$val_name.clone()),*]
            }

            #[allow(unused_mut,unused_variables)]
            pub fn map_subvalues<F>(mut self, mut f: F) -> ::ir::Value
                where F: FnMut(Value) -> Value {

                $(*self.$val_name = f(*self.$val_name.clone()));*;
                self.into()
            }
        }

        impl ::ir::InstructionTrait for $inst { }

        impl Into<::ir::Instruction> for $inst
        {
            fn into(self) -> ir::Instruction {
                ir::Instruction::$inst(self)
            }
        }

        impl Into<::ir::Value> for $inst
        {
            fn into(self) -> ir::Value {
                ir::Value::Instruction(self.into())
            }
        }
    }
}
