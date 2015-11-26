
use ir;
use lang::{Module,Function,Block};
use std;

// TODO: check that each Register has only one user

macro_rules! condition {
    ($cond:expr, $msg:expr) => {
        if !($cond) {
            return Err($msg.into());
        }
    }
}

/// The result of verification.
pub type Result = std::result::Result<(),String>;

/// Verifies the well-formedness of a module.
pub fn verify(module: &ir::Module) -> Result {

    for func in module.functions() {
        try!(self::verify_function(module, func));
    }

    Ok(())
}

/// Verifies that a function is well-formed.
pub fn verify_function(module: &ir::Module,
                       func: &ir::Function) -> Result {

    try!(util::verify_ident(func.name()));

    condition!(func.blocks().next().is_some(),
               "every function must have at least one basic block");

    for block in func.blocks() {
        try!(self::verify_block(module, block));

        condition!(func.blocks().filter(|a| a.name() == block.name()).count() == 1,
                   "basic blocks must have unique names for each function");
    }

    Ok(())
}

/// Verifies that a block is well-formed.
pub fn verify_block(module: &ir::Module,
                    block: &ir::Block) -> Result {

    use lang::Value;

    try!(util::verify_ident(block.name()));

    for value in block.values() {
        // TODO: only the last instruction in a block can be a terminator.

        try!(self::verify_value(module, value));

        condition!(value.ty().is_void(),
                   "all top-level values must be of type void");
    }


    if let Some(value) = block.values().last() {
        condition!(value.is_terminator(),
                   "every basic block must end with a terminating instruction");
    } else { // block is empty
        return Err("there must be at least one instruction in a basic block".into());
    }

    Ok(())
}

/// Verifies that a value is well-formed.
pub fn verify_value(module: &ir::Module,
                    value: &ir::Value) -> Result {
    match *value {
        ir::Value::Instruction(ref val) => values::instruction(module, val),
        _ => Ok(()),
    }
}

mod values
{
    use ir::{Module,Instruction};
    use super::Result;

    pub fn instruction(module: &Module,
                       inst: &Instruction) -> Result
    {
        match *inst {
            Instruction::Call(ref i) => instruction::call(module, i),
            Instruction::Break(ref i) => instruction::br(module, i),
            Instruction::Return(ref i) => instruction::ret(module, i),
            Instruction::Add(ref i) => instruction::binary_arith(module,i),
            Instruction::Sub(ref i) => instruction::binary_arith(module,i),
            Instruction::Mul(ref i) => instruction::binary_arith(module,i),
            Instruction::Div(ref i) => instruction::binary_arith(module,i),
            Instruction::Shl(ref i) => instruction::binary_arith(module,i),
            Instruction::Shr(ref i) => instruction::binary_arith(module,i),
        }
    }

    pub mod instruction
    {
        use ir::{Module,instruction};
        use super::super::Result;

        // TODO: check that arguments are the same in type and number
        pub fn call(_module: &Module,
                    inst: &instruction::Call) -> Result {

            condition!(inst.target().is_function_ref(),
                       "call instructions must refer to functions");


            Ok(())
        }

        // TODO: check that the referenced block is in the same function
        // as the instruction.
        pub fn br(_module: &Module,
                  inst: &instruction::Break) -> Result {

            condition!(inst.target().is_block_ref(),
                       "break instructions must refer to basic blocks");

            Ok(())
        }

        // TODO: check that the type is consistent with the function.
        pub fn ret(_module: &Module,
                   _inst: &instruction::Return) -> Result {

            Ok(())
        }

        pub fn binary_arith<I>(_module: &Module,
                               inst: &I) -> Result
            where I: instruction::Binary {

            let (lhs,rhs) = inst.operands();

            condition!(lhs.ty() == rhs.ty(),
                       format!("binary arithmetic operations must have operands of the same type"));
            Ok(())
        }
    }
}

/// Utility methods.
mod util
{
    use super::Result;

    /// Checks if an identifier is valid.
    pub fn verify_ident(ident: &str) -> Result
    {
        let maybe_first_char = ident.chars().next();

        let first_char = match maybe_first_char {
            Some(c) => c,
            None => return Err("identifier cannot be empty".into()),
        };

        condition!(first_char.is_alphabetic() || first_char == '_',
                   "identifier must start with an alphabetic character or _");

        let valid = ident.chars().all(|c| c.is_alphanumeric() || c == '_');

        if valid {
            Ok(())
        } else {
            Err("identifier can only contain alphanumeric characters or numbers".into())
        }
    }
}
