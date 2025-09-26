#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctl0: CTL0,
    _reserved1: [u8; 0x02],
    ctl1: CTL1,
    _reserved2: [u8; 0x02],
    smcfg: SMCFG,
    _reserved3: [u8; 0x02],
    dmainten: DMAINTEN,
    _reserved4: [u8; 0x02],
    intf: INTF,
    _reserved5: [u8; 0x02],
    swevg: SWEVG,
    _reserved6: [u8; 0x02],
    _reserved_6_chctl0: [u8; 0x02],
    _reserved7: [u8; 0x02],
    _reserved_7_chctl1: [u8; 0x02],
    _reserved8: [u8; 0x02],
    chctl2: CHCTL2,
    _reserved9: [u8; 0x02],
    cnt: CNT,
    _reserved10: [u8; 0x02],
    psc: PSC,
    _reserved11: [u8; 0x02],
    car: CAR,
    _reserved12: [u8; 0x02],
    crep: CREP,
    _reserved13: [u8; 0x02],
    ch0cv: CH0CV,
    _reserved14: [u8; 0x02],
    ch1cv: CH1CV,
    _reserved15: [u8; 0x02],
    ch2cv: CH2CV,
    _reserved16: [u8; 0x02],
    ch3cv: CH3CV,
    _reserved17: [u8; 0x02],
    cchp: CCHP,
    _reserved18: [u8; 0x02],
    dmacfg: DMACFG,
    _reserved19: [u8; 0x02],
    dmatb: DMATB,
}
impl RegisterBlock {
    #[doc = "0x00 - control register 0"]
    #[inline(always)]
    pub const fn ctl0(&self) -> &CTL0 {
        &self.ctl0
    }
    #[doc = "0x04 - control register 1"]
    #[inline(always)]
    pub const fn ctl1(&self) -> &CTL1 {
        &self.ctl1
    }
    #[doc = "0x08 - slave mode configuration register"]
    #[inline(always)]
    pub const fn smcfg(&self) -> &SMCFG {
        &self.smcfg
    }
    #[doc = "0x0c - DMA/Interrupt enable register"]
    #[inline(always)]
    pub const fn dmainten(&self) -> &DMAINTEN {
        &self.dmainten
    }
    #[doc = "0x10 - Interrupt flag register"]
    #[inline(always)]
    pub const fn intf(&self) -> &INTF {
        &self.intf
    }
    #[doc = "0x14 - Software event generation register"]
    #[inline(always)]
    pub const fn swevg(&self) -> &SWEVG {
        &self.swevg
    }
    #[doc = "0x18 - Channel control register 0 (input mode)"]
    #[inline(always)]
    pub const fn chctl0_input(&self) -> &CHCTL0_INPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x18 - Channel control register 0 (output mode)"]
    #[inline(always)]
    pub const fn chctl0_output(&self) -> &CHCTL0_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x1c - Channel control register 1 (input mode)"]
    #[inline(always)]
    pub const fn chctl1_input(&self) -> &CHCTL1_INPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x1c - Channel control register 1 (output mode)"]
    #[inline(always)]
    pub const fn chctl1_output(&self) -> &CHCTL1_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x20 - Channel control register 2"]
    #[inline(always)]
    pub const fn chctl2(&self) -> &CHCTL2 {
        &self.chctl2
    }
    #[doc = "0x24 - counter"]
    #[inline(always)]
    pub const fn cnt(&self) -> &CNT {
        &self.cnt
    }
    #[doc = "0x28 - prescaler"]
    #[inline(always)]
    pub const fn psc(&self) -> &PSC {
        &self.psc
    }
    #[doc = "0x2c - Counter auto reload register"]
    #[inline(always)]
    pub const fn car(&self) -> &CAR {
        &self.car
    }
    #[doc = "0x30 - Counter repetition register"]
    #[inline(always)]
    pub const fn crep(&self) -> &CREP {
        &self.crep
    }
    #[doc = "0x34 - Channel 0 capture/compare value register"]
    #[inline(always)]
    pub const fn ch0cv(&self) -> &CH0CV {
        &self.ch0cv
    }
    #[doc = "0x38 - Channel 1 capture/compare value register"]
    #[inline(always)]
    pub const fn ch1cv(&self) -> &CH1CV {
        &self.ch1cv
    }
    #[doc = "0x3c - Channel 2 capture/compare value register"]
    #[inline(always)]
    pub const fn ch2cv(&self) -> &CH2CV {
        &self.ch2cv
    }
    #[doc = "0x40 - Channel 3 capture/compare value register"]
    #[inline(always)]
    pub const fn ch3cv(&self) -> &CH3CV {
        &self.ch3cv
    }
    #[doc = "0x44 - channel complementary protection register"]
    #[inline(always)]
    pub const fn cchp(&self) -> &CCHP {
        &self.cchp
    }
    #[doc = "0x48 - DMA configuration register"]
    #[inline(always)]
    pub const fn dmacfg(&self) -> &DMACFG {
        &self.dmacfg
    }
    #[doc = "0x4c - DMA transfer buffer register"]
    #[inline(always)]
    pub const fn dmatb(&self) -> &DMATB {
        &self.dmatb
    }
}
#[doc = "CTL0 (rw) register accessor: control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl0`]
module"]
pub type CTL0 = crate::Reg<ctl0::CTL0_SPEC>;
#[doc = "control register 0"]
pub mod ctl0;
#[doc = "CTL1 (rw) register accessor: control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl1`]
module"]
pub type CTL1 = crate::Reg<ctl1::CTL1_SPEC>;
#[doc = "control register 1"]
pub mod ctl1;
#[doc = "SMCFG (rw) register accessor: slave mode configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`smcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smcfg`]
module"]
pub type SMCFG = crate::Reg<smcfg::SMCFG_SPEC>;
#[doc = "slave mode configuration register"]
pub mod smcfg;
#[doc = "DMAINTEN (rw) register accessor: DMA/Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmainten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmainten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmainten`]
module"]
pub type DMAINTEN = crate::Reg<dmainten::DMAINTEN_SPEC>;
#[doc = "DMA/Interrupt enable register"]
pub mod dmainten;
#[doc = "INTF (rw) register accessor: Interrupt flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`intf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf`]
module"]
pub type INTF = crate::Reg<intf::INTF_SPEC>;
#[doc = "Interrupt flag register"]
pub mod intf;
#[doc = "SWEVG (w) register accessor: Software event generation register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swevg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swevg`]
module"]
pub type SWEVG = crate::Reg<swevg::SWEVG_SPEC>;
#[doc = "Software event generation register"]
pub mod swevg;
#[doc = "CHCTL0_Output (rw) register accessor: Channel control register 0 (output mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`chctl0_output::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chctl0_output::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chctl0_output`]
module"]
#[doc(alias = "CHCTL0_Output")]
pub type CHCTL0_OUTPUT = crate::Reg<chctl0_output::CHCTL0_OUTPUT_SPEC>;
#[doc = "Channel control register 0 (output mode)"]
pub mod chctl0_output;
#[doc = "CHCTL0_Input (rw) register accessor: Channel control register 0 (input mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`chctl0_input::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chctl0_input::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chctl0_input`]
module"]
#[doc(alias = "CHCTL0_Input")]
pub type CHCTL0_INPUT = crate::Reg<chctl0_input::CHCTL0_INPUT_SPEC>;
#[doc = "Channel control register 0 (input mode)"]
pub mod chctl0_input;
#[doc = "CHCTL1_Output (rw) register accessor: Channel control register 1 (output mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`chctl1_output::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chctl1_output::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chctl1_output`]
module"]
#[doc(alias = "CHCTL1_Output")]
pub type CHCTL1_OUTPUT = crate::Reg<chctl1_output::CHCTL1_OUTPUT_SPEC>;
#[doc = "Channel control register 1 (output mode)"]
pub mod chctl1_output;
#[doc = "CHCTL1_Input (rw) register accessor: Channel control register 1 (input mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`chctl1_input::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chctl1_input::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chctl1_input`]
module"]
#[doc(alias = "CHCTL1_Input")]
pub type CHCTL1_INPUT = crate::Reg<chctl1_input::CHCTL1_INPUT_SPEC>;
#[doc = "Channel control register 1 (input mode)"]
pub mod chctl1_input;
#[doc = "CHCTL2 (rw) register accessor: Channel control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`chctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chctl2`]
module"]
pub type CHCTL2 = crate::Reg<chctl2::CHCTL2_SPEC>;
#[doc = "Channel control register 2"]
pub mod chctl2;
#[doc = "CNT (rw) register accessor: counter\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`]
module"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "counter"]
pub mod cnt;
#[doc = "PSC (rw) register accessor: prescaler\n\nYou can [`read`](crate::Reg::read) this register and get [`psc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psc`]
module"]
pub type PSC = crate::Reg<psc::PSC_SPEC>;
#[doc = "prescaler"]
pub mod psc;
#[doc = "CAR (rw) register accessor: Counter auto reload register\n\nYou can [`read`](crate::Reg::read) this register and get [`car::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`car::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@car`]
module"]
pub type CAR = crate::Reg<car::CAR_SPEC>;
#[doc = "Counter auto reload register"]
pub mod car;
#[doc = "CREP (rw) register accessor: Counter repetition register\n\nYou can [`read`](crate::Reg::read) this register and get [`crep::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crep::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crep`]
module"]
pub type CREP = crate::Reg<crep::CREP_SPEC>;
#[doc = "Counter repetition register"]
pub mod crep;
#[doc = "CH0CV (rw) register accessor: Channel 0 capture/compare value register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0cv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0cv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0cv`]
module"]
pub type CH0CV = crate::Reg<ch0cv::CH0CV_SPEC>;
#[doc = "Channel 0 capture/compare value register"]
pub mod ch0cv;
#[doc = "CH1CV (rw) register accessor: Channel 1 capture/compare value register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1cv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1cv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1cv`]
module"]
pub type CH1CV = crate::Reg<ch1cv::CH1CV_SPEC>;
#[doc = "Channel 1 capture/compare value register"]
pub mod ch1cv;
#[doc = "CH2CV (rw) register accessor: Channel 2 capture/compare value register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2cv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2cv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2cv`]
module"]
pub type CH2CV = crate::Reg<ch2cv::CH2CV_SPEC>;
#[doc = "Channel 2 capture/compare value register"]
pub mod ch2cv;
#[doc = "CH3CV (rw) register accessor: Channel 3 capture/compare value register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3cv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3cv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3cv`]
module"]
pub type CH3CV = crate::Reg<ch3cv::CH3CV_SPEC>;
#[doc = "Channel 3 capture/compare value register"]
pub mod ch3cv;
#[doc = "CCHP (rw) register accessor: channel complementary protection register\n\nYou can [`read`](crate::Reg::read) this register and get [`cchp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cchp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cchp`]
module"]
pub type CCHP = crate::Reg<cchp::CCHP_SPEC>;
#[doc = "channel complementary protection register"]
pub mod cchp;
#[doc = "DMACFG (rw) register accessor: DMA configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacfg`]
module"]
pub type DMACFG = crate::Reg<dmacfg::DMACFG_SPEC>;
#[doc = "DMA configuration register"]
pub mod dmacfg;
#[doc = "DMATB (rw) register accessor: DMA transfer buffer register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmatb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmatb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmatb`]
module"]
pub type DMATB = crate::Reg<dmatb::DMATB_SPEC>;
#[doc = "DMA transfer buffer register"]
pub mod dmatb;
