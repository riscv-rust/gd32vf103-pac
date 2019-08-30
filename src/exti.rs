#[doc = r" Register block"]
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
#[doc = "Interrupt enable register (EXTI_INTEN)"]
pub struct INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt enable register (EXTI_INTEN)"]
pub mod inten;
#[doc = "Event enable register (EXTI_EVEN)"]
pub struct EVEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event enable register (EXTI_EVEN)"]
pub mod even;
#[doc = "Rising Edge Trigger Enable register (EXTI_RTEN)"]
pub struct RTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Rising Edge Trigger Enable register (EXTI_RTEN)"]
pub mod rten;
#[doc = "Falling Egde Trigger Enable register (EXTI_FTEN)"]
pub struct FTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Falling Egde Trigger Enable register (EXTI_FTEN)"]
pub mod ften;
#[doc = "Software interrupt event register (EXTI_SWIEV)"]
pub struct SWIEV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software interrupt event register (EXTI_SWIEV)"]
pub mod swiev;
#[doc = "Pending register (EXTI_PD)"]
pub struct PD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pending register (EXTI_PD)"]
pub mod pd;
