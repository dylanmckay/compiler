use mc::{Encodable,Register};
use mc::backends::avr::instructions::Instruction;

pub trait FRdRr : Instruction
{
    /// Gets the subopcode.
    fn subopcode(&self) -> u8;

    fn source(&self) -> Register;
    fn destination(&self) -> Register;
}

macro_rules! define_instruction_rdrr {

    ($ty:ident, $mnemonic:expr, $subopcode:expr) => {
        pub struct $ty
        {
            pub rd: Register,
            pub rr: Register,
        }

        impl $ty
        {
            pub fn new(rd: Register, rr: Register) -> Self {
                $ty {
                    rd: rd,
                    rr: rr,
                }
            }
        }

        impl Instruction for $ty
        {
            fn mnemonic() -> &'static str { $mnemonic }
        }

        impl FRdRr for $ty {
            fn subopcode(&self) -> u8 { $subopcode }
            fn source(&self) -> Register { self.rr }
            fn destination(&self) -> Register { self.rd }
        }
    }
}

define_instruction_rdrr!(Add, "add", 0);
define_instruction_rdrr!(Sub, "sub", 1);


impl<T> Encodable for T where T: FRdRr
{
    fn encode(&self) -> (u64,u8) {
       let mut encoding: u64 = 0;
       
       let subopcode = self.subopcode() as u64; 
       let rd = self.destination().number() as u64;
       let rr = self.source().number() as u64;

       encoding |= (subopcode & 0b111111) << 10;
       encoding |= (rr & 0b10000) << 4;
       encoding |= (rd & 0b10000) << 3;
       encoding |= (rd & 0b01111) << 4;
       encoding |= (rr & 0b01111) << 0;

       (encoding,2) 
    }
}
