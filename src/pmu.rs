#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - power control register"]
    pub ctl: CTL,
    #[doc = "0x04 - power control/status register"]
    pub cs: CS,
}
#[doc = "power control register"]
pub struct CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "power control register"]
pub mod ctl;
#[doc = "power control/status register"]
pub struct CS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "power control/status register"]
pub mod cs;
