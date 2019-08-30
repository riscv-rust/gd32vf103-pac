#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub ctl: CTL,
    #[doc = "0x04 - Clock configuration register 0 (RCU_CFG0)"]
    pub cfg0: CFG0,
    #[doc = "0x08 - Clock interrupt register (RCU_INT)"]
    pub int: INT,
    #[doc = "0x0c - APB2 reset register (RCU_APB2RST)"]
    pub apb2rst: APB2RST,
    #[doc = "0x10 - APB1 reset register (RCU_APB1RST)"]
    pub apb1rst: APB1RST,
    #[doc = "0x14 - AHB enable register"]
    pub ahben: AHBEN,
    #[doc = "0x18 - APB2 clock enable register (RCU_APB2EN)"]
    pub apb2en: APB2EN,
    #[doc = "0x1c - APB1 clock enable register (RCU_APB1EN)"]
    pub apb1en: APB1EN,
    #[doc = "0x20 - Backup domain control register (RCU_BDCTL)"]
    pub bdctl: BDCTL,
    #[doc = "0x24 - Reset source /clock register (RCU_RSTSCK)"]
    pub rstsck: RSTSCK,
    #[doc = "0x28 - AHB reset register"]
    pub ahbrst: AHBRST,
    #[doc = "0x2c - Clock Configuration register 1"]
    pub cfg1: CFG1,
    _reserved0: [u8; 4usize],
    #[doc = "0x34 - Deep sleep mode Voltage register"]
    pub dsv: DSV,
}
#[doc = "Control register"]
pub struct CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control register"]
pub mod ctl;
#[doc = "Clock configuration register 0 (RCU_CFG0)"]
pub struct CFG0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock configuration register 0 (RCU_CFG0)"]
pub mod cfg0;
#[doc = "Clock interrupt register (RCU_INT)"]
pub struct INT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock interrupt register (RCU_INT)"]
pub mod int;
#[doc = "APB2 reset register (RCU_APB2RST)"]
pub struct APB2RST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APB2 reset register (RCU_APB2RST)"]
pub mod apb2rst;
#[doc = "APB1 reset register (RCU_APB1RST)"]
pub struct APB1RST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APB1 reset register (RCU_APB1RST)"]
pub mod apb1rst;
#[doc = "AHB enable register"]
pub struct AHBEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB enable register"]
pub mod ahben;
#[doc = "APB2 clock enable register (RCU_APB2EN)"]
pub struct APB2EN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APB2 clock enable register (RCU_APB2EN)"]
pub mod apb2en;
#[doc = "APB1 clock enable register (RCU_APB1EN)"]
pub struct APB1EN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APB1 clock enable register (RCU_APB1EN)"]
pub mod apb1en;
#[doc = "Backup domain control register (RCU_BDCTL)"]
pub struct BDCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup domain control register (RCU_BDCTL)"]
pub mod bdctl;
#[doc = "Reset source /clock register (RCU_RSTSCK)"]
pub struct RSTSCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset source /clock register (RCU_RSTSCK)"]
pub mod rstsck;
#[doc = "AHB reset register"]
pub struct AHBRST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB reset register"]
pub mod ahbrst;
#[doc = "Clock Configuration register 1"]
pub struct CFG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Configuration register 1"]
pub mod cfg1;
#[doc = "Deep sleep mode Voltage register"]
pub struct DSV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Deep sleep mode Voltage register"]
pub mod dsv;
