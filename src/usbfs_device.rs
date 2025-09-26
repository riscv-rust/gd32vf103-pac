#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dcfg: DCFG,
    dctl: DCTL,
    dstat: DSTAT,
    _reserved3: [u8; 0x04],
    diepinten: DIEPINTEN,
    doepinten: DOEPINTEN,
    daepint: DAEPINT,
    daepinten: DAEPINTEN,
    _reserved7: [u8; 0x08],
    dvbusdt: DVBUSDT,
    dvbuspt: DVBUSPT,
    _reserved9: [u8; 0x04],
    diepfeinten: DIEPFEINTEN,
    _reserved10: [u8; 0xc8],
    diep0ctl: DIEP0CTL,
    _reserved11: [u8; 0x04],
    diep0intf: DIEP0INTF,
    _reserved12: [u8; 0x04],
    diep0len: DIEP0LEN,
    _reserved13: [u8; 0x04],
    diep0tfstat: DIEP0TFSTAT,
    _reserved14: [u8; 0x04],
    diep1ctl: DIEP1CTL,
    _reserved15: [u8; 0x04],
    diep1intf: DIEP1INTF,
    _reserved16: [u8; 0x04],
    diep1len: DIEP1LEN,
    _reserved17: [u8; 0x04],
    diep1tfstat: DIEP1TFSTAT,
    _reserved18: [u8; 0x04],
    diep2ctl: DIEP2CTL,
    _reserved19: [u8; 0x04],
    diep2intf: DIEP2INTF,
    _reserved20: [u8; 0x04],
    diep2len: DIEP2LEN,
    _reserved21: [u8; 0x04],
    diep2tfstat: DIEP2TFSTAT,
    _reserved22: [u8; 0x04],
    diep3ctl: DIEP3CTL,
    _reserved23: [u8; 0x04],
    diep3intf: DIEP3INTF,
    _reserved24: [u8; 0x04],
    diep3len: DIEP3LEN,
    _reserved25: [u8; 0x04],
    diep3tfstat: DIEP3TFSTAT,
    _reserved26: [u8; 0x0184],
    doep0ctl: DOEP0CTL,
    _reserved27: [u8; 0x04],
    doep0intf: DOEP0INTF,
    _reserved28: [u8; 0x04],
    doep0len: DOEP0LEN,
    _reserved29: [u8; 0x0c],
    doep1ctl: DOEP1CTL,
    _reserved30: [u8; 0x04],
    doep1intf: DOEP1INTF,
    _reserved31: [u8; 0x04],
    doep1len: DOEP1LEN,
    _reserved32: [u8; 0x0c],
    doep2ctl: DOEP2CTL,
    _reserved33: [u8; 0x04],
    doep2intf: DOEP2INTF,
    _reserved34: [u8; 0x04],
    doep2len: DOEP2LEN,
    _reserved35: [u8; 0x0c],
    doep3ctl: DOEP3CTL,
    _reserved36: [u8; 0x04],
    doep3intf: DOEP3INTF,
    _reserved37: [u8; 0x04],
    doep3len: DOEP3LEN,
}
impl RegisterBlock {
    #[doc = "0x00 - device configuration register (DCFG)"]
    #[inline(always)]
    pub const fn dcfg(&self) -> &DCFG {
        &self.dcfg
    }
    #[doc = "0x04 - device control register (DCTL)"]
    #[inline(always)]
    pub const fn dctl(&self) -> &DCTL {
        &self.dctl
    }
    #[doc = "0x08 - device status register (DSTAT)"]
    #[inline(always)]
    pub const fn dstat(&self) -> &DSTAT {
        &self.dstat
    }
    #[doc = "0x10 - device IN endpoint common interrupt mask register (DIEPINTEN)"]
    #[inline(always)]
    pub const fn diepinten(&self) -> &DIEPINTEN {
        &self.diepinten
    }
    #[doc = "0x14 - device OUT endpoint common interrupt enable register (DOEPINTEN)"]
    #[inline(always)]
    pub const fn doepinten(&self) -> &DOEPINTEN {
        &self.doepinten
    }
    #[doc = "0x18 - device all endpoints interrupt register (DAEPINT)"]
    #[inline(always)]
    pub const fn daepint(&self) -> &DAEPINT {
        &self.daepint
    }
    #[doc = "0x1c - Device all endpoints interrupt enable register (DAEPINTEN)"]
    #[inline(always)]
    pub const fn daepinten(&self) -> &DAEPINTEN {
        &self.daepinten
    }
    #[doc = "0x28 - device VBUS discharge time register"]
    #[inline(always)]
    pub const fn dvbusdt(&self) -> &DVBUSDT {
        &self.dvbusdt
    }
    #[doc = "0x2c - device VBUS pulsing time register"]
    #[inline(always)]
    pub const fn dvbuspt(&self) -> &DVBUSPT {
        &self.dvbuspt
    }
    #[doc = "0x34 - device IN endpoint FIFO empty interrupt enable register"]
    #[inline(always)]
    pub const fn diepfeinten(&self) -> &DIEPFEINTEN {
        &self.diepfeinten
    }
    #[doc = "0x100 - device IN endpoint 0 control register (DIEP0CTL)"]
    #[inline(always)]
    pub const fn diep0ctl(&self) -> &DIEP0CTL {
        &self.diep0ctl
    }
    #[doc = "0x108 - device endpoint-0 interrupt register"]
    #[inline(always)]
    pub const fn diep0intf(&self) -> &DIEP0INTF {
        &self.diep0intf
    }
    #[doc = "0x110 - device IN endpoint-0 transfer length register"]
    #[inline(always)]
    pub const fn diep0len(&self) -> &DIEP0LEN {
        &self.diep0len
    }
    #[doc = "0x118 - device IN endpoint 0 transmit FIFO status register"]
    #[inline(always)]
    pub const fn diep0tfstat(&self) -> &DIEP0TFSTAT {
        &self.diep0tfstat
    }
    #[doc = "0x120 - device in endpoint-1 control register"]
    #[inline(always)]
    pub const fn diep1ctl(&self) -> &DIEP1CTL {
        &self.diep1ctl
    }
    #[doc = "0x128 - device endpoint-1 interrupt register"]
    #[inline(always)]
    pub const fn diep1intf(&self) -> &DIEP1INTF {
        &self.diep1intf
    }
    #[doc = "0x130 - device IN endpoint-1 transfer length register"]
    #[inline(always)]
    pub const fn diep1len(&self) -> &DIEP1LEN {
        &self.diep1len
    }
    #[doc = "0x138 - device IN endpoint 1 transmit FIFO status register"]
    #[inline(always)]
    pub const fn diep1tfstat(&self) -> &DIEP1TFSTAT {
        &self.diep1tfstat
    }
    #[doc = "0x140 - device endpoint-2 control register"]
    #[inline(always)]
    pub const fn diep2ctl(&self) -> &DIEP2CTL {
        &self.diep2ctl
    }
    #[doc = "0x148 - device endpoint-2 interrupt register"]
    #[inline(always)]
    pub const fn diep2intf(&self) -> &DIEP2INTF {
        &self.diep2intf
    }
    #[doc = "0x150 - device IN endpoint-2 transfer length register"]
    #[inline(always)]
    pub const fn diep2len(&self) -> &DIEP2LEN {
        &self.diep2len
    }
    #[doc = "0x158 - device IN endpoint 2 transmit FIFO status register"]
    #[inline(always)]
    pub const fn diep2tfstat(&self) -> &DIEP2TFSTAT {
        &self.diep2tfstat
    }
    #[doc = "0x160 - device endpoint-3 control register"]
    #[inline(always)]
    pub const fn diep3ctl(&self) -> &DIEP3CTL {
        &self.diep3ctl
    }
    #[doc = "0x168 - device endpoint-3 interrupt register"]
    #[inline(always)]
    pub const fn diep3intf(&self) -> &DIEP3INTF {
        &self.diep3intf
    }
    #[doc = "0x170 - device IN endpoint-3 transfer length register"]
    #[inline(always)]
    pub const fn diep3len(&self) -> &DIEP3LEN {
        &self.diep3len
    }
    #[doc = "0x178 - device IN endpoint 3 transmit FIFO status register"]
    #[inline(always)]
    pub const fn diep3tfstat(&self) -> &DIEP3TFSTAT {
        &self.diep3tfstat
    }
    #[doc = "0x300 - device endpoint-0 control register"]
    #[inline(always)]
    pub const fn doep0ctl(&self) -> &DOEP0CTL {
        &self.doep0ctl
    }
    #[doc = "0x308 - device out endpoint-0 interrupt flag register"]
    #[inline(always)]
    pub const fn doep0intf(&self) -> &DOEP0INTF {
        &self.doep0intf
    }
    #[doc = "0x310 - device OUT endpoint-0 transfer length register"]
    #[inline(always)]
    pub const fn doep0len(&self) -> &DOEP0LEN {
        &self.doep0len
    }
    #[doc = "0x320 - device endpoint-1 control register"]
    #[inline(always)]
    pub const fn doep1ctl(&self) -> &DOEP1CTL {
        &self.doep1ctl
    }
    #[doc = "0x328 - device out endpoint-1 interrupt flag register"]
    #[inline(always)]
    pub const fn doep1intf(&self) -> &DOEP1INTF {
        &self.doep1intf
    }
    #[doc = "0x330 - device OUT endpoint-1 transfer length register"]
    #[inline(always)]
    pub const fn doep1len(&self) -> &DOEP1LEN {
        &self.doep1len
    }
    #[doc = "0x340 - device endpoint-2 control register"]
    #[inline(always)]
    pub const fn doep2ctl(&self) -> &DOEP2CTL {
        &self.doep2ctl
    }
    #[doc = "0x348 - device out endpoint-2 interrupt flag register"]
    #[inline(always)]
    pub const fn doep2intf(&self) -> &DOEP2INTF {
        &self.doep2intf
    }
    #[doc = "0x350 - device OUT endpoint-2 transfer length register"]
    #[inline(always)]
    pub const fn doep2len(&self) -> &DOEP2LEN {
        &self.doep2len
    }
    #[doc = "0x360 - device endpoint-3 control register"]
    #[inline(always)]
    pub const fn doep3ctl(&self) -> &DOEP3CTL {
        &self.doep3ctl
    }
    #[doc = "0x368 - device out endpoint-3 interrupt flag register"]
    #[inline(always)]
    pub const fn doep3intf(&self) -> &DOEP3INTF {
        &self.doep3intf
    }
    #[doc = "0x370 - device OUT endpoint-3 transfer length register"]
    #[inline(always)]
    pub const fn doep3len(&self) -> &DOEP3LEN {
        &self.doep3len
    }
}
#[doc = "DCFG (rw) register accessor: device configuration register (DCFG)\n\nYou can [`read`](crate::Reg::read) this register and get [`dcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcfg`]
module"]
pub type DCFG = crate::Reg<dcfg::DCFG_SPEC>;
#[doc = "device configuration register (DCFG)"]
pub mod dcfg;
#[doc = "DCTL (rw) register accessor: device control register (DCTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`dctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dctl`]
module"]
pub type DCTL = crate::Reg<dctl::DCTL_SPEC>;
#[doc = "device control register (DCTL)"]
pub mod dctl;
#[doc = "DSTAT (r) register accessor: device status register (DSTAT)\n\nYou can [`read`](crate::Reg::read) this register and get [`dstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dstat`]
module"]
pub type DSTAT = crate::Reg<dstat::DSTAT_SPEC>;
#[doc = "device status register (DSTAT)"]
pub mod dstat;
#[doc = "DIEPINTEN (rw) register accessor: device IN endpoint common interrupt mask register (DIEPINTEN)\n\nYou can [`read`](crate::Reg::read) this register and get [`diepinten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepinten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepinten`]
module"]
pub type DIEPINTEN = crate::Reg<diepinten::DIEPINTEN_SPEC>;
#[doc = "device IN endpoint common interrupt mask register (DIEPINTEN)"]
pub mod diepinten;
#[doc = "DOEPINTEN (rw) register accessor: device OUT endpoint common interrupt enable register (DOEPINTEN)\n\nYou can [`read`](crate::Reg::read) this register and get [`doepinten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepinten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepinten`]
module"]
pub type DOEPINTEN = crate::Reg<doepinten::DOEPINTEN_SPEC>;
#[doc = "device OUT endpoint common interrupt enable register (DOEPINTEN)"]
pub mod doepinten;
#[doc = "DAEPINT (r) register accessor: device all endpoints interrupt register (DAEPINT)\n\nYou can [`read`](crate::Reg::read) this register and get [`daepint::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daepint`]
module"]
pub type DAEPINT = crate::Reg<daepint::DAEPINT_SPEC>;
#[doc = "device all endpoints interrupt register (DAEPINT)"]
pub mod daepint;
#[doc = "DAEPINTEN (rw) register accessor: Device all endpoints interrupt enable register (DAEPINTEN)\n\nYou can [`read`](crate::Reg::read) this register and get [`daepinten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daepinten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daepinten`]
module"]
pub type DAEPINTEN = crate::Reg<daepinten::DAEPINTEN_SPEC>;
#[doc = "Device all endpoints interrupt enable register (DAEPINTEN)"]
pub mod daepinten;
#[doc = "DVBUSDT (rw) register accessor: device VBUS discharge time register\n\nYou can [`read`](crate::Reg::read) this register and get [`dvbusdt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvbusdt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dvbusdt`]
module"]
pub type DVBUSDT = crate::Reg<dvbusdt::DVBUSDT_SPEC>;
#[doc = "device VBUS discharge time register"]
pub mod dvbusdt;
#[doc = "DVBUSPT (rw) register accessor: device VBUS pulsing time register\n\nYou can [`read`](crate::Reg::read) this register and get [`dvbuspt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvbuspt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dvbuspt`]
module"]
pub type DVBUSPT = crate::Reg<dvbuspt::DVBUSPT_SPEC>;
#[doc = "device VBUS pulsing time register"]
pub mod dvbuspt;
#[doc = "DIEPFEINTEN (rw) register accessor: device IN endpoint FIFO empty interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`diepfeinten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepfeinten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepfeinten`]
module"]
pub type DIEPFEINTEN = crate::Reg<diepfeinten::DIEPFEINTEN_SPEC>;
#[doc = "device IN endpoint FIFO empty interrupt enable register"]
pub mod diepfeinten;
#[doc = "DIEP0CTL (rw) register accessor: device IN endpoint 0 control register (DIEP0CTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`diep0ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep0ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep0ctl`]
module"]
pub type DIEP0CTL = crate::Reg<diep0ctl::DIEP0CTL_SPEC>;
#[doc = "device IN endpoint 0 control register (DIEP0CTL)"]
pub mod diep0ctl;
#[doc = "DIEP1CTL (rw) register accessor: device in endpoint-1 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep1ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep1ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep1ctl`]
module"]
pub type DIEP1CTL = crate::Reg<diep1ctl::DIEP1CTL_SPEC>;
#[doc = "device in endpoint-1 control register"]
pub mod diep1ctl;
#[doc = "DIEP2CTL (rw) register accessor: device endpoint-2 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep2ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep2ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep2ctl`]
module"]
pub type DIEP2CTL = crate::Reg<diep2ctl::DIEP2CTL_SPEC>;
#[doc = "device endpoint-2 control register"]
pub mod diep2ctl;
#[doc = "DIEP3CTL (rw) register accessor: device endpoint-3 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep3ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep3ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep3ctl`]
module"]
pub type DIEP3CTL = crate::Reg<diep3ctl::DIEP3CTL_SPEC>;
#[doc = "device endpoint-3 control register"]
pub mod diep3ctl;
#[doc = "DOEP0CTL (rw) register accessor: device endpoint-0 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep0ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep0ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep0ctl`]
module"]
pub type DOEP0CTL = crate::Reg<doep0ctl::DOEP0CTL_SPEC>;
#[doc = "device endpoint-0 control register"]
pub mod doep0ctl;
#[doc = "DOEP1CTL (rw) register accessor: device endpoint-1 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep1ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep1ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep1ctl`]
module"]
pub type DOEP1CTL = crate::Reg<doep1ctl::DOEP1CTL_SPEC>;
#[doc = "device endpoint-1 control register"]
pub mod doep1ctl;
#[doc = "DOEP2CTL (rw) register accessor: device endpoint-2 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep2ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep2ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep2ctl`]
module"]
pub type DOEP2CTL = crate::Reg<doep2ctl::DOEP2CTL_SPEC>;
#[doc = "device endpoint-2 control register"]
pub mod doep2ctl;
#[doc = "DOEP3CTL (rw) register accessor: device endpoint-3 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep3ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep3ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep3ctl`]
module"]
pub type DOEP3CTL = crate::Reg<doep3ctl::DOEP3CTL_SPEC>;
#[doc = "device endpoint-3 control register"]
pub mod doep3ctl;
#[doc = "DIEP0INTF (rw) register accessor: device endpoint-0 interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep0intf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep0intf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep0intf`]
module"]
pub type DIEP0INTF = crate::Reg<diep0intf::DIEP0INTF_SPEC>;
#[doc = "device endpoint-0 interrupt register"]
pub mod diep0intf;
#[doc = "DIEP1INTF (rw) register accessor: device endpoint-1 interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep1intf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep1intf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep1intf`]
module"]
pub type DIEP1INTF = crate::Reg<diep1intf::DIEP1INTF_SPEC>;
#[doc = "device endpoint-1 interrupt register"]
pub mod diep1intf;
#[doc = "DIEP2INTF (rw) register accessor: device endpoint-2 interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep2intf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep2intf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep2intf`]
module"]
pub type DIEP2INTF = crate::Reg<diep2intf::DIEP2INTF_SPEC>;
#[doc = "device endpoint-2 interrupt register"]
pub mod diep2intf;
#[doc = "DIEP3INTF (rw) register accessor: device endpoint-3 interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep3intf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep3intf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep3intf`]
module"]
pub type DIEP3INTF = crate::Reg<diep3intf::DIEP3INTF_SPEC>;
#[doc = "device endpoint-3 interrupt register"]
pub mod diep3intf;
#[doc = "DOEP0INTF (rw) register accessor: device out endpoint-0 interrupt flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep0intf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep0intf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep0intf`]
module"]
pub type DOEP0INTF = crate::Reg<doep0intf::DOEP0INTF_SPEC>;
#[doc = "device out endpoint-0 interrupt flag register"]
pub mod doep0intf;
#[doc = "DOEP1INTF (rw) register accessor: device out endpoint-1 interrupt flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep1intf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep1intf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep1intf`]
module"]
pub type DOEP1INTF = crate::Reg<doep1intf::DOEP1INTF_SPEC>;
#[doc = "device out endpoint-1 interrupt flag register"]
pub mod doep1intf;
#[doc = "DOEP2INTF (rw) register accessor: device out endpoint-2 interrupt flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep2intf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep2intf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep2intf`]
module"]
pub type DOEP2INTF = crate::Reg<doep2intf::DOEP2INTF_SPEC>;
#[doc = "device out endpoint-2 interrupt flag register"]
pub mod doep2intf;
#[doc = "DOEP3INTF (rw) register accessor: device out endpoint-3 interrupt flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep3intf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep3intf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep3intf`]
module"]
pub type DOEP3INTF = crate::Reg<doep3intf::DOEP3INTF_SPEC>;
#[doc = "device out endpoint-3 interrupt flag register"]
pub mod doep3intf;
#[doc = "DIEP0LEN (rw) register accessor: device IN endpoint-0 transfer length register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep0len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep0len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep0len`]
module"]
pub type DIEP0LEN = crate::Reg<diep0len::DIEP0LEN_SPEC>;
#[doc = "device IN endpoint-0 transfer length register"]
pub mod diep0len;
#[doc = "DOEP0LEN (rw) register accessor: device OUT endpoint-0 transfer length register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep0len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep0len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep0len`]
module"]
pub type DOEP0LEN = crate::Reg<doep0len::DOEP0LEN_SPEC>;
#[doc = "device OUT endpoint-0 transfer length register"]
pub mod doep0len;
#[doc = "DIEP1LEN (rw) register accessor: device IN endpoint-1 transfer length register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep1len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep1len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep1len`]
module"]
pub type DIEP1LEN = crate::Reg<diep1len::DIEP1LEN_SPEC>;
#[doc = "device IN endpoint-1 transfer length register"]
pub mod diep1len;
#[doc = "DIEP2LEN (rw) register accessor: device IN endpoint-2 transfer length register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep2len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep2len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep2len`]
module"]
pub type DIEP2LEN = crate::Reg<diep2len::DIEP2LEN_SPEC>;
#[doc = "device IN endpoint-2 transfer length register"]
pub mod diep2len;
#[doc = "DIEP3LEN (rw) register accessor: device IN endpoint-3 transfer length register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep3len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep3len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep3len`]
module"]
pub type DIEP3LEN = crate::Reg<diep3len::DIEP3LEN_SPEC>;
#[doc = "device IN endpoint-3 transfer length register"]
pub mod diep3len;
#[doc = "DOEP1LEN (rw) register accessor: device OUT endpoint-1 transfer length register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep1len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep1len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep1len`]
module"]
pub type DOEP1LEN = crate::Reg<doep1len::DOEP1LEN_SPEC>;
#[doc = "device OUT endpoint-1 transfer length register"]
pub mod doep1len;
#[doc = "DOEP2LEN (rw) register accessor: device OUT endpoint-2 transfer length register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep2len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep2len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep2len`]
module"]
pub type DOEP2LEN = crate::Reg<doep2len::DOEP2LEN_SPEC>;
#[doc = "device OUT endpoint-2 transfer length register"]
pub mod doep2len;
#[doc = "DOEP3LEN (rw) register accessor: device OUT endpoint-3 transfer length register\n\nYou can [`read`](crate::Reg::read) this register and get [`doep3len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doep3len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doep3len`]
module"]
pub type DOEP3LEN = crate::Reg<doep3len::DOEP3LEN_SPEC>;
#[doc = "device OUT endpoint-3 transfer length register"]
pub mod doep3len;
#[doc = "DIEP0TFSTAT (r) register accessor: device IN endpoint 0 transmit FIFO status register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep0tfstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep0tfstat`]
module"]
pub type DIEP0TFSTAT = crate::Reg<diep0tfstat::DIEP0TFSTAT_SPEC>;
#[doc = "device IN endpoint 0 transmit FIFO status register"]
pub mod diep0tfstat;
#[doc = "DIEP1TFSTAT (r) register accessor: device IN endpoint 1 transmit FIFO status register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep1tfstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep1tfstat`]
module"]
pub type DIEP1TFSTAT = crate::Reg<diep1tfstat::DIEP1TFSTAT_SPEC>;
#[doc = "device IN endpoint 1 transmit FIFO status register"]
pub mod diep1tfstat;
#[doc = "DIEP2TFSTAT (r) register accessor: device IN endpoint 2 transmit FIFO status register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep2tfstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep2tfstat`]
module"]
pub type DIEP2TFSTAT = crate::Reg<diep2tfstat::DIEP2TFSTAT_SPEC>;
#[doc = "device IN endpoint 2 transmit FIFO status register"]
pub mod diep2tfstat;
#[doc = "DIEP3TFSTAT (r) register accessor: device IN endpoint 3 transmit FIFO status register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep3tfstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep3tfstat`]
module"]
pub type DIEP3TFSTAT = crate::Reg<diep3tfstat::DIEP3TFSTAT_SPEC>;
#[doc = "device IN endpoint 3 transmit FIFO status register"]
pub mod diep3tfstat;
