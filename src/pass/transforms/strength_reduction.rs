
use pass;
use ir::{self,Instruction};

/// An IR strength reduction pass.
pub struct StrengthReduction;

impl pass::Metadata for StrengthReduction
{
    fn id(&self) -> pass::Id { pass::Id(0x242343a1) }
    fn name(&self) -> &'static str { "Strength reduction" }
}

impl pass::PassMut<ir::Module> for StrengthReduction
{
    fn run_instruction(&mut self, inst: ir::Instruction) -> ir::Instruction {

        match inst {
            Instruction::Mul(i) => self::reduce::mul(i),
            _ => inst,
        }
    }
}

// TODO: blamket impl for all passes
impl Into<pass::Info<ir::Module>> for Box<StrengthReduction>
{
    fn into(self) -> pass::Info<ir::Module> {
        pass::Info::Mutable(self)
    }
}

pub mod reduce
{
    use ir::{self,instructions,Instruction};

    pub fn mul(inst: instructions::Mul) -> ir::Instruction {
        self::mul_pow2_shl(inst)
    }

    pub fn mul_pow2_shl(inst: instructions::Mul) -> ir::Instruction {
        use ::util::Upcast;

        let ty = inst.ty().clone();
        let (lhs,rhs) = inst.multiplicands();

        let lhs_if_shift = lhs.as_constant().and_then(|a| util::get_mul_shift_amount(a));
        let rhs_if_shift = rhs.as_constant().and_then(|a| util::get_mul_shift_amount(a));

        // multiplication is commutative so switch the order if necessary.
        let (value,shift) = match (lhs_if_shift,
                                   rhs_if_shift) {
            (None, None) => return inst.upcast(),
            // constant folding should've caught this
            (Some(_),Some(_)) => return inst.upcast(),

            (None, Some(v)) => (lhs.clone(),v.upcast()),
            (Some(v),None) => (v.upcast(),rhs.clone()),
        };

        Instruction::shl(ty, value, shift).upcast()
    }

    pub mod util {
        use ir::{self,Value,ValueTrait};

        pub fn is_power_of_two(value: &ir::Constant) -> bool {
            use ::num::traits::ToPrimitive;

            // FIXME: this will panic if the value >64bits
            // N.B. this will give that `0` is a power of two.
            //      we don't care because constant folding has already been done.
            value.as_integer()
                 .map(|i| i.value.to_i64().unwrap())
                 .map_or(false, |x| {
                     debug_assert!(x != 0);
                     
                     (x & (x - 1)) == 0
                 })
        }

        /// If `value` is a power of two, this gets the
        /// number of bits that would make an equivalent shift.
        /// 
        /// Returns `None` if the value is not a power of two.
        pub fn get_mul_shift_amount(value: &ir::Constant) -> Option<Value> {
            use ::num::traits::ToPrimitive;

            if !is_power_of_two(&value) {
                return None;
            }

            // FIXME: this will panic if value >64bits
            let const_val = value.as_integer()
                                 .expect("value must be an integer");

            let ty = const_val.ty;
            let val = const_val.value.to_f64().unwrap();

            let log2 = 2f64.log(2.);

            let n = (val.log(2.) / log2) as u64;

            Some(Value::integer(ty, n).unwrap())
        }
    }
}
