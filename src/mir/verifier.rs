use Dag;

pub type Result = ::std::result::Result<(), String>;

// TODO:
// - add/sub operations must have the same type for all operands
// - register values can't refer to themselves
pub fn verify_dag(_dag: &Dag) -> Result {
    Ok(())
}

