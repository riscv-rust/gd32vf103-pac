#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - Backup data register 0"]
    pub data0: DATA0,
    _reserved1: [u8; 0x02],
    #[doc = "0x08 - Backup data register 1"]
    pub data1: DATA1,
    _reserved2: [u8; 0x02],
    #[doc = "0x0c - Backup data register 2"]
    pub data2: DATA2,
    _reserved3: [u8; 0x02],
    #[doc = "0x10 - Backup data register 3"]
    pub data3: DATA3,
    _reserved4: [u8; 0x02],
    #[doc = "0x14 - Backup data register 4"]
    pub data4: DATA4,
    _reserved5: [u8; 0x02],
    #[doc = "0x18 - Backup data register 5"]
    pub data5: DATA5,
    _reserved6: [u8; 0x02],
    #[doc = "0x1c - Backup data register 6"]
    pub data6: DATA6,
    _reserved7: [u8; 0x02],
    #[doc = "0x20 - Backup data register 7"]
    pub data7: DATA7,
    _reserved8: [u8; 0x02],
    #[doc = "0x24 - Backup data register 8"]
    pub data8: DATA8,
    _reserved9: [u8; 0x02],
    #[doc = "0x28 - Backup data register 9"]
    pub data9: DATA9,
    _reserved10: [u8; 0x02],
    #[doc = "0x2c - RTC signal output control register"]
    pub octl: OCTL,
    _reserved11: [u8; 0x02],
    #[doc = "0x30 - Tamper pin control register"]
    pub tpctl: TPCTL,
    _reserved12: [u8; 0x02],
    #[doc = "0x34 - Tamper control and status register"]
    pub tpcs: TPCS,
    _reserved13: [u8; 0x0a],
    #[doc = "0x40 - Backup data register 10"]
    pub data10: DATA10,
    _reserved14: [u8; 0x02],
    #[doc = "0x44 - Backup data register 11"]
    pub data11: DATA11,
    _reserved15: [u8; 0x02],
    #[doc = "0x48 - Backup data register 12"]
    pub data12: DATA12,
    _reserved16: [u8; 0x02],
    #[doc = "0x4c - Backup data register 13"]
    pub data13: DATA13,
    _reserved17: [u8; 0x02],
    #[doc = "0x50 - Backup data register 14"]
    pub data14: DATA14,
    _reserved18: [u8; 0x02],
    #[doc = "0x54 - Backup data register 15"]
    pub data15: DATA15,
    _reserved19: [u8; 0x02],
    #[doc = "0x58 - Backup data register 16"]
    pub data16: DATA16,
    _reserved20: [u8; 0x02],
    #[doc = "0x5c - Backup data register 17"]
    pub data17: DATA17,
    _reserved21: [u8; 0x02],
    #[doc = "0x60 - Backup data register 18"]
    pub data18: DATA18,
    _reserved22: [u8; 0x02],
    #[doc = "0x64 - Backup data register 19"]
    pub data19: DATA19,
    _reserved23: [u8; 0x02],
    #[doc = "0x68 - Backup data register 20"]
    pub data20: DATA20,
    _reserved24: [u8; 0x02],
    #[doc = "0x6c - Backup data register 21"]
    pub data21: DATA21,
    _reserved25: [u8; 0x02],
    #[doc = "0x70 - Backup data register 22"]
    pub data22: DATA22,
    _reserved26: [u8; 0x02],
    #[doc = "0x74 - Backup data register 23"]
    pub data23: DATA23,
    _reserved27: [u8; 0x02],
    #[doc = "0x78 - Backup data register 24"]
    pub data24: DATA24,
    _reserved28: [u8; 0x02],
    #[doc = "0x7c - Backup data register 25"]
    pub data25: DATA25,
    _reserved29: [u8; 0x02],
    #[doc = "0x80 - Backup data register 26"]
    pub data26: DATA26,
    _reserved30: [u8; 0x02],
    #[doc = "0x84 - Backup data register 27"]
    pub data27: DATA27,
    _reserved31: [u8; 0x02],
    #[doc = "0x88 - Backup data register 28"]
    pub data28: DATA28,
    _reserved32: [u8; 0x02],
    #[doc = "0x8c - Backup data register 29"]
    pub data29: DATA29,
    _reserved33: [u8; 0x02],
    #[doc = "0x90 - Backup data register 30"]
    pub data30: DATA30,
    _reserved34: [u8; 0x02],
    #[doc = "0x94 - Backup data register 31"]
    pub data31: DATA31,
    _reserved35: [u8; 0x02],
    #[doc = "0x98 - Backup data register 32"]
    pub data32: DATA32,
    _reserved36: [u8; 0x02],
    #[doc = "0x9c - Backup data register 33"]
    pub data33: DATA33,
    _reserved37: [u8; 0x02],
    #[doc = "0xa0 - Backup data register 34"]
    pub data34: DATA34,
    _reserved38: [u8; 0x02],
    #[doc = "0xa4 - Backup data register 35"]
    pub data35: DATA35,
    _reserved39: [u8; 0x02],
    #[doc = "0xa8 - Backup data register 36"]
    pub data36: DATA36,
    _reserved40: [u8; 0x02],
    #[doc = "0xac - Backup data register 37"]
    pub data37: DATA37,
    _reserved41: [u8; 0x02],
    #[doc = "0xb0 - Backup data register 38"]
    pub data38: DATA38,
    _reserved42: [u8; 0x02],
    #[doc = "0xb4 - Backup data register 39"]
    pub data39: DATA39,
    _reserved43: [u8; 0x02],
    #[doc = "0xb8 - Backup data register 40"]
    pub data40: DATA40,
    _reserved44: [u8; 0x02],
    #[doc = "0xbc - Backup data register 41"]
    pub data41: DATA41,
}
#[doc = "DATA0 (rw) register accessor: an alias for `Reg<DATA0_SPEC>`"]
pub type DATA0 = crate::Reg<data0::DATA0_SPEC>;
#[doc = "Backup data register 0"]
pub mod data0;
#[doc = "DATA1 (rw) register accessor: an alias for `Reg<DATA1_SPEC>`"]
pub type DATA1 = crate::Reg<data1::DATA1_SPEC>;
#[doc = "Backup data register 1"]
pub mod data1;
#[doc = "DATA2 (rw) register accessor: an alias for `Reg<DATA2_SPEC>`"]
pub type DATA2 = crate::Reg<data2::DATA2_SPEC>;
#[doc = "Backup data register 2"]
pub mod data2;
#[doc = "DATA3 (rw) register accessor: an alias for `Reg<DATA3_SPEC>`"]
pub type DATA3 = crate::Reg<data3::DATA3_SPEC>;
#[doc = "Backup data register 3"]
pub mod data3;
#[doc = "DATA4 (rw) register accessor: an alias for `Reg<DATA4_SPEC>`"]
pub type DATA4 = crate::Reg<data4::DATA4_SPEC>;
#[doc = "Backup data register 4"]
pub mod data4;
#[doc = "DATA5 (rw) register accessor: an alias for `Reg<DATA5_SPEC>`"]
pub type DATA5 = crate::Reg<data5::DATA5_SPEC>;
#[doc = "Backup data register 5"]
pub mod data5;
#[doc = "DATA6 (rw) register accessor: an alias for `Reg<DATA6_SPEC>`"]
pub type DATA6 = crate::Reg<data6::DATA6_SPEC>;
#[doc = "Backup data register 6"]
pub mod data6;
#[doc = "DATA7 (rw) register accessor: an alias for `Reg<DATA7_SPEC>`"]
pub type DATA7 = crate::Reg<data7::DATA7_SPEC>;
#[doc = "Backup data register 7"]
pub mod data7;
#[doc = "DATA8 (rw) register accessor: an alias for `Reg<DATA8_SPEC>`"]
pub type DATA8 = crate::Reg<data8::DATA8_SPEC>;
#[doc = "Backup data register 8"]
pub mod data8;
#[doc = "DATA9 (rw) register accessor: an alias for `Reg<DATA9_SPEC>`"]
pub type DATA9 = crate::Reg<data9::DATA9_SPEC>;
#[doc = "Backup data register 9"]
pub mod data9;
#[doc = "DATA10 (rw) register accessor: an alias for `Reg<DATA10_SPEC>`"]
pub type DATA10 = crate::Reg<data10::DATA10_SPEC>;
#[doc = "Backup data register 10"]
pub mod data10;
#[doc = "DATA11 (rw) register accessor: an alias for `Reg<DATA11_SPEC>`"]
pub type DATA11 = crate::Reg<data11::DATA11_SPEC>;
#[doc = "Backup data register 11"]
pub mod data11;
#[doc = "DATA12 (rw) register accessor: an alias for `Reg<DATA12_SPEC>`"]
pub type DATA12 = crate::Reg<data12::DATA12_SPEC>;
#[doc = "Backup data register 12"]
pub mod data12;
#[doc = "DATA13 (rw) register accessor: an alias for `Reg<DATA13_SPEC>`"]
pub type DATA13 = crate::Reg<data13::DATA13_SPEC>;
#[doc = "Backup data register 13"]
pub mod data13;
#[doc = "DATA14 (rw) register accessor: an alias for `Reg<DATA14_SPEC>`"]
pub type DATA14 = crate::Reg<data14::DATA14_SPEC>;
#[doc = "Backup data register 14"]
pub mod data14;
#[doc = "DATA15 (rw) register accessor: an alias for `Reg<DATA15_SPEC>`"]
pub type DATA15 = crate::Reg<data15::DATA15_SPEC>;
#[doc = "Backup data register 15"]
pub mod data15;
#[doc = "DATA16 (rw) register accessor: an alias for `Reg<DATA16_SPEC>`"]
pub type DATA16 = crate::Reg<data16::DATA16_SPEC>;
#[doc = "Backup data register 16"]
pub mod data16;
#[doc = "DATA17 (rw) register accessor: an alias for `Reg<DATA17_SPEC>`"]
pub type DATA17 = crate::Reg<data17::DATA17_SPEC>;
#[doc = "Backup data register 17"]
pub mod data17;
#[doc = "DATA18 (rw) register accessor: an alias for `Reg<DATA18_SPEC>`"]
pub type DATA18 = crate::Reg<data18::DATA18_SPEC>;
#[doc = "Backup data register 18"]
pub mod data18;
#[doc = "DATA19 (rw) register accessor: an alias for `Reg<DATA19_SPEC>`"]
pub type DATA19 = crate::Reg<data19::DATA19_SPEC>;
#[doc = "Backup data register 19"]
pub mod data19;
#[doc = "DATA20 (rw) register accessor: an alias for `Reg<DATA20_SPEC>`"]
pub type DATA20 = crate::Reg<data20::DATA20_SPEC>;
#[doc = "Backup data register 20"]
pub mod data20;
#[doc = "DATA21 (rw) register accessor: an alias for `Reg<DATA21_SPEC>`"]
pub type DATA21 = crate::Reg<data21::DATA21_SPEC>;
#[doc = "Backup data register 21"]
pub mod data21;
#[doc = "DATA22 (rw) register accessor: an alias for `Reg<DATA22_SPEC>`"]
pub type DATA22 = crate::Reg<data22::DATA22_SPEC>;
#[doc = "Backup data register 22"]
pub mod data22;
#[doc = "DATA23 (rw) register accessor: an alias for `Reg<DATA23_SPEC>`"]
pub type DATA23 = crate::Reg<data23::DATA23_SPEC>;
#[doc = "Backup data register 23"]
pub mod data23;
#[doc = "DATA24 (rw) register accessor: an alias for `Reg<DATA24_SPEC>`"]
pub type DATA24 = crate::Reg<data24::DATA24_SPEC>;
#[doc = "Backup data register 24"]
pub mod data24;
#[doc = "DATA25 (rw) register accessor: an alias for `Reg<DATA25_SPEC>`"]
pub type DATA25 = crate::Reg<data25::DATA25_SPEC>;
#[doc = "Backup data register 25"]
pub mod data25;
#[doc = "DATA26 (rw) register accessor: an alias for `Reg<DATA26_SPEC>`"]
pub type DATA26 = crate::Reg<data26::DATA26_SPEC>;
#[doc = "Backup data register 26"]
pub mod data26;
#[doc = "DATA27 (rw) register accessor: an alias for `Reg<DATA27_SPEC>`"]
pub type DATA27 = crate::Reg<data27::DATA27_SPEC>;
#[doc = "Backup data register 27"]
pub mod data27;
#[doc = "DATA28 (rw) register accessor: an alias for `Reg<DATA28_SPEC>`"]
pub type DATA28 = crate::Reg<data28::DATA28_SPEC>;
#[doc = "Backup data register 28"]
pub mod data28;
#[doc = "DATA29 (rw) register accessor: an alias for `Reg<DATA29_SPEC>`"]
pub type DATA29 = crate::Reg<data29::DATA29_SPEC>;
#[doc = "Backup data register 29"]
pub mod data29;
#[doc = "DATA30 (rw) register accessor: an alias for `Reg<DATA30_SPEC>`"]
pub type DATA30 = crate::Reg<data30::DATA30_SPEC>;
#[doc = "Backup data register 30"]
pub mod data30;
#[doc = "DATA31 (rw) register accessor: an alias for `Reg<DATA31_SPEC>`"]
pub type DATA31 = crate::Reg<data31::DATA31_SPEC>;
#[doc = "Backup data register 31"]
pub mod data31;
#[doc = "DATA32 (rw) register accessor: an alias for `Reg<DATA32_SPEC>`"]
pub type DATA32 = crate::Reg<data32::DATA32_SPEC>;
#[doc = "Backup data register 32"]
pub mod data32;
#[doc = "DATA33 (rw) register accessor: an alias for `Reg<DATA33_SPEC>`"]
pub type DATA33 = crate::Reg<data33::DATA33_SPEC>;
#[doc = "Backup data register 33"]
pub mod data33;
#[doc = "DATA34 (rw) register accessor: an alias for `Reg<DATA34_SPEC>`"]
pub type DATA34 = crate::Reg<data34::DATA34_SPEC>;
#[doc = "Backup data register 34"]
pub mod data34;
#[doc = "DATA35 (rw) register accessor: an alias for `Reg<DATA35_SPEC>`"]
pub type DATA35 = crate::Reg<data35::DATA35_SPEC>;
#[doc = "Backup data register 35"]
pub mod data35;
#[doc = "DATA36 (rw) register accessor: an alias for `Reg<DATA36_SPEC>`"]
pub type DATA36 = crate::Reg<data36::DATA36_SPEC>;
#[doc = "Backup data register 36"]
pub mod data36;
#[doc = "DATA37 (rw) register accessor: an alias for `Reg<DATA37_SPEC>`"]
pub type DATA37 = crate::Reg<data37::DATA37_SPEC>;
#[doc = "Backup data register 37"]
pub mod data37;
#[doc = "DATA38 (rw) register accessor: an alias for `Reg<DATA38_SPEC>`"]
pub type DATA38 = crate::Reg<data38::DATA38_SPEC>;
#[doc = "Backup data register 38"]
pub mod data38;
#[doc = "DATA39 (rw) register accessor: an alias for `Reg<DATA39_SPEC>`"]
pub type DATA39 = crate::Reg<data39::DATA39_SPEC>;
#[doc = "Backup data register 39"]
pub mod data39;
#[doc = "DATA40 (rw) register accessor: an alias for `Reg<DATA40_SPEC>`"]
pub type DATA40 = crate::Reg<data40::DATA40_SPEC>;
#[doc = "Backup data register 40"]
pub mod data40;
#[doc = "DATA41 (rw) register accessor: an alias for `Reg<DATA41_SPEC>`"]
pub type DATA41 = crate::Reg<data41::DATA41_SPEC>;
#[doc = "Backup data register 41"]
pub mod data41;
#[doc = "OCTL (rw) register accessor: an alias for `Reg<OCTL_SPEC>`"]
pub type OCTL = crate::Reg<octl::OCTL_SPEC>;
#[doc = "RTC signal output control register"]
pub mod octl;
#[doc = "TPCTL (rw) register accessor: an alias for `Reg<TPCTL_SPEC>`"]
pub type TPCTL = crate::Reg<tpctl::TPCTL_SPEC>;
#[doc = "Tamper pin control register"]
pub mod tpctl;
#[doc = "TPCS (rw) register accessor: an alias for `Reg<TPCS_SPEC>`"]
pub type TPCS = crate::Reg<tpcs::TPCS_SPEC>;
#[doc = "Tamper control and status register"]
pub mod tpcs;
