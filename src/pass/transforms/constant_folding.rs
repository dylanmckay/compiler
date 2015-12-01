
use pass;
use ir;

/// An IR strength reduction pass.
pub struct ConstantFolding;

impl pass::Metadata for ConstantFolding
{
    fn id(&self) -> pass::Id { pass::Id(0x32fabb11) }
    fn name(&self) -> &'static str { "constant folding" }
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
    use ir::{self,Value,Expression,Instruction};
    use ir::value::literal::{Literal,Integer};

    pub fn value(value: Value) -> Value {
        match value.into_expression() {
            Expression::Instruction(i) => Value::new(instruction(i)),
            e => Value::new(e),
        }
    }

    pub fn instruction(inst: Instruction) -> Expression {

        match inst {
            Instruction::Add(i) => arithmetic_binop(i, |a,b| a+b),
            Instruction::Sub(i) => arithmetic_binop(i, |a,b| a-b),
            Instruction::Mul(i) => arithmetic_binop(i, |a,b| a*b),
            Instruction::Div(i) => arithmetic_binop(i, |a,b| a/b),
            Instruction::Shl(i) => arithmetic_binop(i, |a,b| a<<b),
            Instruction::Shr(i) => arithmetic_binop(i, |a,b| a>>b),
            _ => inst.into(),
        }
    }

    pub fn arithmetic_binop<I,FI>(inst: I,
                                  mut f_int: FI) -> Expression
        where I: ir::instruction::Binary,
              FI: FnMut(Integer,Integer) -> Integer {

        use ir::instruction::Binary;

        // make sure the values are constants
        let (lhs,rhs) = match inst.operand_expressions() {
            (&Expression::Literal(ref a),&Expression::Literal(ref b)) => (a.clone(),b.clone()),
            _ => return inst.clone().into(), // we can only fold constants
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
    Instruction::add(1 as i8, 8 as i8) => ir::Value::i8(9),
    Instruction::sub(1 as i8, 8 as i8) => ir::Value::i8(-7),
    Instruction::mul(1 as i8, 8 as i8) => ir::Value::i8(8),
    Instruction::div(10 as i8,2 as i8) => ir::Value::i8(5),
    Instruction::shl(1 as u8,1 as u8) => ir::Value::u8(2),
    Instruction::shr(32 as u8,1 as u8) => ir::Value::u8(16)
});

