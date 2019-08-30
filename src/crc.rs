#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Data register"]
    pub data: DATA,
    #[doc = "0x04 - Free data register"]
    pub fdata: FDATA,
    #[doc = "0x08 - Control register"]
    pub ctl: CTL,
}
#[doc = "Data register"]
pub struct DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data register"]
pub mod data;
#[doc = "Free data register"]
pub struct FDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Free data register"]
pub mod fdata;
#[doc = "Control register"]
pub struct CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control register"]
pub mod ctl;
