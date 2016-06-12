use std;
use bit_vec::BitVec;

#[derive(Clone,PartialEq,Eq)]
pub struct EncodedInstruction
{
    bits: BitVec,
}

impl EncodedInstruction
{
    pub fn from_bit_vec(bits: BitVec) -> Self {
        EncodedInstruction { bits: bits }
    }

    pub fn set_bit(&mut self, i: usize, val: bool) {
        self.bits.set(i, val);
    }
}

impl std::fmt::Debug for EncodedInstruction
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in 0..16 {
            let bit = if self.bits.get(i).unwrap() { '1' } else { '0' };

            if i % 8 == 0 { try!(write!(fmt, " ")); }

            try!(write!(fmt, "{}", bit));
        }

        Ok(())
    }
}

impl From<u16> for EncodedInstruction
{
    fn from(val: u16) -> Self {
        let bytes = [
            ((val & 0xff00) >> 8) as u8,
            (val & 0x00ff) as u8,
        ];

        Self::from_bit_vec(BitVec::from_bytes(&bytes))
    }
}

