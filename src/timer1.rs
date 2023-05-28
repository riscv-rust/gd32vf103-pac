#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register 0"]
    pub ctl0: CTL0,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - control register 1"]
    pub ctl1: CTL1,
    _reserved2: [u8; 0x02],
    #[doc = "0x08 - slave mode control register"]
    pub smcfg: SMCFG,
    _reserved3: [u8; 0x02],
    #[doc = "0x0c - DMA/Interrupt enable register"]
    pub dmainten: DMAINTEN,
    _reserved4: [u8; 0x02],
    #[doc = "0x10 - interrupt flag register"]
    pub intf: INTF,
    _reserved5: [u8; 0x02],
    #[doc = "0x14 - event generation register"]
    pub swevg: SWEVG,
    _reserved6: [u8; 0x02],
    _reserved_6_chctl0: [u8; 0x02],
    _reserved7: [u8; 0x02],
    _reserved_7_chctl1: [u8; 0x02],
    _reserved8: [u8; 0x02],
    #[doc = "0x20 - Channel control register 2"]
    pub chctl2: CHCTL2,
    _reserved9: [u8; 0x02],
    #[doc = "0x24 - Counter register"]
    pub cnt: CNT,
    _reserved10: [u8; 0x02],
    #[doc = "0x28 - Prescaler register"]
    pub psc: PSC,
    _reserved11: [u8; 0x02],
    #[doc = "0x2c - Counter auto reload register"]
    pub car: CAR,
    _reserved12: [u8; 0x06],
    #[doc = "0x34 - Channel 0 capture/compare value register"]
    pub ch0cv: CH0CV,
    _reserved13: [u8; 0x02],
    #[doc = "0x38 - Channel 1 capture/compare value register"]
    pub ch1cv: CH1CV,
    _reserved14: [u8; 0x02],
    #[doc = "0x3c - Channel 2 capture/compare value register"]
    pub ch2cv: CH2CV,
    _reserved15: [u8; 0x02],
    #[doc = "0x40 - Channel 3 capture/compare value register"]
    pub ch3cv: CH3CV,
    _reserved16: [u8; 0x06],
    #[doc = "0x48 - DMA configuration register"]
    pub dmacfg: DMACFG,
    _reserved17: [u8; 0x02],
    #[doc = "0x4c - DMA transfer buffer register"]
    pub dmatb: DMATB,
}
impl RegisterBlock {
    #[doc = "0x18 - Channel control register 0 (input mode)"]
    #[inline(always)]
    pub const fn chctl0_input(&self) -> &CHCTL0_INPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    #[doc = "0x18 - Channel control register 0 (output mode)"]
    #[inline(always)]
    pub const fn chctl0_output(&self) -> &CHCTL0_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    #[doc = "0x1c - Channel control register 1 (input mode)"]
    #[inline(always)]
    pub const fn chctl1_input(&self) -> &CHCTL1_INPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    #[doc = "0x1c - Channel control register 1 (output mode)"]
    #[inline(always)]
    pub const fn chctl1_output(&self) -> &CHCTL1_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
}
#[doc = "CTL0 (rw) register accessor: an alias for `Reg<CTL0_SPEC>`"]
pub type CTL0 = crate::Reg<ctl0::CTL0_SPEC>;
#[doc = "control register 0"]
pub mod ctl0;
#[doc = "CTL1 (rw) register accessor: an alias for `Reg<CTL1_SPEC>`"]
pub type CTL1 = crate::Reg<ctl1::CTL1_SPEC>;
#[doc = "control register 1"]
pub mod ctl1;
#[doc = "SMCFG (rw) register accessor: an alias for `Reg<SMCFG_SPEC>`"]
pub type SMCFG = crate::Reg<smcfg::SMCFG_SPEC>;
#[doc = "slave mode control register"]
pub mod smcfg;
#[doc = "DMAINTEN (rw) register accessor: an alias for `Reg<DMAINTEN_SPEC>`"]
pub type DMAINTEN = crate::Reg<dmainten::DMAINTEN_SPEC>;
#[doc = "DMA/Interrupt enable register"]
pub mod dmainten;
#[doc = "INTF (rw) register accessor: an alias for `Reg<INTF_SPEC>`"]
pub type INTF = crate::Reg<intf::INTF_SPEC>;
#[doc = "interrupt flag register"]
pub mod intf;
#[doc = "SWEVG (w) register accessor: an alias for `Reg<SWEVG_SPEC>`"]
pub type SWEVG = crate::Reg<swevg::SWEVG_SPEC>;
#[doc = "event generation register"]
pub mod swevg;
#[doc = "CHCTL0_Output (rw) register accessor: an alias for `Reg<CHCTL0_OUTPUT_SPEC>`"]
pub type CHCTL0_OUTPUT = crate::Reg<chctl0_output::CHCTL0_OUTPUT_SPEC>;
#[doc = "Channel control register 0 (output mode)"]
pub mod chctl0_output;
#[doc = "CHCTL0_Input (rw) register accessor: an alias for `Reg<CHCTL0_INPUT_SPEC>`"]
pub type CHCTL0_INPUT = crate::Reg<chctl0_input::CHCTL0_INPUT_SPEC>;
#[doc = "Channel control register 0 (input mode)"]
pub mod chctl0_input;
#[doc = "CHCTL1_Output (rw) register accessor: an alias for `Reg<CHCTL1_OUTPUT_SPEC>`"]
pub type CHCTL1_OUTPUT = crate::Reg<chctl1_output::CHCTL1_OUTPUT_SPEC>;
#[doc = "Channel control register 1 (output mode)"]
pub mod chctl1_output;
#[doc = "CHCTL1_Input (rw) register accessor: an alias for `Reg<CHCTL1_INPUT_SPEC>`"]
pub type CHCTL1_INPUT = crate::Reg<chctl1_input::CHCTL1_INPUT_SPEC>;
#[doc = "Channel control register 1 (input mode)"]
pub mod chctl1_input;
#[doc = "CHCTL2 (rw) register accessor: an alias for `Reg<CHCTL2_SPEC>`"]
pub type CHCTL2 = crate::Reg<chctl2::CHCTL2_SPEC>;
#[doc = "Channel control register 2"]
pub mod chctl2;
#[doc = "CNT (rw) register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "Counter register"]
pub mod cnt;
#[doc = "PSC (rw) register accessor: an alias for `Reg<PSC_SPEC>`"]
pub type PSC = crate::Reg<psc::PSC_SPEC>;
#[doc = "Prescaler register"]
pub mod psc;
#[doc = "CAR (rw) register accessor: an alias for `Reg<CAR_SPEC>`"]
pub type CAR = crate::Reg<car::CAR_SPEC>;
#[doc = "Counter auto reload register"]
pub mod car;
#[doc = "CH0CV (rw) register accessor: an alias for `Reg<CH0CV_SPEC>`"]
pub type CH0CV = crate::Reg<ch0cv::CH0CV_SPEC>;
#[doc = "Channel 0 capture/compare value register"]
pub mod ch0cv;
#[doc = "CH1CV (rw) register accessor: an alias for `Reg<CH1CV_SPEC>`"]
pub type CH1CV = crate::Reg<ch1cv::CH1CV_SPEC>;
#[doc = "Channel 1 capture/compare value register"]
pub mod ch1cv;
#[doc = "CH2CV (rw) register accessor: an alias for `Reg<CH2CV_SPEC>`"]
pub type CH2CV = crate::Reg<ch2cv::CH2CV_SPEC>;
#[doc = "Channel 2 capture/compare value register"]
pub mod ch2cv;
#[doc = "CH3CV (rw) register accessor: an alias for `Reg<CH3CV_SPEC>`"]
pub type CH3CV = crate::Reg<ch3cv::CH3CV_SPEC>;
#[doc = "Channel 3 capture/compare value register"]
pub mod ch3cv;
#[doc = "DMACFG (rw) register accessor: an alias for `Reg<DMACFG_SPEC>`"]
pub type DMACFG = crate::Reg<dmacfg::DMACFG_SPEC>;
#[doc = "DMA configuration register"]
pub mod dmacfg;
#[doc = "DMATB (rw) register accessor: an alias for `Reg<DMATB_SPEC>`"]
pub type DMATB = crate::Reg<dmatb::DMATB_SPEC>;
#[doc = "DMA transfer buffer register"]
pub mod dmatb;
