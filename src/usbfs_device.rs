#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - device configuration register (DCFG)"]
    pub dcfg: DCFG,
    #[doc = "0x04 - device control register (DCTL)"]
    pub dctl: DCTL,
    #[doc = "0x08 - device status register (DSTAT)"]
    pub dstat: DSTAT,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - device IN endpoint common interrupt mask register (DIEPINTEN)"]
    pub diepinten: DIEPINTEN,
    #[doc = "0x14 - device OUT endpoint common interrupt enable register (DOEPINTEN)"]
    pub doepinten: DOEPINTEN,
    #[doc = "0x18 - device all endpoints interrupt register (DAEPINT)"]
    pub daepint: DAEPINT,
    #[doc = "0x1c - Device all endpoints interrupt enable register (DAEPINTEN)"]
    pub daepinten: DAEPINTEN,
    _reserved1: [u8; 8usize],
    #[doc = "0x28 - device VBUS discharge time register"]
    pub dvbusdt: DVBUSDT,
    #[doc = "0x2c - device VBUS pulsing time register"]
    pub dvbuspt: DVBUSPT,
    _reserved2: [u8; 4usize],
    #[doc = "0x34 - device IN endpoint FIFO empty interrupt enable register"]
    pub diepfeinten: DIEPFEINTEN,
    _reserved3: [u8; 200usize],
    #[doc = "0x100 - device IN endpoint 0 control register (DIEP0CTL)"]
    pub diep0ctl: DIEP0CTL,
    _reserved4: [u8; 4usize],
    #[doc = "0x108 - device endpoint-0 interrupt register"]
    pub diep0intf: DIEP0INTF,
    _reserved5: [u8; 4usize],
    #[doc = "0x110 - device IN endpoint-0 transfer length register"]
    pub diep0len: DIEP0LEN,
    _reserved6: [u8; 4usize],
    #[doc = "0x118 - device IN endpoint 0 transmit FIFO status register"]
    pub diep0tfstat: DIEP0TFSTAT,
    _reserved7: [u8; 4usize],
    #[doc = "0x120 - device in endpoint-1 control register"]
    pub diep1ctl: DIEP1CTL,
    _reserved8: [u8; 4usize],
    #[doc = "0x128 - device endpoint-1 interrupt register"]
    pub diep1intf: DIEP1INTF,
    _reserved9: [u8; 4usize],
    #[doc = "0x130 - device IN endpoint-1 transfer length register"]
    pub diep1len: DIEP1LEN,
    _reserved10: [u8; 4usize],
    #[doc = "0x138 - device IN endpoint 1 transmit FIFO status register"]
    pub diep1tfstat: DIEP1TFSTAT,
    _reserved11: [u8; 4usize],
    #[doc = "0x140 - device endpoint-2 control register"]
    pub diep2ctl: DIEP2CTL,
    _reserved12: [u8; 4usize],
    #[doc = "0x148 - device endpoint-2 interrupt register"]
    pub diep2intf: DIEP2INTF,
    _reserved13: [u8; 4usize],
    #[doc = "0x150 - device IN endpoint-2 transfer length register"]
    pub diep2len: DIEP2LEN,
    _reserved14: [u8; 4usize],
    #[doc = "0x158 - device IN endpoint 2 transmit FIFO status register"]
    pub diep2tfstat: DIEP2TFSTAT,
    _reserved15: [u8; 4usize],
    #[doc = "0x160 - device endpoint-3 control register"]
    pub diep3ctl: DIEP3CTL,
    _reserved16: [u8; 4usize],
    #[doc = "0x168 - device endpoint-3 interrupt register"]
    pub diep3intf: DIEP3INTF,
    _reserved17: [u8; 4usize],
    #[doc = "0x170 - device IN endpoint-3 transfer length register"]
    pub diep3len: DIEP3LEN,
    _reserved18: [u8; 4usize],
    #[doc = "0x178 - device IN endpoint 3 transmit FIFO status register"]
    pub diep3tfstat: DIEP3TFSTAT,
    _reserved19: [u8; 388usize],
    #[doc = "0x300 - device endpoint-0 control register"]
    pub doep0ctl: DOEP0CTL,
    _reserved20: [u8; 4usize],
    #[doc = "0x308 - device out endpoint-0 interrupt flag register"]
    pub doep0intf: DOEP0INTF,
    _reserved21: [u8; 4usize],
    #[doc = "0x310 - device OUT endpoint-0 transfer length register"]
    pub doep0len: DOEP0LEN,
    _reserved22: [u8; 12usize],
    #[doc = "0x320 - device endpoint-1 control register"]
    pub doep1ctl: DOEP1CTL,
    _reserved23: [u8; 4usize],
    #[doc = "0x328 - device out endpoint-1 interrupt flag register"]
    pub doep1intf: DOEP1INTF,
    _reserved24: [u8; 4usize],
    #[doc = "0x330 - device OUT endpoint-1 transfer length register"]
    pub doep1len: DOEP1LEN,
    _reserved25: [u8; 12usize],
    #[doc = "0x340 - device endpoint-2 control register"]
    pub doep2ctl: DOEP2CTL,
    _reserved26: [u8; 4usize],
    #[doc = "0x348 - device out endpoint-2 interrupt flag register"]
    pub doep2intf: DOEP2INTF,
    _reserved27: [u8; 4usize],
    #[doc = "0x350 - device OUT endpoint-2 transfer length register"]
    pub doep2len: DOEP2LEN,
    _reserved28: [u8; 12usize],
    #[doc = "0x360 - device endpoint-3 control register"]
    pub doep3ctl: DOEP3CTL,
    _reserved29: [u8; 4usize],
    #[doc = "0x368 - device out endpoint-3 interrupt flag register"]
    pub doep3intf: DOEP3INTF,
    _reserved30: [u8; 4usize],
    #[doc = "0x370 - device OUT endpoint-3 transfer length register"]
    pub doep3len: DOEP3LEN,
}
#[doc = "device configuration register (DCFG)"]
pub struct DCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device configuration register (DCFG)"]
pub mod dcfg;
#[doc = "device control register (DCTL)"]
pub struct DCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device control register (DCTL)"]
pub mod dctl;
#[doc = "device status register (DSTAT)"]
pub struct DSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device status register (DSTAT)"]
pub mod dstat;
#[doc = "device IN endpoint common interrupt mask register (DIEPINTEN)"]
pub struct DIEPINTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device IN endpoint common interrupt mask register (DIEPINTEN)"]
pub mod diepinten;
#[doc = "device OUT endpoint common interrupt enable register (DOEPINTEN)"]
pub struct DOEPINTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device OUT endpoint common interrupt enable register (DOEPINTEN)"]
pub mod doepinten;
#[doc = "device all endpoints interrupt register (DAEPINT)"]
pub struct DAEPINT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device all endpoints interrupt register (DAEPINT)"]
pub mod daepint;
#[doc = "Device all endpoints interrupt enable register (DAEPINTEN)"]
pub struct DAEPINTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device all endpoints interrupt enable register (DAEPINTEN)"]
pub mod daepinten;
#[doc = "device VBUS discharge time register"]
pub struct DVBUSDT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device VBUS discharge time register"]
pub mod dvbusdt;
#[doc = "device VBUS pulsing time register"]
pub struct DVBUSPT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device VBUS pulsing time register"]
pub mod dvbuspt;
#[doc = "device IN endpoint FIFO empty interrupt enable register"]
pub struct DIEPFEINTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device IN endpoint FIFO empty interrupt enable register"]
pub mod diepfeinten;
#[doc = "device IN endpoint 0 control register (DIEP0CTL)"]
pub struct DIEP0CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device IN endpoint 0 control register (DIEP0CTL)"]
pub mod diep0ctl;
#[doc = "device in endpoint-1 control register"]
pub struct DIEP1CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device in endpoint-1 control register"]
pub mod diep1ctl;
#[doc = "device endpoint-2 control register"]
pub struct DIEP2CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device endpoint-2 control register"]
pub mod diep2ctl;
#[doc = "device endpoint-3 control register"]
pub struct DIEP3CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device endpoint-3 control register"]
pub mod diep3ctl;
#[doc = "device endpoint-0 control register"]
pub struct DOEP0CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device endpoint-0 control register"]
pub mod doep0ctl;
#[doc = "device endpoint-1 control register"]
pub struct DOEP1CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device endpoint-1 control register"]
pub mod doep1ctl;
#[doc = "device endpoint-2 control register"]
pub struct DOEP2CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device endpoint-2 control register"]
pub mod doep2ctl;
#[doc = "device endpoint-3 control register"]
pub struct DOEP3CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device endpoint-3 control register"]
pub mod doep3ctl;
#[doc = "device endpoint-0 interrupt register"]
pub struct DIEP0INTF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device endpoint-0 interrupt register"]
pub mod diep0intf;
#[doc = "device endpoint-1 interrupt register"]
pub struct DIEP1INTF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device endpoint-1 interrupt register"]
pub mod diep1intf;
#[doc = "device endpoint-2 interrupt register"]
pub struct DIEP2INTF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device endpoint-2 interrupt register"]
pub mod diep2intf;
#[doc = "device endpoint-3 interrupt register"]
pub struct DIEP3INTF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device endpoint-3 interrupt register"]
pub mod diep3intf;
#[doc = "device out endpoint-0 interrupt flag register"]
pub struct DOEP0INTF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device out endpoint-0 interrupt flag register"]
pub mod doep0intf;
#[doc = "device out endpoint-1 interrupt flag register"]
pub struct DOEP1INTF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device out endpoint-1 interrupt flag register"]
pub mod doep1intf;
#[doc = "device out endpoint-2 interrupt flag register"]
pub struct DOEP2INTF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device out endpoint-2 interrupt flag register"]
pub mod doep2intf;
#[doc = "device out endpoint-3 interrupt flag register"]
pub struct DOEP3INTF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device out endpoint-3 interrupt flag register"]
pub mod doep3intf;
#[doc = "device IN endpoint-0 transfer length register"]
pub struct DIEP0LEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device IN endpoint-0 transfer length register"]
pub mod diep0len;
#[doc = "device OUT endpoint-0 transfer length register"]
pub struct DOEP0LEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device OUT endpoint-0 transfer length register"]
pub mod doep0len;
#[doc = "device IN endpoint-1 transfer length register"]
pub struct DIEP1LEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device IN endpoint-1 transfer length register"]
pub mod diep1len;
#[doc = "device IN endpoint-2 transfer length register"]
pub struct DIEP2LEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device IN endpoint-2 transfer length register"]
pub mod diep2len;
#[doc = "device IN endpoint-3 transfer length register"]
pub struct DIEP3LEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device IN endpoint-3 transfer length register"]
pub mod diep3len;
#[doc = "device OUT endpoint-1 transfer length register"]
pub struct DOEP1LEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device OUT endpoint-1 transfer length register"]
pub mod doep1len;
#[doc = "device OUT endpoint-2 transfer length register"]
pub struct DOEP2LEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device OUT endpoint-2 transfer length register"]
pub mod doep2len;
#[doc = "device OUT endpoint-3 transfer length register"]
pub struct DOEP3LEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device OUT endpoint-3 transfer length register"]
pub mod doep3len;
#[doc = "device IN endpoint 0 transmit FIFO status register"]
pub struct DIEP0TFSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device IN endpoint 0 transmit FIFO status register"]
pub mod diep0tfstat;
#[doc = "device IN endpoint 1 transmit FIFO status register"]
pub struct DIEP1TFSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device IN endpoint 1 transmit FIFO status register"]
pub mod diep1tfstat;
#[doc = "device IN endpoint 2 transmit FIFO status register"]
pub struct DIEP2TFSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device IN endpoint 2 transmit FIFO status register"]
pub mod diep2tfstat;
#[doc = "device IN endpoint 3 transmit FIFO status register"]
pub struct DIEP3TFSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device IN endpoint 3 transmit FIFO status register"]
pub mod diep3tfstat;
