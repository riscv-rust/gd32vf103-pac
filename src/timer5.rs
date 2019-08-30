#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register 0"]
    pub ctl0: CTL0,
    _reserved0: [u8; 2usize],
    #[doc = "0x04 - control register 1"]
    pub ctl1: CTL1,
    _reserved1: [u8; 6usize],
    #[doc = "0x0c - DMA/Interrupt enable register"]
    pub dmainten: DMAINTEN,
    _reserved2: [u8; 2usize],
    #[doc = "0x10 - Interrupt flag register"]
    pub intf: INTF,
    _reserved3: [u8; 2usize],
    #[doc = "0x14 - event generation register"]
    pub swevg: SWEVG,
    _reserved4: [u8; 14usize],
    #[doc = "0x24 - Counter register"]
    pub cnt: CNT,
    _reserved5: [u8; 2usize],
    #[doc = "0x28 - Prescaler register"]
    pub psc: PSC,
    _reserved6: [u8; 2usize],
    #[doc = "0x2c - Counter auto reload register"]
    pub car: CAR,
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
#[doc = "DMA/Interrupt enable register"]
pub struct DMAINTEN {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "DMA/Interrupt enable register"]
pub mod dmainten;
#[doc = "Interrupt flag register"]
pub struct INTF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Interrupt flag register"]
pub mod intf;
#[doc = "event generation register"]
pub struct SWEVG {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "event generation register"]
pub mod swevg;
#[doc = "Counter register"]
pub struct CNT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Counter register"]
pub mod cnt;
#[doc = "Prescaler register"]
pub struct PSC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Prescaler register"]
pub mod psc;
#[doc = "Counter auto reload register"]
pub struct CAR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Counter auto reload register"]
pub mod car;
