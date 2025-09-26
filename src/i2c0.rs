#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctl0: CTL0,
    _reserved1: [u8; 0x02],
    ctl1: CTL1,
    _reserved2: [u8; 0x02],
    saddr0: SADDR0,
    _reserved3: [u8; 0x02],
    saddr1: SADDR1,
    _reserved4: [u8; 0x02],
    data: DATA,
    _reserved5: [u8; 0x02],
    stat0: STAT0,
    _reserved6: [u8; 0x02],
    stat1: STAT1,
    _reserved7: [u8; 0x02],
    ckcfg: CKCFG,
    _reserved8: [u8; 0x02],
    rt: RT,
    _reserved9: [u8; 0x6e],
    fmpcfg: FMPCFG,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register 0"]
    #[inline(always)]
    pub const fn ctl0(&self) -> &CTL0 {
        &self.ctl0
    }
    #[doc = "0x04 - Control register 1"]
    #[inline(always)]
    pub const fn ctl1(&self) -> &CTL1 {
        &self.ctl1
    }
    #[doc = "0x08 - Slave address register 0"]
    #[inline(always)]
    pub const fn saddr0(&self) -> &SADDR0 {
        &self.saddr0
    }
    #[doc = "0x0c - Slave address register 1"]
    #[inline(always)]
    pub const fn saddr1(&self) -> &SADDR1 {
        &self.saddr1
    }
    #[doc = "0x10 - Transfer buffer register"]
    #[inline(always)]
    pub const fn data(&self) -> &DATA {
        &self.data
    }
    #[doc = "0x14 - Transfer status register 0"]
    #[inline(always)]
    pub const fn stat0(&self) -> &STAT0 {
        &self.stat0
    }
    #[doc = "0x18 - Transfer status register 1"]
    #[inline(always)]
    pub const fn stat1(&self) -> &STAT1 {
        &self.stat1
    }
    #[doc = "0x1c - Clock configure register"]
    #[inline(always)]
    pub const fn ckcfg(&self) -> &CKCFG {
        &self.ckcfg
    }
    #[doc = "0x20 - Rise time register"]
    #[inline(always)]
    pub const fn rt(&self) -> &RT {
        &self.rt
    }
    #[doc = "0x90 - Fast mode plus configure register"]
    #[inline(always)]
    pub const fn fmpcfg(&self) -> &FMPCFG {
        &self.fmpcfg
    }
}
#[doc = "CTL0 (rw) register accessor: Control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl0`]
module"]
pub type CTL0 = crate::Reg<ctl0::CTL0_SPEC>;
#[doc = "Control register 0"]
pub mod ctl0;
#[doc = "CTL1 (rw) register accessor: Control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl1`]
module"]
pub type CTL1 = crate::Reg<ctl1::CTL1_SPEC>;
#[doc = "Control register 1"]
pub mod ctl1;
#[doc = "SADDR0 (rw) register accessor: Slave address register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`saddr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saddr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saddr0`]
module"]
pub type SADDR0 = crate::Reg<saddr0::SADDR0_SPEC>;
#[doc = "Slave address register 0"]
pub mod saddr0;
#[doc = "SADDR1 (rw) register accessor: Slave address register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`saddr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saddr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saddr1`]
module"]
pub type SADDR1 = crate::Reg<saddr1::SADDR1_SPEC>;
#[doc = "Slave address register 1"]
pub mod saddr1;
#[doc = "DATA (rw) register accessor: Transfer buffer register\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`]
module"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "Transfer buffer register"]
pub mod data;
#[doc = "STAT0 (rw) register accessor: Transfer status register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`stat0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat0`]
module"]
pub type STAT0 = crate::Reg<stat0::STAT0_SPEC>;
#[doc = "Transfer status register 0"]
pub mod stat0;
#[doc = "STAT1 (r) register accessor: Transfer status register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`stat1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat1`]
module"]
pub type STAT1 = crate::Reg<stat1::STAT1_SPEC>;
#[doc = "Transfer status register 1"]
pub mod stat1;
#[doc = "CKCFG (rw) register accessor: Clock configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`ckcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ckcfg`]
module"]
pub type CKCFG = crate::Reg<ckcfg::CKCFG_SPEC>;
#[doc = "Clock configure register"]
pub mod ckcfg;
#[doc = "RT (rw) register accessor: Rise time register\n\nYou can [`read`](crate::Reg::read) this register and get [`rt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rt`]
module"]
pub type RT = crate::Reg<rt::RT_SPEC>;
#[doc = "Rise time register"]
pub mod rt;
#[doc = "FMPCFG (rw) register accessor: Fast mode plus configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`fmpcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmpcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmpcfg`]
module"]
pub type FMPCFG = crate::Reg<fmpcfg::FMPCFG_SPEC>;
#[doc = "Fast mode plus configure register"]
pub mod fmpcfg;
