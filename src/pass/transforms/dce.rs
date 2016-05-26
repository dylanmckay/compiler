use {Metadata,Id,Info,Transform};
use ir;

// TODO: This removes all but the most trivial dead code.
// We should handle the cases where
//   * function isn't used
//   * bb isn't used
//   * global isn't used
// and perform a full analysis using the dominator tree.

/// A dead code elimination pass.
pub struct DeadCodeElimination;

impl Metadata for DeadCodeElimination
{
    fn id(&self) -> Id { Id(0xbb2af3bc) }
    fn name(&self) -> &'static str { "dead code elimination" }
}

impl Transform for DeadCodeElimination
{
    fn run_block(&mut self, block: ir::Block) -> ir::Block {
        self::deadcode::eliminate(block)
    }
}

// TODO: blamket impl for all passes
impl Into<Info> for Box<DeadCodeElimination>
{
    fn into(self) -> Info {
        Info::Transform(self)
    }
}

pub mod deadcode
{
    use ir;

    /// Eliminates dead code.
    pub fn eliminate(block: ir::Block) -> ir::Block {
        block.filter(|v| v.node.is_critical())
    }
}
