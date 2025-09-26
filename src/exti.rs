#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    inten: INTEN,
    even: EVEN,
    rten: RTEN,
    ften: FTEN,
    swiev: SWIEV,
    pd: PD,
}
impl RegisterBlock {
    #[doc = "0x00 - Interrupt enable register (EXTI_INTEN)"]
    #[inline(always)]
    pub const fn inten(&self) -> &INTEN {
        &self.inten
    }
    #[doc = "0x04 - Event enable register (EXTI_EVEN)"]
    #[inline(always)]
    pub const fn even(&self) -> &EVEN {
        &self.even
    }
    #[doc = "0x08 - Rising Edge Trigger Enable register (EXTI_RTEN)"]
    #[inline(always)]
    pub const fn rten(&self) -> &RTEN {
        &self.rten
    }
    #[doc = "0x0c - Falling Egde Trigger Enable register (EXTI_FTEN)"]
    #[inline(always)]
    pub const fn ften(&self) -> &FTEN {
        &self.ften
    }
    #[doc = "0x10 - Software interrupt event register (EXTI_SWIEV)"]
    #[inline(always)]
    pub const fn swiev(&self) -> &SWIEV {
        &self.swiev
    }
    #[doc = "0x14 - Pending register (EXTI_PD)"]
    #[inline(always)]
    pub const fn pd(&self) -> &PD {
        &self.pd
    }
}
#[doc = "INTEN (rw) register accessor: Interrupt enable register (EXTI_INTEN)\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Interrupt enable register (EXTI_INTEN)"]
pub mod inten;
#[doc = "EVEN (rw) register accessor: Event enable register (EXTI_EVEN)\n\nYou can [`read`](crate::Reg::read) this register and get [`even::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`even::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@even`]
module"]
pub type EVEN = crate::Reg<even::EVEN_SPEC>;
#[doc = "Event enable register (EXTI_EVEN)"]
pub mod even;
#[doc = "RTEN (rw) register accessor: Rising Edge Trigger Enable register (EXTI_RTEN)\n\nYou can [`read`](crate::Reg::read) this register and get [`rten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rten`]
module"]
pub type RTEN = crate::Reg<rten::RTEN_SPEC>;
#[doc = "Rising Edge Trigger Enable register (EXTI_RTEN)"]
pub mod rten;
#[doc = "FTEN (rw) register accessor: Falling Egde Trigger Enable register (EXTI_FTEN)\n\nYou can [`read`](crate::Reg::read) this register and get [`ften::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ften::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ften`]
module"]
pub type FTEN = crate::Reg<ften::FTEN_SPEC>;
#[doc = "Falling Egde Trigger Enable register (EXTI_FTEN)"]
pub mod ften;
#[doc = "SWIEV (rw) register accessor: Software interrupt event register (EXTI_SWIEV)\n\nYou can [`read`](crate::Reg::read) this register and get [`swiev::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swiev::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swiev`]
module"]
pub type SWIEV = crate::Reg<swiev::SWIEV_SPEC>;
#[doc = "Software interrupt event register (EXTI_SWIEV)"]
pub mod swiev;
#[doc = "PD (rw) register accessor: Pending register (EXTI_PD)\n\nYou can [`read`](crate::Reg::read) this register and get [`pd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd`]
module"]
pub type PD = crate::Reg<pd::PD_SPEC>;
#[doc = "Pending register (EXTI_PD)"]
pub mod pd;
