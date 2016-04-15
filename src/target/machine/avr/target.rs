use Target;
use machine;

use avr::registers;
use avr::OpCode;

/// The AVR target.
pub struct AVR
{
    register_info: registers::Info,
}

impl AVR
{
    pub fn new() -> Self {
        AVR {
            register_info: registers::Info::new(),
        }
    }
}

impl Target for AVR
{
    fn name(&self) -> &'static str { "AVR" }
}

impl machine::MachineTarget for AVR
{
    type OpCode = OpCode;

    fn pointer_width(&self) -> u16 { 16 }

    fn register_info(&self) -> &machine::RegisterInfo {
        &self.register_info
    }
}

