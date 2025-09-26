#[doc = r"Register block"]
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
#[doc = "STAT (rw) register accessor: status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "status register"]
pub mod stat;
#[doc = "CTL0 (rw) register accessor: control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl0`]
module"]
pub type CTL0 = crate::Reg<ctl0::CTL0_SPEC>;
#[doc = "control register 0"]
pub mod ctl0;
#[doc = "CTL1 (rw) register accessor: control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl1`]
module"]
pub type CTL1 = crate::Reg<ctl1::CTL1_SPEC>;
#[doc = "control register 1"]
pub mod ctl1;
#[doc = "SAMPT0 (rw) register accessor: Sample time register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sampt0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sampt0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sampt0`]
module"]
pub type SAMPT0 = crate::Reg<sampt0::SAMPT0_SPEC>;
#[doc = "Sample time register 0"]
pub mod sampt0;
#[doc = "SAMPT1 (rw) register accessor: Sample time register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sampt1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sampt1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sampt1`]
module"]
pub type SAMPT1 = crate::Reg<sampt1::SAMPT1_SPEC>;
#[doc = "Sample time register 1"]
pub mod sampt1;
#[doc = "IOFF0 (rw) register accessor: Inserted channel data offset register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioff0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioff0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioff0`]
module"]
pub type IOFF0 = crate::Reg<ioff0::IOFF0_SPEC>;
#[doc = "Inserted channel data offset register 0"]
pub mod ioff0;
#[doc = "IOFF1 (rw) register accessor: Inserted channel data offset register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioff1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioff1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioff1`]
module"]
pub type IOFF1 = crate::Reg<ioff1::IOFF1_SPEC>;
#[doc = "Inserted channel data offset register 1"]
pub mod ioff1;
#[doc = "IOFF2 (rw) register accessor: Inserted channel data offset register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioff2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioff2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioff2`]
module"]
pub type IOFF2 = crate::Reg<ioff2::IOFF2_SPEC>;
#[doc = "Inserted channel data offset register 2"]
pub mod ioff2;
#[doc = "IOFF3 (rw) register accessor: Inserted channel data offset register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioff3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioff3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioff3`]
module"]
pub type IOFF3 = crate::Reg<ioff3::IOFF3_SPEC>;
#[doc = "Inserted channel data offset register 3"]
pub mod ioff3;
#[doc = "WDHT (rw) register accessor: watchdog higher threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdht::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdht::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdht`]
module"]
pub type WDHT = crate::Reg<wdht::WDHT_SPEC>;
#[doc = "watchdog higher threshold register"]
pub mod wdht;
#[doc = "WDLT (rw) register accessor: watchdog lower threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdlt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdlt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdlt`]
module"]
pub type WDLT = crate::Reg<wdlt::WDLT_SPEC>;
#[doc = "watchdog lower threshold register"]
pub mod wdlt;
#[doc = "RSQ0 (rw) register accessor: regular sequence register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsq0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsq0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsq0`]
module"]
pub type RSQ0 = crate::Reg<rsq0::RSQ0_SPEC>;
#[doc = "regular sequence register 0"]
pub mod rsq0;
#[doc = "RSQ1 (rw) register accessor: regular sequence register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsq1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsq1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsq1`]
module"]
pub type RSQ1 = crate::Reg<rsq1::RSQ1_SPEC>;
#[doc = "regular sequence register 1"]
pub mod rsq1;
#[doc = "RSQ2 (rw) register accessor: regular sequence register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsq2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsq2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsq2`]
module"]
pub type RSQ2 = crate::Reg<rsq2::RSQ2_SPEC>;
#[doc = "regular sequence register 2"]
pub mod rsq2;
#[doc = "ISQ (rw) register accessor: Inserted sequence register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isq`]
module"]
pub type ISQ = crate::Reg<isq::ISQ_SPEC>;
#[doc = "Inserted sequence register"]
pub mod isq;
#[doc = "IDATA0 (r) register accessor: Inserted data register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idata0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idata0`]
module"]
pub type IDATA0 = crate::Reg<idata0::IDATA0_SPEC>;
#[doc = "Inserted data register 0"]
pub mod idata0;
#[doc = "IDATA1 (r) register accessor: Inserted data register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idata1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idata1`]
module"]
pub type IDATA1 = crate::Reg<idata1::IDATA1_SPEC>;
#[doc = "Inserted data register 1"]
pub mod idata1;
#[doc = "IDATA2 (r) register accessor: Inserted data register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idata2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idata2`]
module"]
pub type IDATA2 = crate::Reg<idata2::IDATA2_SPEC>;
#[doc = "Inserted data register 2"]
pub mod idata2;
#[doc = "IDATA3 (r) register accessor: Inserted data register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idata3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idata3`]
module"]
pub type IDATA3 = crate::Reg<idata3::IDATA3_SPEC>;
#[doc = "Inserted data register 3"]
pub mod idata3;
#[doc = "RDATA (r) register accessor: regular data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdata::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdata`]
module"]
pub type RDATA = crate::Reg<rdata::RDATA_SPEC>;
#[doc = "regular data register"]
pub mod rdata;
