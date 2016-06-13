use machine;
use select;
use mir;

/// A target.
pub trait Target
{
    fn name(&self) -> &'static str;

    fn create_legalizer(&self) -> select::Legalizer;
    fn create_selector(&self) -> machine::Selector;
}

// TODO: this doesn't belong here, but it's good for testing.
pub fn assemble(target: &Target, dag: mir::Dag) {
    let legalizer = target.create_legalizer();
    let mut selector = target.create_selector();

    let dag = legalizer.legalize(dag);
    // let instructions = selector.select(dag);
    //
    // println!("{:#?}", instructions);
    //
    // let encoded_instructions: Vec<_> = instructions.iter().map(|i| i.encode()).collect();
    //
    // println!("{:#?}", encoded_instructions);
}

