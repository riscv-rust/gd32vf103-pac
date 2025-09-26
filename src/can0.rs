#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub ctl: CTL,
    #[doc = "0x04 - Status register"]
    pub stat: STAT,
    #[doc = "0x08 - Transmit status register"]
    pub tstat: TSTAT,
    #[doc = "0x0c - Receive message FIFO0 register"]
    pub rfifo0: RFIFO0,
    #[doc = "0x10 - Receive message FIFO1 register"]
    pub rfifo1: RFIFO1,
    #[doc = "0x14 - Interrupt enable register"]
    pub inten: INTEN,
    #[doc = "0x18 - Error register"]
    pub err: ERR,
    #[doc = "0x1c - Bit timing register"]
    pub bt: BT,
    _reserved8: [u8; 0x0160],
    #[doc = "0x180 - Transmit mailbox identifier register 0"]
    pub tmi0: TMI0,
    #[doc = "0x184 - Transmit mailbox property register 0"]
    pub tmp0: TMP0,
    #[doc = "0x188 - Transmit mailbox data0 register"]
    pub tmdata00: TMDATA00,
    #[doc = "0x18c - Transmit mailbox data1 register"]
    pub tmdata10: TMDATA10,
    #[doc = "0x190 - Transmit mailbox identifier register 1"]
    pub tmi1: TMI1,
    #[doc = "0x194 - Transmit mailbox property register 1"]
    pub tmp1: TMP1,
    #[doc = "0x198 - Transmit mailbox data0 register"]
    pub tmdata01: TMDATA01,
    #[doc = "0x19c - Transmit mailbox data1 register"]
    pub tmdata11: TMDATA11,
    #[doc = "0x1a0 - Transmit mailbox identifier register 2"]
    pub tmi2: TMI2,
    #[doc = "0x1a4 - Transmit mailbox property register 2"]
    pub tmp2: TMP2,
    #[doc = "0x1a8 - Transmit mailbox data0 register"]
    pub tmdata02: TMDATA02,
    #[doc = "0x1ac - Transmit mailbox data1 register"]
    pub tmdata12: TMDATA12,
    #[doc = "0x1b0 - Receive FIFO mailbox identifier register"]
    pub rfifomi0: RFIFOMI0,
    #[doc = "0x1b4 - Receive FIFO0 mailbox property register"]
    pub rfifomp0: RFIFOMP0,
    #[doc = "0x1b8 - Receive FIFO0 mailbox data0 register"]
    pub rfifomdata00: RFIFOMDATA00,
    #[doc = "0x1bc - Receive FIFO0 mailbox data1 register"]
    pub rfifomdata10: RFIFOMDATA10,
    #[doc = "0x1c0 - Receive FIFO1 mailbox identifier register"]
    pub rfifomi1: RFIFOMI1,
    #[doc = "0x1c4 - Receive FIFO1 mailbox property register"]
    pub rfifomp1: RFIFOMP1,
    #[doc = "0x1c8 - Receive FIFO1 mailbox data0 register"]
    pub rfifomdata01: RFIFOMDATA01,
    #[doc = "0x1cc - Receive FIFO1 mailbox data1 register"]
    pub rfifomdata11: RFIFOMDATA11,
    _reserved28: [u8; 0x30],
    #[doc = "0x200 - Filter control register"]
    pub fctl: FCTL,
    #[doc = "0x204 - Filter mode configuration register"]
    pub fmcfg: FMCFG,
    _reserved30: [u8; 0x04],
    #[doc = "0x20c - Filter scale configuration register"]
    pub fscfg: FSCFG,
    _reserved31: [u8; 0x04],
    #[doc = "0x214 - Filter associated FIFO register"]
    pub fafifo: FAFIFO,
    _reserved32: [u8; 0x04],
    #[doc = "0x21c - Filter working register"]
    pub fw: FW,
    _reserved33: [u8; 0x20],
    #[doc = "0x240 - Filter 0 data 0 register"]
    pub f0data0: F0DATA0,
    #[doc = "0x244 - Filter 0 data 1 register"]
    pub f0data1: F0DATA1,
    #[doc = "0x248 - Filter 1 data 0 register"]
    pub f1data0: F1DATA0,
    #[doc = "0x24c - Filter 1 data 1 register"]
    pub f1data1: F1DATA1,
    #[doc = "0x250 - Filter 2 data 0 register"]
    pub f2data0: F2DATA0,
    #[doc = "0x254 - Filter 2 data 1 register"]
    pub f2data1: F2DATA1,
    #[doc = "0x258 - Filter 3 data 0 register"]
    pub f3data0: F3DATA0,
    #[doc = "0x25c - Filter 3 data 1 register"]
    pub f3data1: F3DATA1,
    #[doc = "0x260 - Filter 4 data 0 register"]
    pub f4data0: F4DATA0,
    #[doc = "0x264 - Filter 4 data 1 register"]
    pub f4data1: F4DATA1,
    #[doc = "0x268 - Filter 5 data 0 register"]
    pub f5data0: F5DATA0,
    #[doc = "0x26c - Filter 5 data 1 register"]
    pub f5data1: F5DATA1,
    #[doc = "0x270 - Filter 6 data 0 register"]
    pub f6data0: F6DATA0,
    #[doc = "0x274 - Filter 6 data 1 register"]
    pub f6data1: F6DATA1,
    #[doc = "0x278 - Filter 7 data 0 register"]
    pub f7data0: F7DATA0,
    #[doc = "0x27c - Filter 7 data 1 register"]
    pub f7data1: F7DATA1,
    #[doc = "0x280 - Filter 8 data 0 register"]
    pub f8data0: F8DATA0,
    #[doc = "0x284 - Filter 8 data 1 register"]
    pub f8data1: F8DATA1,
    #[doc = "0x288 - Filter 9 data 0 register"]
    pub f9data0: F9DATA0,
    #[doc = "0x28c - Filter 9 data 1 register"]
    pub f9data1: F9DATA1,
    #[doc = "0x290 - Filter 10 data 0 register"]
    pub f10data0: F10DATA0,
    #[doc = "0x294 - Filter 10 data 1 register"]
    pub f10data1: F10DATA1,
    #[doc = "0x298 - Filter 11 data 0 register"]
    pub f11data0: F11DATA0,
    #[doc = "0x29c - Filter 11 data 1 register"]
    pub f11data1: F11DATA1,
    #[doc = "0x2a0 - Filter 12 data 0 register"]
    pub f12data0: F12DATA0,
    #[doc = "0x2a4 - Filter 12 data 1 register"]
    pub f12data1: F12DATA1,
    #[doc = "0x2a8 - Filter 13 data 0 register"]
    pub f13data0: F13DATA0,
    #[doc = "0x2ac - Filter 13 data 1 register"]
    pub f13data1: F13DATA1,
    #[doc = "0x2b0 - Filter 14 data 0 register"]
    pub f14data0: F14DATA0,
    #[doc = "0x2b4 - Filter 14 data 1 register"]
    pub f14data1: F14DATA1,
    #[doc = "0x2b8 - Filter 15 data 0 register"]
    pub f15data0: F15DATA0,
    #[doc = "0x2bc - Filter 15 data 1 register"]
    pub f15data1: F15DATA1,
    #[doc = "0x2c0 - Filter 16 data 0 register"]
    pub f16data0: F16DATA0,
    #[doc = "0x2c4 - Filter 16 data 1 register"]
    pub f16data1: F16DATA1,
    #[doc = "0x2c8 - Filter 17 data 0 register"]
    pub f17data0: F17DATA0,
    #[doc = "0x2cc - Filter 17 data 1 register"]
    pub f17data1: F17DATA1,
    #[doc = "0x2d0 - Filter 18 data 0 register"]
    pub f18data0: F18DATA0,
    #[doc = "0x2d4 - Filter 18 data 1 register"]
    pub f18data1: F18DATA1,
    #[doc = "0x2d8 - Filter 19 data 0 register"]
    pub f19data0: F19DATA0,
    #[doc = "0x2dc - Filter 19 data 1 register"]
    pub f19data1: F19DATA1,
    #[doc = "0x2e0 - Filter 20 data 0 register"]
    pub f20data0: F20DATA0,
    #[doc = "0x2e4 - Filter 20 data 1 register"]
    pub f20data1: F20DATA1,
    #[doc = "0x2e8 - Filter 21 data 0 register"]
    pub f21data0: F21DATA0,
    #[doc = "0x2ec - Filter 21 data 1 register"]
    pub f21data1: F21DATA1,
    #[doc = "0x2f0 - Filter 22 data 0 register"]
    pub f22data0: F22DATA0,
    #[doc = "0x2f4 - Filter 22 data 1 register"]
    pub f22data1: F22DATA1,
    #[doc = "0x2f8 - Filter 23 data 0 register"]
    pub f23data0: F23DATA0,
    #[doc = "0x2fc - Filter 23 data 1 register"]
    pub f23data1: F23DATA1,
    #[doc = "0x300 - Filter 24 data 0 register"]
    pub f24data0: F24DATA0,
    #[doc = "0x304 - Filter 24 data 1 register"]
    pub f24data1: F24DATA1,
    #[doc = "0x308 - Filter 25 data 0 register"]
    pub f25data0: F25DATA0,
    #[doc = "0x30c - Filter 25 data 1 register"]
    pub f25data1: F25DATA1,
    #[doc = "0x310 - Filter 26 data 0 register"]
    pub f26data0: F26DATA0,
    #[doc = "0x314 - Filter 26 data 1 register"]
    pub f26data1: F26DATA1,
    #[doc = "0x318 - Filter 27 data 0 register"]
    pub f27data0: F27DATA0,
    #[doc = "0x31c - Filter 27 data 1 register"]
    pub f27data1: F27DATA1,
}
#[doc = "CTL (rw) register accessor: Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control register"]
pub mod ctl;
#[doc = "STAT (rw) register accessor: Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Status register"]
pub mod stat;
#[doc = "TSTAT (rw) register accessor: Transmit status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tstat`]
module"]
pub type TSTAT = crate::Reg<tstat::TSTAT_SPEC>;
#[doc = "Transmit status register"]
pub mod tstat;
#[doc = "RFIFO0 (rw) register accessor: Receive message FIFO0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfifo0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfifo0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfifo0`]
module"]
pub type RFIFO0 = crate::Reg<rfifo0::RFIFO0_SPEC>;
#[doc = "Receive message FIFO0 register"]
pub mod rfifo0;
#[doc = "RFIFO1 (rw) register accessor: Receive message FIFO1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfifo1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfifo1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfifo1`]
module"]
pub type RFIFO1 = crate::Reg<rfifo1::RFIFO1_SPEC>;
#[doc = "Receive message FIFO1 register"]
pub mod rfifo1;
#[doc = "INTEN (rw) register accessor: Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Interrupt enable register"]
pub mod inten;
#[doc = "ERR (rw) register accessor: Error register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`err::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err`]
module"]
pub type ERR = crate::Reg<err::ERR_SPEC>;
#[doc = "Error register"]
pub mod err;
#[doc = "BT (rw) register accessor: Bit timing register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bt`]
module"]
pub type BT = crate::Reg<bt::BT_SPEC>;
#[doc = "Bit timing register"]
pub mod bt;
#[doc = "TMI0 (rw) register accessor: Transmit mailbox identifier register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmi0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmi0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmi0`]
module"]
pub type TMI0 = crate::Reg<tmi0::TMI0_SPEC>;
#[doc = "Transmit mailbox identifier register 0"]
pub mod tmi0;
#[doc = "TMP0 (rw) register accessor: Transmit mailbox property register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmp0`]
module"]
pub type TMP0 = crate::Reg<tmp0::TMP0_SPEC>;
#[doc = "Transmit mailbox property register 0"]
pub mod tmp0;
#[doc = "TMDATA00 (rw) register accessor: Transmit mailbox data0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmdata00::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmdata00::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmdata00`]
module"]
pub type TMDATA00 = crate::Reg<tmdata00::TMDATA00_SPEC>;
#[doc = "Transmit mailbox data0 register"]
pub mod tmdata00;
#[doc = "TMDATA10 (rw) register accessor: Transmit mailbox data1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmdata10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmdata10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmdata10`]
module"]
pub type TMDATA10 = crate::Reg<tmdata10::TMDATA10_SPEC>;
#[doc = "Transmit mailbox data1 register"]
pub mod tmdata10;
#[doc = "TMI1 (rw) register accessor: Transmit mailbox identifier register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmi1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmi1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmi1`]
module"]
pub type TMI1 = crate::Reg<tmi1::TMI1_SPEC>;
#[doc = "Transmit mailbox identifier register 1"]
pub mod tmi1;
#[doc = "TMP1 (rw) register accessor: Transmit mailbox property register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmp1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmp1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmp1`]
module"]
pub type TMP1 = crate::Reg<tmp1::TMP1_SPEC>;
#[doc = "Transmit mailbox property register 1"]
pub mod tmp1;
#[doc = "TMDATA01 (rw) register accessor: Transmit mailbox data0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmdata01::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmdata01::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmdata01`]
module"]
pub type TMDATA01 = crate::Reg<tmdata01::TMDATA01_SPEC>;
#[doc = "Transmit mailbox data0 register"]
pub mod tmdata01;
#[doc = "TMDATA11 (rw) register accessor: Transmit mailbox data1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmdata11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmdata11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmdata11`]
module"]
pub type TMDATA11 = crate::Reg<tmdata11::TMDATA11_SPEC>;
#[doc = "Transmit mailbox data1 register"]
pub mod tmdata11;
#[doc = "TMI2 (rw) register accessor: Transmit mailbox identifier register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmi2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmi2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmi2`]
module"]
pub type TMI2 = crate::Reg<tmi2::TMI2_SPEC>;
#[doc = "Transmit mailbox identifier register 2"]
pub mod tmi2;
#[doc = "TMP2 (rw) register accessor: Transmit mailbox property register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmp2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmp2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmp2`]
module"]
pub type TMP2 = crate::Reg<tmp2::TMP2_SPEC>;
#[doc = "Transmit mailbox property register 2"]
pub mod tmp2;
#[doc = "TMDATA02 (rw) register accessor: Transmit mailbox data0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmdata02::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmdata02::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmdata02`]
module"]
pub type TMDATA02 = crate::Reg<tmdata02::TMDATA02_SPEC>;
#[doc = "Transmit mailbox data0 register"]
pub mod tmdata02;
#[doc = "TMDATA12 (rw) register accessor: Transmit mailbox data1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmdata12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmdata12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmdata12`]
module"]
pub type TMDATA12 = crate::Reg<tmdata12::TMDATA12_SPEC>;
#[doc = "Transmit mailbox data1 register"]
pub mod tmdata12;
#[doc = "RFIFOMI0 (r) register accessor: Receive FIFO mailbox identifier register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfifomi0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfifomi0`]
module"]
pub type RFIFOMI0 = crate::Reg<rfifomi0::RFIFOMI0_SPEC>;
#[doc = "Receive FIFO mailbox identifier register"]
pub mod rfifomi0;
#[doc = "RFIFOMP0 (r) register accessor: Receive FIFO0 mailbox property register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfifomp0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfifomp0`]
module"]
pub type RFIFOMP0 = crate::Reg<rfifomp0::RFIFOMP0_SPEC>;
#[doc = "Receive FIFO0 mailbox property register"]
pub mod rfifomp0;
#[doc = "RFIFOMDATA00 (r) register accessor: Receive FIFO0 mailbox data0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfifomdata00::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfifomdata00`]
module"]
pub type RFIFOMDATA00 = crate::Reg<rfifomdata00::RFIFOMDATA00_SPEC>;
#[doc = "Receive FIFO0 mailbox data0 register"]
pub mod rfifomdata00;
#[doc = "RFIFOMDATA10 (r) register accessor: Receive FIFO0 mailbox data1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfifomdata10::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfifomdata10`]
module"]
pub type RFIFOMDATA10 = crate::Reg<rfifomdata10::RFIFOMDATA10_SPEC>;
#[doc = "Receive FIFO0 mailbox data1 register"]
pub mod rfifomdata10;
#[doc = "RFIFOMI1 (r) register accessor: Receive FIFO1 mailbox identifier register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfifomi1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfifomi1`]
module"]
pub type RFIFOMI1 = crate::Reg<rfifomi1::RFIFOMI1_SPEC>;
#[doc = "Receive FIFO1 mailbox identifier register"]
pub mod rfifomi1;
#[doc = "RFIFOMP1 (r) register accessor: Receive FIFO1 mailbox property register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfifomp1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfifomp1`]
module"]
pub type RFIFOMP1 = crate::Reg<rfifomp1::RFIFOMP1_SPEC>;
#[doc = "Receive FIFO1 mailbox property register"]
pub mod rfifomp1;
#[doc = "RFIFOMDATA01 (r) register accessor: Receive FIFO1 mailbox data0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfifomdata01::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfifomdata01`]
module"]
pub type RFIFOMDATA01 = crate::Reg<rfifomdata01::RFIFOMDATA01_SPEC>;
#[doc = "Receive FIFO1 mailbox data0 register"]
pub mod rfifomdata01;
#[doc = "RFIFOMDATA11 (r) register accessor: Receive FIFO1 mailbox data1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfifomdata11::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfifomdata11`]
module"]
pub type RFIFOMDATA11 = crate::Reg<rfifomdata11::RFIFOMDATA11_SPEC>;
#[doc = "Receive FIFO1 mailbox data1 register"]
pub mod rfifomdata11;
#[doc = "FCTL (rw) register accessor: Filter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fctl`]
module"]
pub type FCTL = crate::Reg<fctl::FCTL_SPEC>;
#[doc = "Filter control register"]
pub mod fctl;
#[doc = "FMCFG (rw) register accessor: Filter mode configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmcfg`]
module"]
pub type FMCFG = crate::Reg<fmcfg::FMCFG_SPEC>;
#[doc = "Filter mode configuration register"]
pub mod fmcfg;
#[doc = "FSCFG (rw) register accessor: Filter scale configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fscfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fscfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fscfg`]
module"]
pub type FSCFG = crate::Reg<fscfg::FSCFG_SPEC>;
#[doc = "Filter scale configuration register"]
pub mod fscfg;
#[doc = "FAFIFO (rw) register accessor: Filter associated FIFO register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fafifo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fafifo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fafifo`]
module"]
pub type FAFIFO = crate::Reg<fafifo::FAFIFO_SPEC>;
#[doc = "Filter associated FIFO register"]
pub mod fafifo;
#[doc = "FW (rw) register accessor: Filter working register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fw`]
module"]
pub type FW = crate::Reg<fw::FW_SPEC>;
#[doc = "Filter working register"]
pub mod fw;
#[doc = "F0DATA0 (rw) register accessor: Filter 0 data 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f0data0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f0data0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f0data0`]
module"]
pub type F0DATA0 = crate::Reg<f0data0::F0DATA0_SPEC>;
#[doc = "Filter 0 data 0 register"]
pub mod f0data0;
#[doc = "F0DATA1 (rw) register accessor: Filter 0 data 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f0data1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f0data1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f0data1`]
module"]
pub type F0DATA1 = crate::Reg<f0data1::F0DATA1_SPEC>;
#[doc = "Filter 0 data 1 register"]
pub mod f0data1;
#[doc = "F1DATA0 (rw) register accessor: Filter 1 data 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f1data0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f1data0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f1data0`]
module"]
pub type F1DATA0 = crate::Reg<f1data0::F1DATA0_SPEC>;
#[doc = "Filter 1 data 0 register"]
pub mod f1data0;
#[doc = "F1DATA1 (rw) register accessor: Filter 1 data 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f1data1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f1data1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f1data1`]
module"]
pub type F1DATA1 = crate::Reg<f1data1::F1DATA1_SPEC>;
#[doc = "Filter 1 data 1 register"]
pub mod f1data1;
#[doc = "F2DATA0 (rw) register accessor: Filter 2 data 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f2data0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f2data0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f2data0`]
module"]
pub type F2DATA0 = crate::Reg<f2data0::F2DATA0_SPEC>;
#[doc = "Filter 2 data 0 register"]
pub mod f2data0;
#[doc = "F2DATA1 (rw) register accessor: Filter 2 data 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f2data1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f2data1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f2data1`]
module"]
pub type F2DATA1 = crate::Reg<f2data1::F2DATA1_SPEC>;
#[doc = "Filter 2 data 1 register"]
pub mod f2data1;
#[doc = "F3DATA0 (rw) register accessor: Filter 3 data 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f3data0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f3data0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f3data0`]
module"]
pub type F3DATA0 = crate::Reg<f3data0::F3DATA0_SPEC>;
#[doc = "Filter 3 data 0 register"]
pub mod f3data0;
#[doc = "F3DATA1 (rw) register accessor: Filter 3 data 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f3data1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f3data1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f3data1`]
module"]
pub type F3DATA1 = crate::Reg<f3data1::F3DATA1_SPEC>;
#[doc = "Filter 3 data 1 register"]
pub mod f3data1;
#[doc = "F4DATA0 (rw) register accessor: Filter 4 data 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f4data0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f4data0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f4data0`]
module"]
pub type F4DATA0 = crate::Reg<f4data0::F4DATA0_SPEC>;
#[doc = "Filter 4 data 0 register"]
pub mod f4data0;
#[doc = "F4DATA1 (rw) register accessor: Filter 4 data 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f4data1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f4data1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f4data1`]
module"]
pub type F4DATA1 = crate::Reg<f4data1::F4DATA1_SPEC>;
#[doc = "Filter 4 data 1 register"]
pub mod f4data1;
#[doc = "F5DATA0 (rw) register accessor: Filter 5 data 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f5data0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f5data0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f5data0`]
module"]
pub type F5DATA0 = crate::Reg<f5data0::F5DATA0_SPEC>;
#[doc = "Filter 5 data 0 register"]
pub mod f5data0;
#[doc = "F5DATA1 (rw) register accessor: Filter 5 data 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f5data1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f5data1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f5data1`]
module"]
pub type F5DATA1 = crate::Reg<f5data1::F5DATA1_SPEC>;
#[doc = "Filter 5 data 1 register"]
pub mod f5data1;
#[doc = "F6DATA0 (rw) register accessor: Filter 6 data 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f6data0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f6data0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f6data0`]
module"]
pub type F6DATA0 = crate::Reg<f6data0::F6DATA0_SPEC>;
#[doc = "Filter 6 data 0 register"]
pub mod f6data0;
#[doc = "F6DATA1 (rw) register accessor: Filter 6 data 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f6data1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f6data1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f6data1`]
module"]
pub type F6DATA1 = crate::Reg<f6data1::F6DATA1_SPEC>;
#[doc = "Filter 6 data 1 register"]
pub mod f6data1;
#[doc = "F7DATA0 (rw) register accessor: Filter 7 data 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f7data0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f7data0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f7data0`]
module"]
pub type F7DATA0 = crate::Reg<f7data0::F7DATA0_SPEC>;
#[doc = "Filter 7 data 0 register"]
pub mod f7data0;
#[doc = "F7DATA1 (rw) register accessor: Filter 7 data 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f7data1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f7data1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f7data1`]
module"]
pub type F7DATA1 = crate::Reg<f7data1::F7DATA1_SPEC>;
#[doc = "Filter 7 data 1 register"]
pub mod f7data1;
#[doc = "F8DATA0 (rw) register accessor: Filter 8 data 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f8data0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f8data0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f8data0`]
module"]
pub type F8DATA0 = crate::Reg<f8data0::F8DATA0_SPEC>;
#[doc = "Filter 8 data 0 register"]
pub mod f8data0;
#[doc = "F8DATA1 (rw) register accessor: Filter 8 data 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f8data1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f8data1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f8data1`]
module"]
pub type F8DATA1 = crate::Reg<f8data1::F8DATA1_SPEC>;
#[doc = "Filter 8 data 1 register"]
pub mod f8data1;
#[doc = "F9DATA0 (rw) register accessor: Filter 9 data 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f9data0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f9data0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f9data0`]
module"]
pub type F9DATA0 = crate::Reg<f9data0::F9DATA0_SPEC>;
#[doc = "Filter 9 data 0 register"]
pub mod f9data0;
#[doc = "F9DATA1 (rw) register accessor: Filter 9 data 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f9data1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f9data1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f9data1`]
module"]
pub type F9DATA1 = crate::Reg<f9data1::F9DATA1_SPEC>;
#[doc = "Filter 9 data 1 register"]
pub mod f9data1;
#[doc = "F10DATA0 (rw) register accessor: Filter 10 data 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f10data0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f10data0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f10data0`]
module"]
pub type F10DATA0 = crate::Reg<f10data0::F10DATA0_SPEC>;
#[doc = "Filter 10 data 0 register"]
pub mod f10data0;
#[doc = "F10DATA1 (rw) register accessor: Filter 10 data 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f10data1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f10data1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f10data1`]
module"]
pub type F10DATA1 = crate::Reg<f10data1::F10DATA1_SPEC>;
#[doc = "Filter 10 data 1 register"]
pub mod f10data1;
#[doc = "F11DATA0 (rw) register accessor: Filter 11 data 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f11data0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f11data0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f11data0`]
module"]
pub type F11DATA0 = crate::Reg<f11data0::F11DATA0_SPEC>;
#[doc = "Filter 11 data 0 register"]
pub mod f11data0;
#[doc = "F11DATA1 (rw) register accessor: Filter 11 data 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f11data1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f11data1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f11data1`]
module"]
pub type F11DATA1 = crate::Reg<f11data1::F11DATA1_SPEC>;
#[doc = "Filter 11 data 1 register"]
pub mod f11data1;
#[doc = "F12DATA0 (rw) register accessor: Filter 12 data 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f12data0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f12data0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f12data0`]
module"]
pub type F12DATA0 = crate::Reg<f12data0::F12DATA0_SPEC>;
#[doc = "Filter 12 data 0 register"]
pub mod f12data0;
#[doc = "F12DATA1 (rw) register accessor: Filter 12 data 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f12data1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f12data1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f12data1`]
module"]
pub type F12DATA1 = crate::Reg<f12data1::F12DATA1_SPEC>;
#[doc = "Filter 12 data 1 register"]
pub mod f12data1;
#[doc = "F13DATA0 (rw) register accessor: Filter 13 data 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f13data0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f13data0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f13data0`]
module"]
pub type F13DATA0 = crate::Reg<f13data0::F13DATA0_SPEC>;
#[doc = "Filter 13 data 0 register"]
pub mod f13data0;
#[doc = "F13DATA1 (rw) register accessor: Filter 13 data 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f13data1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f13data1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f13data1`]
module"]
pub type F13DATA1 = crate::Reg<f13data1::F13DATA1_SPEC>;
#[doc = "Filter 13 data 1 register"]
pub mod f13data1;
#[doc = "F14DATA0 (rw) register accessor: Filter 14 data 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f14data0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f14data0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f14data0`]
module"]
pub type F14DATA0 = crate::Reg<f14data0::F14DATA0_SPEC>;
#[doc = "Filter 14 data 0 register"]
pub mod f14data0;
#[doc = "F14DATA1 (rw) register accessor: Filter 14 data 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f14data1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f14data1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f14data1`]
module"]
pub type F14DATA1 = crate::Reg<f14data1::F14DATA1_SPEC>;
#[doc = "Filter 14 data 1 register"]
pub mod f14data1;
#[doc = "F15DATA0 (rw) register accessor: Filter 15 data 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f15data0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f15data0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f15data0`]
module"]
pub type F15DATA0 = crate::Reg<f15data0::F15DATA0_SPEC>;
#[doc = "Filter 15 data 0 register"]
pub mod f15data0;
#[doc = "F15DATA1 (rw) register accessor: Filter 15 data 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f15data1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f15data1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f15data1`]
module"]
pub type F15DATA1 = crate::Reg<f15data1::F15DATA1_SPEC>;
#[doc = "Filter 15 data 1 register"]
pub mod f15data1;
#[doc = "F16DATA0 (rw) register accessor: Filter 16 data 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f16data0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f16data0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f16data0`]
module"]
pub type F16DATA0 = crate::Reg<f16data0::F16DATA0_SPEC>;
#[doc = "Filter 16 data 0 register"]
pub mod f16data0;
#[doc = "F16DATA1 (rw) register accessor: Filter 16 data 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f16data1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f16data1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f16data1`]
module"]
pub type F16DATA1 = crate::Reg<f16data1::F16DATA1_SPEC>;
#[doc = "Filter 16 data 1 register"]
pub mod f16data1;
#[doc = "F17DATA0 (rw) register accessor: Filter 17 data 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f17data0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f17data0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f17data0`]
module"]
pub type F17DATA0 = crate::Reg<f17data0::F17DATA0_SPEC>;
#[doc = "Filter 17 data 0 register"]
pub mod f17data0;
#[doc = "F17DATA1 (rw) register accessor: Filter 17 data 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f17data1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f17data1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f17data1`]
module"]
pub type F17DATA1 = crate::Reg<f17data1::F17DATA1_SPEC>;
#[doc = "Filter 17 data 1 register"]
pub mod f17data1;
#[doc = "F18DATA0 (rw) register accessor: Filter 18 data 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f18data0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f18data0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f18data0`]
module"]
pub type F18DATA0 = crate::Reg<f18data0::F18DATA0_SPEC>;
#[doc = "Filter 18 data 0 register"]
pub mod f18data0;
#[doc = "F18DATA1 (rw) register accessor: Filter 18 data 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f18data1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f18data1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f18data1`]
module"]
pub type F18DATA1 = crate::Reg<f18data1::F18DATA1_SPEC>;
#[doc = "Filter 18 data 1 register"]
pub mod f18data1;
#[doc = "F19DATA0 (rw) register accessor: Filter 19 data 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f19data0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f19data0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f19data0`]
module"]
pub type F19DATA0 = crate::Reg<f19data0::F19DATA0_SPEC>;
#[doc = "Filter 19 data 0 register"]
pub mod f19data0;
#[doc = "F19DATA1 (rw) register accessor: Filter 19 data 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f19data1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f19data1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f19data1`]
module"]
pub type F19DATA1 = crate::Reg<f19data1::F19DATA1_SPEC>;
#[doc = "Filter 19 data 1 register"]
pub mod f19data1;
#[doc = "F20DATA0 (rw) register accessor: Filter 20 data 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f20data0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f20data0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f20data0`]
module"]
pub type F20DATA0 = crate::Reg<f20data0::F20DATA0_SPEC>;
#[doc = "Filter 20 data 0 register"]
pub mod f20data0;
#[doc = "F20DATA1 (rw) register accessor: Filter 20 data 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f20data1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f20data1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f20data1`]
module"]
pub type F20DATA1 = crate::Reg<f20data1::F20DATA1_SPEC>;
#[doc = "Filter 20 data 1 register"]
pub mod f20data1;
#[doc = "F21DATA0 (rw) register accessor: Filter 21 data 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f21data0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f21data0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f21data0`]
module"]
pub type F21DATA0 = crate::Reg<f21data0::F21DATA0_SPEC>;
#[doc = "Filter 21 data 0 register"]
pub mod f21data0;
#[doc = "F21DATA1 (rw) register accessor: Filter 21 data 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f21data1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f21data1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f21data1`]
module"]
pub type F21DATA1 = crate::Reg<f21data1::F21DATA1_SPEC>;
#[doc = "Filter 21 data 1 register"]
pub mod f21data1;
#[doc = "F22DATA0 (rw) register accessor: Filter 22 data 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f22data0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f22data0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f22data0`]
module"]
pub type F22DATA0 = crate::Reg<f22data0::F22DATA0_SPEC>;
#[doc = "Filter 22 data 0 register"]
pub mod f22data0;
#[doc = "F22DATA1 (rw) register accessor: Filter 22 data 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f22data1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f22data1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f22data1`]
module"]
pub type F22DATA1 = crate::Reg<f22data1::F22DATA1_SPEC>;
#[doc = "Filter 22 data 1 register"]
pub mod f22data1;
#[doc = "F23DATA0 (rw) register accessor: Filter 23 data 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f23data0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f23data0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f23data0`]
module"]
pub type F23DATA0 = crate::Reg<f23data0::F23DATA0_SPEC>;
#[doc = "Filter 23 data 0 register"]
pub mod f23data0;
#[doc = "F23DATA1 (rw) register accessor: Filter 23 data 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f23data1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f23data1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f23data1`]
module"]
pub type F23DATA1 = crate::Reg<f23data1::F23DATA1_SPEC>;
#[doc = "Filter 23 data 1 register"]
pub mod f23data1;
#[doc = "F24DATA0 (rw) register accessor: Filter 24 data 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f24data0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f24data0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f24data0`]
module"]
pub type F24DATA0 = crate::Reg<f24data0::F24DATA0_SPEC>;
#[doc = "Filter 24 data 0 register"]
pub mod f24data0;
#[doc = "F24DATA1 (rw) register accessor: Filter 24 data 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f24data1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f24data1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f24data1`]
module"]
pub type F24DATA1 = crate::Reg<f24data1::F24DATA1_SPEC>;
#[doc = "Filter 24 data 1 register"]
pub mod f24data1;
#[doc = "F25DATA0 (rw) register accessor: Filter 25 data 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f25data0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f25data0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f25data0`]
module"]
pub type F25DATA0 = crate::Reg<f25data0::F25DATA0_SPEC>;
#[doc = "Filter 25 data 0 register"]
pub mod f25data0;
#[doc = "F25DATA1 (rw) register accessor: Filter 25 data 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f25data1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f25data1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f25data1`]
module"]
pub type F25DATA1 = crate::Reg<f25data1::F25DATA1_SPEC>;
#[doc = "Filter 25 data 1 register"]
pub mod f25data1;
#[doc = "F26DATA0 (rw) register accessor: Filter 26 data 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f26data0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f26data0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f26data0`]
module"]
pub type F26DATA0 = crate::Reg<f26data0::F26DATA0_SPEC>;
#[doc = "Filter 26 data 0 register"]
pub mod f26data0;
#[doc = "F26DATA1 (rw) register accessor: Filter 26 data 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f26data1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f26data1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f26data1`]
module"]
pub type F26DATA1 = crate::Reg<f26data1::F26DATA1_SPEC>;
#[doc = "Filter 26 data 1 register"]
pub mod f26data1;
#[doc = "F27DATA0 (rw) register accessor: Filter 27 data 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f27data0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f27data0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f27data0`]
module"]
pub type F27DATA0 = crate::Reg<f27data0::F27DATA0_SPEC>;
#[doc = "Filter 27 data 0 register"]
pub mod f27data0;
#[doc = "F27DATA1 (rw) register accessor: Filter 27 data 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f27data1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f27data1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f27data1`]
module"]
pub type F27DATA1 = crate::Reg<f27data1::F27DATA1_SPEC>;
#[doc = "Filter 27 data 1 register"]
pub mod f27data1;
