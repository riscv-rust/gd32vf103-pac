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
#[doc = "CTL0 (rw) register accessor: an alias for `Reg<CTL0_SPEC>`"]
pub type CTL0 = crate::Reg<ctl0::CTL0_SPEC>;
#[doc = "port control register 0"]
pub mod ctl0;
#[doc = "CTL1 (rw) register accessor: an alias for `Reg<CTL1_SPEC>`"]
pub type CTL1 = crate::Reg<ctl1::CTL1_SPEC>;
#[doc = "port control register 1"]
pub mod ctl1;
#[doc = "ISTAT (r) register accessor: an alias for `Reg<ISTAT_SPEC>`"]
pub type ISTAT = crate::Reg<istat::ISTAT_SPEC>;
#[doc = "Port input status register"]
pub mod istat;
#[doc = "OCTL (rw) register accessor: an alias for `Reg<OCTL_SPEC>`"]
pub type OCTL = crate::Reg<octl::OCTL_SPEC>;
#[doc = "Port output control register"]
pub mod octl;
#[doc = "BOP (w) register accessor: an alias for `Reg<BOP_SPEC>`"]
pub type BOP = crate::Reg<bop::BOP_SPEC>;
#[doc = "Port bit operate register"]
pub mod bop;
#[doc = "BC (w) register accessor: an alias for `Reg<BC_SPEC>`"]
pub type BC = crate::Reg<bc::BC_SPEC>;
#[doc = "Port bit clear register"]
pub mod bc;
#[doc = "LOCK (rw) register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "GPIO port configuration lock register"]
pub mod lock;
