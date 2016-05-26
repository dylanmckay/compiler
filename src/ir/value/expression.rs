use {types,value,Type,Instruction,Name,Block,
     Parameter,Function,Global,Condition};
use std::fmt;
use util;
use super::Value;
use num::bigint::ToBigInt;
use bit_vec::BitVec;

pub trait ExpressionTrait : Clone + fmt::Debug + Into<Expression>
{
}

#[derive(Clone,Debug,PartialEq,Eq)]
pub enum Expression
{
    Literal(value::Literal),
    Register(value::Register),
    Instruction(Instruction),

    GlobalRef(value::GlobalRef),
    BlockRef(value::BlockRef),
    FunctionRef(value::FunctionRef),
    RegisterRef(value::RegisterRef),
    ArgumentRef(value::ArgumentRef),
    /// A stub expression for storing an expression that hasn't
    /// been resolved yet.
    UnresolvedRef(util::Id),

    String(value::String),
}

impl Expression
{
    pub fn decimal(_ty: types::Decimal, _bits: BitVec) -> Self {
        unimplemented!();
    }

    pub fn strukt(_fields: Vec<Value>) -> Self {
        unimplemented!();
    }

    pub fn unit_struct() -> Self {
        unimplemented!();
    }

    /// Creates an integer, returning `None` if `val` cannot fit into `ty`.
    pub fn integer<T: ToBigInt>(ty: types::Integer, val: T) -> Option<Self> {
        value::Literal::integer(ty,val).map(|i| i.into())
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

    pub fn global_ref(global: &Global) -> Expression {
        value::GlobalRef::reference(global).into()
    }

    pub fn block_ref(block: &Block) -> Expression {
        value::BlockRef::reference(block).into()
    }

    pub fn function_ref(func: &Function) -> Expression {
        value::FunctionRef::reference(func).into()
    }

    pub fn register_ref(reg: &value::Register) -> Expression {
        value::RegisterRef::reference(reg).into()
    }

    pub fn argument_ref(param: &Parameter) -> Expression {
        value::ArgumentRef::reference(param).into()
    }

    /// Creates a new string.
    pub fn string<S>(text: S) -> Self
        where S: Into<String> {
        value::String::new(text.into()).into()
    }

    /// Creates an unnamed register.
    pub fn register<V>(value: V) -> Expression
        where V: Into<Value> {
        value::Register::unnamed(value.into()).into()
    }

    /// Creates a named register.
    pub fn register_named<I>(name: I, value: Expression) -> Expression
        where I: Into<String> {

        let name = Name::named(name);
        value::Register::new(name, value.into()).into()
    }

    pub fn instruction<I>(inst: I) -> Self
        where I: Into<Instruction> {
        Expression::Instruction(inst.into())
    }

    pub fn add<V>(lhs: V, rhs: V) -> Self
        where V: Into<Value> {
        Expression::instruction(
            Instruction::add(lhs, rhs)
        )
    }

    pub fn sub<V>(lhs: V, rhs: V) -> Self
        where V: Into<Value> {
        Expression::instruction(
            Instruction::sub(lhs, rhs)
        )
    }

    pub fn mul<V>(lhs: V, rhs: V) -> Self
        where V: Into<Value> {
        Expression::instruction(
            Instruction::mul(lhs, rhs)
        )
    }

    pub fn div<V>(lhs: V, rhs: V) -> Self
        where V: Into<Value> {
        Expression::instruction(
            Instruction::div(lhs, rhs)
        )
    }

    pub fn shl<V>(value: V, amount: V) -> Self
        where V: Into<Value> {
        Expression::instruction(
            Instruction::shl(value, amount)
        )
    }

    pub fn shr<V>(value: V, amount: V) -> Self
        where V: Into<Value> {
        Expression::instruction(
            Instruction::shr(value, amount)
        )
    }

    pub fn call<V>(target: V) -> Self
        where V: Into<Value> {
        Expression::instruction(
            Instruction::call(target)
        )
    }

    pub fn br<V>(condition: Condition, target: V) -> Self
        where V: Into<Value> {
        Expression::instruction(
            Instruction::br(condition, target)
        )
    }

    pub fn ret(value: Value) -> Self {
        Expression::instruction(
            Instruction::ret(value)
        )
    }

    pub fn ret_void() -> Self {
        Expression::instruction(
            Instruction::ret_void()
        )
    }


    pub fn as_literal(&self) -> Option<&value::Literal> {
        match *self {
            Expression::Literal(ref v) => Some(v),
            _ => None,
        }
    }

    pub fn is_literal(&self) -> bool {
        match *self {
            Expression::Literal(..) => true,
            _ => false,
        }
    }

    /// Checks if the value is an instruction.
    pub fn is_instruction(&self) -> bool {
        match *self {
            Expression::Instruction(..) => true,
            _ => false,
        }
    }

    pub fn is_function_ref(&self) -> bool {
        match *self {
            Expression::FunctionRef(..) => true,
            _ => false,
        }
    }

    pub fn is_block_ref(&self) -> bool {
        match *self {
            Expression::BlockRef(..) => true,
            _ => false,
        }
    }

    pub fn is_register_ref(&self) -> bool {
        if let Expression::RegisterRef(..) = *self { true } else { false }
    }

    pub fn is_argument_ref(&self) -> bool {
        if let Expression::ArgumentRef(..) = *self { true } else { false }
    }

    pub fn ty(&self) -> Type {
         match *self {
            Expression::Literal(ref val) => val.ty(),
            Expression::Register(ref val) => val.ty(),
            Expression::Instruction(ref val) => val.ty(),
            Expression::GlobalRef(ref val) => val.ty(),
            Expression::BlockRef(ref val) => val.ty(),
            Expression::FunctionRef(ref val) => val.ty(),
            Expression::RegisterRef(ref val) => val.ty(),
            Expression::ArgumentRef(ref val) => val.ty(),
            Expression::UnresolvedRef(id) => {
                panic!("unresolved reference: {}", id);
            },
            Expression::String(ref val) => val.ty(),
        }
    }

    /// Checks if the value is simple.
    ///
    /// Simple values are printed without parentheses.
    pub fn is_simple(&self) -> bool {
         match *self{
             Expression::Literal(..) => true,
             Expression::Register(..) => true,
             Expression::Instruction(..) => false,
             Expression::GlobalRef(..) => true,
             Expression::BlockRef(..) => true,
             Expression::FunctionRef(..) => true,
             Expression::RegisterRef(..) => true,
             Expression::ArgumentRef(..) => true,
             Expression::UnresolvedRef(id) => {
                 panic!("unresolved reference: {}", id);
             },
             Expression::String(..) => true,
         }
    }

    pub fn subvalues(&self) -> Vec<&Value> {
        match *self {
            Expression::Instruction(ref i) => i.subvalues(),
            _ => Vec::new(),
        }
    }

    pub fn map_subvalues<F>(self, f: F) -> Self
        where F: FnMut(Value) -> Value {
        match self {
            Expression::Instruction(i) => i.map_subvalues(f).into(),
            _ => self,
        }
    }

    /// Flattens the value into registers.
    /// 
    /// This takes all subvalues that are not simple and
    /// hoists them out of the value, into a register which is
    /// then appended to the block, and then the register reference
    /// is then used.
    /// 
    /// This will convert an SSA value into a non-SSA value.
    pub fn flatten(self, block: &mut Block) -> Self {
        // only instructions need flattening
        if let Expression::Instruction(i) = self {
            i.flatten(block).into()
        } else {
            self
        }
    }

    pub fn is_single_critical(&self) -> bool {
        match *self {
            Expression::Literal(..) => false,
            Expression::Instruction(ref i) => i.is_single_critical(),
            _ => true,
        }
    }

    /// Checks if a value is critical.
    ///
    /// Recursively checks whether this value is critical.
    pub fn is_critical(&self) -> bool {
        let subvalues_critical = self.subvalues().iter().any(|a| a.node.is_critical());

        self.is_single_critical() || subvalues_critical
    }

    /// Checks if the value is a terminator.
    pub fn is_terminator(&self) -> bool {
        // only instructions can be terminators
        if let Expression::Instruction(ref inst) = *self {
            inst.is_terminator()
        } else {
            false
        }
    }
}

impl ExpressionTrait for Expression { }

macro_rules! impl_expression {
    ($ty:ident) => {
        impl ::ExpressionTrait for $ty { }

        impl Into<::Expression> for $ty
        {
            fn into(self) -> ::Expression {
                ::Expression::$ty(self)
            }
        }
    }
}

/// Implements Into<Expression> for u8,i32,etc
macro_rules! impl_into_value_for_integers {
    (
        $( $ty:ident ),*
    ) => {
        $(
            impl Into<Expression> for $ty
            {
                fn into(self) -> Expression {
                    Expression::$ty(self)
                }
            }

            impl Into<Value> for $ty
            {
                fn into(self) -> Value {
                    Value::new(self.into())
                }
            }
        )*
    }
}

impl_into_value_for_integers!(u8,u16,u32,u64,i8,i16,i32,i64);
