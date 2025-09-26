#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - power and clock gating control register (PWRCLKCTL)"]
    pub pwrclkctl: PWRCLKCTL,
}
#[doc = "PWRCLKCTL (rw) register accessor: power and clock gating control register (PWRCLKCTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrclkctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrclkctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrclkctl`]
module"]
pub type PWRCLKCTL = crate::Reg<pwrclkctl::PWRCLKCTL_SPEC>;
#[doc = "power and clock gating control register (PWRCLKCTL)"]
pub mod pwrclkctl;
