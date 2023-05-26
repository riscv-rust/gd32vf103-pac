#[doc = r"Register block"]
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
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "control register"]
pub mod ctl;
#[doc = "SWT (w) register accessor: an alias for `Reg<SWT_SPEC>`"]
pub type SWT = crate::Reg<swt::SWT_SPEC>;
#[doc = "software trigger register"]
pub mod swt;
#[doc = "DAC0_R12DH (rw) register accessor: an alias for `Reg<DAC0_R12DH_SPEC>`"]
pub type DAC0_R12DH = crate::Reg<dac0_r12dh::DAC0_R12DH_SPEC>;
#[doc = "DAC0 12-bit right-aligned data holding register"]
pub mod dac0_r12dh;
#[doc = "DAC0_L12DH (rw) register accessor: an alias for `Reg<DAC0_L12DH_SPEC>`"]
pub type DAC0_L12DH = crate::Reg<dac0_l12dh::DAC0_L12DH_SPEC>;
#[doc = "DAC0 12-bit left-aligned data holding register"]
pub mod dac0_l12dh;
#[doc = "DAC0_R8DH (rw) register accessor: an alias for `Reg<DAC0_R8DH_SPEC>`"]
pub type DAC0_R8DH = crate::Reg<dac0_r8dh::DAC0_R8DH_SPEC>;
#[doc = "DAC0 8-bit right aligned data holding register"]
pub mod dac0_r8dh;
#[doc = "DAC1_R12DH (rw) register accessor: an alias for `Reg<DAC1_R12DH_SPEC>`"]
pub type DAC1_R12DH = crate::Reg<dac1_r12dh::DAC1_R12DH_SPEC>;
#[doc = "DAC1 12-bit right-aligned data holding register"]
pub mod dac1_r12dh;
#[doc = "DAC1_L12DH (rw) register accessor: an alias for `Reg<DAC1_L12DH_SPEC>`"]
pub type DAC1_L12DH = crate::Reg<dac1_l12dh::DAC1_L12DH_SPEC>;
#[doc = "DAC1 12-bit left aligned data holding register"]
pub mod dac1_l12dh;
#[doc = "DAC1_R8DH (rw) register accessor: an alias for `Reg<DAC1_R8DH_SPEC>`"]
pub type DAC1_R8DH = crate::Reg<dac1_r8dh::DAC1_R8DH_SPEC>;
#[doc = "DAC1 8-bit right aligned data holding register"]
pub mod dac1_r8dh;
#[doc = "DACC_R12DH (rw) register accessor: an alias for `Reg<DACC_R12DH_SPEC>`"]
pub type DACC_R12DH = crate::Reg<dacc_r12dh::DACC_R12DH_SPEC>;
#[doc = "DAC concurrent mode 12-bit right-aligned data holding register"]
pub mod dacc_r12dh;
#[doc = "DACC_L12DH (rw) register accessor: an alias for `Reg<DACC_L12DH_SPEC>`"]
pub type DACC_L12DH = crate::Reg<dacc_l12dh::DACC_L12DH_SPEC>;
#[doc = "DAC concurrent mode 12-bit left aligned data holding register"]
pub mod dacc_l12dh;
#[doc = "DACC_R8DH (rw) register accessor: an alias for `Reg<DACC_R8DH_SPEC>`"]
pub type DACC_R8DH = crate::Reg<dacc_r8dh::DACC_R8DH_SPEC>;
#[doc = "DAC concurrent mode 8-bit right aligned data holding register"]
pub mod dacc_r8dh;
#[doc = "DAC0_DO (r) register accessor: an alias for `Reg<DAC0_DO_SPEC>`"]
pub type DAC0_DO = crate::Reg<dac0_do::DAC0_DO_SPEC>;
#[doc = "DAC0 data output register"]
pub mod dac0_do;
#[doc = "DAC1_DO (r) register accessor: an alias for `Reg<DAC1_DO_SPEC>`"]
pub type DAC1_DO = crate::Reg<dac1_do::DAC1_DO_SPEC>;
#[doc = "DAC1 data output register"]
pub mod dac1_do;
