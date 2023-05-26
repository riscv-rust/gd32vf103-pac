#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SRAM/NOR flash control register 0"]
    pub snctl0: SNCTL0,
    #[doc = "0x04 - SRAM/NOR flash timing configuration register 0"]
    pub sntcfg0: SNTCFG0,
    #[doc = "0x08 - SRAM/NOR flash control register 1"]
    pub snctl1: SNCTL1,
}
#[doc = "SNCTL0 (rw) register accessor: an alias for `Reg<SNCTL0_SPEC>`"]
pub type SNCTL0 = crate::Reg<snctl0::SNCTL0_SPEC>;
#[doc = "SRAM/NOR flash control register 0"]
pub mod snctl0;
#[doc = "SNTCFG0 (rw) register accessor: an alias for `Reg<SNTCFG0_SPEC>`"]
pub type SNTCFG0 = crate::Reg<sntcfg0::SNTCFG0_SPEC>;
#[doc = "SRAM/NOR flash timing configuration register 0"]
pub mod sntcfg0;
#[doc = "SNCTL1 (rw) register accessor: an alias for `Reg<SNCTL1_SPEC>`"]
pub type SNCTL1 = crate::Reg<snctl1::SNCTL1_SPEC>;
#[doc = "SRAM/NOR flash control register 1"]
pub mod snctl1;
