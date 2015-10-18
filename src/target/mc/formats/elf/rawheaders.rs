

/// An ELF header.
/// `P` - The pointer type.
#[derive(Copy,Clone,Debug)]
#[repr(C,packed)]
pub struct HeaderData<P>
{
    pub e_ident: IdentifierData,

    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,

    pub e_entry: P,
    pub e_phoff: P,
    pub e_shoff: P,

    pub e_flags: u32,
    /// The size of the header
    /// Normally 64 bytes for 64 bit and 52 for 32 bit.
    pub e_ehsize: u16,
    pub e_phnum:  u16,

    pub e_shentsize: u16,
    pub e_shnum:     u16,
    pub e_shstrndx:  u16,
}

#[derive(Copy,Clone,Debug)]
#[repr(C,packed)]
#[allow(non_snake_case)]
/// The `e_ident` field in the ELF header.
pub struct IdentifierData
{
    // magic number bytes
    pub EI_MAG0: u8,
    pub EI_MAG1: u8,
    pub EI_MAG2: u8,
    pub EI_MAG3: u8,

    pub EI_CLASS: u8,
    pub EI_DATA:  u8,

    pub EI_VERSION: u8,
    pub EI_OSABI: u8,

    pub EI_ABIVERSION: u8,
    pub EI_PAD: [u8; 7],
}

/// Holds program header data.
#[derive(Copy,Clone,Debug)]
#[repr(C,packed)]
#[allow(non_snake_case)]
pub struct ProgramHeaderData<P>
{
    pub p_type: u32,
    pub p_flags: u32,

    pub p_offset: P,
    pub p_vaddr:  P,
    pub p_paddr:  P,
    pub p_filesz: P,
    pub p_memsz:  P,
    pub p_align:  P,
}

/// Defines an enum associated with a value.
macro_rules! define_enum {
    (
        $name:ident : $ty:ident ;

        $(
            $member:ident => $val:expr
        ),*
    ) => {
        #[derive(Copy,Clone,Debug)]
        pub enum $name
        {
            $( $member ),*
        }

        impl $name
        {
            pub fn value(self) -> $ty {
                match self {
                    $( $name::$member => $val ),*
                }
            }
        }
    }
}

define_enum! {
    PT : u32;

    NULL    => 0,
    LOAD    => 1,
    DYNAMIC => 2,
    INTERP  => 3,
    NOTE    => 4,
    SHLIB   => 5,
    PHDR    => 6,
    TLS     => 7,
    LOOS    => 0x60000000,
    HIOS    => 0x6fffffff,
    LOPROC  => 0x70000000,
    HIPROC  => 0x7fffffff
}

