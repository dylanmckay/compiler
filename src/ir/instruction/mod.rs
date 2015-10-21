
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


#[macro_use]
pub mod instruction
{
    use std::fmt;
    use ir::{self,instruction,Value,Type};

    pub trait InstructionTrait : fmt::Debug +
                                 Into<Value> +
                                 ir::ValueTrait
    {
    }

    /// An instruction with one operand.
    pub trait Unary : InstructionTrait
    {
        fn operand(&self) -> &Value;
    }

    /// An instruction with two operands.
    pub trait Binary : InstructionTrait
    {
        fn operands(&self) -> (&Value,&Value);
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

        pub fn ret(value: Option<ir::Value>) -> Instruction {
            instruction::Return::new(value).into()
        }

        pub fn ret_void() -> Instruction {
            Self::ret(None)
        }

        pub fn br(target: ir::Value) -> Self {
            instruction::Break::unconditional(target).into()
        }

        /// Flattens the instruction.
        ///
        /// Subvalues are placed into registers in the block.
        /// The resulting instruction is returned.
        pub fn flatten(self, block: &mut ir::Block) -> Self {
            self.map_subvalues(|v| {
                if let Value::Instruction(mut i) = v {

                    // Recursively flatten subvalues
                    i = i.flatten(block);

                    if i.ty().is_void() {
                        i.into()
                    } else { // instruction does not give void
                        let new_reg = ir::value::Register::unnamed(i.into());
                        let reg_ref = Value::register_ref(&new_reg);

                        block.add(new_reg);
                        reg_ref
                    }


                } else { // don't assign simple values to registers
                    v
                }
            })
        }
    }

    impl InstructionTrait for Instruction { }
    impl ir::ValueTrait for Instruction { }

    impl Into<Value> for Instruction
    {
        fn into(self) -> Value {
            Value::Instruction(self)
        }
    }

    impl Instruction
    {
        pub fn subvalues(&self) -> Vec<&Value> {
             match self {
                &Instruction::Add(ref instr) => instr.subvalues(),
                &Instruction::Sub(ref instr) => instr.subvalues(),
                &Instruction::Mul(ref instr) => instr.subvalues(),
                &Instruction::Div(ref instr) => instr.subvalues(),
                &Instruction::Shl(ref instr) => instr.subvalues(),
                &Instruction::Shr(ref instr) => instr.subvalues(),
                &Instruction::Call(ref instr) => instr.subvalues(),
                &Instruction::Break(ref instr) => instr.subvalues(),
                &Instruction::Return(ref instr) => instr.subvalues(),
             }
        }
        
        pub fn map_subvalues<F>(self, f: F) -> Self
            where F: FnMut(Value) -> Value {

            match self {
               ir::Instruction::Add(instr) => instr.map_subvalues(f).into(),
               ir::Instruction::Sub(instr) => instr.map_subvalues(f).into(),
               ir::Instruction::Mul(instr) => instr.map_subvalues(f).into(),
               ir::Instruction::Div(instr) => instr.map_subvalues(f).into(),
               ir::Instruction::Shl(instr) => instr.map_subvalues(f).into(),
               ir::Instruction::Shr(instr) => instr.map_subvalues(f).into(),
               ir::Instruction::Call(instr) => instr.map_subvalues(f).into(),
               ir::Instruction::Break(instr) => instr.map_subvalues(f).into(),
               ir::Instruction::Return(instr) => instr.map_subvalues(f).into(),
            }
        }

        pub fn is_single_critical(&self) -> bool {
            match self {
                &ir::Instruction::Add(..) => false,
                &ir::Instruction::Sub(..) => false,
                &ir::Instruction::Mul(..) => false,
                &ir::Instruction::Div(..) => false,
                &ir::Instruction::Shl(..) => false,
                &ir::Instruction::Shr(..) => false,
                &ir::Instruction::Call(..) => true,
                &ir::Instruction::Break(..) => true,
                &ir::Instruction::Return(..) => true,
            }
        }

        pub fn is_terminator(&self) -> bool {
            match self {
                &ir::Instruction::Return(..) => true,
                &ir::Instruction::Break(..) => true,
                _ => false,
            }
        }

        pub fn ty(&self) -> ir::Type {
            match self {
                &Instruction::Add(ref instr) => instr.ty(),
                &Instruction::Sub(ref instr) => instr.ty(),
                &Instruction::Mul(ref instr) => instr.ty(),
                &Instruction::Div(ref instr) => instr.ty(),
                &Instruction::Shl(ref instr) => instr.ty(),
                &Instruction::Shr(ref instr) => instr.ty(),
                &Instruction::Call(ref instr) => instr.ty(),
                &Instruction::Break(ref instr) => instr.ty(),
                &Instruction::Return(ref instr) => instr.ty(),
             }
        }

    }

    /// Implements several traits for an instruction.
    macro_rules! impl_instruction {
        // An instruction with no operands.
        ($inst:ident) => {
            impl_instruction_internal!($inst);
        };

        // A unary instruction.
        ($inst:ident : $op:ident) => {
            impl_instruction_internal!($inst: $op);

            impl ::ir::instruction::Unary for $inst {
                fn operand(&self) -> &::ir::Value {
                    &self.$op
                }
            }
        };

        // A binary instruction.
        ($inst:ident : $op1:ident, $op2:ident) => {
            impl_instruction_internal!($inst: $op1, $op2);

            impl ::ir::instruction::Binary for $inst {
                fn operands(&self) -> (&::ir::Value,&::ir::Value) {
                    (&self.$op1,
                     &self.$op2)
                }
            }
        };
    }

    /// Implements several traits for an instruction.
    /// **Note**: Do not use this macro directly.
    macro_rules! impl_instruction_internal {
        // No operands.
        (
            $inst:ident
        ) => {
            impl_instruction_internal!($inst: );
        };

        // One or more operands.
        (
            $inst:ident: $($val_name:ident),*
        ) => {
            impl $inst
            {
                pub fn subvalues(&self) -> Vec<&::ir::Value> {
                    vec![$(&self.$val_name),*]
                }

                #[allow(unused_mut,unused_variables)]
                pub fn map_subvalues<F>(mut self, mut f: F) -> Self
                    where F: FnMut(Value) -> Value {

                    $(*self.$val_name = f(*self.$val_name.clone()));*;
                    self.into()
                }
            }

            impl ::ir::InstructionTrait for $inst { }

            impl ::ir::ValueTrait for $inst { }

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

