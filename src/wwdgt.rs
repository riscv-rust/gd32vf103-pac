#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub ctl: CTL,
    #[doc = "0x04 - Configuration register"]
    pub cfg: CFG,
    #[doc = "0x08 - Status register"]
    pub stat: STAT,
}
#[doc = "Control register"]
pub struct CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control register"]
pub mod ctl;
#[doc = "Configuration register"]
pub struct CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration register"]
pub mod cfg;
#[doc = "Status register"]
pub struct STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register"]
pub mod stat;
