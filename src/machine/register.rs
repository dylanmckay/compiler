use regalloc;

pub trait RegisterInfo
{
    /// Gets the register classes the target supports.
    fn classes(&self)
        -> &'static [&'static RegisterClass];
}

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Register
{
    pub name: &'static str,
    pub number: u32,
}

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct RegisterClass
{
    pub name: &'static str,
    pub bit_width: u32,
    pub registers: &'static [&'static Register],
}

impl regalloc::Register for &'static Register
{

}

impl regalloc::RegisterClass for &'static RegisterClass
{

}

