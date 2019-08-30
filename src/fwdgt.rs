#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub ctl: CTL,
    #[doc = "0x04 - Prescaler register"]
    pub psc: PSC,
    #[doc = "0x08 - Reload register"]
    pub rld: RLD,
    #[doc = "0x0c - Status register"]
    pub stat: STAT,
}
#[doc = "Control register"]
pub struct CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control register"]
pub mod ctl;
#[doc = "Prescaler register"]
pub struct PSC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Prescaler register"]
pub mod psc;
#[doc = "Reload register"]
pub struct RLD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reload register"]
pub mod rld;
#[doc = "Status register"]
pub struct STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register"]
pub mod stat;
