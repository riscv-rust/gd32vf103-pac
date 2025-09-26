#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    snctl0: SNCTL0,
    sntcfg0: SNTCFG0,
    snctl1: SNCTL1,
}
impl RegisterBlock {
    #[doc = "0x00 - SRAM/NOR flash control register 0"]
    #[inline(always)]
    pub const fn snctl0(&self) -> &SNCTL0 {
        &self.snctl0
    }
    #[doc = "0x04 - SRAM/NOR flash timing configuration register 0"]
    #[inline(always)]
    pub const fn sntcfg0(&self) -> &SNTCFG0 {
        &self.sntcfg0
    }
    #[doc = "0x08 - SRAM/NOR flash control register 1"]
    #[inline(always)]
    pub const fn snctl1(&self) -> &SNCTL1 {
        &self.snctl1
    }
}
#[doc = "SNCTL0 (rw) register accessor: SRAM/NOR flash control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`snctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@snctl0`]
module"]
pub type SNCTL0 = crate::Reg<snctl0::SNCTL0_SPEC>;
#[doc = "SRAM/NOR flash control register 0"]
pub mod snctl0;
#[doc = "SNTCFG0 (rw) register accessor: SRAM/NOR flash timing configuration register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sntcfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sntcfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sntcfg0`]
module"]
pub type SNTCFG0 = crate::Reg<sntcfg0::SNTCFG0_SPEC>;
#[doc = "SRAM/NOR flash timing configuration register 0"]
pub mod sntcfg0;
#[doc = "SNCTL1 (rw) register accessor: SRAM/NOR flash control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`snctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@snctl1`]
module"]
pub type SNCTL1 = crate::Reg<snctl1::SNCTL1_SPEC>;
#[doc = "SRAM/NOR flash control register 1"]
pub mod snctl1;
