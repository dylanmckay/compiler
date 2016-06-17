use {Instruction, RegisterClass, Register, Operand};

pub trait Target : Sized + InstructionBuilder<Target=Self>
{
    type Instruction: Instruction<Operand=Self::Operand>;
    type Operand: Operand<Register=Self::Register, RegisterClass=Self::RegisterClass>;
    type RegisterClass: RegisterClass<Register=Self::Register>;
    type Register: Register;
}

pub trait InstructionBuilder : Sized
{
    type Target: Target;

    fn create_push(source: &<Self::Target as Target>::Register)
        -> <Self::Target as Target>::Instruction;
    fn create_pop(dest: &<Self::Target as Target>::Register)
        -> <Self::Target as Target>::Instruction;
}

