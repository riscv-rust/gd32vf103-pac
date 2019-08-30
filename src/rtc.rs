#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC interrupt enable register"]
    pub inten: INTEN,
    #[doc = "0x04 - control register"]
    pub ctl: CTL,
    #[doc = "0x08 - RTC prescaler high register"]
    pub psch: PSCH,
    #[doc = "0x0c - RTC prescaler low register"]
    pub pscl: PSCL,
    #[doc = "0x10 - RTC divider high register"]
    pub divh: DIVH,
    #[doc = "0x14 - RTC divider low register"]
    pub divl: DIVL,
    #[doc = "0x18 - RTC counter high register"]
    pub cnth: CNTH,
    #[doc = "0x1c - RTC counter low register"]
    pub cntl: CNTL,
    #[doc = "0x20 - Alarm high register"]
    pub alrmh: ALRMH,
    #[doc = "0x24 - RTC alarm low register"]
    pub alrml: ALRML,
}
#[doc = "RTC interrupt enable register"]
pub struct INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC interrupt enable register"]
pub mod inten;
#[doc = "control register"]
pub struct CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "control register"]
pub mod ctl;
#[doc = "RTC prescaler high register"]
pub struct PSCH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC prescaler high register"]
pub mod psch;
#[doc = "RTC prescaler low register"]
pub struct PSCL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC prescaler low register"]
pub mod pscl;
#[doc = "RTC divider high register"]
pub struct DIVH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC divider high register"]
pub mod divh;
#[doc = "RTC divider low register"]
pub struct DIVL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC divider low register"]
pub mod divl;
#[doc = "RTC counter high register"]
pub struct CNTH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC counter high register"]
pub mod cnth;
#[doc = "RTC counter low register"]
pub struct CNTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC counter low register"]
pub mod cntl;
#[doc = "Alarm high register"]
pub struct ALRMH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Alarm high register"]
pub mod alrmh;
#[doc = "RTC alarm low register"]
pub struct ALRML {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC alarm low register"]
pub mod alrml;
