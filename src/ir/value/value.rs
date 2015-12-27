use {Global,Function,Parameter,Block,types,Expression,Type,Condition};
use value;
use util;

use num::bigint::ToBigInt;
use bit_vec::BitVec;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Value
{
    pub expression: Expression,
}

impl Value
{
    pub fn new(expression: Expression) -> Self {
        Value {
            expression: expression,
        }
    }

    pub fn ty(&self) -> Type {
        self.expression.ty()
    }

    pub fn global_ref(global: &Global) -> Self {
        Value::new(Expression::global_ref(global))
    }

    pub fn function_ref(func: &Function) -> Self {
        Value::new(Expression::function_ref(func))
    }

    pub fn block_ref(block: &Block) -> Self {
        Value::new(Expression::block_ref(block))
    }

    pub fn register_ref(register: &value::Register) -> Self {
        Value::new(Expression::register_ref(register))
    }

    pub fn argument_ref(param: &Parameter) -> Self {
        Value::new(Expression::argument_ref(param))
    }

    pub fn add<V>(lhs: V, rhs: V) -> Self
        where V: Into<Value> {
        Value::new(Expression::add(lhs, rhs))
    }

    pub fn sub<V>(lhs: V, rhs: V) -> Self
        where V: Into<Value> {
        Value::new(Expression::sub(lhs, rhs))
    }

    pub fn mul<V>(lhs: V, rhs: V) -> Self
        where V: Into<Value> {
        Value::new(Expression::mul(lhs, rhs))
    }

    pub fn div<V>(lhs: V, rhs: V) -> Self
        where V: Into<Value> {
        Value::new(Expression::div(lhs, rhs))
    }

    pub fn shl<V>(value: V, amount: V) -> Self
        where V: Into<Value> {
        Value::new(Expression::shl(value, amount))
    }

    pub fn shr<V>(value: V, amount: V) -> Self
        where V: Into<Value> {
        Value::new(Expression::shr(value, amount))
    }

    pub fn call(target: Value) -> Self {
        Value::new(Expression::call(target))
    }

    pub fn br(condition: Condition, target: Value) -> Self {
        Value::new(Expression::br(condition, target))
    }

    pub fn ret(value: Value) -> Self {
        Value::new(Expression::ret(value))
    }

    pub fn ret_void() -> Self {
        Value::new(Expression::ret_void())
    }

    /// Creates an integer, returning `None` if `val` cannot fit into `ty`.
    pub fn integer<T: ToBigInt>(ty: types::Integer, val: T)
        -> Option<Self> {
        Expression::integer(ty, val).map(|i| {
            Value::new(i.into())
        })
    }

    pub fn decimal(ty: types::Decimal, bits: BitVec) -> Self {
        Value::new(Expression::decimal(ty, bits))
    }

    pub fn strukt(fields: Vec<Value>) -> Self {
        Value::new(Expression::strukt(fields))
    }

    pub fn unit_struct() -> Self {
        Value::new(value::Literal::unit_struct().into())
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

    pub fn map_subvalues<F>(mut self, f: F) -> Self
        where F: FnMut(Value) -> Value {
        self.expression = self.expression.map_subvalues(f);
        self
    }

    pub fn map_expression<F>(mut self, mut f: F) -> Self
        where F: FnMut(Expression) -> Expression {
        self.expression = f(self.expression);
        self
    }
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

    fn subvalues(&self) -> Vec<&Self> {
        self.expression.subvalues()
    }

    fn map_subvalues<F>(mut self, f: F) -> Self
        where F: FnMut(Self) -> Self {
        self.expression = self.expression.map_subvalues(f);
        self
    }

    fn flatten(mut self, block: &mut Block) -> Self {
        self.expression = self.expression.flatten(block);
        self
    }

    fn is_single_critical(&self) -> bool {
        self.expression.is_single_critical()
    }

    fn is_simple(&self) -> bool {
        self.expression.is_simple()
    }

    fn ty(&self) -> Type {
        self.expression.ty()
    }

    fn is_terminator(&self) -> bool {
        self.expression.is_terminator()
    }
}

