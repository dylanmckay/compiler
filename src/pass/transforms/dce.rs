
use pass;
use ir;

// TODO: This removes all but the most trivial dead code.
// We should handle the cases where
//   * function isn't used
//   * bb isn't used
//   * global isn't used
// and perform a full analysis using the dominator tree.

/// A dead code elimination pass.
pub struct DeadCodeElimination;

impl pass::Metadata for DeadCodeElimination
{
    fn id(&self) -> pass::Id { pass::Id(0xbb2af3bc) }
    fn name(&self) -> &'static str { "Dead code elimination" }
}

impl pass::PassMut<ir::Module> for DeadCodeElimination
{
    fn run_block(&mut self, block: ir::Block)
        -> ir::Block {

        self::deadcode::eliminate(block)
    }
}

// TODO: blamket impl for all passes
impl Into<pass::Info<ir::Module>> for Box<DeadCodeElimination>
{
    fn into(self) -> pass::Info<ir::Module> {
        pass::Info::Mutable(self)
    }
}

pub mod deadcode
{
    use lang;

    /// Eliminates dead code.
    pub fn eliminate<B>(block: B) -> B
        where B: lang::Block {
        use lang::Value;

        block.filter(|v| v.is_critical())
    }
}
