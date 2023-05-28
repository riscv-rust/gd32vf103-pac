#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Event control register"]
    pub ec: EC,
    #[doc = "0x04 - AFIO port configuration register 0"]
    pub pcf0: PCF0,
    #[doc = "0x08 - EXTI sources selection register 0"]
    pub extiss0: EXTISS0,
    #[doc = "0x0c - EXTI sources selection register 1"]
    pub extiss1: EXTISS1,
    #[doc = "0x10 - EXTI sources selection register 2"]
    pub extiss2: EXTISS2,
    #[doc = "0x14 - EXTI sources selection register 3"]
    pub extiss3: EXTISS3,
    _reserved6: [u8; 0x04],
    #[doc = "0x1c - AFIO port configuration register 1"]
    pub pcf1: PCF1,
}
#[doc = "EC (rw) register accessor: an alias for `Reg<EC_SPEC>`"]
pub type EC = crate::Reg<ec::EC_SPEC>;
#[doc = "Event control register"]
pub mod ec;
#[doc = "PCF0 (rw) register accessor: an alias for `Reg<PCF0_SPEC>`"]
pub type PCF0 = crate::Reg<pcf0::PCF0_SPEC>;
#[doc = "AFIO port configuration register 0"]
pub mod pcf0;
#[doc = "EXTISS0 (rw) register accessor: an alias for `Reg<EXTISS0_SPEC>`"]
pub type EXTISS0 = crate::Reg<extiss0::EXTISS0_SPEC>;
#[doc = "EXTI sources selection register 0"]
pub mod extiss0;
#[doc = "EXTISS1 (rw) register accessor: an alias for `Reg<EXTISS1_SPEC>`"]
pub type EXTISS1 = crate::Reg<extiss1::EXTISS1_SPEC>;
#[doc = "EXTI sources selection register 1"]
pub mod extiss1;
#[doc = "EXTISS2 (rw) register accessor: an alias for `Reg<EXTISS2_SPEC>`"]
pub type EXTISS2 = crate::Reg<extiss2::EXTISS2_SPEC>;
#[doc = "EXTI sources selection register 2"]
pub mod extiss2;
#[doc = "EXTISS3 (rw) register accessor: an alias for `Reg<EXTISS3_SPEC>`"]
pub type EXTISS3 = crate::Reg<extiss3::EXTISS3_SPEC>;
#[doc = "EXTI sources selection register 3"]
pub mod extiss3;
#[doc = "PCF1 (rw) register accessor: an alias for `Reg<PCF1_SPEC>`"]
pub type PCF1 = crate::Reg<pcf1::PCF1_SPEC>;
#[doc = "AFIO port configuration register 1"]
pub mod pcf1;
