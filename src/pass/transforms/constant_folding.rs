
use pass;
use ir::{self,Instruction};

/// An IR strength reduction pass.
pub struct ConstantFolding;

impl pass::Metadata for ConstantFolding
{
    fn id(&self) -> pass::Id { pass::Id(0x32fabb11) }
    fn name(&self) -> &'static str { "Constant folding" }
}

impl pass::PassMut<ir::Module> for ConstantFolding
{
    fn run_instruction(&mut self, inst: ir::Instruction) -> ir::Instruction {

        match inst {
            _ => inst,
        }
    }
}

// TODO: blamket impl for all passes
impl Into<pass::Info<ir::Module>> for Box<ConstantFolding>
{
    fn into(self) -> pass::Info<ir::Module> {
        pass::Info::Mutable(self)
    }
}
