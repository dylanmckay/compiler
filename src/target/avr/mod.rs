use Target;
use machine;

mod registers;

pub struct AVR;

impl Target for AVR
{
    fn name(&self) -> &'static str { "AVR" }
}

impl machine::MachineTarget for AVR
{
    fn pointer_width(&self) -> u16 { 16 }

    fn register_classes(&self)
        -> &'static [&'static machine::RegisterClass] {
        registers::CLASSES
    }
}

