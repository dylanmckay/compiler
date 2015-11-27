
use pass;
use ir;

/// An IR strength reduction pass.
pub struct Inliner;

impl pass::Metadata for Inliner
{
    fn id(&self) -> pass::Id { pass::Id(0x32bbc291) }
    fn name(&self) -> &'static str { "inliner" }
}

impl pass::Transform<ir::Expression> for Inliner
{
    fn run_function(&mut self, 
                    func: ir::Function,
                    module: &ir::Module)
        -> ir::Function {

        use ::util::Identifiable;

        let _function = module.get_function(func.get_id());
        func
    }
}

// TODO: blamket impl for all passes
impl Into<pass::Info<ir::Expression>> for Box<Inliner>
{
    fn into(self) -> pass::Info<ir::Expression> {
        pass::Info::Transform(self)
    }
}

pub fn inline(inst: ir::Instruction) -> ir::Expression {
    match inst {
        ir::Instruction::Call(i) => self::inline_call(i),
        _ => panic!("{} instructions cannot be inlined"),
    }
}

pub fn inline_call(_inst: ir::instruction::Call) -> ir::Expression {
    unimplemented!();
}

