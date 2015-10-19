
use ir;
use lang;
use isel;

// TODO: handle globals
//

/// A directed-acyclic-graph.
pub type Dag = lang::Module<isel::Node>;

impl Dag
{
    /// Creates the DAG from a module.
    pub fn from_module(module: ir::Module) -> Self {
        unimplemented!();
    }
}

/// Utility functions for DAG creation.
mod create
{
    use isel;
    use ir::{Value,Instruction};
    use ir::value;

    /// Creates a node from an IR value.
    /// **Note**: The value must be in SSA form.
    pub fn node(inst: &Value) -> isel::Node
    {
        match inst {
            &Value::Literal(ref literal) => self::literal(literal),
            &Value::Instruction(ref inst) => self::instruction(inst),
            &Value::Register(..) => panic!("the value must be in SSA form"),
            _ => unimplemented!(),
        }
    }

    pub fn literal(literal: &value::Literal) -> isel::Node
    {
        match literal {
            &value::Literal::Integer(ref i) => self::integer(i),
            &value::Literal::Decimal(..) => unimplemented!(),
            // FIXME: at this point, there shouldn't be any struct literals
            &value::Literal::Struct(..) => unimplemented!(),
        }
    }

    pub fn integer(integer: &value::literal::Integer) -> isel::Node
    {
        let value = integer.value.clone();
        isel::Node::integer(value)
    }

    pub fn instruction(inst: &Instruction) -> isel::Node
    {
        let op = match inst {
            &Instruction::Add(..) => isel::Operation::Add,
            &Instruction::Sub(..) => isel::Operation::Sub,
            &Instruction::Mul(..) => isel::Operation::Mul,
            &Instruction::Div(..) => isel::Operation::Div,
            &Instruction::Shl(..) => isel::Operation::Shl,
            &Instruction::Shr(..) => isel::Operation::Shr,
            &Instruction::Call(..) => isel::Operation::Call,
            &Instruction::Jump(..) => isel::Operation::Jump,
            &Instruction::Return(..) => isel::Operation::Return,
        };

        let children = inst.subvalues().into_iter().map(|v| self::node(&v));

        isel::Node::operation(op, children)
    }
}
