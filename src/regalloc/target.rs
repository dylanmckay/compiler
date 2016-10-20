use {TargetInstruction, TargetRegisterClass, TargetRegister, TargetOperand};

pub trait Target : Sized + InstructionBuilder<Target=Self>
{
    type Instruction: TargetInstruction<TargetOperand=Self::Operand>;
    type Operand: TargetOperand<Register=Self::Register, RegisterClass=Self::RegisterClass>+'static;
    type RegisterClass: TargetRegisterClass<TargetRegister=Self::Register>;
    type Register: TargetRegister;
}

pub trait InstructionBuilder : Sized
{
    type Target: Target;

    fn create_push(source: &<Self::Target as Target>::Register)
        -> <Self::Target as Target>::Instruction;
    fn create_pop(dest: &<Self::Target as Target>::Register)
        -> <Self::Target as Target>::Instruction;
}

