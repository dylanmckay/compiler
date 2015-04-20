
use std::fmt;

/// Specifies an operating system.
#[derive(Copy,Clone,Debug)]
pub enum OsKind
{
    None,
    Windows,
    Unix(UnixKind),
}

impl fmt::Display for OsKind
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error>
    {
        match self {
            &OsKind::None    => "None".fmt(fmt),
            &OsKind::Windows => "Windows".fmt(fmt),
            &OsKind::Unix(kind) => kind.fmt(fmt),
        }
    }
}

#[derive(Copy,Clone,Debug)]
#[allow(non_camel_case_types)]
pub enum UnixKind
{
    Unknown,
    Linux,
    SystemV,
    HP_UX,
    Solaris,
    AIX,
    IRIX,
    BSD(BSDKind),
}

impl fmt::Display for UnixKind
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error>
    {
        match self {
            &UnixKind::Unknown => "Unix".fmt(fmt),
            &UnixKind::Linux   => "Linux".fmt(fmt),
            &UnixKind::SystemV => "SystemV".fmt(fmt),
            &UnixKind::HP_UX   => "HP-UX".fmt(fmt),
            &UnixKind::Solaris => "Solaris".fmt(fmt),
            &UnixKind::AIX     => "AIX".fmt(fmt),
            &UnixKind::IRIX    => "IRIX".fmt(fmt),
            &UnixKind::BSD(kind) => {
                try!(kind.fmt(fmt));
                "BSD".fmt(fmt)
            }
        }
    }
}

#[derive(Copy,Clone,Debug)]
pub enum BSDKind
{
    Net,
    Free,
    Open,
}

impl fmt::Display for BSDKind {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error>
    {
        match self {
            &BSDKind::Net => "Net",
            &BSDKind::Free => "Free",
            &BSDKind::Open => "Open",
        }.fmt(fmt)
    }
}
