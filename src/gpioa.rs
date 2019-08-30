#[doc = r" Register block"]
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
#[doc = "port control register 0"]
pub struct CTL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "port control register 0"]
pub mod ctl0;
#[doc = "port control register 1"]
pub struct CTL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "port control register 1"]
pub mod ctl1;
#[doc = "Port input status register"]
pub struct ISTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port input status register"]
pub mod istat;
#[doc = "Port output control register"]
pub struct OCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port output control register"]
pub mod octl;
#[doc = "Port bit operate register"]
pub struct BOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port bit operate register"]
pub mod bop;
#[doc = "Port bit clear register"]
pub struct BC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port bit clear register"]
pub mod bc;
#[doc = "GPIO port configuration lock register"]
pub struct LOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO port configuration lock register"]
pub mod lock;
