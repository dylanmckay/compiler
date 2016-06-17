use {Instruction, RegisterClass, Register};

pub trait Target
{
    type Instruction: Instruction;
    type RegisterClass: RegisterClass;
    type Register: Register;
}
