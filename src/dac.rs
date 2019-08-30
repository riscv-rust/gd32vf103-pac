#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub ctl: CTL,
    #[doc = "0x04 - software trigger register"]
    pub swt: SWT,
    #[doc = "0x08 - DAC0 12-bit right-aligned data holding register"]
    pub dac0_r12dh: DAC0_R12DH,
    #[doc = "0x0c - DAC0 12-bit left-aligned data holding register"]
    pub dac0_l12dh: DAC0_L12DH,
    #[doc = "0x10 - DAC0 8-bit right aligned data holding register"]
    pub dac0_r8dh: DAC0_R8DH,
    #[doc = "0x14 - DAC1 12-bit right-aligned data holding register"]
    pub dac1_r12dh: DAC1_R12DH,
    #[doc = "0x18 - DAC1 12-bit left aligned data holding register"]
    pub dac1_l12dh: DAC1_L12DH,
    #[doc = "0x1c - DAC1 8-bit right aligned data holding register"]
    pub dac1_r8dh: DAC1_R8DH,
    #[doc = "0x20 - DAC concurrent mode 12-bit right-aligned data holding register"]
    pub dacc_r12dh: DACC_R12DH,
    #[doc = "0x24 - DAC concurrent mode 12-bit left aligned data holding register"]
    pub dacc_l12dh: DACC_L12DH,
    #[doc = "0x28 - DAC concurrent mode 8-bit right aligned data holding register"]
    pub dacc_r8dh: DACC_R8DH,
    #[doc = "0x2c - DAC0 data output register"]
    pub dac0_do: DAC0_DO,
    #[doc = "0x30 - DAC1 data output register"]
    pub dac1_do: DAC1_DO,
}
#[doc = "control register"]
pub struct CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "control register"]
pub mod ctl;
#[doc = "software trigger register"]
pub struct SWT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "software trigger register"]
pub mod swt;
#[doc = "DAC0 12-bit right-aligned data holding register"]
pub struct DAC0_R12DH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DAC0 12-bit right-aligned data holding register"]
pub mod dac0_r12dh;
#[doc = "DAC0 12-bit left-aligned data holding register"]
pub struct DAC0_L12DH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DAC0 12-bit left-aligned data holding register"]
pub mod dac0_l12dh;
#[doc = "DAC0 8-bit right aligned data holding register"]
pub struct DAC0_R8DH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DAC0 8-bit right aligned data holding register"]
pub mod dac0_r8dh;
#[doc = "DAC1 12-bit right-aligned data holding register"]
pub struct DAC1_R12DH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DAC1 12-bit right-aligned data holding register"]
pub mod dac1_r12dh;
#[doc = "DAC1 12-bit left aligned data holding register"]
pub struct DAC1_L12DH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DAC1 12-bit left aligned data holding register"]
pub mod dac1_l12dh;
#[doc = "DAC1 8-bit right aligned data holding register"]
pub struct DAC1_R8DH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DAC1 8-bit right aligned data holding register"]
pub mod dac1_r8dh;
#[doc = "DAC concurrent mode 12-bit right-aligned data holding register"]
pub struct DACC_R12DH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DAC concurrent mode 12-bit right-aligned data holding register"]
pub mod dacc_r12dh;
#[doc = "DAC concurrent mode 12-bit left aligned data holding register"]
pub struct DACC_L12DH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DAC concurrent mode 12-bit left aligned data holding register"]
pub mod dacc_l12dh;
#[doc = "DAC concurrent mode 8-bit right aligned data holding register"]
pub struct DACC_R8DH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DAC concurrent mode 8-bit right aligned data holding register"]
pub mod dacc_r8dh;
#[doc = "DAC0 data output register"]
pub struct DAC0_DO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DAC0 data output register"]
pub mod dac0_do;
#[doc = "DAC1 data output register"]
pub struct DAC1_DO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DAC1 data output register"]
pub mod dac1_do;
