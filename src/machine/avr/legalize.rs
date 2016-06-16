use select;

pub fn legalizer() -> select::Legalizer {
    let l = select::legalize::default::machine::eight_bit();

    l
}

