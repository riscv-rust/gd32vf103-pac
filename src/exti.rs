#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt enable register (EXTI_INTEN)"]
    pub inten: INTEN,
    #[doc = "0x04 - Event enable register (EXTI_EVEN)"]
    pub even: EVEN,
    #[doc = "0x08 - Rising Edge Trigger Enable register (EXTI_RTEN)"]
    pub rten: RTEN,
    #[doc = "0x0c - Falling Egde Trigger Enable register (EXTI_FTEN)"]
    pub ften: FTEN,
    #[doc = "0x10 - Software interrupt event register (EXTI_SWIEV)"]
    pub swiev: SWIEV,
    #[doc = "0x14 - Pending register (EXTI_PD)"]
    pub pd: PD,
}
#[doc = "INTEN (rw) register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Interrupt enable register (EXTI_INTEN)"]
pub mod inten;
#[doc = "EVEN (rw) register accessor: an alias for `Reg<EVEN_SPEC>`"]
pub type EVEN = crate::Reg<even::EVEN_SPEC>;
#[doc = "Event enable register (EXTI_EVEN)"]
pub mod even;
#[doc = "RTEN (rw) register accessor: an alias for `Reg<RTEN_SPEC>`"]
pub type RTEN = crate::Reg<rten::RTEN_SPEC>;
#[doc = "Rising Edge Trigger Enable register (EXTI_RTEN)"]
pub mod rten;
#[doc = "FTEN (rw) register accessor: an alias for `Reg<FTEN_SPEC>`"]
pub type FTEN = crate::Reg<ften::FTEN_SPEC>;
#[doc = "Falling Egde Trigger Enable register (EXTI_FTEN)"]
pub mod ften;
#[doc = "SWIEV (rw) register accessor: an alias for `Reg<SWIEV_SPEC>`"]
pub type SWIEV = crate::Reg<swiev::SWIEV_SPEC>;
#[doc = "Software interrupt event register (EXTI_SWIEV)"]
pub mod swiev;
#[doc = "PD (rw) register accessor: an alias for `Reg<PD_SPEC>`"]
pub type PD = crate::Reg<pd::PD_SPEC>;
#[doc = "Pending register (EXTI_PD)"]
pub mod pd;
