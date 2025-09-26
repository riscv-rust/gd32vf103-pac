#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - port control register 0"]
    pub ctl0: CTL0,
    #[doc = "0x04 - port control register 1"]
    pub ctl1: CTL1,
    #[doc = "0x08 - Port input status register"]
    pub istat: ISTAT,
    #[doc = "0x0c - Port output control register"]
    pub octl: OCTL,
    #[doc = "0x10 - Port bit operate register"]
    pub bop: BOP,
    #[doc = "0x14 - Port bit clear register"]
    pub bc: BC,
    #[doc = "0x18 - GPIO port configuration lock register"]
    pub lock: LOCK,
}
#[doc = "CTL0 (rw) register accessor: port control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl0`]
module"]
pub type CTL0 = crate::Reg<ctl0::CTL0_SPEC>;
#[doc = "port control register 0"]
pub mod ctl0;
#[doc = "CTL1 (rw) register accessor: port control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl1`]
module"]
pub type CTL1 = crate::Reg<ctl1::CTL1_SPEC>;
#[doc = "port control register 1"]
pub mod ctl1;
#[doc = "ISTAT (r) register accessor: Port input status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`istat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@istat`]
module"]
pub type ISTAT = crate::Reg<istat::ISTAT_SPEC>;
#[doc = "Port input status register"]
pub mod istat;
#[doc = "OCTL (rw) register accessor: Port output control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`octl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`octl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@octl`]
module"]
pub type OCTL = crate::Reg<octl::OCTL_SPEC>;
#[doc = "Port output control register"]
pub mod octl;
#[doc = "BOP (w) register accessor: Port bit operate register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bop`]
module"]
pub type BOP = crate::Reg<bop::BOP_SPEC>;
#[doc = "Port bit operate register"]
pub mod bop;
#[doc = "BC (w) register accessor: Port bit clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bc`]
module"]
pub type BC = crate::Reg<bc::BC_SPEC>;
#[doc = "Port bit clear register"]
pub mod bc;
#[doc = "LOCK (rw) register accessor: GPIO port configuration lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`]
module"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "GPIO port configuration lock register"]
pub mod lock;
