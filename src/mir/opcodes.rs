#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub enum OpCode
{
    Add,
    Sub,
    Mul,
    Div,
    Shl,
    Shr,
    Ret,
    /// Signed extension,
    /// `(sext 16 %value)`
    Sext,
    /// Zero extension.
    /// `(sext 16 %value)`
    Zext,
    /// Set a register.
    /// `(set %reg, %value)`
    Set,
}

impl OpCode
{
    pub fn mnemonic(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }
}

