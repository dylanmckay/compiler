use std;

pub trait RegisterClass : Clone + std::fmt::Debug
{
    type Register: Register;

    fn registers(&self) -> Vec<Self::Register>;
}

pub trait Register : Clone + std::fmt::Debug
{

}

