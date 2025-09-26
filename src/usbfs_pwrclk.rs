#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pwrclkctl: PWRCLKCTL,
}
impl RegisterBlock {
    #[doc = "0x00 - power and clock gating control register (PWRCLKCTL)"]
    #[inline(always)]
    pub const fn pwrclkctl(&self) -> &PWRCLKCTL {
        &self.pwrclkctl
    }
}
#[doc = "PWRCLKCTL (rw) register accessor: power and clock gating control register (PWRCLKCTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrclkctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrclkctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrclkctl`]
module"]
pub type PWRCLKCTL = crate::Reg<pwrclkctl::PWRCLKCTL_SPEC>;
#[doc = "power and clock gating control register (PWRCLKCTL)"]
pub mod pwrclkctl;
