#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - wait state counter register"]
    pub ws: WS,
    #[doc = "0x04 - Unlock key register 0"]
    pub key0: KEY0,
    #[doc = "0x08 - Option byte unlock key register"]
    pub obkey: OBKEY,
    #[doc = "0x0c - Status register 0"]
    pub stat0: STAT0,
    #[doc = "0x10 - Control register 0"]
    pub ctl0: CTL0,
    #[doc = "0x14 - Address register 0"]
    pub addr0: ADDR0,
    _reserved0: [u8; 4usize],
    #[doc = "0x1c - Option byte status register"]
    pub obstat: OBSTAT,
    #[doc = "0x20 - Erase/Program Protection register"]
    pub wp: WP,
    _reserved1: [u8; 220usize],
    #[doc = "0x100 - Product ID register"]
    pub pid: PID,
}
#[doc = "wait state counter register"]
pub struct WS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "wait state counter register"]
pub mod ws;
#[doc = "Unlock key register 0"]
pub struct KEY0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Unlock key register 0"]
pub mod key0;
#[doc = "Option byte unlock key register"]
pub struct OBKEY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Option byte unlock key register"]
pub mod obkey;
#[doc = "Status register 0"]
pub struct STAT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register 0"]
pub mod stat0;
#[doc = "Control register 0"]
pub struct CTL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control register 0"]
pub mod ctl0;
#[doc = "Address register 0"]
pub struct ADDR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Address register 0"]
pub mod addr0;
#[doc = "Option byte status register"]
pub struct OBSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Option byte status register"]
pub mod obstat;
#[doc = "Erase/Program Protection register"]
pub struct WP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Erase/Program Protection register"]
pub mod wp;
#[doc = "Product ID register"]
pub struct PID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Product ID register"]
pub mod pid;
