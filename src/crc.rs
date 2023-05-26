#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Data register"]
    pub data: DATA,
    #[doc = "0x04 - Free data register"]
    pub fdata: FDATA,
    #[doc = "0x08 - Control register"]
    pub ctl: CTL,
}
#[doc = "DATA (rw) register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "Data register"]
pub mod data;
#[doc = "FDATA (rw) register accessor: an alias for `Reg<FDATA_SPEC>`"]
pub type FDATA = crate::Reg<fdata::FDATA_SPEC>;
#[doc = "Free data register"]
pub mod fdata;
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control register"]
pub mod ctl;
