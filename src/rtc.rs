#[doc = r"Register block"]
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
#[doc = "INTEN (rw) register accessor: RTC interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "RTC interrupt enable register"]
pub mod inten;
#[doc = "CTL (rw) register accessor: control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "control register"]
pub mod ctl;
#[doc = "PSCH (w) register accessor: RTC prescaler high register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psch::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psch`]
module"]
pub type PSCH = crate::Reg<psch::PSCH_SPEC>;
#[doc = "RTC prescaler high register"]
pub mod psch;
#[doc = "PSCL (w) register accessor: RTC prescaler low register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pscl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pscl`]
module"]
pub type PSCL = crate::Reg<pscl::PSCL_SPEC>;
#[doc = "RTC prescaler low register"]
pub mod pscl;
#[doc = "DIVH (r) register accessor: RTC divider high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`divh::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@divh`]
module"]
pub type DIVH = crate::Reg<divh::DIVH_SPEC>;
#[doc = "RTC divider high register"]
pub mod divh;
#[doc = "DIVL (r) register accessor: RTC divider low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`divl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@divl`]
module"]
pub type DIVL = crate::Reg<divl::DIVL_SPEC>;
#[doc = "RTC divider low register"]
pub mod divl;
#[doc = "CNTH (rw) register accessor: RTC counter high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnth::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnth::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnth`]
module"]
pub type CNTH = crate::Reg<cnth::CNTH_SPEC>;
#[doc = "RTC counter high register"]
pub mod cnth;
#[doc = "CNTL (rw) register accessor: RTC counter low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntl`]
module"]
pub type CNTL = crate::Reg<cntl::CNTL_SPEC>;
#[doc = "RTC counter low register"]
pub mod cntl;
#[doc = "ALRMH (w) register accessor: Alarm high register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alrmh::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alrmh`]
module"]
pub type ALRMH = crate::Reg<alrmh::ALRMH_SPEC>;
#[doc = "Alarm high register"]
pub mod alrmh;
#[doc = "ALRML (w) register accessor: RTC alarm low register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alrml::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alrml`]
module"]
pub type ALRML = crate::Reg<alrml::ALRML_SPEC>;
#[doc = "RTC alarm low register"]
pub mod alrml;
