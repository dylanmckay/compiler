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
    pub subregs: &'static [&'static Register],
}

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct RegisterClass
{
    pub name: &'static str,
    pub bit_width: u32,
    pub registers: &'static [&'static Register],
}

impl regalloc::TargetRegister for &'static Register
{

}

impl regalloc::TargetRegisterClass for &'static RegisterClass
{
    type TargetRegister = &'static Register;

    fn registers(&self) -> Vec<&'static Register> {
        self.registers.to_owned()
    }
}

