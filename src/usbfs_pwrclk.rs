#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - power and clock gating control register (PWRCLKCTL)"]
    pub pwrclkctl: PWRCLKCTL,
}
#[doc = "power and clock gating control register (PWRCLKCTL)"]
pub struct PWRCLKCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "power and clock gating control register (PWRCLKCTL)"]
pub mod pwrclkctl;
