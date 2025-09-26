#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctl0: CTL0,
    ctl1: CTL1,
    istat: ISTAT,
    octl: OCTL,
    bop: BOP,
    bc: BC,
    lock: LOCK,
}
impl RegisterBlock {
    #[doc = "0x00 - port control register 0"]
    #[inline(always)]
    pub const fn ctl0(&self) -> &CTL0 {
        &self.ctl0
    }
    #[doc = "0x04 - port control register 1"]
    #[inline(always)]
    pub const fn ctl1(&self) -> &CTL1 {
        &self.ctl1
    }
    #[doc = "0x08 - Port input status register"]
    #[inline(always)]
    pub const fn istat(&self) -> &ISTAT {
        &self.istat
    }
    #[doc = "0x0c - Port output control register"]
    #[inline(always)]
    pub const fn octl(&self) -> &OCTL {
        &self.octl
    }
    #[doc = "0x10 - Port bit operate register"]
    #[inline(always)]
    pub const fn bop(&self) -> &BOP {
        &self.bop
    }
    #[doc = "0x14 - Port bit clear register"]
    #[inline(always)]
    pub const fn bc(&self) -> &BC {
        &self.bc
    }
    #[doc = "0x18 - GPIO port configuration lock register"]
    #[inline(always)]
    pub const fn lock(&self) -> &LOCK {
        &self.lock
    }
}
#[doc = "CTL0 (rw) register accessor: port control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl0`]
module"]
pub type CTL0 = crate::Reg<ctl0::CTL0_SPEC>;
#[doc = "port control register 0"]
pub mod ctl0;
#[doc = "CTL1 (rw) register accessor: port control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl1`]
module"]
pub type CTL1 = crate::Reg<ctl1::CTL1_SPEC>;
#[doc = "port control register 1"]
pub mod ctl1;
#[doc = "ISTAT (r) register accessor: Port input status register\n\nYou can [`read`](crate::Reg::read) this register and get [`istat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@istat`]
module"]
pub type ISTAT = crate::Reg<istat::ISTAT_SPEC>;
#[doc = "Port input status register"]
pub mod istat;
#[doc = "OCTL (rw) register accessor: Port output control register\n\nYou can [`read`](crate::Reg::read) this register and get [`octl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`octl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@octl`]
module"]
pub type OCTL = crate::Reg<octl::OCTL_SPEC>;
#[doc = "Port output control register"]
pub mod octl;
#[doc = "BOP (w) register accessor: Port bit operate register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bop`]
module"]
pub type BOP = crate::Reg<bop::BOP_SPEC>;
#[doc = "Port bit operate register"]
pub mod bop;
#[doc = "BC (w) register accessor: Port bit clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bc`]
module"]
pub type BC = crate::Reg<bc::BC_SPEC>;
#[doc = "Port bit clear register"]
pub mod bc;
#[doc = "LOCK (rw) register accessor: GPIO port configuration lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`lock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`]
module"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "GPIO port configuration lock register"]
pub mod lock;
