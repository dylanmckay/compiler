pub use self::instruction::*;

pub use self::add::Add;
pub use self::sub::Sub;
pub use self::mul::Mul;
pub use self::div::Div;
pub use self::shl::Shl;
pub use self::shr::Shr;
pub use self::call::Call;
pub use self::ret::Return;
pub use self::br::Break;
pub use self::copy::Copy;


#[macro_use]
pub mod instruction
{
    use std::fmt;
    use {instruction,Value,Expression,Type,ExpressionTrait,value,Block,
         Condition};

    pub trait InstructionTrait : fmt::Debug +
                                 Into<Expression> +
                                 ExpressionTrait
    {
    }

    /// An instruction with one operand.
    pub trait Unary : InstructionTrait
    {
        fn with_operand(value: Value) -> Self;

        fn operand(&self) -> &Value;

        // TODO: remove this, it was a stub from when
        // Value and Expression were split up
        fn operand_expression(&self) -> &Expression {
            &self.operand().expression
        }
    }

    /// An instruction with two operands.
    pub trait Binary : InstructionTrait
    {
        fn with_operands(lhs: Value, rhs: Value) -> Self;

        fn operands(&self) -> (&Value,&Value);

        fn operand_expressions(&self) -> (&Expression, &Expression) {
            let (lhs_val, rhs_val) = self.operands();
            (&lhs_val.expression, &rhs_val.expression)
        }
    }

    #[derive(Clone,Debug,PartialEq,Eq)]
    pub enum Instruction
    {
        Add(instruction::Add),
        Sub(instruction::Sub),
        Mul(instruction::Mul),
        Div(instruction::Div),
        Shl(instruction::Shl),
        Shr(instruction::Shr),

        Call(instruction::Call),
        Break(instruction::Break),
        Return(instruction::Return),

        Copy(instruction::Copy),
    }

    impl Instruction
    {
        pub fn add<V1,V2>(lhs: V1, rhs: V2) -> Instruction
            where V1: Into<Value>, V2: Into<Value> {
            instruction::Add::new(lhs.into(), rhs.into()).into()
        }

        pub fn sub<V1,V2>(lhs: V1, rhs: V2) -> Instruction
            where V1: Into<Value>, V2: Into<Value> {
            instruction::Sub::new(lhs.into(), rhs.into()).into()
        }

        pub fn mul<V1,V2>(lhs: V1, rhs: V2) -> Instruction
            where V1: Into<Value>, V2: Into<Value> {
            instruction::Mul::new(lhs.into(), rhs.into()).into()
        }

        pub fn div<V1,V2>(lhs: V1, rhs: V2) -> Instruction
            where V1: Into<Value>, V2: Into<Value> {
            instruction::Div::new(lhs.into(), rhs.into()).into()
        }

        pub fn shl<V1,V2>(val: V1, amount: V2) -> Instruction
            where V1: Into<Value>, V2: Into<Value> {
            instruction::Shl::new(val.into(), amount.into()).into()
        }

        pub fn shr<V1,V2>(val: V1, amount: V2) -> Instruction
            where V1: Into<Value>, V2: Into<Value> {
            instruction::Shr::new(val.into(), amount.into()).into()
        }

        pub fn ret<V>(value: V) -> Instruction
            where V: Into<Value> {
            instruction::Return::new(Some(value.into())).into()
        }

        pub fn ret_void() -> Instruction {
            instruction::Return::new(None).into()
        }

        pub fn br<V>(condition: Condition, target: V) -> Self
            where V: Into<Value> {
            instruction::Break::new(condition, target.into()).into()
        }

        pub fn call<V>(target: V) -> Self
            where V: Into<Value> {
            instruction::Call::new(target.into()).into()
        }

        /// Flattens the instruction.
        ///
        /// Subvalues are placed into registers in the block.
        /// The resulting instruction is returned.
        pub fn flatten(self, block: &mut Block) -> Self {
            self.map_subvalues(|v| {
                if let Expression::Instruction(mut i) = v.expression {

                    // Recursively flatten subvalues
                    i = i.flatten(block);

                    if i.ty().is_void() {
                        i.into()
                    } else { // instruction does not give void
                        let new_reg = value::Register::unnamed(Value::new(i.into()));
                        let reg_ref = Value::register_ref(&new_reg);

                        block.append_value(Value::new(new_reg.into()));
                        reg_ref
                    }


                } else { // don't assign simple values to registers
                    v
                }
            })
        }
    }

    impl InstructionTrait for Instruction { }
    impl ExpressionTrait for Instruction { }

    impl Into<Expression> for Instruction
    {
        fn into(self) -> Expression {
            Expression::Instruction(self)
        }
    }

    impl Into<Value> for Instruction
    {
        fn into(self) -> Value {
            Value::new(self.into())
        }
    }

