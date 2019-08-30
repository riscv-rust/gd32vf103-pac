#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ID code register"]
    pub id: ID,
    #[doc = "0x04 - Control register 0"]
    pub ctl: CTL,
}
#[doc = "ID code register"]
pub struct ID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ID code register"]
pub mod id;
#[doc = "Control register 0"]
pub struct CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control register 0"]
pub mod ctl;
