#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer value (lower half)"]
    pub mtime_lo: MTIME_LO,
    #[doc = "0x04 - Timer value (upper half)"]
    pub mtime_hi: MTIME_HI,
    #[doc = "0x08 - Timer comparison value (lower half)"]
    pub mtimecmp_lo: MTIMECMP_LO,
    #[doc = "0x0c - Timer comparison value (upper half)"]
    pub mtimecmp_hi: MTIMECMP_HI,
    _reserved4: [u8; 0x0fe8],
    #[doc = "0xff8 - Timer control register"]
    pub mstop: MSTOP,
    #[doc = "0xffc - Software interrupt register"]
    pub msip: MSIP,
}
#[doc = "mtime_lo (rw) register accessor: Timer value (lower half)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtime_lo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtime_lo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtime_lo`]
module"]
pub type MTIME_LO = crate::Reg<mtime_lo::MTIME_LO_SPEC>;
#[doc = "Timer value (lower half)"]
pub mod mtime_lo;
#[doc = "mtime_hi (rw) register accessor: Timer value (upper half)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtime_hi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtime_hi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtime_hi`]
module"]
pub type MTIME_HI = crate::Reg<mtime_hi::MTIME_HI_SPEC>;
#[doc = "Timer value (upper half)"]
pub mod mtime_hi;
#[doc = "mtimecmp_lo (rw) register accessor: Timer comparison value (lower half)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtimecmp_lo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtimecmp_lo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtimecmp_lo`]
module"]
pub type MTIMECMP_LO = crate::Reg<mtimecmp_lo::MTIMECMP_LO_SPEC>;
#[doc = "Timer comparison value (lower half)"]
pub mod mtimecmp_lo;
#[doc = "mtimecmp_hi (rw) register accessor: Timer comparison value (upper half)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtimecmp_hi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtimecmp_hi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtimecmp_hi`]
module"]
pub type MTIMECMP_HI = crate::Reg<mtimecmp_hi::MTIMECMP_HI_SPEC>;
#[doc = "Timer comparison value (upper half)"]
pub mod mtimecmp_hi;
#[doc = "mstop (rw) register accessor: Timer control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mstop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mstop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mstop`]
module"]
pub type MSTOP = crate::Reg<mstop::MSTOP_SPEC>;
#[doc = "Timer control register"]
pub mod mstop;
#[doc = "msip (rw) register accessor: Software interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msip::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msip::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msip`]
module"]
pub type MSIP = crate::Reg<msip::MSIP_SPEC>;
#[doc = "Software interrupt register"]
pub mod msip;
