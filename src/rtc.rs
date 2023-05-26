#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC interrupt enable register"]
    pub inten: INTEN,
    #[doc = "0x04 - control register"]
    pub ctl: CTL,
    #[doc = "0x08 - RTC prescaler high register"]
    pub psch: PSCH,
    #[doc = "0x0c - RTC prescaler low register"]
    pub pscl: PSCL,
    #[doc = "0x10 - RTC divider high register"]
    pub divh: DIVH,
    #[doc = "0x14 - RTC divider low register"]
    pub divl: DIVL,
    #[doc = "0x18 - RTC counter high register"]
    pub cnth: CNTH,
    #[doc = "0x1c - RTC counter low register"]
    pub cntl: CNTL,
    #[doc = "0x20 - Alarm high register"]
    pub alrmh: ALRMH,
    #[doc = "0x24 - RTC alarm low register"]
    pub alrml: ALRML,
}
#[doc = "INTEN (rw) register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "RTC interrupt enable register"]
pub mod inten;
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "control register"]
pub mod ctl;
#[doc = "PSCH (w) register accessor: an alias for `Reg<PSCH_SPEC>`"]
pub type PSCH = crate::Reg<psch::PSCH_SPEC>;
#[doc = "RTC prescaler high register"]
pub mod psch;
#[doc = "PSCL (w) register accessor: an alias for `Reg<PSCL_SPEC>`"]
pub type PSCL = crate::Reg<pscl::PSCL_SPEC>;
#[doc = "RTC prescaler low register"]
pub mod pscl;
#[doc = "DIVH (r) register accessor: an alias for `Reg<DIVH_SPEC>`"]
pub type DIVH = crate::Reg<divh::DIVH_SPEC>;
#[doc = "RTC divider high register"]
pub mod divh;
#[doc = "DIVL (r) register accessor: an alias for `Reg<DIVL_SPEC>`"]
pub type DIVL = crate::Reg<divl::DIVL_SPEC>;
#[doc = "RTC divider low register"]
pub mod divl;
#[doc = "CNTH (rw) register accessor: an alias for `Reg<CNTH_SPEC>`"]
pub type CNTH = crate::Reg<cnth::CNTH_SPEC>;
#[doc = "RTC counter high register"]
pub mod cnth;
#[doc = "CNTL (rw) register accessor: an alias for `Reg<CNTL_SPEC>`"]
pub type CNTL = crate::Reg<cntl::CNTL_SPEC>;
#[doc = "RTC counter low register"]
pub mod cntl;
#[doc = "ALRMH (w) register accessor: an alias for `Reg<ALRMH_SPEC>`"]
pub type ALRMH = crate::Reg<alrmh::ALRMH_SPEC>;
#[doc = "Alarm high register"]
pub mod alrmh;
#[doc = "ALRML (w) register accessor: an alias for `Reg<ALRML_SPEC>`"]
pub type ALRML = crate::Reg<alrml::ALRML_SPEC>;
#[doc = "RTC alarm low register"]
pub mod alrml;
