
use ir;
use isel;

// TODO: handle globals

/// A directed-acyclic-graph.
#[derive(Clone,Debug)]
pub struct Dag
{
    functions: Vec<Function>,
}

impl Dag
{
    pub fn new<I>(functions: I) -> Self
        where I: Iterator<Item=Function> {
        Dag {
            functions: functions.collect(),
        }
    }

    /// Creates the DAG from a module.
    pub fn from_module(module: ir::Module) -> Self {
        module.into()
    }
}


/// A function.
#[derive(Clone,Debug)]
pub struct Function
{
    name: String,
    blocks: Vec<Block>,
}

impl Function
{
    /// Creates a new function.
    pub fn new<I>(name: String, blocks: I) -> Self
        where I: Iterator<Item=Block> {
        Function {
            name: name,
            blocks: blocks.collect(),
        }
    }
}

/// A basic block.
#[derive(Clone,Debug)]
pub struct Block
{
    nodes: Vec<isel::Node>,
}

impl Block
{
    /// Creates a new block.
    pub fn new<I>(nodes: I) -> Self
        where I: Iterator<Item=isel::Node> {
        Block {
            nodes: nodes.collect(),
        }
    }
}

impl Into<Dag> for ir::Module
{
    fn into(self) -> Dag {
        use lang::Module;

        // TODO: the clone should be unnecessary
        let functions = self.functions().map(|a| a.clone().into());
        Dag::new(functions)
    }
}

impl Into<Function> for ir::Function
{
    fn into(self) -> Function {
        use lang::Function;

        let blocks = self.blocks.into_iter().map(|a| a.into());

        self::Function::new(self.name, blocks)
    }
}

impl Into<Block> for ir::Block
{
    fn into(self) -> Block {
        use lang::Block;

        let nodes = self.subvalues().into_iter().map(|v| create::node(&v));
        self::Block::new(nodes)
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
