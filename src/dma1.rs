#[doc = r"Register block"]
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
    _reserved6: [u8; 0x04],
    #[doc = "0x1c - Channel 1 control register"]
    pub ch1ctl: CH1CTL,
    #[doc = "0x20 - Channel 1 counter register"]
    pub ch1cnt: CH1CNT,
    #[doc = "0x24 - Channel 1 peripheral base address register"]
    pub ch1paddr: CH1PADDR,
    #[doc = "0x28 - Channel 1 memory base address register"]
    pub ch1maddr: CH1MADDR,
    _reserved10: [u8; 0x04],
    #[doc = "0x30 - Channel 2 control register"]
    pub ch2ctl: CH2CTL,
    #[doc = "0x34 - Channel 2 counter register"]
    pub ch2cnt: CH2CNT,
    #[doc = "0x38 - Channel 2 peripheral base address register"]
    pub ch2paddr: CH2PADDR,
    #[doc = "0x3c - Channel 2 memory base address register"]
    pub ch2maddr: CH2MADDR,
    _reserved14: [u8; 0x04],
    #[doc = "0x44 - Channel 3 control register"]
    pub ch3ctl: CH3CTL,
    #[doc = "0x48 - Channel 3 counter register"]
    pub ch3cnt: CH3CNT,
    #[doc = "0x4c - Channel 3 peripheral base address register"]
    pub ch3paddr: CH3PADDR,
    #[doc = "0x50 - Channel 3 memory base address register"]
    pub ch3maddr: CH3MADDR,
    _reserved18: [u8; 0x04],
    #[doc = "0x58 - Channel 4 control register"]
    pub ch4ctl: CH4CTL,
    #[doc = "0x5c - Channel 4 counter register"]
    pub ch4cnt: CH4CNT,
    #[doc = "0x60 - Channel 4 peripheral base address register"]
    pub ch4paddr: CH4PADDR,
    #[doc = "0x64 - Channel 4 memory base address register"]
    pub ch4maddr: CH4MADDR,
}
#[doc = "INTF (r) register accessor: an alias for `Reg<INTF_SPEC>`"]
pub type INTF = crate::Reg<intf::INTF_SPEC>;
#[doc = "Interrupt flag register"]
pub mod intf;
#[doc = "INTC (w) register accessor: an alias for `Reg<INTC_SPEC>`"]
pub type INTC = crate::Reg<intc::INTC_SPEC>;
#[doc = "Interrupt flag clear register"]
pub mod intc;
#[doc = "CH0CTL (rw) register accessor: an alias for `Reg<CH0CTL_SPEC>`"]
pub type CH0CTL = crate::Reg<ch0ctl::CH0CTL_SPEC>;
#[doc = "Channel 0 control register"]
pub mod ch0ctl;
#[doc = "CH0CNT (rw) register accessor: an alias for `Reg<CH0CNT_SPEC>`"]
pub type CH0CNT = crate::Reg<ch0cnt::CH0CNT_SPEC>;
#[doc = "Channel 0 counter register"]
pub mod ch0cnt;
#[doc = "CH0PADDR (rw) register accessor: an alias for `Reg<CH0PADDR_SPEC>`"]
pub type CH0PADDR = crate::Reg<ch0paddr::CH0PADDR_SPEC>;
#[doc = "Channel 0 peripheral base address register"]
pub mod ch0paddr;
#[doc = "CH0MADDR (rw) register accessor: an alias for `Reg<CH0MADDR_SPEC>`"]
pub type CH0MADDR = crate::Reg<ch0maddr::CH0MADDR_SPEC>;
#[doc = "Channel 0 memory base address register"]
pub mod ch0maddr;
#[doc = "CH1CTL (rw) register accessor: an alias for `Reg<CH1CTL_SPEC>`"]
pub type CH1CTL = crate::Reg<ch1ctl::CH1CTL_SPEC>;
#[doc = "Channel 1 control register"]
pub mod ch1ctl;
#[doc = "CH1CNT (rw) register accessor: an alias for `Reg<CH1CNT_SPEC>`"]
pub type CH1CNT = crate::Reg<ch1cnt::CH1CNT_SPEC>;
#[doc = "Channel 1 counter register"]
pub mod ch1cnt;
#[doc = "CH1PADDR (rw) register accessor: an alias for `Reg<CH1PADDR_SPEC>`"]
pub type CH1PADDR = crate::Reg<ch1paddr::CH1PADDR_SPEC>;
#[doc = "Channel 1 peripheral base address register"]
pub mod ch1paddr;
#[doc = "CH1MADDR (rw) register accessor: an alias for `Reg<CH1MADDR_SPEC>`"]
pub type CH1MADDR = crate::Reg<ch1maddr::CH1MADDR_SPEC>;
#[doc = "Channel 1 memory base address register"]
pub mod ch1maddr;
#[doc = "CH2CTL (rw) register accessor: an alias for `Reg<CH2CTL_SPEC>`"]
pub type CH2CTL = crate::Reg<ch2ctl::CH2CTL_SPEC>;
#[doc = "Channel 2 control register"]
pub mod ch2ctl;
#[doc = "CH2CNT (rw) register accessor: an alias for `Reg<CH2CNT_SPEC>`"]
pub type CH2CNT = crate::Reg<ch2cnt::CH2CNT_SPEC>;
#[doc = "Channel 2 counter register"]
pub mod ch2cnt;
#[doc = "CH2PADDR (rw) register accessor: an alias for `Reg<CH2PADDR_SPEC>`"]
pub type CH2PADDR = crate::Reg<ch2paddr::CH2PADDR_SPEC>;
#[doc = "Channel 2 peripheral base address register"]
pub mod ch2paddr;
#[doc = "CH2MADDR (rw) register accessor: an alias for `Reg<CH2MADDR_SPEC>`"]
pub type CH2MADDR = crate::Reg<ch2maddr::CH2MADDR_SPEC>;
#[doc = "Channel 2 memory base address register"]
pub mod ch2maddr;
#[doc = "CH3CTL (rw) register accessor: an alias for `Reg<CH3CTL_SPEC>`"]
pub type CH3CTL = crate::Reg<ch3ctl::CH3CTL_SPEC>;
#[doc = "Channel 3 control register"]
pub mod ch3ctl;
#[doc = "CH3CNT (rw) register accessor: an alias for `Reg<CH3CNT_SPEC>`"]
pub type CH3CNT = crate::Reg<ch3cnt::CH3CNT_SPEC>;
#[doc = "Channel 3 counter register"]
pub mod ch3cnt;
#[doc = "CH3PADDR (rw) register accessor: an alias for `Reg<CH3PADDR_SPEC>`"]
pub type CH3PADDR = crate::Reg<ch3paddr::CH3PADDR_SPEC>;
#[doc = "Channel 3 peripheral base address register"]
pub mod ch3paddr;
#[doc = "CH3MADDR (rw) register accessor: an alias for `Reg<CH3MADDR_SPEC>`"]
pub type CH3MADDR = crate::Reg<ch3maddr::CH3MADDR_SPEC>;
#[doc = "Channel 3 memory base address register"]
pub mod ch3maddr;
#[doc = "CH4CTL (rw) register accessor: an alias for `Reg<CH4CTL_SPEC>`"]
pub type CH4CTL = crate::Reg<ch4ctl::CH4CTL_SPEC>;
#[doc = "Channel 4 control register"]
pub mod ch4ctl;
#[doc = "CH4CNT (rw) register accessor: an alias for `Reg<CH4CNT_SPEC>`"]
pub type CH4CNT = crate::Reg<ch4cnt::CH4CNT_SPEC>;
#[doc = "Channel 4 counter register"]
pub mod ch4cnt;
#[doc = "CH4PADDR (rw) register accessor: an alias for `Reg<CH4PADDR_SPEC>`"]
pub type CH4PADDR = crate::Reg<ch4paddr::CH4PADDR_SPEC>;
#[doc = "Channel 4 peripheral base address register"]
pub mod ch4paddr;
#[doc = "CH4MADDR (rw) register accessor: an alias for `Reg<CH4MADDR_SPEC>`"]
pub type CH4MADDR = crate::Reg<ch4maddr::CH4MADDR_SPEC>;
#[doc = "Channel 4 memory base address register"]
pub mod ch4maddr;
