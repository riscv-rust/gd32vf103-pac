#[doc = r"Register block"]
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
    _reserved12: [u8; 0x04],
    #[doc = "0x34 - Deep sleep mode Voltage register"]
    pub dsv: DSV,
}
#[doc = "CTL (rw) register accessor: Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control register"]
pub mod ctl;
#[doc = "CFG0 (rw) register accessor: Clock configuration register 0 (RCU_CFG0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0`]
module"]
pub type CFG0 = crate::Reg<cfg0::CFG0_SPEC>;
#[doc = "Clock configuration register 0 (RCU_CFG0)"]
pub mod cfg0;
#[doc = "INT (rw) register accessor: Clock interrupt register (RCU_INT)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int`]
module"]
pub type INT = crate::Reg<int::INT_SPEC>;
#[doc = "Clock interrupt register (RCU_INT)"]
pub mod int;
#[doc = "APB2RST (rw) register accessor: APB2 reset register (RCU_APB2RST)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2rst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2rst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2rst`]
module"]
pub type APB2RST = crate::Reg<apb2rst::APB2RST_SPEC>;
#[doc = "APB2 reset register (RCU_APB2RST)"]
pub mod apb2rst;
#[doc = "APB1RST (rw) register accessor: APB1 reset register (RCU_APB1RST)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1rst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1rst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1rst`]
module"]
pub type APB1RST = crate::Reg<apb1rst::APB1RST_SPEC>;
#[doc = "APB1 reset register (RCU_APB1RST)"]
pub mod apb1rst;
#[doc = "AHBEN (rw) register accessor: AHB enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahben::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahben::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahben`]
module"]
pub type AHBEN = crate::Reg<ahben::AHBEN_SPEC>;
#[doc = "AHB enable register"]
pub mod ahben;
#[doc = "APB2EN (rw) register accessor: APB2 clock enable register (RCU_APB2EN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2en`]
module"]
pub type APB2EN = crate::Reg<apb2en::APB2EN_SPEC>;
#[doc = "APB2 clock enable register (RCU_APB2EN)"]
pub mod apb2en;
#[doc = "APB1EN (rw) register accessor: APB1 clock enable register (RCU_APB1EN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1en`]
module"]
pub type APB1EN = crate::Reg<apb1en::APB1EN_SPEC>;
#[doc = "APB1 clock enable register (RCU_APB1EN)"]
pub mod apb1en;
#[doc = "BDCTL (rw) register accessor: Backup domain control register (RCU_BDCTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdctl`]
module"]
pub type BDCTL = crate::Reg<bdctl::BDCTL_SPEC>;
#[doc = "Backup domain control register (RCU_BDCTL)"]
pub mod bdctl;
#[doc = "RSTSCK (rw) register accessor: Reset source /clock register (RCU_RSTSCK)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstsck::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstsck::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstsck`]
module"]
pub type RSTSCK = crate::Reg<rstsck::RSTSCK_SPEC>;
#[doc = "Reset source /clock register (RCU_RSTSCK)"]
pub mod rstsck;
#[doc = "AHBRST (rw) register accessor: AHB reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbrst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbrst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbrst`]
module"]
pub type AHBRST = crate::Reg<ahbrst::AHBRST_SPEC>;
#[doc = "AHB reset register"]
pub mod ahbrst;
#[doc = "CFG1 (rw) register accessor: Clock Configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg1`]
module"]
pub type CFG1 = crate::Reg<cfg1::CFG1_SPEC>;
#[doc = "Clock Configuration register 1"]
pub mod cfg1;
#[doc = "DSV (rw) register accessor: Deep sleep mode Voltage register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsv`]
module"]
pub type DSV = crate::Reg<dsv::DSV_SPEC>;
#[doc = "Deep sleep mode Voltage register"]
pub mod dsv;
