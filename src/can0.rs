#[doc = r" Register block"]
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
    _reserved0: [u8; 352usize],
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
    _reserved1: [u8; 48usize],
    #[doc = "0x200 - Filter control register"]
    pub fctl: FCTL,
    #[doc = "0x204 - Filter mode configuration register"]
    pub fmcfg: FMCFG,
    _reserved2: [u8; 4usize],
    #[doc = "0x20c - Filter scale configuration register"]
    pub fscfg: FSCFG,
    _reserved3: [u8; 4usize],
    #[doc = "0x214 - Filter associated FIFO register"]
    pub fafifo: FAFIFO,
    _reserved4: [u8; 4usize],
    #[doc = "0x21c - Filter working register"]
    pub fw: FW,
    _reserved5: [u8; 32usize],
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
#[doc = "Control register"]
pub struct CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control register"]
pub mod ctl;
#[doc = "Status register"]
pub struct STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register"]
pub mod stat;
#[doc = "Transmit status register"]
pub struct TSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit status register"]
pub mod tstat;
#[doc = "Receive message FIFO0 register"]
pub struct RFIFO0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive message FIFO0 register"]
pub mod rfifo0;
#[doc = "Receive message FIFO1 register"]
pub struct RFIFO1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive message FIFO1 register"]
pub mod rfifo1;
#[doc = "Interrupt enable register"]
pub struct INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt enable register"]
pub mod inten;
#[doc = "Error register"]
pub struct ERR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error register"]
pub mod err;
#[doc = "Bit timing register"]
pub struct BT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bit timing register"]
pub mod bt;
#[doc = "Transmit mailbox identifier register 0"]
pub struct TMI0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit mailbox identifier register 0"]
pub mod tmi0;
#[doc = "Transmit mailbox property register 0"]
pub struct TMP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit mailbox property register 0"]
pub mod tmp0;
#[doc = "Transmit mailbox data0 register"]
pub struct TMDATA00 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit mailbox data0 register"]
pub mod tmdata00;
#[doc = "Transmit mailbox data1 register"]
pub struct TMDATA10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit mailbox data1 register"]
pub mod tmdata10;
#[doc = "Transmit mailbox identifier register 1"]
pub struct TMI1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit mailbox identifier register 1"]
pub mod tmi1;
#[doc = "Transmit mailbox property register 1"]
pub struct TMP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit mailbox property register 1"]
pub mod tmp1;
#[doc = "Transmit mailbox data0 register"]
pub struct TMDATA01 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit mailbox data0 register"]
pub mod tmdata01;
#[doc = "Transmit mailbox data1 register"]
pub struct TMDATA11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit mailbox data1 register"]
pub mod tmdata11;
#[doc = "Transmit mailbox identifier register 2"]
pub struct TMI2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit mailbox identifier register 2"]
pub mod tmi2;
#[doc = "Transmit mailbox property register 2"]
pub struct TMP2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit mailbox property register 2"]
pub mod tmp2;
#[doc = "Transmit mailbox data0 register"]
pub struct TMDATA02 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit mailbox data0 register"]
pub mod tmdata02;
#[doc = "Transmit mailbox data1 register"]
pub struct TMDATA12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit mailbox data1 register"]
pub mod tmdata12;
#[doc = "Receive FIFO mailbox identifier register"]
pub struct RFIFOMI0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive FIFO mailbox identifier register"]
pub mod rfifomi0;
#[doc = "Receive FIFO0 mailbox property register"]
pub struct RFIFOMP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive FIFO0 mailbox property register"]
pub mod rfifomp0;
#[doc = "Receive FIFO0 mailbox data0 register"]
pub struct RFIFOMDATA00 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive FIFO0 mailbox data0 register"]
pub mod rfifomdata00;
#[doc = "Receive FIFO0 mailbox data1 register"]
pub struct RFIFOMDATA10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive FIFO0 mailbox data1 register"]
pub mod rfifomdata10;
#[doc = "Receive FIFO1 mailbox identifier register"]
pub struct RFIFOMI1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive FIFO1 mailbox identifier register"]
pub mod rfifomi1;
#[doc = "Receive FIFO1 mailbox property register"]
pub struct RFIFOMP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive FIFO1 mailbox property register"]
pub mod rfifomp1;
#[doc = "Receive FIFO1 mailbox data0 register"]
pub struct RFIFOMDATA01 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive FIFO1 mailbox data0 register"]
pub mod rfifomdata01;
#[doc = "Receive FIFO1 mailbox data1 register"]
pub struct RFIFOMDATA11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive FIFO1 mailbox data1 register"]
pub mod rfifomdata11;
#[doc = "Filter control register"]
pub struct FCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter control register"]
pub mod fctl;
#[doc = "Filter mode configuration register"]
pub struct FMCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter mode configuration register"]
pub mod fmcfg;
#[doc = "Filter scale configuration register"]
pub struct FSCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter scale configuration register"]
pub mod fscfg;
#[doc = "Filter associated FIFO register"]
pub struct FAFIFO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter associated FIFO register"]
pub mod fafifo;
#[doc = "Filter working register"]
pub struct FW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter working register"]
pub mod fw;
#[doc = "Filter 0 data 0 register"]
pub struct F0DATA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 0 data 0 register"]
pub mod f0data0;
#[doc = "Filter 0 data 1 register"]
pub struct F0DATA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 0 data 1 register"]
pub mod f0data1;
#[doc = "Filter 1 data 0 register"]
pub struct F1DATA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 1 data 0 register"]
pub mod f1data0;
#[doc = "Filter 1 data 1 register"]
pub struct F1DATA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 1 data 1 register"]
pub mod f1data1;
#[doc = "Filter 2 data 0 register"]
pub struct F2DATA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 2 data 0 register"]
pub mod f2data0;
#[doc = "Filter 2 data 1 register"]
pub struct F2DATA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 2 data 1 register"]
pub mod f2data1;
#[doc = "Filter 3 data 0 register"]
pub struct F3DATA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 3 data 0 register"]
pub mod f3data0;
#[doc = "Filter 3 data 1 register"]
pub struct F3DATA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 3 data 1 register"]
pub mod f3data1;
#[doc = "Filter 4 data 0 register"]
pub struct F4DATA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 4 data 0 register"]
pub mod f4data0;
#[doc = "Filter 4 data 1 register"]
pub struct F4DATA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 4 data 1 register"]
pub mod f4data1;
#[doc = "Filter 5 data 0 register"]
pub struct F5DATA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 5 data 0 register"]
pub mod f5data0;
#[doc = "Filter 5 data 1 register"]
pub struct F5DATA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 5 data 1 register"]
pub mod f5data1;
#[doc = "Filter 6 data 0 register"]
pub struct F6DATA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 6 data 0 register"]
pub mod f6data0;
#[doc = "Filter 6 data 1 register"]
pub struct F6DATA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 6 data 1 register"]
pub mod f6data1;
#[doc = "Filter 7 data 0 register"]
pub struct F7DATA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 7 data 0 register"]
pub mod f7data0;
#[doc = "Filter 7 data 1 register"]
pub struct F7DATA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 7 data 1 register"]
pub mod f7data1;
#[doc = "Filter 8 data 0 register"]
pub struct F8DATA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 8 data 0 register"]
pub mod f8data0;
#[doc = "Filter 8 data 1 register"]
pub struct F8DATA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 8 data 1 register"]
pub mod f8data1;
#[doc = "Filter 9 data 0 register"]
pub struct F9DATA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 9 data 0 register"]
pub mod f9data0;
#[doc = "Filter 9 data 1 register"]
pub struct F9DATA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 9 data 1 register"]
pub mod f9data1;
#[doc = "Filter 10 data 0 register"]
pub struct F10DATA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 10 data 0 register"]
pub mod f10data0;
#[doc = "Filter 10 data 1 register"]
pub struct F10DATA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 10 data 1 register"]
pub mod f10data1;
#[doc = "Filter 11 data 0 register"]
pub struct F11DATA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 11 data 0 register"]
pub mod f11data0;
#[doc = "Filter 11 data 1 register"]
pub struct F11DATA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 11 data 1 register"]
pub mod f11data1;
#[doc = "Filter 12 data 0 register"]
pub struct F12DATA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 12 data 0 register"]
pub mod f12data0;
#[doc = "Filter 12 data 1 register"]
pub struct F12DATA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 12 data 1 register"]
pub mod f12data1;
#[doc = "Filter 13 data 0 register"]
pub struct F13DATA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 13 data 0 register"]
pub mod f13data0;
#[doc = "Filter 13 data 1 register"]
pub struct F13DATA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 13 data 1 register"]
pub mod f13data1;
#[doc = "Filter 14 data 0 register"]
pub struct F14DATA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 14 data 0 register"]
pub mod f14data0;
#[doc = "Filter 14 data 1 register"]
pub struct F14DATA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 14 data 1 register"]
pub mod f14data1;
#[doc = "Filter 15 data 0 register"]
pub struct F15DATA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 15 data 0 register"]
pub mod f15data0;
#[doc = "Filter 15 data 1 register"]
pub struct F15DATA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 15 data 1 register"]
pub mod f15data1;
#[doc = "Filter 16 data 0 register"]
pub struct F16DATA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 16 data 0 register"]
pub mod f16data0;
#[doc = "Filter 16 data 1 register"]
pub struct F16DATA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 16 data 1 register"]
pub mod f16data1;
#[doc = "Filter 17 data 0 register"]
pub struct F17DATA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 17 data 0 register"]
pub mod f17data0;
#[doc = "Filter 17 data 1 register"]
pub struct F17DATA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 17 data 1 register"]
pub mod f17data1;
#[doc = "Filter 18 data 0 register"]
pub struct F18DATA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 18 data 0 register"]
pub mod f18data0;
#[doc = "Filter 18 data 1 register"]
pub struct F18DATA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 18 data 1 register"]
pub mod f18data1;
#[doc = "Filter 19 data 0 register"]
pub struct F19DATA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 19 data 0 register"]
pub mod f19data0;
#[doc = "Filter 19 data 1 register"]
pub struct F19DATA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 19 data 1 register"]
pub mod f19data1;
#[doc = "Filter 20 data 0 register"]
pub struct F20DATA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 20 data 0 register"]
pub mod f20data0;
#[doc = "Filter 20 data 1 register"]
pub struct F20DATA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 20 data 1 register"]
pub mod f20data1;
#[doc = "Filter 21 data 0 register"]
pub struct F21DATA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 21 data 0 register"]
pub mod f21data0;
#[doc = "Filter 21 data 1 register"]
pub struct F21DATA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 21 data 1 register"]
pub mod f21data1;
#[doc = "Filter 22 data 0 register"]
pub struct F22DATA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 22 data 0 register"]
pub mod f22data0;
#[doc = "Filter 22 data 1 register"]
pub struct F22DATA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 22 data 1 register"]
pub mod f22data1;
#[doc = "Filter 23 data 0 register"]
pub struct F23DATA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 23 data 0 register"]
pub mod f23data0;
#[doc = "Filter 23 data 1 register"]
pub struct F23DATA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 23 data 1 register"]
pub mod f23data1;
#[doc = "Filter 24 data 0 register"]
pub struct F24DATA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 24 data 0 register"]
pub mod f24data0;
#[doc = "Filter 24 data 1 register"]
pub struct F24DATA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 24 data 1 register"]
pub mod f24data1;
#[doc = "Filter 25 data 0 register"]
pub struct F25DATA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 25 data 0 register"]
pub mod f25data0;
#[doc = "Filter 25 data 1 register"]
pub struct F25DATA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 25 data 1 register"]
pub mod f25data1;
#[doc = "Filter 26 data 0 register"]
pub struct F26DATA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 26 data 0 register"]
pub mod f26data0;
#[doc = "Filter 26 data 1 register"]
pub struct F26DATA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 26 data 1 register"]
pub mod f26data1;
#[doc = "Filter 27 data 0 register"]
pub struct F27DATA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 27 data 0 register"]
pub mod f27data0;
#[doc = "Filter 27 data 1 register"]
pub struct F27DATA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter 27 data 1 register"]
pub mod f27data1;
