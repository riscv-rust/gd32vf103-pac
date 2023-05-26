#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - cliccfg Register"]
    pub cliccfg: CLICCFG,
    _reserved1: [u8; 0x03],
    #[doc = "0x04 - clicinfo Register"]
    pub clicinfo: CLICINFO,
    _reserved2: [u8; 0x03],
    #[doc = "0x0b - MTH Register"]
    pub mth: MTH,
    _reserved3: [u8; 0x0ff4],
    #[doc = "0x1000..0x115c - Core-local Interrupt Controller Interrupt Registers"]
    pub clicints: [CLICINTS; 87],
}
#[doc = "Core-local Interrupt Controller Interrupt Registers"]
pub use self::clicints::CLICINTS;
#[doc = r"Cluster"]
#[doc = "Core-local Interrupt Controller Interrupt Registers"]
pub mod clicints;
#[doc = "CLICCFG (rw) register accessor: an alias for `Reg<CLICCFG_SPEC>`"]
pub type CLICCFG = crate::Reg<cliccfg::CLICCFG_SPEC>;
#[doc = "cliccfg Register"]
pub mod cliccfg;
#[doc = "CLICINFO (r) register accessor: an alias for `Reg<CLICINFO_SPEC>`"]
pub type CLICINFO = crate::Reg<clicinfo::CLICINFO_SPEC>;
#[doc = "clicinfo Register"]
pub mod clicinfo;
#[doc = "MTH (rw) register accessor: an alias for `Reg<MTH_SPEC>`"]
pub type MTH = crate::Reg<mth::MTH_SPEC>;
#[doc = "MTH Register"]
pub mod mth;
