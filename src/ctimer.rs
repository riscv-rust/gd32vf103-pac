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
#[doc = "mtime_lo (rw) register accessor: an alias for `Reg<MTIME_LO_SPEC>`"]
pub type MTIME_LO = crate::Reg<mtime_lo::MTIME_LO_SPEC>;
#[doc = "Timer value (lower half)"]
pub mod mtime_lo;
#[doc = "mtime_hi (rw) register accessor: an alias for `Reg<MTIME_HI_SPEC>`"]
pub type MTIME_HI = crate::Reg<mtime_hi::MTIME_HI_SPEC>;
#[doc = "Timer value (upper half)"]
pub mod mtime_hi;
#[doc = "mtimecmp_lo (rw) register accessor: an alias for `Reg<MTIMECMP_LO_SPEC>`"]
pub type MTIMECMP_LO = crate::Reg<mtimecmp_lo::MTIMECMP_LO_SPEC>;
#[doc = "Timer comparison value (lower half)"]
pub mod mtimecmp_lo;
#[doc = "mtimecmp_hi (rw) register accessor: an alias for `Reg<MTIMECMP_HI_SPEC>`"]
pub type MTIMECMP_HI = crate::Reg<mtimecmp_hi::MTIMECMP_HI_SPEC>;
#[doc = "Timer comparison value (upper half)"]
pub mod mtimecmp_hi;
#[doc = "mstop (rw) register accessor: an alias for `Reg<MSTOP_SPEC>`"]
pub type MSTOP = crate::Reg<mstop::MSTOP_SPEC>;
#[doc = "Timer control register"]
pub mod mstop;
#[doc = "msip (rw) register accessor: an alias for `Reg<MSIP_SPEC>`"]
pub type MSIP = crate::Reg<msip::MSIP_SPEC>;
#[doc = "Software interrupt register"]
pub mod msip;
