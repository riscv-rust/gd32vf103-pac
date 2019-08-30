#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - cliccfg Register"]
    pub cliccfg: CLICCFG,
    _reserved0: [u8; 3usize],
    #[doc = "0x04 - clicinfo Register"]
    pub clicinfo: CLICINFO,
    _reserved1: [u8; 3usize],
    #[doc = "0x0b - MTH Register"]
    pub mth: MTH,
    _reserved2: [u8; 4084usize],
    #[doc = "0x1000 - Core-local Interrupt Controller Interrupt Registers"]
    pub clicints: [CLICINTS; 86],
}
#[doc = r" Register block"]
#[repr(C)]
pub struct CLICINTS {
    #[doc = "0x00 - clicintip Register"]
    pub clicintip: self::clicints::CLICINTIP,
    #[doc = "0x01 - clicintie Register"]
    pub clicintie: self::clicints::CLICINTIE,
    #[doc = "0x02 - clicintattr Register"]
    pub clicintattr: self::clicints::CLICINTATTR,
    #[doc = "0x03 - clicintctl Register"]
    pub clicintctl: self::clicints::CLICINTCTL,
}
#[doc = r" Register block"]
#[doc = "Core-local Interrupt Controller Interrupt Registers"]
pub mod clicints;
#[doc = "cliccfg Register"]
pub struct CLICCFG {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "cliccfg Register"]
pub mod cliccfg;
#[doc = "clicinfo Register"]
pub struct CLICINFO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "clicinfo Register"]
pub mod clicinfo;
#[doc = "MTH Register"]
pub struct MTH {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "MTH Register"]
pub mod mth;
