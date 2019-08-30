#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register 0"]
    pub ctl0: CTL0,
    _reserved0: [u8; 2usize],
    #[doc = "0x04 - control register 1"]
    pub ctl1: CTL1,
    _reserved1: [u8; 2usize],
    #[doc = "0x08 - status register"]
    pub stat: STAT,
    _reserved2: [u8; 2usize],
    #[doc = "0x0c - data register"]
    pub data: DATA,
    _reserved3: [u8; 2usize],
    #[doc = "0x10 - CRC polynomial register"]
    pub crcpoly: CRCPOLY,
    _reserved4: [u8; 2usize],
    #[doc = "0x14 - RX CRC register"]
    pub rcrc: RCRC,
    _reserved5: [u8; 2usize],
    #[doc = "0x18 - TX CRC register"]
    pub tcrc: TCRC,
    _reserved6: [u8; 2usize],
    #[doc = "0x1c - I2S control register"]
    pub i2sctl: I2SCTL,
    _reserved7: [u8; 2usize],
    #[doc = "0x20 - I2S prescaler register"]
    pub i2spsc: I2SPSC,
}
#[doc = "control register 0"]
pub struct CTL0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "control register 0"]
pub mod ctl0;
#[doc = "control register 1"]
pub struct CTL1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "control register 1"]
pub mod ctl1;
#[doc = "status register"]
pub struct STAT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "status register"]
pub mod stat;
#[doc = "data register"]
pub struct DATA {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "data register"]
pub mod data;
#[doc = "CRC polynomial register"]
pub struct CRCPOLY {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "CRC polynomial register"]
pub mod crcpoly;
#[doc = "RX CRC register"]
pub struct RCRC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "RX CRC register"]
pub mod rcrc;
#[doc = "TX CRC register"]
pub struct TCRC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TX CRC register"]
pub mod tcrc;
#[doc = "I2S control register"]
pub struct I2SCTL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "I2S control register"]
pub mod i2sctl;
#[doc = "I2S prescaler register"]
pub struct I2SPSC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "I2S prescaler register"]
pub mod i2spsc;
