
use pass;
use ir;

// FIXME: do not perform strength reduction if the value would overflow

/// An IR strength reduction pass.
pub struct StrengthReduction;

impl pass::Metadata for StrengthReduction
{
    fn id(&self) -> pass::Id { pass::Id(0x242343a1) }
    fn name(&self) -> &'static str { "strength reduction" }
}

impl pass::Transform<ir::Value> for StrengthReduction
{
    fn run_value(&mut self, value: ir::Value) -> ir::Value {

        // check if the value is an instruction
        let inst = match value.into_expression() {
            ir::Expression::Instruction(i) => i,
            e => return ir::Value::new(e),
        };

        ir::Value::new(self::reduce::reduce(inst).into())
    }
}

// TODO: blamket impl for all passes
impl Into<pass::Info<ir::Value>> for Box<StrengthReduction>
{
    fn into(self) -> pass::Info<ir::Value> {
        pass::Info::Transform(self)
    }
}

pub mod reduce
{
    use ir::{self,instruction,Instruction,Value};

    pub fn reduce(inst: Instruction) -> ir::Instruction {

        match inst {
            Instruction::Mul(i) => self::mul(i),
            Instruction::Div(i) => self::div(i),
            _ => inst,
        }
    }

    pub fn mul(inst: instruction::Mul) -> ir::Instruction {
        self::mul_pow2_shl(inst)
    }

    pub fn div(inst: instruction::Div) -> ir::Instruction {
        self::div_pow2_shr(inst)
    }

    /// Converts a multiplication instruction into a shift instruction if possible.
    pub fn mul_pow2_shl(inst: instruction::Mul) -> ir::Instruction {
        use ir::instruction::Binary;

        let (lhs,rhs) = inst.operands();
        match maybe_shift_expressions(lhs.clone(),rhs.clone()) {
            Some((value,amount)) => Instruction::shl(value,amount).into(),
            None => inst.clone().into(),
        }
    }

    /// Converts a division instruction into a shift instruction if possible.
    pub fn div_pow2_shr(inst: instruction::Div) -> ir::Instruction {
        use ir::instruction::Binary;

        let (lhs,rhs) = inst.operands();
        match maybe_shift_expressions(lhs.clone(),rhs.clone()) {
            Some((value,amount)) => Instruction::shr(value,amount).into(),
            None => inst.clone().into(),
        }
    }

    /// Tries to convert the operands of a mul or div instruction into the operands
    /// of a shift instruction.
    pub fn maybe_shift_expressions(lhs: ir::Expression, rhs: ir::Expression)
        -> Option<(ir::Expression,ir::Expression)> {

        let lhs_if_shift = lhs.as_literal().and_then(|a| util::get_shift_amount(a));
        let rhs_if_shift = rhs.as_literal().and_then(|a| util::get_shift_amount(a));

        // multiplication is commutative so switch the order if necessary.
        match (lhs_if_shift,
                                   rhs_if_shift) {
            (None, None) => None,
            // Constant folding should've caught this, but handle it anyway.
            // Both operands could be treated as the shift amount,
            // so use RHS.
            (Some(_),Some(v)) => Some((lhs.clone(),v.into())),

            (None, Some(v)) => Some((lhs.clone(),v.into())),
            (Some(v),None) => Some((v.into(),rhs.clone())),
        }
    }

    pub mod util {
        use ir::{value,Expression,Value};

        /// Checks if a value is an integer and a power of two.
        pub fn is_power_of_two(value: &value::Literal) -> bool {
            use ::num::traits::ToPrimitive;

            // FIXME: this will panic if the value >64bits
            // N.B. this will give that `0` is a power of two.
            //      we don't care because constant folding has already been done.
            value.as_integer()
                 .map(|i| i.value().to_i64().unwrap())
                 .map_or(false, |x| {
                     debug_assert!(x != 0);
                     
                     (x & (x - 1)) == 0
                 })
        }

        /// If `expr` is a power of two, this gets the
        /// number of bits that would make an equivalent shift.
        /// 
        /// Returns `None` if the expr is not a power of two.
        pub fn get_shift_amount(expr: &value::Literal) -> Option<Expression> {
            use ::num::traits::ToPrimitive;

            if !is_power_of_two(&expr) {
                return None;
            }

            // FIXME: this will panic if expr >64bits
            let const_val = expr.as_integer()
                                 .expect("expr must be an integer");

            let ty = const_val.integer_ty();
            let val = const_val.value().to_f64().unwrap();

            let log2 = 2f64.log(2.);

            let n = (val.log(2.) / log2) as u64;

            Some(Expression::integer(ty, n).unwrap())
        }
    }
}

value_mapping_test!(test_mul_div_shift : reduce::reduce {

    // i8
    Instruction::mul(2 as i8,1 as i8) => Instruction::shl(2 as i8,0 as i8),
    Instruction::mul(2 as i8,2 as i8) => Instruction::shl(2 as i8,1 as i8),
    Instruction::div(2 as i8,1 as i8) => Instruction::shr(2 as i8,0 as i8),
    Instruction::div(2 as i8,2 as i8) => Instruction::shr(2 as i8,1 as i8),

    // u32
    Instruction::mul(2 as u32,1 as u32) => Instruction::shl(2 as u32,0 as u32),
    Instruction::mul(2 as u32,2 as u32) => Instruction::shl(2 as u32,1 as u32),
    Instruction::div(2 as u32,1 as u32) => Instruction::shr(2 as u32,0 as u32),
    Instruction::div(2 as u32,2 as u32) => Instruction::shr(2 as u32,1 as u32),

    // Cases we shouldn't handle (non-powers of two).
    Instruction::mul(5 as i16,3 as i16) => Instruction::mul(5 as i16,3 as i16),
    Instruction::div(5 as i16,3 as i16) => Instruction::div(5 as i16,3 as i16)
});
