#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - power and clock gating control register (PWRCLKCTL)"]
    pub pwrclkctl: PWRCLKCTL,
}
#[doc = "PWRCLKCTL (rw) register accessor: an alias for `Reg<PWRCLKCTL_SPEC>`"]
pub type PWRCLKCTL = crate::Reg<pwrclkctl::PWRCLKCTL_SPEC>;
#[doc = "power and clock gating control register (PWRCLKCTL)"]
pub mod pwrclkctl;
