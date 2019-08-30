#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt flag register"]
    pub intf: INTF,
    #[doc = "0x04 - Interrupt flag clear register"]
    pub intc: INTC,
    #[doc = "0x08 - Channel 0 control register"]
    pub ch0ctl: CH0CTL,
    #[doc = "0x0c - Channel 0 counter register"]
    pub ch0cnt: CH0CNT,
    #[doc = "0x10 - Channel 0 peripheral base address register"]
    pub ch0paddr: CH0PADDR,
    #[doc = "0x14 - Channel 0 memory base address register"]
    pub ch0maddr: CH0MADDR,
    _reserved0: [u8; 4usize],
    #[doc = "0x1c - Channel 1 control register"]
    pub ch1ctl: CH1CTL,
    #[doc = "0x20 - Channel 1 counter register"]
    pub ch1cnt: CH1CNT,
    #[doc = "0x24 - Channel 1 peripheral base address register"]
    pub ch1paddr: CH1PADDR,
    #[doc = "0x28 - Channel 1 memory base address register"]
    pub ch1maddr: CH1MADDR,
    _reserved1: [u8; 4usize],
    #[doc = "0x30 - Channel 2 control register"]
    pub ch2ctl: CH2CTL,
    #[doc = "0x34 - Channel 2 counter register"]
    pub ch2cnt: CH2CNT,
    #[doc = "0x38 - Channel 2 peripheral base address register"]
    pub ch2paddr: CH2PADDR,
    #[doc = "0x3c - Channel 2 memory base address register"]
    pub ch2maddr: CH2MADDR,
    _reserved2: [u8; 4usize],
    #[doc = "0x44 - Channel 3 control register"]
    pub ch3ctl: CH3CTL,
    #[doc = "0x48 - Channel 3 counter register"]
    pub ch3cnt: CH3CNT,
    #[doc = "0x4c - Channel 3 peripheral base address register"]
    pub ch3paddr: CH3PADDR,
    #[doc = "0x50 - Channel 3 memory base address register"]
    pub ch3maddr: CH3MADDR,
    _reserved3: [u8; 4usize],
    #[doc = "0x58 - Channel 4 control register"]
    pub ch4ctl: CH4CTL,
    #[doc = "0x5c - Channel 4 counter register"]
    pub ch4cnt: CH4CNT,
    #[doc = "0x60 - Channel 4 peripheral base address register"]
    pub ch4paddr: CH4PADDR,
    #[doc = "0x64 - Channel 4 memory base address register"]
    pub ch4maddr: CH4MADDR,
}
#[doc = "Interrupt flag register"]
pub struct INTF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt flag register"]
pub mod intf;
#[doc = "Interrupt flag clear register"]
pub struct INTC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt flag clear register"]
pub mod intc;
#[doc = "Channel 0 control register"]
pub struct CH0CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 0 control register"]
pub mod ch0ctl;
#[doc = "Channel 0 counter register"]
pub struct CH0CNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 0 counter register"]
pub mod ch0cnt;
#[doc = "Channel 0 peripheral base address register"]
pub struct CH0PADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 0 peripheral base address register"]
pub mod ch0paddr;
#[doc = "Channel 0 memory base address register"]
pub struct CH0MADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 0 memory base address register"]
pub mod ch0maddr;
#[doc = "Channel 1 control register"]
pub struct CH1CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 1 control register"]
pub mod ch1ctl;
#[doc = "Channel 1 counter register"]
pub struct CH1CNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 1 counter register"]
pub mod ch1cnt;
#[doc = "Channel 1 peripheral base address register"]
pub struct CH1PADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 1 peripheral base address register"]
pub mod ch1paddr;
#[doc = "Channel 1 memory base address register"]
pub struct CH1MADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 1 memory base address register"]
pub mod ch1maddr;
#[doc = "Channel 2 control register"]
pub struct CH2CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 2 control register"]
pub mod ch2ctl;
#[doc = "Channel 2 counter register"]
pub struct CH2CNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 2 counter register"]
pub mod ch2cnt;
#[doc = "Channel 2 peripheral base address register"]
pub struct CH2PADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 2 peripheral base address register"]
pub mod ch2paddr;
#[doc = "Channel 2 memory base address register"]
pub struct CH2MADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 2 memory base address register"]
pub mod ch2maddr;
#[doc = "Channel 3 control register"]
pub struct CH3CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 3 control register"]
pub mod ch3ctl;
#[doc = "Channel 3 counter register"]
pub struct CH3CNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 3 counter register"]
pub mod ch3cnt;
#[doc = "Channel 3 peripheral base address register"]
pub struct CH3PADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 3 peripheral base address register"]
pub mod ch3paddr;
#[doc = "Channel 3 memory base address register"]
pub struct CH3MADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 3 memory base address register"]
pub mod ch3maddr;
#[doc = "Channel 4 control register"]
pub struct CH4CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 4 control register"]
pub mod ch4ctl;
#[doc = "Channel 4 counter register"]
pub struct CH4CNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 4 counter register"]
pub mod ch4cnt;
#[doc = "Channel 4 peripheral base address register"]
pub struct CH4PADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 4 peripheral base address register"]
pub mod ch4paddr;
#[doc = "Channel 4 memory base address register"]
pub struct CH4MADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 4 memory base address register"]
pub mod ch4maddr;
