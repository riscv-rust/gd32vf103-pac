#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    stat: STAT,
    data: DATA,
    baud: BAUD,
    ctl0: CTL0,
    ctl1: CTL1,
    ctl2: CTL2,
    gp: GP,
}
impl RegisterBlock {
    #[doc = "0x00 - Status register"]
    #[inline(always)]
    pub const fn stat(&self) -> &STAT {
        &self.stat
    }
    #[doc = "0x04 - Data register"]
    #[inline(always)]
    pub const fn data(&self) -> &DATA {
        &self.data
    }
    #[doc = "0x08 - Baud rate register"]
    #[inline(always)]
    pub const fn baud(&self) -> &BAUD {
        &self.baud
    }
    #[doc = "0x0c - Control register 0"]
    #[inline(always)]
    pub const fn ctl0(&self) -> &CTL0 {
        &self.ctl0
    }
    #[doc = "0x10 - Control register 1"]
    #[inline(always)]
    pub const fn ctl1(&self) -> &CTL1 {
        &self.ctl1
    }
    #[doc = "0x14 - Control register 2"]
    #[inline(always)]
    pub const fn ctl2(&self) -> &CTL2 {
        &self.ctl2
    }
    #[doc = "0x18 - Guard time and prescaler register"]
    #[inline(always)]
    pub const fn gp(&self) -> &GP {
        &self.gp
    }
}
#[doc = "STAT (rw) register accessor: Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Status register"]
pub mod stat;
#[doc = "DATA (rw) register accessor: Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`]
module"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "Data register"]
pub mod data;
#[doc = "BAUD (rw) register accessor: Baud rate register\n\nYou can [`read`](crate::Reg::read) this register and get [`baud::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`baud::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@baud`]
module"]
pub type BAUD = crate::Reg<baud::BAUD_SPEC>;
#[doc = "Baud rate register"]
pub mod baud;
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
#[doc = "CTL2 (rw) register accessor: Control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl2`]
module"]
pub type CTL2 = crate::Reg<ctl2::CTL2_SPEC>;
#[doc = "Control register 2"]
pub mod ctl2;
#[doc = "GP (rw) register accessor: Guard time and prescaler register\n\nYou can [`read`](crate::Reg::read) this register and get [`gp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gp`]
module"]
pub type GP = crate::Reg<gp::GP_SPEC>;
#[doc = "Guard time and prescaler register"]
pub mod gp;
