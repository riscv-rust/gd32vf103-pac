#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SRAM/NOR flash control register 0"]
    pub snctl0: SNCTL0,
    #[doc = "0x04 - SRAM/NOR flash timing configuration register 0"]
    pub sntcfg0: SNTCFG0,
    #[doc = "0x08 - SRAM/NOR flash control register 1"]
    pub snctl1: SNCTL1,
}
#[doc = "SRAM/NOR flash control register 0"]
pub struct SNCTL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM/NOR flash control register 0"]
pub mod snctl0;
#[doc = "SRAM/NOR flash timing configuration register 0"]
pub struct SNTCFG0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM/NOR flash timing configuration register 0"]
pub mod sntcfg0;
#[doc = "SRAM/NOR flash control register 1"]
pub struct SNCTL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM/NOR flash control register 1"]
pub mod snctl1;
