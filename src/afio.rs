#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Event control register"]
    pub ec: EC,
    #[doc = "0x04 - AFIO port configuration register 0"]
    pub pcf0: PCF0,
    #[doc = "0x08 - EXTI sources selection register 0"]
    pub extiss0: EXTISS0,
    #[doc = "0x0c - EXTI sources selection register 1"]
    pub extiss1: EXTISS1,
    #[doc = "0x10 - EXTI sources selection register 2"]
    pub extiss2: EXTISS2,
    #[doc = "0x14 - EXTI sources selection register 3"]
    pub extiss3: EXTISS3,
    _reserved0: [u8; 4usize],
    #[doc = "0x1c - AFIO port configuration register 1"]
    pub pcf1: PCF1,
}
#[doc = "Event control register"]
pub struct EC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event control register"]
pub mod ec;
#[doc = "AFIO port configuration register 0"]
pub struct PCF0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AFIO port configuration register 0"]
pub mod pcf0;
#[doc = "EXTI sources selection register 0"]
pub struct EXTISS0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EXTI sources selection register 0"]
pub mod extiss0;
#[doc = "EXTI sources selection register 1"]
pub struct EXTISS1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EXTI sources selection register 1"]
pub mod extiss1;
#[doc = "EXTI sources selection register 2"]
pub struct EXTISS2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EXTI sources selection register 2"]
pub mod extiss2;
#[doc = "EXTI sources selection register 3"]
pub struct EXTISS3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EXTI sources selection register 3"]
pub mod extiss3;
#[doc = "AFIO port configuration register 1"]
pub struct PCF1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AFIO port configuration register 1"]
pub mod pcf1;
