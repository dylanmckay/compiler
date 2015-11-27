
use ir::{self,Instruction,Expression};

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Return
{
    value: Option<Box<ir::Expression>>,
}

impl Return
{
    pub fn new(value: Option<ir::Expression>) -> Return {
        Return {
            value: value.map(Box::new),
        }
    }

    pub fn value(value: ir::Expression) -> Return {
        Return::new(Some(value))
    }

    pub fn void() -> Return {
        Return::new(None)
    }

    pub fn subvalues(&self) -> Vec<&Expression> {
        if let Some(ref value) = self.value {
            vec![value]
        } else {
            vec![]
        }
    }

    pub fn subvalue(&self) -> Option<&ir::Expression> {
        if let Some(ref val) = self.value {
            Some(val)
        } else {
            None
        }
    }

    pub fn map_subvalues<F>(mut self, mut f: F) -> Self
        where F: FnMut(Expression) -> Expression {

        let value = match self.value {
            Some(val) => Some(Box::new(f(*val.clone()))),
            None => self.value,
        };

        self.value = value;
        self
    }

    pub fn ty(&self) -> ir::Type { ir::Type::void() }
}

impl Into<Instruction> for Return
{
    fn into(self) -> Instruction {
        ir::Instruction::Return(self)
    }
}

impl Into<Expression> for Return
{
    fn into(self) -> Expression {
        ir::Expression::Instruction(self.into())
    }
}
