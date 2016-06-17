use {MachineTarget, Selector, RegisterClass, Register, Instruction, Operand};

use target;
use select;
use regalloc;

use avr::instruction;
use avr;

/// The AVR target.
pub struct AVR;

/// The global AVR target.
static TARGET: AVR = AVR;

impl AVR
{
    pub fn register() {
        target::register(&TARGET)
    }
}

impl target::Target for AVR
{
    fn name(&self) -> &'static str { "avr" }
    fn display_name(&self) -> &'static str { "AVR" }
}

impl MachineTarget for AVR
{
    fn pointer_width(&self) -> u16 { 16 }

    fn create_legalizer(&self) -> select::Legalizer {
        avr::legalize::legalizer()
    }

    fn create_selector(&self) -> Selector {
        avr::select::selector()
    }
}

impl regalloc::Target for AVR
{
    type Instruction = Box<Instruction>;
    type RegisterClass = &'static RegisterClass;
    type Register = &'static Register;
    type Operand = Operand;
}

impl regalloc::InstructionBuilder for AVR
{
    type Target = AVR;

    fn create_push(source: &&'static Register) -> Box<Instruction> {
        Box::new(instruction::PUSHRd::new(Operand::Register(source)))
    }

    fn create_pop(dest: &&'static Register) -> Box<Instruction> {
        Box::new(instruction::POPRd::new(Operand::Register(dest)))
    }
}

