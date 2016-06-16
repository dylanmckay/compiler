use Dag;

pub type Result = ::std::result::Result<(), String>;

// TODO:
// - add/sub operations must have the same type for all operands
// - register values can't refer to themselves
pub fn verify_dag(dag: &Dag) -> Result {
    try!(self::all_top_level_nodes_are_typeless(dag));

    Ok(())
}

fn all_top_level_nodes_are_typeless(dag: &Dag) -> Result {
    for node in dag.nodes.iter() {
        if !node.ty().is_nothing() {
            return Err(format!("all top level nodes must be typeless: {:?}", node));
        }
    }

    Ok(())
}

