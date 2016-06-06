#[derive(Clone,Debug,PartialEq,Eq)]
pub enum Type
{
    Integer { bit_width: u32 },
    Nothing,
}