    impl Instruction
    {
        pub fn subvalues(&self) -> Vec<&Value> {
            match *self {
               Instruction::Add(ref instr) => instr.subvalues(),
               Instruction::Sub(ref instr) => instr.subvalues(),
               Instruction::Mul(ref instr) => instr.subvalues(),
               Instruction::Div(ref instr) => instr.subvalues(),
               Instruction::Shl(ref instr) => instr.subvalues(),
               Instruction::Shr(ref instr) => instr.subvalues(),
               Instruction::Call(ref instr) => instr.subvalues(),
               Instruction::Break(ref instr) => instr.subvalues(),
               Instruction::Return(ref instr) => instr.subvalues(),
               Instruction::Copy(ref instr) => instr.subvalues(),
            }
        }

        pub fn map_subvalues<F>(self, f: F) -> Self
            where F: FnMut(Value) -> Value {

            match self {
               Instruction::Add(instr) => instr.map_subvalues(f).into(),
               Instruction::Sub(instr) => instr.map_subvalues(f).into(),
               Instruction::Mul(instr) => instr.map_subvalues(f).into(),
               Instruction::Div(instr) => instr.map_subvalues(f).into(),
               Instruction::Shl(instr) => instr.map_subvalues(f).into(),
               Instruction::Shr(instr) => instr.map_subvalues(f).into(),
               Instruction::Call(instr) => instr.map_subvalues(f).into(),
               Instruction::Break(instr) => instr.map_subvalues(f).into(),
               Instruction::Return(instr) => instr.map_subvalues(f).into(),
               Instruction::Copy(instr) => instr.map_subvalues(f).into(),
            }
        }

        pub fn is_single_critical(&self) -> bool {
            match *self {
                Instruction::Add(..) => false,
                Instruction::Sub(..) => false,
                Instruction::Mul(..) => false,
                Instruction::Div(..) => false,
                Instruction::Shl(..) => false,
                Instruction::Shr(..) => false,
                Instruction::Call(..) => true,
                Instruction::Break(..) => true,
                Instruction::Return(..) => true,
                Instruction::Copy(..) => false,
            }
        }

        pub fn is_terminator(&self) -> bool {
            match *self {
                Instruction::Return(..) => true,
                Instruction::Break(..) => true,
                _ => false,
            }
        }

        pub fn ty(&self) -> Type {
            match *self {
                Instruction::Add(ref instr) => instr.ty(),
                Instruction::Sub(ref instr) => instr.ty(),
                Instruction::Mul(ref instr) => instr.ty(),
                Instruction::Div(ref instr) => instr.ty(),
                Instruction::Shl(ref instr) => instr.ty(),
                Instruction::Shr(ref instr) => instr.ty(),
                Instruction::Call(ref instr) => instr.ty(),
                Instruction::Break(ref instr) => instr.ty(),
                Instruction::Return(ref instr) => instr.ty(),
                Instruction::Copy(ref instr) => instr.ty(),
             }
        }

    }

    macro_rules! impl_instruction_unary {
        ($inst:ident : $op:ident) => {
            impl ::instruction::Unary for $inst {
                fn with_operand(op: ::Value) -> Self {
                    $inst::new(op)
                }

                fn operand(&self) -> &::Value {
                    &self.$op
                }
            }
        }
    }

    macro_rules! impl_instruction_binary {
        ($inst:ident : $lhs:ident, $rhs:ident) => {
            impl ::instruction::Binary for $inst {
                fn with_operands(lhs: ::Value, rhs: ::Value) -> Self {
                    $inst::new(lhs, rhs)
                }

                fn operands(&self) -> (&::Value,&::Value) {
                    (&self.$lhs,
                     &self.$rhs)
                }
            }
        }
    }

    /// Implements several traits for an instruction.
    macro_rules! impl_instruction {
        // No operands.
        (
            $inst:ident
        ) => {
            impl_instruction!($inst: );
        };

        // One or more operands.
        (
            $inst:ident: $($val_name:ident),*
        ) => {
            impl $inst
            {
                pub fn subvalues(&self) -> Vec<&::Value> {
                    vec![$(&self.$val_name),*]
                }

                #[allow(unused_mut,unused_variables)]
                pub fn map_subvalues<F>(mut self, mut f: F) -> Self
                    where F: FnMut(::Value) -> ::Value {

                    $(*self.$val_name = f(*self.$val_name.clone()));*;
                    self.into()
                }
            }

            impl ::InstructionTrait for $inst { }

            impl ::ExpressionTrait for $inst { }

            impl Into<::Instruction> for $inst
            {
                fn into(self) -> Instruction {
                    Instruction::$inst(self)
                }
            }

            impl Into<::Expression> for $inst
            {
                fn into(self) -> Expression {
                    Expression::Instruction(self.into())
                }
            }
        }
    }
}

pub mod add;
pub mod sub;
pub mod mul;
pub mod div;
pub mod shl;
pub mod shr;
pub mod call;
pub mod ret;
pub mod br;
pub mod copy;

