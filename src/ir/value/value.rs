use ir;

use ir::{Expression,Type};
use ir::types;
use util;

use num::bigint::ToBigInt;
use bit_vec::BitVec;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Value
{
    expression: Expression,
}

impl Value
{
    pub fn new(expression: Expression) -> Self {
        Value {
            expression: expression,
        }
    }

    pub fn expression(&self) -> &Expression {
        &self.expression
    }

    pub fn expression_mut(&mut self) -> &mut Expression {
        &mut self.expression
    }

    pub fn into_expression(self) -> Expression {
        self.expression
    }

    pub fn ty(&self) -> ir::Type {
        self.expression.ty()
    }

    pub fn global_ref(global: &ir::Global) -> Self {
        unimplemented!();
    }

    pub fn function_ref(func: &ir::Function) -> Self {
        //Value::new(Expression::function_ref(func))
        unimplemented!();
    }

    pub fn block_ref(block: &ir::Block) -> Self {
        unimplemented!();
    }

    pub fn register_ref(register: &ir::value::Register) -> Self {
        unimplemented!();
    }

    pub fn add<V>(lhs: V, rhs: V) -> Self
        where V: Into<Value> {
        unimplemented!();
    }

    pub fn sub<V>(lhs: V, rhs: V) -> Self
        where V: Into<Value> {
        unimplemented!();
    }

    pub fn mul<V>(lhs: V, rhs: V) -> Self
        where V: Into<Value> {
        unimplemented!();
    }

    pub fn div<V>(lhs: V, rhs: V) -> Self
        where V: Into<Value> {
        unimplemented!();
    }

    pub fn shl<V>(value: V, amount: V) -> Self
        where V: Into<Value> {
        unimplemented!();
    }

    pub fn shr<V>(value: V, amount: V) -> Self
        where V: Into<Value> {
        unimplemented!();
    }

    pub fn call(target: ir::Value) -> Self {
        //Value::new(Expression::call(target))
        unimplemented!();
    }

    pub fn br(target: ir::Value) -> Self {
        unimplemented!();
    }

    pub fn ret(value: Option<::ir::Value>) -> Self {
        unimplemented!();
    }

    pub fn ret_void() -> Self {
        Value::ret(None)
    }

    /// Creates an integer, returning `None` if `val` cannot fit into `ty`.
    pub fn integer<T: ToBigInt>(ty: types::Integer, val: T)
        -> Option<Self> {
        ir::value::Literal::integer(ty,val).map(|i| {
            Value::new(i.into())
        })
    }

    pub fn decimal(ty: types::Decimal, bits: BitVec) -> Self {
        Value::new(ir::value::Literal::decimal(ty,bits).into())
    }

    pub fn strukt(fields: Vec<Value>) -> Self {
        Value::new(ir::value::Literal::strukt(fields).into())
    }

    pub fn unit_struct() -> Self {
        Value::new(ir::value::Literal::unit_struct().into())
    }

    /// Creates a signed integer value.
    pub fn i<T: ToBigInt>(bit_width: u16, value: T) -> Self {
        let ty = types::Integer::new(util::IntegerKind::Signed, bit_width);
        Self::integer(ty, value).unwrap()
    }

    /// Creates an unsigned integer value.
    pub fn u<T: ToBigInt>(bit_width: u16, value: T) -> Self {
       let ty = types::Integer::new(util::IntegerKind::Unsigned, bit_width);
       Self::integer(ty, value).unwrap()
    }

    pub fn u8(value: u8)   -> Self { Self::u(8, value) }
    pub fn u16(value: u16) -> Self { Self::u(16, value) }
    pub fn u32(value: u32) -> Self { Self::u(32, value) }
    pub fn u64(value: u64) -> Self { Self::u(64, value) }
    pub fn i8(value: i8)   -> Self { Self::i(8, value) }
    pub fn i16(value: i16) -> Self { Self::i(16, value) }
    pub fn i32(value: i32) -> Self { Self::i(32, value) }
    pub fn i64(value: i64) -> Self { Self::i(64, value) }
}

impl Into<Expression> for Value
{
    fn into(self) -> Expression {
        self.expression
    }
}

impl Into<Value> for Expression
{
    fn into(self) -> Value {
        Value::new(self)
    }
}

impl ::lang::Value for Value
{
    type Type = Type;

    // FIXME: subvalue support is patchy

    fn subvalues(&self) -> Vec<&Self> {
        match self.expression {
            Expression::Instruction(ref i) => i.subvalues(),
            _ => Vec::new(),
        }
    }

    fn map_subvalues<F>(self, _f: F) -> Self
        where F: FnMut(Self) -> Self {
        // TODO: fix this
        unimplemented!();
        //
        // let expr = match self.expression {
        //     Expression::Instruction(i) => i.map_subvalues(f).into(),
        //     e => e,
        // };
        //
        // self.expression = expr;
        // self
    }

    fn flatten(mut self, block: &mut ir::Block) -> Self {
        // only instructions need flattening
        let expr = if let Expression::Instruction(i) = self.expression {
            i.flatten(block).into()
        } else {
            self.expression
        };

        self.expression = expr;
        self
    }

    fn is_single_critical(&self) -> bool {
        match self.expression {
            ir::Expression::Literal(..) => false,
            ir::Expression::Instruction(ref i) => i.is_single_critical(),
            _ => true,
        }
    }

    fn is_simple(&self) -> bool {
        self.expression.is_simple()
    }

    fn ty(&self) -> ir::Type {
        self.expression.ty()
    }

    fn is_terminator(&self) -> bool {
        // only instructions can be terminators
        if let ir::Expression::Instruction(ref inst) = self.expression {
            inst.is_terminator()
        } else {
            false
        }
    }
}

