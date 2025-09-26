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
#[doc = "CTL (rw) register accessor: control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "control register"]
pub mod ctl;
#[doc = "SWT (w) register accessor: software trigger register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swt`]
module"]
pub type SWT = crate::Reg<swt::SWT_SPEC>;
#[doc = "software trigger register"]
pub mod swt;
#[doc = "DAC0_R12DH (rw) register accessor: DAC0 12-bit right-aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac0_r12dh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac0_r12dh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac0_r12dh`]
module"]
pub type DAC0_R12DH = crate::Reg<dac0_r12dh::DAC0_R12DH_SPEC>;
#[doc = "DAC0 12-bit right-aligned data holding register"]
pub mod dac0_r12dh;
#[doc = "DAC0_L12DH (rw) register accessor: DAC0 12-bit left-aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac0_l12dh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac0_l12dh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac0_l12dh`]
module"]
pub type DAC0_L12DH = crate::Reg<dac0_l12dh::DAC0_L12DH_SPEC>;
#[doc = "DAC0 12-bit left-aligned data holding register"]
pub mod dac0_l12dh;
#[doc = "DAC0_R8DH (rw) register accessor: DAC0 8-bit right aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac0_r8dh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac0_r8dh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac0_r8dh`]
module"]
pub type DAC0_R8DH = crate::Reg<dac0_r8dh::DAC0_R8DH_SPEC>;
#[doc = "DAC0 8-bit right aligned data holding register"]
pub mod dac0_r8dh;
#[doc = "DAC1_R12DH (rw) register accessor: DAC1 12-bit right-aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac1_r12dh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac1_r12dh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac1_r12dh`]
module"]
pub type DAC1_R12DH = crate::Reg<dac1_r12dh::DAC1_R12DH_SPEC>;
#[doc = "DAC1 12-bit right-aligned data holding register"]
pub mod dac1_r12dh;
#[doc = "DAC1_L12DH (rw) register accessor: DAC1 12-bit left aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac1_l12dh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac1_l12dh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac1_l12dh`]
module"]
pub type DAC1_L12DH = crate::Reg<dac1_l12dh::DAC1_L12DH_SPEC>;
#[doc = "DAC1 12-bit left aligned data holding register"]
pub mod dac1_l12dh;
#[doc = "DAC1_R8DH (rw) register accessor: DAC1 8-bit right aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac1_r8dh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac1_r8dh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac1_r8dh`]
module"]
pub type DAC1_R8DH = crate::Reg<dac1_r8dh::DAC1_R8DH_SPEC>;
#[doc = "DAC1 8-bit right aligned data holding register"]
pub mod dac1_r8dh;
#[doc = "DACC_R12DH (rw) register accessor: DAC concurrent mode 12-bit right-aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dacc_r12dh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dacc_r12dh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dacc_r12dh`]
module"]
pub type DACC_R12DH = crate::Reg<dacc_r12dh::DACC_R12DH_SPEC>;
#[doc = "DAC concurrent mode 12-bit right-aligned data holding register"]
pub mod dacc_r12dh;
#[doc = "DACC_L12DH (rw) register accessor: DAC concurrent mode 12-bit left aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dacc_l12dh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dacc_l12dh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dacc_l12dh`]
module"]
pub type DACC_L12DH = crate::Reg<dacc_l12dh::DACC_L12DH_SPEC>;
#[doc = "DAC concurrent mode 12-bit left aligned data holding register"]
pub mod dacc_l12dh;
#[doc = "DACC_R8DH (rw) register accessor: DAC concurrent mode 8-bit right aligned data holding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dacc_r8dh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dacc_r8dh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dacc_r8dh`]
module"]
pub type DACC_R8DH = crate::Reg<dacc_r8dh::DACC_R8DH_SPEC>;
#[doc = "DAC concurrent mode 8-bit right aligned data holding register"]
pub mod dacc_r8dh;
#[doc = "DAC0_DO (r) register accessor: DAC0 data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac0_do::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac0_do`]
module"]
pub type DAC0_DO = crate::Reg<dac0_do::DAC0_DO_SPEC>;
#[doc = "DAC0 data output register"]
pub mod dac0_do;
#[doc = "DAC1_DO (r) register accessor: DAC1 data output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac1_do::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac1_do`]
module"]
pub type DAC1_DO = crate::Reg<dac1_do::DAC1_DO_SPEC>;
#[doc = "DAC1 data output register"]
pub mod dac1_do;
