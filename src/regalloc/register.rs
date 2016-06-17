use std;

pub trait RegisterClass : Clone + PartialEq + Eq + std::fmt::Debug
{
    type Register: Register;

    fn registers(&self) -> Vec<Self::Register>;

    fn contains(&self, register: Self::Register) -> bool {
        self.registers().into_iter().any(|r| r == register)
    }
}

pub trait Register : Clone + PartialEq + Eq + std::fmt::Debug
{
}

