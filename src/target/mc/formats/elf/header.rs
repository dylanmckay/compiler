
use target::mc::formats::BinaryKind;
//use mc::formats::elf::rawheaders::HeaderData;
use util;


/// The magic number embedded at the top of an ELF file.
//static ELF_MAGIC_NUMBER: [u8; 4] = [ 0x7f, 'E' as u8, 'L' as u8, 'F' as u8];

/// The ELF version.
//static ELF_VERSION_NUMBER: u8 = 1;

#[derive(Copy,Clone,Debug)]
/// An ELF header.
/// `P` - The pointer type to use.
pub struct Header<P>
{
    pub class: Class,
    pub byteorder: util::ByteOrder,
    pub architecture: util::Architecture,
    pub os: util::os::OsKind,
    pub kind: BinaryKind,

    pub entry_point_address: P,
    pub program_header_offset: P,
    pub section_header_offset: P,

    pub e_flags: u32,
}

/// Specifies the ELF file class.
/// This is eithr 32-bit or 64-bit.
#[derive(Copy,Clone,Debug)]
pub enum Class
{
    C32,
    C64,
}

impl Class {
    /// Gets the class number as used by the `e_ident[EI_CLASS]` field
    /// in the ELF header.
    pub fn number(self) -> u8 {
        match self {
            Class::C32 => 1,
            Class::C64 => 2,
        }
    }
}

/// Contains functions for mapping enums to ELF header field values.
pub mod mapping
{
    use util;
    use target::mc::formats::BinaryKind;

    /// Gets the byte order number as used by the `e_ident[EI_DATA]` field
    /// in the ELF header.
    pub fn get_byte_order_number(order: util::ByteOrder) -> u8 {
        use util::ByteOrder;

        match order {
            ByteOrder::LittleEndian => 1,
            ByteOrder::BigEndian    => 2,
        }
    }

    /// Gets the os number as used by the `e_indent[EI_OSABI]` field in the ELF header.
    /// Returns None if the OS is unknown.
    pub fn get_os_number(os: util::os::OsKind) -> Option<u8> {
        use util::os::{OsKind,UnixKind,BSDKind};

        match os {
            OsKind::None => Some(0),
            OsKind::Unix(unixkind) => match unixkind {
                UnixKind::Linux =>   Some(0x03),
                UnixKind::HP_UX =>   Some(0x01),
                UnixKind::Solaris => Some(0x06),
                UnixKind::AIX =>     Some(0x07),
                UnixKind::IRIX =>    Some(0x08),
                
                UnixKind::BSD(bsdkind) => match bsdkind {
                    BSDKind::Net =>  Some(0x02),
                    BSDKind::Free => Some(0x09),
                    BSDKind::Open => Some(0x0c),

                },
                // all other unixes default to zero
                _ => Some(0),
            },
            
            // we don't recognize this OS
     
       _ => None,
        }
    }

    pub fn get_elf_kind_number(kind: BinaryKind) -> u8 {
        match kind {
            BinaryKind::Object =>       0x01,
            BinaryKind::Executable =>   0x02,
            BinaryKind::SharedObject => 0x03,
        }
    }

    /// Gets the machine number as used in the `e_machine` field
    /// in the ELF header.
    /// Returns `None` if the machine is unknown.
    pub fn get_elf_machine(arch: util::Architecture) -> Option<u8>
    {
        match arch {
            util::Architecture::x86 => Some(0x03),
            util::Architecture::x86_64 => Some(0x3e),
            util::Architecture::AVR => Some(83),
        }
    }
}
