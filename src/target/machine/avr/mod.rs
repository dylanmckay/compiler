use Target;
use machine;

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
        regs::CLASSES
    }
}

mod regs
{
    use machine::{Register,RegisterClass};

    macro_rules! define_gpr {
        ($ident:ident, $name:expr, $number: expr) => {
            pub const $ident: Register = Register {
                name: $name,
                number: $number,
            };
        }
    }

    define_gpr!(R0, "r0", 0);
    define_gpr!(R1, "r1", 1);
    define_gpr!(R2, "r2", 2);
    define_gpr!(R3, "r3", 3);
    define_gpr!(R4, "r4", 4);
    define_gpr!(R5, "r5", 5);
    define_gpr!(R6, "r6", 6);
    define_gpr!(R7, "r7", 7);
    define_gpr!(R8, "r8", 8);
    define_gpr!(R9, "r9", 9);
    define_gpr!(R10, "r10", 10);
    define_gpr!(R11, "r11", 11);
    define_gpr!(R12, "r12", 12);
    define_gpr!(R13, "r13", 13);
    define_gpr!(R14, "r14", 14);
    define_gpr!(R15, "r15", 15);
    define_gpr!(R16, "r16", 16);
    define_gpr!(R17, "r17", 17);
    define_gpr!(R18, "r18", 18);
    define_gpr!(R19, "r19", 19);
    define_gpr!(R20, "r20", 20);
    define_gpr!(R21, "r21", 21);
    define_gpr!(R22, "r22", 22);
    define_gpr!(R23, "r23", 23);
    define_gpr!(R24, "r24", 24);
    define_gpr!(R25, "r25", 25);
    define_gpr!(R26, "r26", 26);
    define_gpr!(R27, "r27", 27);
    define_gpr!(R28, "r28", 28);
    define_gpr!(R29, "r29", 29);
    define_gpr!(R30, "r30", 30);
    define_gpr!(R31, "r31", 31);

    pub const GPR8: RegisterClass = RegisterClass {
        name: "GPR8",
        registers: &[
            &R0,
            &R1,
            &R2,
            &R3,
            &R4,
            &R5,
            &R6,
            &R7,
            &R8,
            &R9,
            &R10,
            &R11,
            &R12,
            &R13,
            &R14,
            &R15,
            &R16,
            &R17,
            &R18,
            &R19,
            &R20,
            &R21,
            &R22,
            &R23,
            &R24,
            &R25,
            &R26,
            &R27,
            &R28,
            &R29,
            &R30,
            &R31,
        ],
    };

    pub const CLASSES: &'static [&'static RegisterClass] = &[
        &GPR8,
    ];
}
