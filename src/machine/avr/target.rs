use {MachineTarget, RegisterInfo, Selector};

use target;
use select;

use avr::registers;
use avr;

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

impl target::Target for AVR
{
    fn name(&self) -> &'static str { "AVR" }
}

impl MachineTarget for AVR
{
    fn pointer_width(&self) -> u16 { 16 }

    fn register_info(&self) -> &RegisterInfo {
        &self.register_info
    }

    fn create_legalizer(&self) -> select::Legalizer {
        avr::legalize::legalizer()
    }

    fn create_selector(&self) -> Selector {
        avr::select::selector()
    }
}

