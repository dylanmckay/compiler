use machine;
use select;
use mir;

use Pattern;

/// A target.
pub trait Target
{
    fn name(&self) -> &'static str;

    fn create_legalizer(&self) -> select::Legalizer;
    fn create_selector(&self) -> select::Selector<Box<machine::Instruction>>;

    fn selection_patterns(&self) -> Vec<Pattern>;
}

// TODO: this doesn't belong here, but it's good for testing.
pub fn assemble(target: &Target, dag: mir::Dag) {
    let legalizer = target.create_legalizer();
    let mut selector = target.create_selector();
    let patterns = target.selection_patterns();

    println!("{:#?}", patterns);

    let dag = legalizer.legalize(dag);
    let instructions = selector.select(dag);

    println!("{:#?}", instructions);

    let encoded_instructions: Vec<_> = instructions.iter().map(|i| i.encode()).collect();

    println!("{:#?}", encoded_instructions);

}

