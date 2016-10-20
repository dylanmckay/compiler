use {Node, ValueInfo};

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

    pub fn value_infos(&self, operands: &[Node]) -> Vec<ValueInfo> {
        match *self {
            OpCode::Set => {
                assert_eq!(operands.len(), 2);

                vec![ValueInfo::Output, ValueInfo::Input]
            },
            // Everything else is all-inputs.
            _ => {
                operands.iter().map(|_| ValueInfo::Input).collect()
            },
        }
    }
}

