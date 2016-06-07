use Dag;
use Register;

pub type Result = ::std::result::Result<(), String>;

// TODO:
// - add/sub operations must have the same type for all operands
// - register values can't refer to themselves
pub fn verify_dag(dag: &Dag) -> Result {
    try!(self::verify_registers_not_used_before_defined(dag));

    Ok(())
}

fn verify_registers_not_used_before_defined(dag: &Dag) -> Result {
    let mut defined_registers: Vec<Register> = Vec::new();

    let is_valid = dag.registers.iter().any(|register| {
        let result = if defined_registers.iter().any(|r| r.id == register.id) {
            false
        } else {
            true
        };

        defined_registers.push(register.clone());

        result
    });

    if is_valid {
        Ok(())
    } else {
        Err("register used befoe it was defined".to_owned())
    }
}

