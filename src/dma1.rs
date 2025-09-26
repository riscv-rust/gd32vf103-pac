#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    intf: INTF,
    intc: INTC,
    ch0ctl: CH0CTL,
    ch0cnt: CH0CNT,
    ch0paddr: CH0PADDR,
    ch0maddr: CH0MADDR,
    _reserved6: [u8; 0x04],
    ch1ctl: CH1CTL,
    ch1cnt: CH1CNT,
    ch1paddr: CH1PADDR,
    ch1maddr: CH1MADDR,
    _reserved10: [u8; 0x04],
    ch2ctl: CH2CTL,
    ch2cnt: CH2CNT,
    ch2paddr: CH2PADDR,
    ch2maddr: CH2MADDR,
    _reserved14: [u8; 0x04],
    ch3ctl: CH3CTL,
    ch3cnt: CH3CNT,
    ch3paddr: CH3PADDR,
    ch3maddr: CH3MADDR,
    _reserved18: [u8; 0x04],
    ch4ctl: CH4CTL,
    ch4cnt: CH4CNT,
    ch4paddr: CH4PADDR,
    ch4maddr: CH4MADDR,
}
impl RegisterBlock {
    #[doc = "0x00 - Interrupt flag register"]
    #[inline(always)]
    pub const fn intf(&self) -> &INTF {
        &self.intf
    }
    #[doc = "0x04 - Interrupt flag clear register"]
    #[inline(always)]
    pub const fn intc(&self) -> &INTC {
        &self.intc
    }
    #[doc = "0x08 - Channel 0 control register"]
    #[inline(always)]
    pub const fn ch0ctl(&self) -> &CH0CTL {
        &self.ch0ctl
    }
    #[doc = "0x0c - Channel 0 counter register"]
    #[inline(always)]
    pub const fn ch0cnt(&self) -> &CH0CNT {
        &self.ch0cnt
    }
    #[doc = "0x10 - Channel 0 peripheral base address register"]
    #[inline(always)]
    pub const fn ch0paddr(&self) -> &CH0PADDR {
        &self.ch0paddr
    }
    #[doc = "0x14 - Channel 0 memory base address register"]
    #[inline(always)]
    pub const fn ch0maddr(&self) -> &CH0MADDR {
        &self.ch0maddr
    }
    #[doc = "0x1c - Channel 1 control register"]
    #[inline(always)]
    pub const fn ch1ctl(&self) -> &CH1CTL {
        &self.ch1ctl
    }
    #[doc = "0x20 - Channel 1 counter register"]
    #[inline(always)]
    pub const fn ch1cnt(&self) -> &CH1CNT {
        &self.ch1cnt
    }
    #[doc = "0x24 - Channel 1 peripheral base address register"]
    #[inline(always)]
    pub const fn ch1paddr(&self) -> &CH1PADDR {
        &self.ch1paddr
    }
    #[doc = "0x28 - Channel 1 memory base address register"]
    #[inline(always)]
    pub const fn ch1maddr(&self) -> &CH1MADDR {
        &self.ch1maddr
    }
    #[doc = "0x30 - Channel 2 control register"]
    #[inline(always)]
    pub const fn ch2ctl(&self) -> &CH2CTL {
        &self.ch2ctl
    }
    #[doc = "0x34 - Channel 2 counter register"]
    #[inline(always)]
    pub const fn ch2cnt(&self) -> &CH2CNT {
        &self.ch2cnt
    }
    #[doc = "0x38 - Channel 2 peripheral base address register"]
    #[inline(always)]
    pub const fn ch2paddr(&self) -> &CH2PADDR {
        &self.ch2paddr
    }
    #[doc = "0x3c - Channel 2 memory base address register"]
    #[inline(always)]
    pub const fn ch2maddr(&self) -> &CH2MADDR {
        &self.ch2maddr
    }
    #[doc = "0x44 - Channel 3 control register"]
    #[inline(always)]
    pub const fn ch3ctl(&self) -> &CH3CTL {
        &self.ch3ctl
    }
    #[doc = "0x48 - Channel 3 counter register"]
    #[inline(always)]
    pub const fn ch3cnt(&self) -> &CH3CNT {
        &self.ch3cnt
    }
    #[doc = "0x4c - Channel 3 peripheral base address register"]
    #[inline(always)]
    pub const fn ch3paddr(&self) -> &CH3PADDR {
        &self.ch3paddr
    }
    #[doc = "0x50 - Channel 3 memory base address register"]
    #[inline(always)]
    pub const fn ch3maddr(&self) -> &CH3MADDR {
        &self.ch3maddr
    }
    #[doc = "0x58 - Channel 4 control register"]
    #[inline(always)]
    pub const fn ch4ctl(&self) -> &CH4CTL {
        &self.ch4ctl
    }
    #[doc = "0x5c - Channel 4 counter register"]
    #[inline(always)]
    pub const fn ch4cnt(&self) -> &CH4CNT {
        &self.ch4cnt
    }
    #[doc = "0x60 - Channel 4 peripheral base address register"]
    #[inline(always)]
    pub const fn ch4paddr(&self) -> &CH4PADDR {
        &self.ch4paddr
    }
    #[doc = "0x64 - Channel 4 memory base address register"]
    #[inline(always)]
    pub const fn ch4maddr(&self) -> &CH4MADDR {
        &self.ch4maddr
    }
}
#[doc = "INTF (r) register accessor: Interrupt flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`intf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf`]
module"]
pub type INTF = crate::Reg<intf::INTF_SPEC>;
#[doc = "Interrupt flag register"]
pub mod intf;
#[doc = "INTC (w) register accessor: Interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intc`]
module"]
pub type INTC = crate::Reg<intc::INTC_SPEC>;
#[doc = "Interrupt flag clear register"]
pub mod intc;
#[doc = "CH0CTL (rw) register accessor: Channel 0 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0ctl`]
module"]
pub type CH0CTL = crate::Reg<ch0ctl::CH0CTL_SPEC>;
#[doc = "Channel 0 control register"]
pub mod ch0ctl;
#[doc = "CH0CNT (rw) register accessor: Channel 0 counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0cnt`]
module"]
pub type CH0CNT = crate::Reg<ch0cnt::CH0CNT_SPEC>;
#[doc = "Channel 0 counter register"]
pub mod ch0cnt;
#[doc = "CH0PADDR (rw) register accessor: Channel 0 peripheral base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0paddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0paddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0paddr`]
module"]
pub type CH0PADDR = crate::Reg<ch0paddr::CH0PADDR_SPEC>;
#[doc = "Channel 0 peripheral base address register"]
pub mod ch0paddr;
#[doc = "CH0MADDR (rw) register accessor: Channel 0 memory base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0maddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0maddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0maddr`]
module"]
pub type CH0MADDR = crate::Reg<ch0maddr::CH0MADDR_SPEC>;
#[doc = "Channel 0 memory base address register"]
pub mod ch0maddr;
#[doc = "CH1CTL (rw) register accessor: Channel 1 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1ctl`]
module"]
pub type CH1CTL = crate::Reg<ch1ctl::CH1CTL_SPEC>;
#[doc = "Channel 1 control register"]
pub mod ch1ctl;
#[doc = "CH1CNT (rw) register accessor: Channel 1 counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1cnt`]
module"]
pub type CH1CNT = crate::Reg<ch1cnt::CH1CNT_SPEC>;
#[doc = "Channel 1 counter register"]
pub mod ch1cnt;
#[doc = "CH1PADDR (rw) register accessor: Channel 1 peripheral base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1paddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1paddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1paddr`]
module"]
pub type CH1PADDR = crate::Reg<ch1paddr::CH1PADDR_SPEC>;
#[doc = "Channel 1 peripheral base address register"]
pub mod ch1paddr;
#[doc = "CH1MADDR (rw) register accessor: Channel 1 memory base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1maddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1maddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1maddr`]
module"]
pub type CH1MADDR = crate::Reg<ch1maddr::CH1MADDR_SPEC>;
#[doc = "Channel 1 memory base address register"]
pub mod ch1maddr;
#[doc = "CH2CTL (rw) register accessor: Channel 2 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2ctl`]
module"]
pub type CH2CTL = crate::Reg<ch2ctl::CH2CTL_SPEC>;
#[doc = "Channel 2 control register"]
pub mod ch2ctl;
#[doc = "CH2CNT (rw) register accessor: Channel 2 counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2cnt`]
module"]
pub type CH2CNT = crate::Reg<ch2cnt::CH2CNT_SPEC>;
#[doc = "Channel 2 counter register"]
pub mod ch2cnt;
#[doc = "CH2PADDR (rw) register accessor: Channel 2 peripheral base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2paddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2paddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2paddr`]
module"]
pub type CH2PADDR = crate::Reg<ch2paddr::CH2PADDR_SPEC>;
#[doc = "Channel 2 peripheral base address register"]
pub mod ch2paddr;
#[doc = "CH2MADDR (rw) register accessor: Channel 2 memory base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2maddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2maddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2maddr`]
module"]
pub type CH2MADDR = crate::Reg<ch2maddr::CH2MADDR_SPEC>;
#[doc = "Channel 2 memory base address register"]
pub mod ch2maddr;
#[doc = "CH3CTL (rw) register accessor: Channel 3 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3ctl`]
module"]
pub type CH3CTL = crate::Reg<ch3ctl::CH3CTL_SPEC>;
#[doc = "Channel 3 control register"]
pub mod ch3ctl;
#[doc = "CH3CNT (rw) register accessor: Channel 3 counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3cnt`]
module"]
pub type CH3CNT = crate::Reg<ch3cnt::CH3CNT_SPEC>;
#[doc = "Channel 3 counter register"]
pub mod ch3cnt;
#[doc = "CH3PADDR (rw) register accessor: Channel 3 peripheral base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3paddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3paddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3paddr`]
module"]
pub type CH3PADDR = crate::Reg<ch3paddr::CH3PADDR_SPEC>;
#[doc = "Channel 3 peripheral base address register"]
pub mod ch3paddr;
#[doc = "CH3MADDR (rw) register accessor: Channel 3 memory base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3maddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3maddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3maddr`]
module"]
pub type CH3MADDR = crate::Reg<ch3maddr::CH3MADDR_SPEC>;
#[doc = "Channel 3 memory base address register"]
pub mod ch3maddr;
#[doc = "CH4CTL (rw) register accessor: Channel 4 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4ctl`]
module"]
pub type CH4CTL = crate::Reg<ch4ctl::CH4CTL_SPEC>;
#[doc = "Channel 4 control register"]
pub mod ch4ctl;
#[doc = "CH4CNT (rw) register accessor: Channel 4 counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4cnt`]
module"]
pub type CH4CNT = crate::Reg<ch4cnt::CH4CNT_SPEC>;
#[doc = "Channel 4 counter register"]
pub mod ch4cnt;
#[doc = "CH4PADDR (rw) register accessor: Channel 4 peripheral base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4paddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4paddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4paddr`]
module"]
pub type CH4PADDR = crate::Reg<ch4paddr::CH4PADDR_SPEC>;
#[doc = "Channel 4 peripheral base address register"]
pub mod ch4paddr;
#[doc = "CH4MADDR (rw) register accessor: Channel 4 memory base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4maddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4maddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4maddr`]
module"]
pub type CH4MADDR = crate::Reg<ch4maddr::CH4MADDR_SPEC>;
#[doc = "Channel 4 memory base address register"]
pub mod ch4maddr;
