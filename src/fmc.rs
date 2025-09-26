#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - wait state counter register"]
    pub ws: WS,
    #[doc = "0x04 - Unlock key register 0"]
    pub key0: KEY0,
    #[doc = "0x08 - Option byte unlock key register"]
    pub obkey: OBKEY,
    #[doc = "0x0c - Status register 0"]
    pub stat0: STAT0,
    #[doc = "0x10 - Control register 0"]
    pub ctl0: CTL0,
    #[doc = "0x14 - Address register 0"]
    pub addr0: ADDR0,
    _reserved6: [u8; 0x04],
    #[doc = "0x1c - Option byte status register"]
    pub obstat: OBSTAT,
    #[doc = "0x20 - Erase/Program Protection register"]
    pub wp: WP,
    _reserved8: [u8; 0xdc],
    #[doc = "0x100 - Product ID register"]
    pub pid: PID,
}
#[doc = "WS (rw) register accessor: wait state counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ws::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ws::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ws`]
module"]
pub type WS = crate::Reg<ws::WS_SPEC>;
#[doc = "wait state counter register"]
pub mod ws;
#[doc = "KEY0 (w) register accessor: Unlock key register 0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key0`]
module"]
pub type KEY0 = crate::Reg<key0::KEY0_SPEC>;
#[doc = "Unlock key register 0"]
pub mod key0;
#[doc = "OBKEY (w) register accessor: Option byte unlock key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`obkey::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obkey`]
module"]
pub type OBKEY = crate::Reg<obkey::OBKEY_SPEC>;
#[doc = "Option byte unlock key register"]
pub mod obkey;
#[doc = "STAT0 (rw) register accessor: Status register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat0`]
module"]
pub type STAT0 = crate::Reg<stat0::STAT0_SPEC>;
#[doc = "Status register 0"]
pub mod stat0;
#[doc = "CTL0 (rw) register accessor: Control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl0`]
module"]
pub type CTL0 = crate::Reg<ctl0::CTL0_SPEC>;
#[doc = "Control register 0"]
pub mod ctl0;
#[doc = "ADDR0 (w) register accessor: Address register 0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr0`]
module"]
pub type ADDR0 = crate::Reg<addr0::ADDR0_SPEC>;
#[doc = "Address register 0"]
pub mod addr0;
#[doc = "OBSTAT (r) register accessor: Option byte status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`obstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obstat`]
module"]
pub type OBSTAT = crate::Reg<obstat::OBSTAT_SPEC>;
#[doc = "Option byte status register"]
pub mod obstat;
#[doc = "WP (r) register accessor: Erase/Program Protection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wp`]
module"]
pub type WP = crate::Reg<wp::WP_SPEC>;
#[doc = "Erase/Program Protection register"]
pub mod wp;
#[doc = "PID (r) register accessor: Product ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pid::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pid`]
module"]
pub type PID = crate::Reg<pid::PID_SPEC>;
#[doc = "Product ID register"]
pub mod pid;
