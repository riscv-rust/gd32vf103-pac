#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - status register"]
    pub stat: STAT,
    #[doc = "0x04 - control register 0"]
    pub ctl0: CTL0,
    #[doc = "0x08 - control register 1"]
    pub ctl1: CTL1,
    #[doc = "0x0c - Sample time register 0"]
    pub sampt0: SAMPT0,
    #[doc = "0x10 - Sample time register 1"]
    pub sampt1: SAMPT1,
    #[doc = "0x14 - Inserted channel data offset register 0"]
    pub ioff0: IOFF0,
    #[doc = "0x18 - Inserted channel data offset register 1"]
    pub ioff1: IOFF1,
    #[doc = "0x1c - Inserted channel data offset register 2"]
    pub ioff2: IOFF2,
    #[doc = "0x20 - Inserted channel data offset register 3"]
    pub ioff3: IOFF3,
    #[doc = "0x24 - watchdog higher threshold register"]
    pub wdht: WDHT,
    #[doc = "0x28 - watchdog lower threshold register"]
    pub wdlt: WDLT,
    #[doc = "0x2c - regular sequence register 0"]
    pub rsq0: RSQ0,
    #[doc = "0x30 - regular sequence register 1"]
    pub rsq1: RSQ1,
    #[doc = "0x34 - regular sequence register 2"]
    pub rsq2: RSQ2,
    #[doc = "0x38 - Inserted sequence register"]
    pub isq: ISQ,
    #[doc = "0x3c - Inserted data register 0"]
    pub idata0: IDATA0,
    #[doc = "0x40 - Inserted data register 1"]
    pub idata1: IDATA1,
    #[doc = "0x44 - Inserted data register 2"]
    pub idata2: IDATA2,
    #[doc = "0x48 - Inserted data register 3"]
    pub idata3: IDATA3,
    #[doc = "0x4c - regular data register"]
    pub rdata: RDATA,
}
#[doc = "status register"]
pub struct STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "status register"]
pub mod stat;
#[doc = "control register 0"]
pub struct CTL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "control register 0"]
pub mod ctl0;
#[doc = "control register 1"]
pub struct CTL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "control register 1"]
pub mod ctl1;
#[doc = "Sample time register 0"]
pub struct SAMPT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sample time register 0"]
pub mod sampt0;
#[doc = "Sample time register 1"]
pub struct SAMPT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sample time register 1"]
pub mod sampt1;
#[doc = "Inserted channel data offset register 0"]
pub struct IOFF0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Inserted channel data offset register 0"]
pub mod ioff0;
#[doc = "Inserted channel data offset register 1"]
pub struct IOFF1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Inserted channel data offset register 1"]
pub mod ioff1;
#[doc = "Inserted channel data offset register 2"]
pub struct IOFF2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Inserted channel data offset register 2"]
pub mod ioff2;
#[doc = "Inserted channel data offset register 3"]
pub struct IOFF3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Inserted channel data offset register 3"]
pub mod ioff3;
#[doc = "watchdog higher threshold register"]
pub struct WDHT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "watchdog higher threshold register"]
pub mod wdht;
#[doc = "watchdog lower threshold register"]
pub struct WDLT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "watchdog lower threshold register"]
pub mod wdlt;
#[doc = "regular sequence register 0"]
pub struct RSQ0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "regular sequence register 0"]
pub mod rsq0;
#[doc = "regular sequence register 1"]
pub struct RSQ1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "regular sequence register 1"]
pub mod rsq1;
#[doc = "regular sequence register 2"]
pub struct RSQ2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "regular sequence register 2"]
pub mod rsq2;
#[doc = "Inserted sequence register"]
pub struct ISQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Inserted sequence register"]
pub mod isq;
#[doc = "Inserted data register 0"]
pub struct IDATA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Inserted data register 0"]
pub mod idata0;
#[doc = "Inserted data register 1"]
pub struct IDATA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Inserted data register 1"]
pub mod idata1;
#[doc = "Inserted data register 2"]
pub struct IDATA2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Inserted data register 2"]
pub mod idata2;
#[doc = "Inserted data register 3"]
pub struct IDATA3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Inserted data register 3"]
pub mod idata3;
#[doc = "regular data register"]
pub struct RDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "regular data register"]
pub mod rdata;
