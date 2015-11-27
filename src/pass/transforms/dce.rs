
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
    fn name(&self) -> &'static str { "dead code elimination" }
}

impl pass::Transform<ir::Expression> for DeadCodeElimination
{
    fn run_block(&mut self, block: ir::Block)
        -> ir::Block {

        self::deadcode::eliminate(block)
    }
}

// TODO: blamket impl for all passes
impl Into<pass::Info<ir::Expression>> for Box<DeadCodeElimination>
{
    fn into(self) -> pass::Info<ir::Expression> {
        pass::Info::Transform(self)
    }
}

pub mod deadcode
{
    use lang;

    /// Eliminates dead code.
    pub fn eliminate<V>(block: lang::Block<V>) -> lang::Block<V>
        where V: lang::Value {

        block.filter(|v| v.is_critical())
    }
}
