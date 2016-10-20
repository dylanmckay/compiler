use std;

pub trait TargetRegisterClass : Clone + PartialEq + Eq + std::fmt::Debug
{
    type TargetRegister: TargetRegister;

    fn registers(&self) -> Vec<Self::TargetRegister>;

    fn contains(&self, register: Self::TargetRegister) -> bool {
        self.registers().into_iter().any(|r| r == register)
    }
}

pub trait TargetRegister : Clone + PartialEq + Eq + std::fmt::Debug
{
}

