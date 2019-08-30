#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Status register"]
    pub stat: STAT,
    #[doc = "0x04 - Data register"]
    pub data: DATA,
    #[doc = "0x08 - Baud rate register"]
    pub baud: BAUD,
    #[doc = "0x0c - Control register 0"]
    pub ctl0: CTL0,
    #[doc = "0x10 - Control register 1"]
    pub ctl1: CTL1,
    #[doc = "0x14 - Control register 2"]
    pub ctl2: CTL2,
    #[doc = "0x18 - Guard time and prescaler register"]
    pub gp: GP,
}
#[doc = "Status register"]
pub struct STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register"]
pub mod stat;
#[doc = "Data register"]
pub struct DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data register"]
pub mod data;
#[doc = "Baud rate register"]
pub struct BAUD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Baud rate register"]
pub mod baud;
#[doc = "Control register 0"]
pub struct CTL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control register 0"]
pub mod ctl0;
#[doc = "Control register 1"]
pub struct CTL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control register 1"]
pub mod ctl1;
#[doc = "Control register 2"]
pub struct CTL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control register 2"]
pub mod ctl2;
#[doc = "Guard time and prescaler register"]
pub struct GP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Guard time and prescaler register"]
pub mod gp;
