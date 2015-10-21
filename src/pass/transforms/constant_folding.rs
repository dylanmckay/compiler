
use pass;
use ir;

/// An IR strength reduction pass.
pub struct ConstantFolding;

impl pass::Metadata for ConstantFolding
{
    fn id(&self) -> pass::Id { pass::Id(0x32fabb11) }
    fn name(&self) -> &'static str { "Constant folding" }
}

impl pass::Transform<ir::Value> for ConstantFolding
{
    fn run_value(&mut self, value: ir::Value) -> ir::Value {
        self::fold::value(value)
    }
}

// TODO: blamket impl for all passes
impl Into<pass::Info<ir::Value>> for Box<ConstantFolding>
{
    fn into(self) -> pass::Info<ir::Value> {
        pass::Info::Transform(self)
    }
}

pub mod fold
{
    use ir::{Value,Instruction};
    use ir::value::literal::{Literal,Integer};

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
            (Value::Literal(a),Value::Literal(b)) => (a,b),
            _ => return inst.into(), // we can only fold constants
        };

        match (lhs,rhs) {
            (Literal::Integer(li),Literal::Integer(ri)) => {
                f_int(li,ri).into()
            },
            _ => inst.into(),
        }
    }
}


value_mapping_test!(test_binops : fold::instruction {
    Instruction::add(ir::Value::i8(1),ir::Value::i8(8)) => ir::Value::i8(9),
    Instruction::sub(Value::i8(1),Value::i8(8)) => ir::Value::i8(-7),
    Instruction::mul(Value::i8(1),Value::i8(8)) => ir::Value::i8(8),
    Instruction::div(Value::i8(10),Value::i8(2)) => ir::Value::i8(5),
    Instruction::shl(Value::u8(1),Value::u8(1)) => ir::Value::u8(2),
    Instruction::shr(Value::u8(32),Value::u8(1)) => ir::Value::u8(16)
});

