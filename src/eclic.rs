#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cliccfg: CLICCFG,
    _reserved1: [u8; 0x03],
    clicinfo: CLICINFO,
    _reserved2: [u8; 0x03],
    mth: MTH,
    _reserved3: [u8; 0x0ff4],
    clicints: [CLICINTS; 87],
}
impl RegisterBlock {
    #[doc = "0x00 - cliccfg Register"]
    #[inline(always)]
    pub const fn cliccfg(&self) -> &CLICCFG {
        &self.cliccfg
    }
    #[doc = "0x04 - clicinfo Register"]
    #[inline(always)]
    pub const fn clicinfo(&self) -> &CLICINFO {
        &self.clicinfo
    }
    #[doc = "0x0b - MTH Register"]
    #[inline(always)]
    pub const fn mth(&self) -> &MTH {
        &self.mth
    }
    #[doc = "0x1000..0x115c - Core-local Interrupt Controller Interrupt Registers"]
    #[inline(always)]
    pub const fn clicints(&self, n: usize) -> &CLICINTS {
        &self.clicints[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1000..0x115c - Core-local Interrupt Controller Interrupt Registers"]
    #[inline(always)]
    pub fn clicints_iter(&self) -> impl Iterator<Item = &CLICINTS> {
        self.clicints.iter()
    }
}
#[doc = "Core-local Interrupt Controller Interrupt Registers"]
pub use self::clicints::CLICINTS;
#[doc = r"Cluster"]
#[doc = "Core-local Interrupt Controller Interrupt Registers"]
pub mod clicints;
#[doc = "CLICCFG (rw) register accessor: cliccfg Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cliccfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cliccfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cliccfg`]
module"]
pub type CLICCFG = crate::Reg<cliccfg::CLICCFG_SPEC>;
#[doc = "cliccfg Register"]
pub mod cliccfg;
#[doc = "CLICINFO (r) register accessor: clicinfo Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clicinfo::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clicinfo`]
module"]
pub type CLICINFO = crate::Reg<clicinfo::CLICINFO_SPEC>;
#[doc = "clicinfo Register"]
pub mod clicinfo;
#[doc = "MTH (rw) register accessor: MTH Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mth::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mth::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mth`]
module"]
pub type MTH = crate::Reg<mth::MTH_SPEC>;
#[doc = "MTH Register"]
pub mod mth;
