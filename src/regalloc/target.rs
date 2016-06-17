use {Instruction, RegisterClass, Register};

pub trait Target : Sized + InstructionBuilder<Target=Self>
{
    type Instruction: Instruction;
    type RegisterClass: RegisterClass;
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

