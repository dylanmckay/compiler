
use std::fmt;

#[derive(Copy,Clone,Debug)]
#[allow(non_camel_case_types)]
pub enum Architecture
{
    x86,
    x86_64,
    AVR,
}

impl fmt::Display for Architecture
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(),fmt::Error>
    {
        match *self {
            Architecture::x86 => "x86",
            Architecture::x86_64 => "x86-64",
            Architecture::AVR => "AVR",
        }.fmt(fmt)
    }
}
