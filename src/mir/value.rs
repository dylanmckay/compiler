use ir;

#[derive(Clone,Debug,PartialEq,Eq)]
pub enum Value
{
    Integer {
        bit_width: u16,
        value: i64,
    },
}

impl Value
{
    pub fn from_ir(value: &ir::Value) -> Self {
        use num::traits::ToPrimitive;

        match value.expression {
            ir::Expression::Literal(ref literal) => {
                match *literal {
                    ir::value::Literal::Integer(ref i) => {
                        Value::Integer {
                            bit_width: i.integer_ty().width(),
                            value: i.value().to_i64().unwrap(),
                        }
                    },
                    _ => unimplemented!(),
                }
            },
            _ => unimplemented!(),
        }
    }
}
