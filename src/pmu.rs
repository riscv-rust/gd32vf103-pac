#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctl: CTL,
    cs: CS,
}
impl RegisterBlock {
    #[doc = "0x00 - power control register"]
    #[inline(always)]
    pub const fn ctl(&self) -> &CTL {
        &self.ctl
    }
    #[doc = "0x04 - power control/status register"]
    #[inline(always)]
    pub const fn cs(&self) -> &CS {
        &self.cs
    }
}
#[doc = "CTL (rw) register accessor: power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "power control register"]
pub mod ctl;
#[doc = "CS (rw) register accessor: power control/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cs`]
module"]
pub type CS = crate::Reg<cs::CS_SPEC>;
#[doc = "power control/status register"]
pub mod cs;
