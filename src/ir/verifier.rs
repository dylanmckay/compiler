
use ir;
use lang::{Module,Function,Block};
use std;

/// The result of verification.
pub type Result = std::result::Result<(),String>;

/// Verifies the well-formedness of a module.
pub fn verify(module: &ir::Module) -> Result {

    for func in module.functions() {
        try!(self::verify_function(module, func));
    }

    Ok(())
}

pub fn verify_function(module: &ir::Module,
                       func: &ir::Function) -> Result {

    try!(util::verify_name(func.name()));

    for block in func.blocks() {
        try!(self::verify_block(module, block));
    }

    Ok(())
}

pub fn verify_block(module: &ir::Module,
                    block: &ir::Block) -> Result {

    try!(util::verify_name(block.name()));

    for value in block.subvalues().iter() {
        try!(self::verify_value(module, value));
    }

    Ok(())
}

pub fn verify_value(module: &ir::Module,
                    value: &ir::Value) -> Result {
    Ok(())
}

/// Utility methods.
mod util
{
    use ir;
    use super::Result;

    /// Checks if a name is valid.
    pub fn verify_name(name: &ir::Name) -> Result
    {
        match name {
            &ir::Name::Unnamed(..) => Ok(()),
            &ir::Name::Named(ref ident) => self::verify_ident(ident),
        }
    }

    /// Checks if an identifier is valid.
    pub fn verify_ident(ident: &str) -> Result
    {
        let maybe_first_char = ident.chars().next();

        let first_char = match maybe_first_char {
            Some(c) => c,
            None => return Err("identifier cannot be empty".into()),
        };

        if !(first_char.is_alphabetic() || first_char == '_') {
            return Err("identifier must start with an alphabetic character or _".into());
        }

        let valid = ident.chars().all(|c| c.is_alphanumeric() || c == '_');

        match valid {
            true => Ok(()),
            false => Err("identifier can only contain alphanumeric characters or numbers".into()),
        }
    }
}
