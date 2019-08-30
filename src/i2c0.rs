#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register 0"]
    pub ctl0: CTL0,
    _reserved0: [u8; 2usize],
    #[doc = "0x04 - Control register 1"]
    pub ctl1: CTL1,
    _reserved1: [u8; 2usize],
    #[doc = "0x08 - Slave address register 0"]
    pub saddr0: SADDR0,
    _reserved2: [u8; 2usize],
    #[doc = "0x0c - Slave address register 1"]
    pub saddr1: SADDR1,
    _reserved3: [u8; 2usize],
    #[doc = "0x10 - Transfer buffer register"]
    pub data: DATA,
    _reserved4: [u8; 2usize],
    #[doc = "0x14 - Transfer status register 0"]
    pub stat0: STAT0,
    _reserved5: [u8; 2usize],
    #[doc = "0x18 - Transfer status register 1"]
    pub stat1: STAT1,
    _reserved6: [u8; 2usize],
    #[doc = "0x1c - Clock configure register"]
    pub ckcfg: CKCFG,
    _reserved7: [u8; 2usize],
    #[doc = "0x20 - Rise time register"]
    pub rt: RT,
}
#[doc = "Control register 0"]
pub struct CTL0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Control register 0"]
pub mod ctl0;
#[doc = "Control register 1"]
pub struct CTL1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Control register 1"]
pub mod ctl1;
#[doc = "Slave address register 0"]
pub struct SADDR0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Slave address register 0"]
pub mod saddr0;
#[doc = "Slave address register 1"]
pub struct SADDR1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Slave address register 1"]
pub mod saddr1;
#[doc = "Transfer buffer register"]
pub struct DATA {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Transfer buffer register"]
pub mod data;
#[doc = "Transfer status register 0"]
pub struct STAT0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Transfer status register 0"]
pub mod stat0;
#[doc = "Transfer status register 1"]
pub struct STAT1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Transfer status register 1"]
pub mod stat1;
#[doc = "Clock configure register"]
pub struct CKCFG {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Clock configure register"]
pub mod ckcfg;
#[doc = "Rise time register"]
pub struct RT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Rise time register"]
pub mod rt;
