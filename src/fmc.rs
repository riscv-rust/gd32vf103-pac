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
#[doc = "WS (rw) register accessor: an alias for `Reg<WS_SPEC>`"]
pub type WS = crate::Reg<ws::WS_SPEC>;
#[doc = "wait state counter register"]
pub mod ws;
#[doc = "KEY0 (w) register accessor: an alias for `Reg<KEY0_SPEC>`"]
pub type KEY0 = crate::Reg<key0::KEY0_SPEC>;
#[doc = "Unlock key register 0"]
pub mod key0;
#[doc = "OBKEY (w) register accessor: an alias for `Reg<OBKEY_SPEC>`"]
pub type OBKEY = crate::Reg<obkey::OBKEY_SPEC>;
#[doc = "Option byte unlock key register"]
pub mod obkey;
#[doc = "STAT0 (rw) register accessor: an alias for `Reg<STAT0_SPEC>`"]
pub type STAT0 = crate::Reg<stat0::STAT0_SPEC>;
#[doc = "Status register 0"]
pub mod stat0;
#[doc = "CTL0 (rw) register accessor: an alias for `Reg<CTL0_SPEC>`"]
pub type CTL0 = crate::Reg<ctl0::CTL0_SPEC>;
#[doc = "Control register 0"]
pub mod ctl0;
#[doc = "ADDR0 (w) register accessor: an alias for `Reg<ADDR0_SPEC>`"]
pub type ADDR0 = crate::Reg<addr0::ADDR0_SPEC>;
#[doc = "Address register 0"]
pub mod addr0;
#[doc = "OBSTAT (r) register accessor: an alias for `Reg<OBSTAT_SPEC>`"]
pub type OBSTAT = crate::Reg<obstat::OBSTAT_SPEC>;
#[doc = "Option byte status register"]
pub mod obstat;
#[doc = "WP (r) register accessor: an alias for `Reg<WP_SPEC>`"]
pub type WP = crate::Reg<wp::WP_SPEC>;
#[doc = "Erase/Program Protection register"]
pub mod wp;
#[doc = "PID (r) register accessor: an alias for `Reg<PID_SPEC>`"]
pub type PID = crate::Reg<pid::PID_SPEC>;
#[doc = "Product ID register"]
pub mod pid;
