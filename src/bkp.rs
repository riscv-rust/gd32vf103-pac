#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4usize],
    #[doc = "0x04 - Backup data register 0"]
    pub data0: DATA0,
    _reserved1: [u8; 2usize],
    #[doc = "0x08 - Backup data register 1"]
    pub data1: DATA1,
    _reserved2: [u8; 2usize],
    #[doc = "0x0c - Backup data register 2"]
    pub data2: DATA2,
    _reserved3: [u8; 2usize],
    #[doc = "0x10 - Backup data register 3"]
    pub data3: DATA3,
    _reserved4: [u8; 2usize],
    #[doc = "0x14 - Backup data register 4"]
    pub data4: DATA4,
    _reserved5: [u8; 2usize],
    #[doc = "0x18 - Backup data register 5"]
    pub data5: DATA5,
    _reserved6: [u8; 2usize],
    #[doc = "0x1c - Backup data register 6"]
    pub data6: DATA6,
    _reserved7: [u8; 2usize],
    #[doc = "0x20 - Backup data register 7"]
    pub data7: DATA7,
    _reserved8: [u8; 2usize],
    #[doc = "0x24 - Backup data register 8"]
    pub data8: DATA8,
    _reserved9: [u8; 2usize],
    #[doc = "0x28 - Backup data register 9"]
    pub data9: DATA9,
    _reserved10: [u8; 2usize],
    #[doc = "0x2c - RTC signal output control register"]
    pub octl: OCTL,
    _reserved11: [u8; 2usize],
    #[doc = "0x30 - Tamper pin control register"]
    pub tpctl: TPCTL,
    _reserved12: [u8; 2usize],
    #[doc = "0x34 - Tamper control and status register"]
    pub tpcs: TPCS,
    _reserved13: [u8; 10usize],
    #[doc = "0x40 - Backup data register 10"]
    pub data10: DATA10,
    _reserved14: [u8; 2usize],
    #[doc = "0x44 - Backup data register 11"]
    pub data11: DATA11,
    _reserved15: [u8; 2usize],
    #[doc = "0x48 - Backup data register 12"]
    pub data12: DATA12,
    _reserved16: [u8; 2usize],
    #[doc = "0x4c - Backup data register 13"]
    pub data13: DATA13,
    _reserved17: [u8; 2usize],
    #[doc = "0x50 - Backup data register 14"]
    pub data14: DATA14,
    _reserved18: [u8; 2usize],
    #[doc = "0x54 - Backup data register 15"]
    pub data15: DATA15,
    _reserved19: [u8; 2usize],
    #[doc = "0x58 - Backup data register 16"]
    pub data16: DATA16,
    _reserved20: [u8; 2usize],
    #[doc = "0x5c - Backup data register 17"]
    pub data17: DATA17,
    _reserved21: [u8; 2usize],
    #[doc = "0x60 - Backup data register 18"]
    pub data18: DATA18,
    _reserved22: [u8; 2usize],
    #[doc = "0x64 - Backup data register 19"]
    pub data19: DATA19,
    _reserved23: [u8; 2usize],
    #[doc = "0x68 - Backup data register 20"]
    pub data20: DATA20,
    _reserved24: [u8; 2usize],
    #[doc = "0x6c - Backup data register 21"]
    pub data21: DATA21,
    _reserved25: [u8; 2usize],
    #[doc = "0x70 - Backup data register 22"]
    pub data22: DATA22,
    _reserved26: [u8; 2usize],
    #[doc = "0x74 - Backup data register 23"]
    pub data23: DATA23,
    _reserved27: [u8; 2usize],
    #[doc = "0x78 - Backup data register 24"]
    pub data24: DATA24,
    _reserved28: [u8; 2usize],
    #[doc = "0x7c - Backup data register 25"]
    pub data25: DATA25,
    _reserved29: [u8; 2usize],
    #[doc = "0x80 - Backup data register 26"]
    pub data26: DATA26,
    _reserved30: [u8; 2usize],
    #[doc = "0x84 - Backup data register 27"]
    pub data27: DATA27,
    _reserved31: [u8; 2usize],
    #[doc = "0x88 - Backup data register 28"]
    pub data28: DATA28,
    _reserved32: [u8; 2usize],
    #[doc = "0x8c - Backup data register 29"]
    pub data29: DATA29,
    _reserved33: [u8; 2usize],
    #[doc = "0x90 - Backup data register 30"]
    pub data30: DATA30,
    _reserved34: [u8; 2usize],
    #[doc = "0x94 - Backup data register 31"]
    pub data31: DATA31,
    _reserved35: [u8; 2usize],
    #[doc = "0x98 - Backup data register 32"]
    pub data32: DATA32,
    _reserved36: [u8; 2usize],
    #[doc = "0x9c - Backup data register 33"]
    pub data33: DATA33,
    _reserved37: [u8; 2usize],
    #[doc = "0xa0 - Backup data register 34"]
    pub data34: DATA34,
    _reserved38: [u8; 2usize],
    #[doc = "0xa4 - Backup data register 35"]
    pub data35: DATA35,
    _reserved39: [u8; 2usize],
    #[doc = "0xa8 - Backup data register 36"]
    pub data36: DATA36,
    _reserved40: [u8; 2usize],
    #[doc = "0xac - Backup data register 37"]
    pub data37: DATA37,
    _reserved41: [u8; 2usize],
    #[doc = "0xb0 - Backup data register 38"]
    pub data38: DATA38,
    _reserved42: [u8; 2usize],
    #[doc = "0xb4 - Backup data register 39"]
    pub data39: DATA39,
    _reserved43: [u8; 2usize],
    #[doc = "0xb8 - Backup data register 40"]
    pub data40: DATA40,
    _reserved44: [u8; 2usize],
    #[doc = "0xbc - Backup data register 41"]
    pub data41: DATA41,
}
#[doc = "Backup data register 0"]
pub struct DATA0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Backup data register 0"]
pub mod data0;
#[doc = "Backup data register 1"]
pub struct DATA1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Backup data register 1"]
pub mod data1;
#[doc = "Backup data register 2"]
pub struct DATA2 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Backup data register 2"]
pub mod data2;
#[doc = "Backup data register 3"]
pub struct DATA3 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Backup data register 3"]
pub mod data3;
#[doc = "Backup data register 4"]
pub struct DATA4 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Backup data register 4"]
pub mod data4;
#[doc = "Backup data register 5"]
pub struct DATA5 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Backup data register 5"]
pub mod data5;
#[doc = "Backup data register 6"]
pub struct DATA6 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Backup data register 6"]
pub mod data6;
#[doc = "Backup data register 7"]
pub struct DATA7 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Backup data register 7"]
pub mod data7;
#[doc = "Backup data register 8"]
pub struct DATA8 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Backup data register 8"]
pub mod data8;
#[doc = "Backup data register 9"]
pub struct DATA9 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Backup data register 9"]
pub mod data9;
#[doc = "Backup data register 10"]
pub struct DATA10 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Backup data register 10"]
pub mod data10;
#[doc = "Backup data register 11"]
pub struct DATA11 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Backup data register 11"]
pub mod data11;
#[doc = "Backup data register 12"]
pub struct DATA12 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Backup data register 12"]
pub mod data12;
#[doc = "Backup data register 13"]
pub struct DATA13 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Backup data register 13"]
pub mod data13;
#[doc = "Backup data register 14"]
pub struct DATA14 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Backup data register 14"]
pub mod data14;
#[doc = "Backup data register 15"]
pub struct DATA15 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Backup data register 15"]
pub mod data15;
#[doc = "Backup data register 16"]
pub struct DATA16 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Backup data register 16"]
pub mod data16;
#[doc = "Backup data register 17"]
pub struct DATA17 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Backup data register 17"]
pub mod data17;
#[doc = "Backup data register 18"]
pub struct DATA18 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Backup data register 18"]
pub mod data18;
#[doc = "Backup data register 19"]
pub struct DATA19 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Backup data register 19"]
pub mod data19;
#[doc = "Backup data register 20"]
pub struct DATA20 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Backup data register 20"]
pub mod data20;
#[doc = "Backup data register 21"]
pub struct DATA21 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Backup data register 21"]
pub mod data21;
#[doc = "Backup data register 22"]
pub struct DATA22 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Backup data register 22"]
pub mod data22;
#[doc = "Backup data register 23"]
pub struct DATA23 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Backup data register 23"]
pub mod data23;
#[doc = "Backup data register 24"]
pub struct DATA24 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Backup data register 24"]
pub mod data24;
#[doc = "Backup data register 25"]
pub struct DATA25 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Backup data register 25"]
pub mod data25;
#[doc = "Backup data register 26"]
pub struct DATA26 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Backup data register 26"]
pub mod data26;
#[doc = "Backup data register 27"]
pub struct DATA27 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Backup data register 27"]
pub mod data27;
#[doc = "Backup data register 28"]
pub struct DATA28 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Backup data register 28"]
pub mod data28;
#[doc = "Backup data register 29"]
pub struct DATA29 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Backup data register 29"]
pub mod data29;
#[doc = "Backup data register 30"]
pub struct DATA30 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Backup data register 30"]
pub mod data30;
#[doc = "Backup data register 31"]
pub struct DATA31 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Backup data register 31"]
pub mod data31;
#[doc = "Backup data register 32"]
pub struct DATA32 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Backup data register 32"]
pub mod data32;
#[doc = "Backup data register 33"]
pub struct DATA33 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Backup data register 33"]
pub mod data33;
#[doc = "Backup data register 34"]
pub struct DATA34 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Backup data register 34"]
pub mod data34;
#[doc = "Backup data register 35"]
pub struct DATA35 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Backup data register 35"]
pub mod data35;
#[doc = "Backup data register 36"]
pub struct DATA36 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Backup data register 36"]
pub mod data36;
#[doc = "Backup data register 37"]
pub struct DATA37 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Backup data register 37"]
pub mod data37;
#[doc = "Backup data register 38"]
pub struct DATA38 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Backup data register 38"]
pub mod data38;
#[doc = "Backup data register 39"]
pub struct DATA39 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Backup data register 39"]
pub mod data39;
#[doc = "Backup data register 40"]
pub struct DATA40 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Backup data register 40"]
pub mod data40;
#[doc = "Backup data register 41"]
pub struct DATA41 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Backup data register 41"]
pub mod data41;
#[doc = "RTC signal output control register"]
pub struct OCTL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "RTC signal output control register"]
pub mod octl;
#[doc = "Tamper pin control register"]
pub struct TPCTL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Tamper pin control register"]
pub mod tpctl;
#[doc = "Tamper control and status register"]
pub struct TPCS {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Tamper control and status register"]
pub mod tpcs;
