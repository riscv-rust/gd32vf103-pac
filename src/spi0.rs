#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctl0: CTL0,
    _reserved1: [u8; 0x02],
    ctl1: CTL1,
    _reserved2: [u8; 0x02],
    stat: STAT,
    _reserved3: [u8; 0x02],
    data: DATA,
    _reserved4: [u8; 0x02],
    crcpoly: CRCPOLY,
    _reserved5: [u8; 0x02],
    rcrc: RCRC,
    _reserved6: [u8; 0x02],
    tcrc: TCRC,
    _reserved7: [u8; 0x02],
    i2sctl: I2SCTL,
    _reserved8: [u8; 0x02],
    i2spsc: I2SPSC,
}
impl RegisterBlock {
    #[doc = "0x00 - control register 0"]
    #[inline(always)]
    pub const fn ctl0(&self) -> &CTL0 {
        &self.ctl0
    }
    #[doc = "0x04 - control register 1"]
    #[inline(always)]
    pub const fn ctl1(&self) -> &CTL1 {
        &self.ctl1
    }
    #[doc = "0x08 - status register"]
    #[inline(always)]
    pub const fn stat(&self) -> &STAT {
        &self.stat
    }
    #[doc = "0x0c - data register"]
    #[inline(always)]
    pub const fn data(&self) -> &DATA {
        &self.data
    }
    #[doc = "0x10 - CRC polynomial register"]
    #[inline(always)]
    pub const fn crcpoly(&self) -> &CRCPOLY {
        &self.crcpoly
    }
    #[doc = "0x14 - RX CRC register"]
    #[inline(always)]
    pub const fn rcrc(&self) -> &RCRC {
        &self.rcrc
    }
    #[doc = "0x18 - TX CRC register"]
    #[inline(always)]
    pub const fn tcrc(&self) -> &TCRC {
        &self.tcrc
    }
    #[doc = "0x1c - I2S control register"]
    #[inline(always)]
    pub const fn i2sctl(&self) -> &I2SCTL {
        &self.i2sctl
    }
    #[doc = "0x20 - I2S prescaler register"]
    #[inline(always)]
    pub const fn i2spsc(&self) -> &I2SPSC {
        &self.i2spsc
    }
}
#[doc = "CTL0 (rw) register accessor: control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl0`]
module"]
pub type CTL0 = crate::Reg<ctl0::CTL0_SPEC>;
#[doc = "control register 0"]
pub mod ctl0;
#[doc = "CTL1 (rw) register accessor: control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl1`]
module"]
pub type CTL1 = crate::Reg<ctl1::CTL1_SPEC>;
#[doc = "control register 1"]
pub mod ctl1;
#[doc = "STAT (rw) register accessor: status register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "status register"]
pub mod stat;
#[doc = "DATA (rw) register accessor: data register\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`]
module"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "data register"]
pub mod data;
#[doc = "CRCPOLY (rw) register accessor: CRC polynomial register\n\nYou can [`read`](crate::Reg::read) this register and get [`crcpoly::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcpoly::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcpoly`]
module"]
pub type CRCPOLY = crate::Reg<crcpoly::CRCPOLY_SPEC>;
#[doc = "CRC polynomial register"]
pub mod crcpoly;
#[doc = "RCRC (r) register accessor: RX CRC register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcrc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcrc`]
module"]
pub type RCRC = crate::Reg<rcrc::RCRC_SPEC>;
#[doc = "RX CRC register"]
pub mod rcrc;
#[doc = "TCRC (r) register accessor: TX CRC register\n\nYou can [`read`](crate::Reg::read) this register and get [`tcrc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcrc`]
module"]
pub type TCRC = crate::Reg<tcrc::TCRC_SPEC>;
#[doc = "TX CRC register"]
pub mod tcrc;
#[doc = "I2SCTL (rw) register accessor: I2S control register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2sctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2sctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2sctl`]
module"]
pub type I2SCTL = crate::Reg<i2sctl::I2SCTL_SPEC>;
#[doc = "I2S control register"]
pub mod i2sctl;
#[doc = "I2SPSC (rw) register accessor: I2S prescaler register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2spsc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2spsc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2spsc`]
module"]
pub type I2SPSC = crate::Reg<i2spsc::I2SPSC_SPEC>;
#[doc = "I2S prescaler register"]
pub mod i2spsc;
