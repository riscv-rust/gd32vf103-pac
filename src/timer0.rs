#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register 0"]
    pub ctl0: CTL0,
    _reserved0: [u8; 2usize],
    #[doc = "0x04 - control register 1"]
    pub ctl1: CTL1,
    _reserved1: [u8; 2usize],
    #[doc = "0x08 - slave mode configuration register"]
    pub smcfg: SMCFG,
    _reserved2: [u8; 2usize],
    #[doc = "0x0c - DMA/Interrupt enable register"]
    pub dmainten: DMAINTEN,
    _reserved3: [u8; 2usize],
    #[doc = "0x10 - Interrupt flag register"]
    pub intf: INTF,
    _reserved4: [u8; 2usize],
    #[doc = "0x14 - Software event generation register"]
    pub swevg: SWEVG,
    _reserved5: [u8; 2usize],
    #[doc = "0x18 - Channel control register 0 (output mode)"]
    pub chctl0_output: CHCTL0_OUTPUT,
    _reserved6: [u8; 2usize],
    #[doc = "0x1c - Channel control register 1 (output mode)"]
    pub chctl1_output: CHCTL1_OUTPUT,
    _reserved7: [u8; 2usize],
    #[doc = "0x20 - Channel control register 2"]
    pub chctl2: CHCTL2,
    _reserved8: [u8; 2usize],
    #[doc = "0x24 - counter"]
    pub cnt: CNT,
    _reserved9: [u8; 2usize],
    #[doc = "0x28 - prescaler"]
    pub psc: PSC,
    _reserved10: [u8; 2usize],
    #[doc = "0x2c - Counter auto reload register"]
    pub car: CAR,
    _reserved11: [u8; 2usize],
    #[doc = "0x30 - Counter repetition register"]
    pub crep: CREP,
    _reserved12: [u8; 2usize],
    #[doc = "0x34 - Channel 0 capture/compare value register"]
    pub ch0cv: CH0CV,
    _reserved13: [u8; 2usize],
    #[doc = "0x38 - Channel 1 capture/compare value register"]
    pub ch1cv: CH1CV,
    _reserved14: [u8; 2usize],
    #[doc = "0x3c - Channel 2 capture/compare value register"]
    pub ch2cv: CH2CV,
    _reserved15: [u8; 2usize],
    #[doc = "0x40 - Channel 3 capture/compare value register"]
    pub ch3cv: CH3CV,
    _reserved16: [u8; 2usize],
    #[doc = "0x44 - channel complementary protection register"]
    pub cchp: CCHP,
    _reserved17: [u8; 2usize],
    #[doc = "0x48 - DMA configuration register"]
    pub dmacfg: DMACFG,
    _reserved18: [u8; 2usize],
    #[doc = "0x4c - DMA transfer buffer register"]
    pub dmatb: DMATB,
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
#[doc = "slave mode configuration register"]
pub struct SMCFG {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "slave mode configuration register"]
pub mod smcfg;
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
#[doc = "Software event generation register"]
pub struct SWEVG {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Software event generation register"]
pub mod swevg;
#[doc = "Channel control register 0 (output mode)"]
pub struct CHCTL0_OUTPUT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Channel control register 0 (output mode)"]
pub mod chctl0_output;
#[doc = "Channel control register 0 (input mode)"]
pub struct CHCTL0_INPUT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Channel control register 0 (input mode)"]
pub mod chctl0_input;
#[doc = "Channel control register 1 (output mode)"]
pub struct CHCTL1_OUTPUT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Channel control register 1 (output mode)"]
pub mod chctl1_output;
#[doc = "Channel control register 1 (input mode)"]
pub struct CHCTL1_INPUT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Channel control register 1 (input mode)"]
pub mod chctl1_input;
#[doc = "Channel control register 2"]
pub struct CHCTL2 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Channel control register 2"]
pub mod chctl2;
#[doc = "counter"]
pub struct CNT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "counter"]
pub mod cnt;
#[doc = "prescaler"]
pub struct PSC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "prescaler"]
pub mod psc;
#[doc = "Counter auto reload register"]
pub struct CAR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Counter auto reload register"]
pub mod car;
#[doc = "Counter repetition register"]
pub struct CREP {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Counter repetition register"]
pub mod crep;
#[doc = "Channel 0 capture/compare value register"]
pub struct CH0CV {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Channel 0 capture/compare value register"]
pub mod ch0cv;
#[doc = "Channel 1 capture/compare value register"]
pub struct CH1CV {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Channel 1 capture/compare value register"]
pub mod ch1cv;
#[doc = "Channel 2 capture/compare value register"]
pub struct CH2CV {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Channel 2 capture/compare value register"]
pub mod ch2cv;
#[doc = "Channel 3 capture/compare value register"]
pub struct CH3CV {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Channel 3 capture/compare value register"]
pub mod ch3cv;
#[doc = "channel complementary protection register"]
pub struct CCHP {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "channel complementary protection register"]
pub mod cchp;
#[doc = "DMA configuration register"]
pub struct DMACFG {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "DMA configuration register"]
pub mod dmacfg;
#[doc = "DMA transfer buffer register"]
pub struct DMATB {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "DMA transfer buffer register"]
pub mod dmatb;
