use {Metadata,Id,Info,Transform};
use ir;

/// An IR strength reduction pass.
pub struct Inliner;

impl Metadata for Inliner
{
    fn id(&self) -> Id { Id(0x32bbc291) }
    fn name(&self) -> &'static str { "inliner" }
}

impl Transform for Inliner
{
    fn run_function(&mut self, 
                    func: ir::Function,
                    _module: &ir::Module)
        -> ir::Function {
        func
    }
}

// TODO: blamket impl for all passes
impl Into<Info> for Box<Inliner>
{
    fn into(self) -> Info {
        Info::Transform(self)
    }
}

pub fn inline(inst: ir::Instruction) -> ir::Value {
    match inst {
        ir::Instruction::Call(i) => self::inline_call(i),
        _ => panic!("{} instructions cannot be inlined"),
    }
}

pub fn inline_call(_inst: ir::instruction::Call) -> ir::Value {
    unimplemented!();
}

