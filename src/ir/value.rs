
use ir::{self, types, Type, TypeTrait};
use bit_vec::BitVec;
use std::fmt;
use util::Upcast;

use num::bigint::ToBigInt;

pub trait ValueTrait : Clone + fmt::Display + fmt::Debug + Upcast<Value>
{
    fn ty(&self) -> Type;
}

#[derive(Clone,Debug)]
pub enum Value
{
    Constant(ir::Constant),

    BasicBlock(ir::BasicBlock),
    Function(ir::Function),
}

impl Value
{
    /// Creates an integer, returning `None` if `val` cannot fit into `ty`.
    pub fn constant_integer<T: ToBigInt>(ty: types::Integer, val: T) -> Option<Value> {
        ir::Constant::integer(ty,val).map(|i| i.upcast())
    }

    pub fn constant_float(ty: types::Float, bits: BitVec) -> Value {
        ir::Constant::float(ty,bits).upcast()
    }

    pub fn constant_struct(fields: Vec<Value>) -> Value {
        ir::Constant::strukt(fields).upcast()
    }

    pub fn constant_unit_struct() -> Value {
        ir::Constant::unit_struct().upcast()
    }
}

impl ValueTrait for Value
{
    fn ty(&self) -> Type {
        match self {
            &Value::Constant(ref val) => val.ty(),
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
            &Value::BasicBlock(ref val) => val.fmt(fmt),
            &Value::Function(ref val) => val.fmt(fmt),
        }
    }
}

impl_upcast!(Value);
