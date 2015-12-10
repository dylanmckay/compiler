use {Value,Condition,Instruction,Expression,Type};

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Break
{
    cond: Condition,
    target: Box<Value>,
}

impl Break
{
    /// Creates a branch.
    pub fn new(cond: Condition,
               target: Value) -> Self {
        Break {
            cond: cond,
            target: Box::new(target),
        }
    }

    /// Creates an unconditional branch.
    pub fn unconditional(target: Value) -> Self {
        Break::new(Condition::True, target)
    }

    pub fn condition(&self) -> &Condition {
        &self.cond
    }

    pub fn target(&self) -> &Value {
        &self.target
    }

    pub fn ty(&self) -> Type { Type::void() }
}

impl_instruction!(Break: target);

