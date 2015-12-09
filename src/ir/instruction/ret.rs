
use {Instruction,Value,Expression,Type};

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Return
{
    value: Option<Box<Value>>,
}

impl Return
{
    pub fn new(value: Option<Value>) -> Return {
        Return {
            value: value.map(Box::new),
        }
    }

    pub fn value(value: Value) -> Return {
        Return::new(Some(value))
    }

    pub fn void() -> Return {
        Return::new(None)
    }

    pub fn subvalues(&self) -> Vec<&Value> {
        if let Some(ref value) = self.value {
            vec![value]
        } else {
            vec![]
        }
    }

    pub fn subvalue(&self) -> Option<&Value> {
        if let Some(ref val) = self.value {
            Some(val)
        } else {
            None
        }
    }

    pub fn map_subvalues<F>(mut self, mut f: F) -> Self
        where F: FnMut(Value) -> Value {

        let value = match self.value {
            Some(val) => Some(Box::new(f(*val.clone()))),
            None => self.value,
        };

        self.value = value;
        self
    }

    pub fn ty(&self) -> Type { Type::void() }
}

impl Into<Instruction> for Return
{
    fn into(self) -> Instruction {
        Instruction::Return(self)
    }
}

impl Into<Expression> for Return
{
    fn into(self) -> Expression {
        Expression::Instruction(self.into())
    }
}
