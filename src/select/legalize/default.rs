pub mod machine
{
    use Legalizer;

    use legalize::Action;
    use mir::OpCode;

    use mir;

    const BASIC_ARITHMETIC_OPCODES: &'static [OpCode] = &[
        OpCode::Add,
        OpCode::Sub,
    ];

    pub fn core(n: u32) -> Legalizer {
        let mut l = Legalizer::new(n);

        l.on(OpCode::Ret, &[mir::Type::Nothing], Action::Legal);
        l
    }

    pub fn n_bit_architecture(n: u32) -> Legalizer
    {
        let word_type = mir::Type::i(n);
        let dword_type = mir::Type::i(n * 2);
        let qword_type = mir::Type::i(n * 4);

        let mut l = self::core(n);

        for opcode in BASIC_ARITHMETIC_OPCODES.iter().cloned() {
            l.on(
                opcode,
                &[word_type.clone()],
                Action::Legal,
            );

            l.on(
                opcode,
                &[dword_type.clone()],
                Action::Expand,
            );

            l.on(
                opcode,
                &[qword_type.clone()],
                Action::Expand,
            );
        }

        l
    }

    pub fn eight_bit() -> Legalizer { self::n_bit_architecture(8) }
}

