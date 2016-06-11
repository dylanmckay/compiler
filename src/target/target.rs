use select;
use mir;

/// A target.
pub trait Target
{
    fn name(&self) -> &'static str;

    fn create_legalizer(&self) -> select::Legalizer;
    fn create_selector(&self) -> select::Selector<()>;
}

// TODO: this doesn't belong here, but it's good for testing.
pub fn assemble(target: &Target, dag: mir::Dag) {
    let legalizer = target.create_legalizer();
    let mut selector = target.create_selector();

    let dag = legalizer.legalize(dag);
    let dag = selector.select(dag);

    println!("{:#?}", dag);
}

