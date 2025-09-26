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
    _reserved22: [u8; 0x04],
    #[doc = "0x6c - Channel 5 control register"]
    pub ch5ctl: CH5CTL,
    #[doc = "0x70 - Channel 5 counter register"]
    pub ch5cnt: CH5CNT,
    #[doc = "0x74 - Channel 5 peripheral base address register"]
    pub ch5paddr: CH5PADDR,
    #[doc = "0x78 - Channel 5 memory base address register"]
    pub ch5maddr: CH5MADDR,
    _reserved26: [u8; 0x04],
    #[doc = "0x80 - Channel 6 control register"]
    pub ch6ctl: CH6CTL,
    #[doc = "0x84 - Channel 6 counter register"]
    pub ch6cnt: CH6CNT,
    #[doc = "0x88 - Channel 6 peripheral base address register"]
    pub ch6paddr: CH6PADDR,
    #[doc = "0x8c - Channel 6 memory base address register"]
    pub ch6maddr: CH6MADDR,
}
#[doc = "INTF (r) register accessor: Interrupt flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf`]
module"]
pub type INTF = crate::Reg<intf::INTF_SPEC>;
#[doc = "Interrupt flag register"]
pub mod intf;
#[doc = "INTC (w) register accessor: Interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intc`]
module"]
pub type INTC = crate::Reg<intc::INTC_SPEC>;
#[doc = "Interrupt flag clear register"]
pub mod intc;
#[doc = "CH0CTL (rw) register accessor: Channel 0 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0ctl`]
module"]
pub type CH0CTL = crate::Reg<ch0ctl::CH0CTL_SPEC>;
#[doc = "Channel 0 control register"]
pub mod ch0ctl;
#[doc = "CH0CNT (rw) register accessor: Channel 0 counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0cnt`]
module"]
pub type CH0CNT = crate::Reg<ch0cnt::CH0CNT_SPEC>;
#[doc = "Channel 0 counter register"]
pub mod ch0cnt;
#[doc = "CH0PADDR (rw) register accessor: Channel 0 peripheral base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0paddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0paddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0paddr`]
module"]
pub type CH0PADDR = crate::Reg<ch0paddr::CH0PADDR_SPEC>;
#[doc = "Channel 0 peripheral base address register"]
pub mod ch0paddr;
#[doc = "CH0MADDR (rw) register accessor: Channel 0 memory base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0maddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0maddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0maddr`]
module"]
pub type CH0MADDR = crate::Reg<ch0maddr::CH0MADDR_SPEC>;
#[doc = "Channel 0 memory base address register"]
pub mod ch0maddr;
#[doc = "CH1CTL (rw) register accessor: Channel 1 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1ctl`]
module"]
pub type CH1CTL = crate::Reg<ch1ctl::CH1CTL_SPEC>;
#[doc = "Channel 1 control register"]
pub mod ch1ctl;
#[doc = "CH1CNT (rw) register accessor: Channel 1 counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1cnt`]
module"]
pub type CH1CNT = crate::Reg<ch1cnt::CH1CNT_SPEC>;
#[doc = "Channel 1 counter register"]
pub mod ch1cnt;
#[doc = "CH1PADDR (rw) register accessor: Channel 1 peripheral base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1paddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1paddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1paddr`]
module"]
pub type CH1PADDR = crate::Reg<ch1paddr::CH1PADDR_SPEC>;
#[doc = "Channel 1 peripheral base address register"]
pub mod ch1paddr;
#[doc = "CH1MADDR (rw) register accessor: Channel 1 memory base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1maddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1maddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1maddr`]
module"]
pub type CH1MADDR = crate::Reg<ch1maddr::CH1MADDR_SPEC>;
#[doc = "Channel 1 memory base address register"]
pub mod ch1maddr;
#[doc = "CH2CTL (rw) register accessor: Channel 2 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2ctl`]
module"]
pub type CH2CTL = crate::Reg<ch2ctl::CH2CTL_SPEC>;
#[doc = "Channel 2 control register"]
pub mod ch2ctl;
#[doc = "CH2CNT (rw) register accessor: Channel 2 counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2cnt`]
module"]
pub type CH2CNT = crate::Reg<ch2cnt::CH2CNT_SPEC>;
#[doc = "Channel 2 counter register"]
pub mod ch2cnt;
#[doc = "CH2PADDR (rw) register accessor: Channel 2 peripheral base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2paddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2paddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2paddr`]
module"]
pub type CH2PADDR = crate::Reg<ch2paddr::CH2PADDR_SPEC>;
#[doc = "Channel 2 peripheral base address register"]
pub mod ch2paddr;
#[doc = "CH2MADDR (rw) register accessor: Channel 2 memory base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2maddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2maddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2maddr`]
module"]
pub type CH2MADDR = crate::Reg<ch2maddr::CH2MADDR_SPEC>;
#[doc = "Channel 2 memory base address register"]
pub mod ch2maddr;
#[doc = "CH3CTL (rw) register accessor: Channel 3 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3ctl`]
module"]
pub type CH3CTL = crate::Reg<ch3ctl::CH3CTL_SPEC>;
#[doc = "Channel 3 control register"]
pub mod ch3ctl;
#[doc = "CH3CNT (rw) register accessor: Channel 3 counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3cnt`]
module"]
pub type CH3CNT = crate::Reg<ch3cnt::CH3CNT_SPEC>;
#[doc = "Channel 3 counter register"]
pub mod ch3cnt;
#[doc = "CH3PADDR (rw) register accessor: Channel 3 peripheral base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3paddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3paddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3paddr`]
module"]
pub type CH3PADDR = crate::Reg<ch3paddr::CH3PADDR_SPEC>;
#[doc = "Channel 3 peripheral base address register"]
pub mod ch3paddr;
#[doc = "CH3MADDR (rw) register accessor: Channel 3 memory base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3maddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3maddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3maddr`]
module"]
pub type CH3MADDR = crate::Reg<ch3maddr::CH3MADDR_SPEC>;
#[doc = "Channel 3 memory base address register"]
pub mod ch3maddr;
#[doc = "CH4CTL (rw) register accessor: Channel 4 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4ctl`]
module"]
pub type CH4CTL = crate::Reg<ch4ctl::CH4CTL_SPEC>;
#[doc = "Channel 4 control register"]
pub mod ch4ctl;
#[doc = "CH4CNT (rw) register accessor: Channel 4 counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4cnt`]
module"]
pub type CH4CNT = crate::Reg<ch4cnt::CH4CNT_SPEC>;
#[doc = "Channel 4 counter register"]
pub mod ch4cnt;
#[doc = "CH4PADDR (rw) register accessor: Channel 4 peripheral base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4paddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4paddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4paddr`]
module"]
pub type CH4PADDR = crate::Reg<ch4paddr::CH4PADDR_SPEC>;
#[doc = "Channel 4 peripheral base address register"]
pub mod ch4paddr;
#[doc = "CH4MADDR (rw) register accessor: Channel 4 memory base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4maddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4maddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4maddr`]
module"]
pub type CH4MADDR = crate::Reg<ch4maddr::CH4MADDR_SPEC>;
#[doc = "Channel 4 memory base address register"]
pub mod ch4maddr;
#[doc = "CH5CTL (rw) register accessor: Channel 5 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5ctl`]
module"]
pub type CH5CTL = crate::Reg<ch5ctl::CH5CTL_SPEC>;
#[doc = "Channel 5 control register"]
pub mod ch5ctl;
#[doc = "CH5CNT (rw) register accessor: Channel 5 counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5cnt`]
module"]
pub type CH5CNT = crate::Reg<ch5cnt::CH5CNT_SPEC>;
#[doc = "Channel 5 counter register"]
pub mod ch5cnt;
#[doc = "CH5PADDR (rw) register accessor: Channel 5 peripheral base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5paddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5paddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5paddr`]
module"]
pub type CH5PADDR = crate::Reg<ch5paddr::CH5PADDR_SPEC>;
#[doc = "Channel 5 peripheral base address register"]
pub mod ch5paddr;
#[doc = "CH5MADDR (rw) register accessor: Channel 5 memory base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch5maddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch5maddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5maddr`]
module"]
pub type CH5MADDR = crate::Reg<ch5maddr::CH5MADDR_SPEC>;
#[doc = "Channel 5 memory base address register"]
pub mod ch5maddr;
#[doc = "CH6CTL (rw) register accessor: Channel 6 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch6ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6ctl`]
module"]
pub type CH6CTL = crate::Reg<ch6ctl::CH6CTL_SPEC>;
#[doc = "Channel 6 control register"]
pub mod ch6ctl;
#[doc = "CH6CNT (rw) register accessor: Channel 6 counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch6cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6cnt`]
module"]
pub type CH6CNT = crate::Reg<ch6cnt::CH6CNT_SPEC>;
#[doc = "Channel 6 counter register"]
pub mod ch6cnt;
#[doc = "CH6PADDR (rw) register accessor: Channel 6 peripheral base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch6paddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6paddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6paddr`]
module"]
pub type CH6PADDR = crate::Reg<ch6paddr::CH6PADDR_SPEC>;
#[doc = "Channel 6 peripheral base address register"]
pub mod ch6paddr;
#[doc = "CH6MADDR (rw) register accessor: Channel 6 memory base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch6maddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6maddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6maddr`]
module"]
pub type CH6MADDR = crate::Reg<ch6maddr::CH6MADDR_SPEC>;
#[doc = "Channel 6 memory base address register"]
pub mod ch6maddr;
