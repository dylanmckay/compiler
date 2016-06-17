use {MachineTarget, Selector, RegisterClass, Register, Instruction, Operand};

use target;
use select;
use regalloc;

use avr::instruction;
use avr;
use std::io;

/// The AVR target.
pub struct AVR;

/// The global AVR target.
static TARGET: AVR = AVR;

static SUPPORTED_OUTPUTS: &'static [target::OutputType] = &[
    target::OutputType::Assembly,
];

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

    fn output_types(&self) -> &'static [target::OutputType] {
        SUPPORTED_OUTPUTS
    }

    fn generate(&self,
                output_type: target::OutputType,
                input: &mut io::Read,
                output: &mut io::Write)
        -> Result<(), target::Error> {
        ::generate::generate(self, output_type, input, output)
    }
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

