
use ir::{self,types,Type};
use bit_vec::BitVec;
use std::fmt;
use lang;

use num::bigint::ToBigInt;

pub trait ValueTrait : Clone + fmt::Display + fmt::Debug + Into<Value>
{
    fn ty(&self) -> Type;
}

#[derive(Clone,Debug)]
pub enum Value
{
    Constant(ir::Constant),

    Instruction(ir::Instruction),
    BasicBlock(ir::BasicBlock),
    Function(ir::Function),
}

impl Value
{
    /// Creates an integer, returning `None` if `val` cannot fit into `ty`.
    pub fn integer<T: ToBigInt>(ty: types::Integer, val: T) -> Option<Value> {
        ir::Constant::integer(ty,val).map(|i| i.into())
    }

    pub fn float(ty: types::Float, bits: BitVec) -> Value {
        ir::Constant::float(ty,bits).into()
    }

    pub fn strukt(fields: Vec<Value>) -> Value {
        ir::Constant::strukt(fields).into()
    }

    pub fn unit_struct() -> Value {
        ir::Constant::unit_struct().into()
    }

    pub fn as_constant(&self) -> Option<&ir::Constant> {
        match self {
            &Value::Constant(ref v) => Some(v),
            _ => None,
        }
    }
}

impl lang::Value for Value
{
    fn subvalues(&self) -> Vec<Self> {
        unimplemented!();
    }

    fn map_subvalues<F>(self, f: F) -> Self
        where F: FnMut(Self) -> Self {
        use lang::BasicBlock;

        match self {
            Value::Instruction(i) => i.map_subvalues(f),
            Value::BasicBlock(i) => i.map_subvalues(f).into(),
            _ => self,
        }
    }
}

impl ValueTrait for Value
{
    fn ty(&self) -> Type {
        match self {
            &Value::Constant(ref val) => val.ty(),
            &Value::Instruction(ref val) => val.ty(),
            &Value::BasicBlock(ref val) => val.ty(),
            &Value::Function(ref val) => val.ty(),
        }
    }
}

impl fmt::Display for Value
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        match self {
            &Value::Constant(ref val) => val.fmt(fmt),
            &Value::Instruction(ref val) => val.fmt(fmt),
            &Value::BasicBlock(ref val) => val.fmt(fmt),
            &Value::Function(ref val) => val.fmt(fmt),
        }
    }
}

