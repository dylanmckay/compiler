
use pass;
use lang;

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

impl<M> pass::PassMut<M> for DeadCodeElimination
    where M: lang::Module
{
    fn run_block(&mut self, block: <<M as lang::Module>::Function as lang::Function>::Block)
        -> <<M as lang::Module>::Function as lang::Function>::Block {

        self::deadcode::eliminate(block)
    }
}

// TODO: blamket impl for all passes
impl<M> Into<pass::Info<M>> for Box<DeadCodeElimination>
    where M: lang::Module
{
    fn into(self) -> pass::Info<M> {
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
