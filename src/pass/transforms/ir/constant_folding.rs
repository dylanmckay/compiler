
use pass;
use ir;

/// An IR strength reduction pass.
pub struct ConstantFolding;

impl pass::Metadata for ConstantFolding
{
    fn id(&self) -> pass::Id { pass::Id(0x32fabb11) }
    fn name(&self) -> &'static str { "Constant folding" }
}

impl pass::PassMut<ir::Module> for ConstantFolding
{
    fn run_value(&mut self, value: ir::Value) -> ir::Value {
        self::fold::value(value)
    }
}

// TODO: blamket impl for all passes
impl Into<pass::Info<ir::Module>> for Box<ConstantFolding>
{
    fn into(self) -> pass::Info<ir::Module> {
        pass::Info::Mutable(self)
    }
}

pub mod fold
{
    use ir::{Value,Instruction,Constant};
    use ir::constant::Integer;

    pub fn value(value: Value) -> Value {
        match value {
            Value::Instruction(i) => instruction(i),
            _ => value,
        }
    }

    pub fn instruction(inst: Instruction) -> Value {
        use ir::instruction::Binary;
        let inst_copy = inst.clone();

        match inst {
            Instruction::Add(i) => arithmetic_binop(inst_copy, i.operands(), |a,b| a+b),
            Instruction::Sub(i) => arithmetic_binop(inst_copy, i.operands(), |a,b| a-b),
            Instruction::Mul(i) => arithmetic_binop(inst_copy, i.operands(), |a,b| a*b),
            Instruction::Div(i) => arithmetic_binop(inst_copy, i.operands(), |a,b| a/b),
            Instruction::Shl(i) => arithmetic_binop(inst_copy, i.operands(), |a,b| a<<b),
            Instruction::Shr(i) => arithmetic_binop(inst_copy, i.operands(), |a,b| a>>b),
            _ => inst.into(),
        }
    }

    pub fn arithmetic_binop<FI>(inst: Instruction,
                                values: (Value,Value),
                                mut f_int: FI) -> Value
        where FI: FnMut(Integer,Integer) -> Integer {

        // make sure the values are constants
        let (lhs,rhs) = match values {
            (Value::Constant(a),Value::Constant(b)) => (a,b),
            _ => return inst.into(), // we can only fold constants
        };

        match (lhs,rhs) {
            (Constant::Integer(li),Constant::Integer(ri)) => {
                f_int(li,ri).into()
            },
            _ => inst.into(),
        }
    }
}
