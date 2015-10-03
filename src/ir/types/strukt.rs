
use ir::types::{Type,TypeTrait};
use std::{self, fmt};

/// Represents a struct.
#[derive(Clone,Eq,PartialEq,Debug)]
pub struct Struct
{
    pub fields: Vec<Type>,
}

impl Struct
{
    /// The unit struct type.
    pub fn unit() -> Struct {
        Struct {
            fields: Vec::new(),
        }
    }

    /// Creates a new structure.
    pub fn new(fields: Vec<Type>) -> Struct {
        Struct {
            fields: fields,
        }
    }

    /// Returns a new structure with an added field.
    pub fn field(self, ty: Type) -> Struct {
        let mut fields = self.fields.clone();
        fields.push(ty);

        Struct::new(fields)
    }

    /// Gets the fields of the structure.
    pub fn fields<'a>(&'a self) -> std::slice::Iter<'a,Type> {
        self.fields.iter()
    }
}

impl fmt::Display for Struct
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error> {
        use util;

        write!(fmt, "type {{ {} }}", util::comma_separated_values(self.fields.iter()))
    }
}

impl TypeTrait for Struct
{
    fn size(&self) -> u64 {
        self.fields.iter().map(|ref ty| ty.size() as u64).sum()
    }
}

impl_type!(Struct);
