#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - cliccfg Register"]
    pub cliccfg: CLICCFG,
    _reserved0: [u8; 3usize],
    #[doc = "0x04 - clicinfo Register"]
    pub clicinfo: CLICINFO,
    _reserved1: [u8; 3usize],
    #[doc = "0x0b - MTH Register"]
    pub mth: MTH,
    _reserved2: [u8; 4084usize],
    #[doc = "0x1000 - clicintip Register"]
    pub clicintip_0: CLICINTIP_0,
    #[doc = "0x1001 - clicintie Register"]
    pub clicintie_0: CLICINTIE_0,
    #[doc = "0x1002 - clicintattr Register"]
    pub clicintattr_0: CLICINTATTR_0,
    #[doc = "0x1003 - clicintctl Register"]
    pub clicintctl_0: CLICINTCTL_0,
    #[doc = "0x1004 - clicintip Register"]
    pub clicintip_1: CLICINTIP_1,
    #[doc = "0x1005 - clicintie Register"]
    pub clicintie_1: CLICINTIE_1,
    #[doc = "0x1006 - clicintattr Register"]
    pub clicintattr_1: CLICINTATTR_1,
    #[doc = "0x1007 - clicintctl Register"]
    pub clicintctl_1: CLICINTCTL_1,
    #[doc = "0x1008 - clicintip Register"]
    pub clicintip_2: CLICINTIP_2,
    #[doc = "0x1009 - clicintie Register"]
    pub clicintie_2: CLICINTIE_2,
    #[doc = "0x100a - clicintattr Register"]
    pub clicintattr_2: CLICINTATTR_2,
    #[doc = "0x100b - clicintctl Register"]
    pub clicintctl_2: CLICINTCTL_2,
    #[doc = "0x100c - clicintip Register"]
    pub clicintip_3: CLICINTIP_3,
    #[doc = "0x100d - clicintie Register"]
    pub clicintie_3: CLICINTIE_3,
    #[doc = "0x100e - clicintattr Register"]
    pub clicintattr_3: CLICINTATTR_3,
    #[doc = "0x100f - clicintctl Register"]
    pub clicintctl_3: CLICINTCTL_3,
    #[doc = "0x1010 - clicintip Register"]
    pub clicintip_4: CLICINTIP_4,
    #[doc = "0x1011 - clicintie Register"]
    pub clicintie_4: CLICINTIE_4,
    #[doc = "0x1012 - clicintattr Register"]
    pub clicintattr_4: CLICINTATTR_4,
    #[doc = "0x1013 - clicintctl Register"]
    pub clicintctl_4: CLICINTCTL_4,
    #[doc = "0x1014 - clicintip Register"]
    pub clicintip_5: CLICINTIP_5,
    #[doc = "0x1015 - clicintie Register"]
    pub clicintie_5: CLICINTIE_5,
    #[doc = "0x1016 - clicintattr Register"]
    pub clicintattr_5: CLICINTATTR_5,
    #[doc = "0x1017 - clicintctl Register"]
    pub clicintctl_5: CLICINTCTL_5,
    #[doc = "0x1018 - clicintip Register"]
    pub clicintip_6: CLICINTIP_6,
    #[doc = "0x1019 - clicintie Register"]
    pub clicintie_6: CLICINTIE_6,
    #[doc = "0x101a - clicintattr Register"]
    pub clicintattr_6: CLICINTATTR_6,
    #[doc = "0x101b - clicintctl Register"]
    pub clicintctl_6: CLICINTCTL_6,
    #[doc = "0x101c - clicintip Register"]
    pub clicintip_7: CLICINTIP_7,
    #[doc = "0x101d - clicintie Register"]
    pub clicintie_7: CLICINTIE_7,
    #[doc = "0x101e - clicintattr Register"]
    pub clicintattr_7: CLICINTATTR_7,
    #[doc = "0x101f - clicintctl Register"]
    pub clicintctl_7: CLICINTCTL_7,
    #[doc = "0x1020 - clicintip Register"]
    pub clicintip_8: CLICINTIP_8,
    #[doc = "0x1021 - clicintie Register"]
    pub clicintie_8: CLICINTIE_8,
    #[doc = "0x1022 - clicintattr Register"]
    pub clicintattr_8: CLICINTATTR_8,
    #[doc = "0x1023 - clicintctl Register"]
    pub clicintctl_8: CLICINTCTL_8,
    #[doc = "0x1024 - clicintip Register"]
    pub clicintip_9: CLICINTIP_9,
    #[doc = "0x1025 - clicintie Register"]
    pub clicintie_9: CLICINTIE_9,
    #[doc = "0x1026 - clicintattr Register"]
    pub clicintattr_9: CLICINTATTR_9,
    #[doc = "0x1027 - clicintctl Register"]
    pub clicintctl_9: CLICINTCTL_9,
    #[doc = "0x1028 - clicintip Register"]
    pub clicintip_10: CLICINTIP_10,
    #[doc = "0x1029 - clicintie Register"]
    pub clicintie_10: CLICINTIE_10,
    #[doc = "0x102a - clicintattr Register"]
    pub clicintattr_10: CLICINTATTR_10,
    #[doc = "0x102b - clicintctl Register"]
    pub clicintctl_10: CLICINTCTL_10,
    #[doc = "0x102c - clicintip Register"]
    pub clicintip_11: CLICINTIP_11,
    #[doc = "0x102d - clicintie Register"]
    pub clicintie_11: CLICINTIE_11,
    #[doc = "0x102e - clicintattr Register"]
    pub clicintattr_11: CLICINTATTR_11,
    #[doc = "0x102f - clicintctl Register"]
    pub clicintctl_11: CLICINTCTL_11,
    #[doc = "0x1030 - clicintip Register"]
    pub clicintip_12: CLICINTIP_12,
    #[doc = "0x1031 - clicintie Register"]
    pub clicintie_12: CLICINTIE_12,
    #[doc = "0x1032 - clicintattr Register"]
    pub clicintattr_12: CLICINTATTR_12,
    #[doc = "0x1033 - clicintctl Register"]
    pub clicintctl_12: CLICINTCTL_12,
    #[doc = "0x1034 - clicintip Register"]
    pub clicintip_13: CLICINTIP_13,
    #[doc = "0x1035 - clicintie Register"]
    pub clicintie_13: CLICINTIE_13,
    #[doc = "0x1036 - clicintattr Register"]
    pub clicintattr_13: CLICINTATTR_13,
    #[doc = "0x1037 - clicintctl Register"]
    pub clicintctl_13: CLICINTCTL_13,
    #[doc = "0x1038 - clicintip Register"]
    pub clicintip_14: CLICINTIP_14,
    #[doc = "0x1039 - clicintie Register"]
    pub clicintie_14: CLICINTIE_14,
    #[doc = "0x103a - clicintattr Register"]
    pub clicintattr_14: CLICINTATTR_14,
    #[doc = "0x103b - clicintctl Register"]
    pub clicintctl_14: CLICINTCTL_14,
    #[doc = "0x103c - clicintip Register"]
    pub clicintip_15: CLICINTIP_15,
    #[doc = "0x103d - clicintie Register"]
    pub clicintie_15: CLICINTIE_15,
    #[doc = "0x103e - clicintattr Register"]
    pub clicintattr_15: CLICINTATTR_15,
    #[doc = "0x103f - clicintctl Register"]
    pub clicintctl_15: CLICINTCTL_15,
    #[doc = "0x1040 - clicintip Register"]
    pub clicintip_16: CLICINTIP_16,
    #[doc = "0x1041 - clicintie Register"]
    pub clicintie_16: CLICINTIE_16,
    #[doc = "0x1042 - clicintattr Register"]
    pub clicintattr_16: CLICINTATTR_16,
    #[doc = "0x1043 - clicintctl Register"]
    pub clicintctl_16: CLICINTCTL_16,
    #[doc = "0x1044 - clicintip Register"]
    pub clicintip_17: CLICINTIP_17,
    #[doc = "0x1045 - clicintie Register"]
    pub clicintie_17: CLICINTIE_17,
    #[doc = "0x1046 - clicintattr Register"]
    pub clicintattr_17: CLICINTATTR_17,
    #[doc = "0x1047 - clicintctl Register"]
    pub clicintctl_17: CLICINTCTL_17,
    #[doc = "0x1048 - clicintip Register"]
    pub clicintip_18: CLICINTIP_18,
    #[doc = "0x1049 - clicintie Register"]
    pub clicintie_18: CLICINTIE_18,
    #[doc = "0x104a - clicintattr Register"]
    pub clicintattr_18: CLICINTATTR_18,
    #[doc = "0x104b - clicintctl Register"]
    pub clicintctl_18: CLICINTCTL_18,
    #[doc = "0x104c - clicintip Register"]
    pub clicintip_19: CLICINTIP_19,
    #[doc = "0x104d - clicintie Register"]
    pub clicintie_19: CLICINTIE_19,
    #[doc = "0x104e - clicintattr Register"]
    pub clicintattr_19: CLICINTATTR_19,
    #[doc = "0x104f - clicintctl Register"]
    pub clicintctl_19: CLICINTCTL_19,
    #[doc = "0x1050 - clicintip Register"]
    pub clicintip_20: CLICINTIP_20,
    #[doc = "0x1051 - clicintie Register"]
    pub clicintie_20: CLICINTIE_20,
    #[doc = "0x1052 - clicintattr Register"]
    pub clicintattr_20: CLICINTATTR_20,
    #[doc = "0x1053 - clicintctl Register"]
    pub clicintctl_20: CLICINTCTL_20,
    #[doc = "0x1054 - clicintip Register"]
    pub clicintip_21: CLICINTIP_21,
    #[doc = "0x1055 - clicintie Register"]
    pub clicintie_21: CLICINTIE_21,
    #[doc = "0x1056 - clicintattr Register"]
    pub clicintattr_21: CLICINTATTR_21,
    #[doc = "0x1057 - clicintctl Register"]
    pub clicintctl_21: CLICINTCTL_21,
    #[doc = "0x1058 - clicintip Register"]
    pub clicintip_22: CLICINTIP_22,
    #[doc = "0x1059 - clicintie Register"]
    pub clicintie_22: CLICINTIE_22,
    #[doc = "0x105a - clicintattr Register"]
    pub clicintattr_22: CLICINTATTR_22,
    #[doc = "0x105b - clicintctl Register"]
    pub clicintctl_22: CLICINTCTL_22,
    #[doc = "0x105c - clicintip Register"]
    pub clicintip_23: CLICINTIP_23,
    #[doc = "0x105d - clicintie Register"]
    pub clicintie_23: CLICINTIE_23,
    #[doc = "0x105e - clicintattr Register"]
    pub clicintattr_23: CLICINTATTR_23,
    #[doc = "0x105f - clicintctl Register"]
    pub clicintctl_23: CLICINTCTL_23,
    #[doc = "0x1060 - clicintip Register"]
    pub clicintip_24: CLICINTIP_24,
    #[doc = "0x1061 - clicintie Register"]
    pub clicintie_24: CLICINTIE_24,
    #[doc = "0x1062 - clicintattr Register"]
    pub clicintattr_24: CLICINTATTR_24,
    #[doc = "0x1063 - clicintctl Register"]
    pub clicintctl_24: CLICINTCTL_24,
    #[doc = "0x1064 - clicintip Register"]
    pub clicintip_25: CLICINTIP_25,
    #[doc = "0x1065 - clicintie Register"]
    pub clicintie_25: CLICINTIE_25,
    #[doc = "0x1066 - clicintattr Register"]
    pub clicintattr_25: CLICINTATTR_25,
    #[doc = "0x1067 - clicintctl Register"]
    pub clicintctl_25: CLICINTCTL_25,
    #[doc = "0x1068 - clicintip Register"]
    pub clicintip_26: CLICINTIP_26,
    #[doc = "0x1069 - clicintie Register"]
    pub clicintie_26: CLICINTIE_26,
    #[doc = "0x106a - clicintattr Register"]
    pub clicintattr_26: CLICINTATTR_26,
    #[doc = "0x106b - clicintctl Register"]
    pub clicintctl_26: CLICINTCTL_26,
    #[doc = "0x106c - clicintip Register"]
    pub clicintip_27: CLICINTIP_27,
    #[doc = "0x106d - clicintie Register"]
    pub clicintie_27: CLICINTIE_27,
    #[doc = "0x106e - clicintattr Register"]
    pub clicintattr_27: CLICINTATTR_27,
    #[doc = "0x106f - clicintctl Register"]
    pub clicintctl_27: CLICINTCTL_27,
    #[doc = "0x1070 - clicintip Register"]
    pub clicintip_28: CLICINTIP_28,
    #[doc = "0x1071 - clicintie Register"]
    pub clicintie_28: CLICINTIE_28,
    #[doc = "0x1072 - clicintattr Register"]
    pub clicintattr_28: CLICINTATTR_28,
    #[doc = "0x1073 - clicintctl Register"]
    pub clicintctl_28: CLICINTCTL_28,
    #[doc = "0x1074 - clicintip Register"]
    pub clicintip_29: CLICINTIP_29,
    #[doc = "0x1075 - clicintie Register"]
    pub clicintie_29: CLICINTIE_29,
    #[doc = "0x1076 - clicintattr Register"]
    pub clicintattr_29: CLICINTATTR_29,
    #[doc = "0x1077 - clicintctl Register"]
    pub clicintctl_29: CLICINTCTL_29,
    #[doc = "0x1078 - clicintip Register"]
    pub clicintip_30: CLICINTIP_30,
    #[doc = "0x1079 - clicintie Register"]
    pub clicintie_30: CLICINTIE_30,
    #[doc = "0x107a - clicintattr Register"]
    pub clicintattr_30: CLICINTATTR_30,
    #[doc = "0x107b - clicintctl Register"]
    pub clicintctl_30: CLICINTCTL_30,
    #[doc = "0x107c - clicintip Register"]
    pub clicintip_31: CLICINTIP_31,
    #[doc = "0x107d - clicintie Register"]
    pub clicintie_31: CLICINTIE_31,
    #[doc = "0x107e - clicintattr Register"]
    pub clicintattr_31: CLICINTATTR_31,
    #[doc = "0x107f - clicintctl Register"]
    pub clicintctl_31: CLICINTCTL_31,
    #[doc = "0x1080 - clicintip Register"]
    pub clicintip_32: CLICINTIP_32,
    #[doc = "0x1081 - clicintie Register"]
    pub clicintie_32: CLICINTIE_32,
    #[doc = "0x1082 - clicintattr Register"]
    pub clicintattr_32: CLICINTATTR_32,
    #[doc = "0x1083 - clicintctl Register"]
    pub clicintctl_32: CLICINTCTL_32,
    #[doc = "0x1084 - clicintip Register"]
    pub clicintip_33: CLICINTIP_33,
    #[doc = "0x1085 - clicintie Register"]
    pub clicintie_33: CLICINTIE_33,
    #[doc = "0x1086 - clicintattr Register"]
    pub clicintattr_33: CLICINTATTR_33,
    #[doc = "0x1087 - clicintctl Register"]
    pub clicintctl_33: CLICINTCTL_33,
    #[doc = "0x1088 - clicintip Register"]
    pub clicintip_34: CLICINTIP_34,
    #[doc = "0x1089 - clicintie Register"]
    pub clicintie_34: CLICINTIE_34,
    #[doc = "0x108a - clicintattr Register"]
    pub clicintattr_34: CLICINTATTR_34,
    #[doc = "0x108b - clicintctl Register"]
    pub clicintctl_34: CLICINTCTL_34,
    #[doc = "0x108c - clicintip Register"]
    pub clicintip_35: CLICINTIP_35,
    #[doc = "0x108d - clicintie Register"]
    pub clicintie_35: CLICINTIE_35,
    #[doc = "0x108e - clicintattr Register"]
    pub clicintattr_35: CLICINTATTR_35,
    #[doc = "0x108f - clicintctl Register"]
    pub clicintctl_35: CLICINTCTL_35,
    #[doc = "0x1090 - clicintip Register"]
    pub clicintip_36: CLICINTIP_36,
    #[doc = "0x1091 - clicintie Register"]
    pub clicintie_36: CLICINTIE_36,
    #[doc = "0x1092 - clicintattr Register"]
    pub clicintattr_36: CLICINTATTR_36,
    #[doc = "0x1093 - clicintctl Register"]
    pub clicintctl_36: CLICINTCTL_36,
    #[doc = "0x1094 - clicintip Register"]
    pub clicintip_37: CLICINTIP_37,
    #[doc = "0x1095 - clicintie Register"]
    pub clicintie_37: CLICINTIE_37,
    #[doc = "0x1096 - clicintattr Register"]
    pub clicintattr_37: CLICINTATTR_37,
    #[doc = "0x1097 - clicintctl Register"]
    pub clicintctl_37: CLICINTCTL_37,
    #[doc = "0x1098 - clicintip Register"]
    pub clicintip_38: CLICINTIP_38,
    #[doc = "0x1099 - clicintie Register"]
    pub clicintie_38: CLICINTIE_38,
    #[doc = "0x109a - clicintattr Register"]
    pub clicintattr_38: CLICINTATTR_38,
    #[doc = "0x109b - clicintctl Register"]
    pub clicintctl_38: CLICINTCTL_38,
    #[doc = "0x109c - clicintip Register"]
    pub clicintip_39: CLICINTIP_39,
    #[doc = "0x109d - clicintie Register"]
    pub clicintie_39: CLICINTIE_39,
    #[doc = "0x109e - clicintattr Register"]
    pub clicintattr_39: CLICINTATTR_39,
    #[doc = "0x109f - clicintctl Register"]
    pub clicintctl_39: CLICINTCTL_39,
    #[doc = "0x10a0 - clicintip Register"]
    pub clicintip_40: CLICINTIP_40,
    #[doc = "0x10a1 - clicintie Register"]
    pub clicintie_40: CLICINTIE_40,
    #[doc = "0x10a2 - clicintattr Register"]
    pub clicintattr_40: CLICINTATTR_40,
    #[doc = "0x10a3 - clicintctl Register"]
    pub clicintctl_40: CLICINTCTL_40,
    #[doc = "0x10a4 - clicintip Register"]
    pub clicintip_41: CLICINTIP_41,
    #[doc = "0x10a5 - clicintie Register"]
    pub clicintie_41: CLICINTIE_41,
    #[doc = "0x10a6 - clicintattr Register"]
    pub clicintattr_41: CLICINTATTR_41,
    #[doc = "0x10a7 - clicintctl Register"]
    pub clicintctl_41: CLICINTCTL_41,
    #[doc = "0x10a8 - clicintip Register"]
    pub clicintip_42: CLICINTIP_42,
    #[doc = "0x10a9 - clicintie Register"]
    pub clicintie_42: CLICINTIE_42,
    #[doc = "0x10aa - clicintattr Register"]
    pub clicintattr_42: CLICINTATTR_42,
    #[doc = "0x10ab - clicintctl Register"]
    pub clicintctl_42: CLICINTCTL_42,
    #[doc = "0x10ac - clicintip Register"]
    pub clicintip_43: CLICINTIP_43,
    #[doc = "0x10ad - clicintie Register"]
    pub clicintie_43: CLICINTIE_43,
    #[doc = "0x10ae - clicintattr Register"]
    pub clicintattr_43: CLICINTATTR_43,
    #[doc = "0x10af - clicintctl Register"]
    pub clicintctl_43: CLICINTCTL_43,
    #[doc = "0x10b0 - clicintip Register"]
    pub clicintip_44: CLICINTIP_44,
    #[doc = "0x10b1 - clicintie Register"]
    pub clicintie_44: CLICINTIE_44,
    #[doc = "0x10b2 - clicintattr Register"]
    pub clicintattr_44: CLICINTATTR_44,
    #[doc = "0x10b3 - clicintctl Register"]
    pub clicintctl_44: CLICINTCTL_44,
    #[doc = "0x10b4 - clicintip Register"]
    pub clicintip_45: CLICINTIP_45,
    #[doc = "0x10b5 - clicintie Register"]
    pub clicintie_45: CLICINTIE_45,
    #[doc = "0x10b6 - clicintattr Register"]
    pub clicintattr_45: CLICINTATTR_45,
    #[doc = "0x10b7 - clicintctl Register"]
    pub clicintctl_45: CLICINTCTL_45,
    #[doc = "0x10b8 - clicintip Register"]
    pub clicintip_46: CLICINTIP_46,
    #[doc = "0x10b9 - clicintie Register"]
    pub clicintie_46: CLICINTIE_46,
    #[doc = "0x10ba - clicintattr Register"]
    pub clicintattr_46: CLICINTATTR_46,
    #[doc = "0x10bb - clicintctl Register"]
    pub clicintctl_46: CLICINTCTL_46,
    #[doc = "0x10bc - clicintip Register"]
    pub clicintip_47: CLICINTIP_47,
    #[doc = "0x10bd - clicintie Register"]
    pub clicintie_47: CLICINTIE_47,
    #[doc = "0x10be - clicintattr Register"]
    pub clicintattr_47: CLICINTATTR_47,
    #[doc = "0x10bf - clicintctl Register"]
    pub clicintctl_47: CLICINTCTL_47,
    #[doc = "0x10c0 - clicintip Register"]
    pub clicintip_48: CLICINTIP_48,
    #[doc = "0x10c1 - clicintie Register"]
    pub clicintie_48: CLICINTIE_48,
    #[doc = "0x10c2 - clicintattr Register"]
    pub clicintattr_48: CLICINTATTR_48,
    #[doc = "0x10c3 - clicintctl Register"]
    pub clicintctl_48: CLICINTCTL_48,
    #[doc = "0x10c4 - clicintip Register"]
    pub clicintip_49: CLICINTIP_49,
    #[doc = "0x10c5 - clicintie Register"]
    pub clicintie_49: CLICINTIE_49,
    #[doc = "0x10c6 - clicintattr Register"]
    pub clicintattr_49: CLICINTATTR_49,
    #[doc = "0x10c7 - clicintctl Register"]
    pub clicintctl_49: CLICINTCTL_49,
    #[doc = "0x10c8 - clicintip Register"]
    pub clicintip_50: CLICINTIP_50,
    #[doc = "0x10c9 - clicintie Register"]
    pub clicintie_50: CLICINTIE_50,
    #[doc = "0x10ca - clicintattr Register"]
    pub clicintattr_50: CLICINTATTR_50,
    #[doc = "0x10cb - clicintctl Register"]
    pub clicintctl_50: CLICINTCTL_50,
    #[doc = "0x10cc - clicintip Register"]
    pub clicintip_51: CLICINTIP_51,
    #[doc = "0x10cd - clicintie Register"]
    pub clicintie_51: CLICINTIE_51,
    #[doc = "0x10ce - clicintattr Register"]
    pub clicintattr_51: CLICINTATTR_51,
    #[doc = "0x10cf - clicintctl Register"]
    pub clicintctl_51: CLICINTCTL_51,
    #[doc = "0x10d0 - clicintip Register"]
    pub clicintip_52: CLICINTIP_52,
    #[doc = "0x10d1 - clicintie Register"]
    pub clicintie_52: CLICINTIE_52,
    #[doc = "0x10d2 - clicintattr Register"]
    pub clicintattr_52: CLICINTATTR_52,
    #[doc = "0x10d3 - clicintctl Register"]
    pub clicintctl_52: CLICINTCTL_52,
    #[doc = "0x10d4 - clicintip Register"]
    pub clicintip_53: CLICINTIP_53,
    #[doc = "0x10d5 - clicintie Register"]
    pub clicintie_53: CLICINTIE_53,
    #[doc = "0x10d6 - clicintattr Register"]
    pub clicintattr_53: CLICINTATTR_53,
    #[doc = "0x10d7 - clicintctl Register"]
    pub clicintctl_53: CLICINTCTL_53,
    #[doc = "0x10d8 - clicintip Register"]
    pub clicintip_54: CLICINTIP_54,
    #[doc = "0x10d9 - clicintie Register"]
    pub clicintie_54: CLICINTIE_54,
    #[doc = "0x10da - clicintattr Register"]
    pub clicintattr_54: CLICINTATTR_54,
    #[doc = "0x10db - clicintctl Register"]
    pub clicintctl_54: CLICINTCTL_54,
    #[doc = "0x10dc - clicintip Register"]
    pub clicintip_55: CLICINTIP_55,
    #[doc = "0x10dd - clicintie Register"]
    pub clicintie_55: CLICINTIE_55,
    #[doc = "0x10de - clicintattr Register"]
    pub clicintattr_55: CLICINTATTR_55,
    #[doc = "0x10df - clicintctl Register"]
    pub clicintctl_55: CLICINTCTL_55,
    #[doc = "0x10e0 - clicintip Register"]
    pub clicintip_56: CLICINTIP_56,
    #[doc = "0x10e1 - clicintie Register"]
    pub clicintie_56: CLICINTIE_56,
    #[doc = "0x10e2 - clicintattr Register"]
    pub clicintattr_56: CLICINTATTR_56,
    #[doc = "0x10e3 - clicintctl Register"]
    pub clicintctl_56: CLICINTCTL_56,
    #[doc = "0x10e4 - clicintip Register"]
    pub clicintip_57: CLICINTIP_57,
    #[doc = "0x10e5 - clicintie Register"]
    pub clicintie_57: CLICINTIE_57,
    #[doc = "0x10e6 - clicintattr Register"]
    pub clicintattr_57: CLICINTATTR_57,
    #[doc = "0x10e7 - clicintctl Register"]
    pub clicintctl_57: CLICINTCTL_57,
    #[doc = "0x10e8 - clicintip Register"]
    pub clicintip_58: CLICINTIP_58,
    #[doc = "0x10e9 - clicintie Register"]
    pub clicintie_58: CLICINTIE_58,
    #[doc = "0x10ea - clicintattr Register"]
    pub clicintattr_58: CLICINTATTR_58,
    #[doc = "0x10eb - clicintctl Register"]
    pub clicintctl_58: CLICINTCTL_58,
    #[doc = "0x10ec - clicintip Register"]
    pub clicintip_59: CLICINTIP_59,
    #[doc = "0x10ed - clicintie Register"]
    pub clicintie_59: CLICINTIE_59,
    #[doc = "0x10ee - clicintattr Register"]
    pub clicintattr_59: CLICINTATTR_59,
    #[doc = "0x10ef - clicintctl Register"]
    pub clicintctl_59: CLICINTCTL_59,
    #[doc = "0x10f0 - clicintip Register"]
    pub clicintip_60: CLICINTIP_60,
    #[doc = "0x10f1 - clicintie Register"]
    pub clicintie_60: CLICINTIE_60,
    #[doc = "0x10f2 - clicintattr Register"]
    pub clicintattr_60: CLICINTATTR_60,
    #[doc = "0x10f3 - clicintctl Register"]
    pub clicintctl_60: CLICINTCTL_60,
    #[doc = "0x10f4 - clicintip Register"]
    pub clicintip_61: CLICINTIP_61,
    #[doc = "0x10f5 - clicintie Register"]
    pub clicintie_61: CLICINTIE_61,
    #[doc = "0x10f6 - clicintattr Register"]
    pub clicintattr_61: CLICINTATTR_61,
    #[doc = "0x10f7 - clicintctl Register"]
    pub clicintctl_61: CLICINTCTL_61,
    #[doc = "0x10f8 - clicintip Register"]
    pub clicintip_62: CLICINTIP_62,
    #[doc = "0x10f9 - clicintie Register"]
    pub clicintie_62: CLICINTIE_62,
    #[doc = "0x10fa - clicintattr Register"]
    pub clicintattr_62: CLICINTATTR_62,
    #[doc = "0x10fb - clicintctl Register"]
    pub clicintctl_62: CLICINTCTL_62,
    #[doc = "0x10fc - clicintip Register"]
    pub clicintip_63: CLICINTIP_63,
    #[doc = "0x10fd - clicintie Register"]
    pub clicintie_63: CLICINTIE_63,
    #[doc = "0x10fe - clicintattr Register"]
    pub clicintattr_63: CLICINTATTR_63,
    #[doc = "0x10ff - clicintctl Register"]
    pub clicintctl_63: CLICINTCTL_63,
    #[doc = "0x1100 - clicintip Register"]
    pub clicintip_64: CLICINTIP_64,
    #[doc = "0x1101 - clicintie Register"]
    pub clicintie_64: CLICINTIE_64,
    #[doc = "0x1102 - clicintattr Register"]
    pub clicintattr_64: CLICINTATTR_64,
    #[doc = "0x1103 - clicintctl Register"]
    pub clicintctl_64: CLICINTCTL_64,
    #[doc = "0x1104 - clicintip Register"]
    pub clicintip_65: CLICINTIP_65,
    #[doc = "0x1105 - clicintie Register"]
    pub clicintie_65: CLICINTIE_65,
    #[doc = "0x1106 - clicintattr Register"]
    pub clicintattr_65: CLICINTATTR_65,
    #[doc = "0x1107 - clicintctl Register"]
    pub clicintctl_65: CLICINTCTL_65,
    #[doc = "0x1108 - clicintip Register"]
    pub clicintip_66: CLICINTIP_66,
    #[doc = "0x1109 - clicintie Register"]
    pub clicintie_66: CLICINTIE_66,
    #[doc = "0x110a - clicintattr Register"]
    pub clicintattr_66: CLICINTATTR_66,
    #[doc = "0x110b - clicintctl Register"]
    pub clicintctl_66: CLICINTCTL_66,
    #[doc = "0x110c - clicintip Register"]
    pub clicintip_67: CLICINTIP_67,
    #[doc = "0x110d - clicintie Register"]
    pub clicintie_67: CLICINTIE_67,
    #[doc = "0x110e - clicintattr Register"]
    pub clicintattr_67: CLICINTATTR_67,
    #[doc = "0x110f - clicintctl Register"]
    pub clicintctl_67: CLICINTCTL_67,
    #[doc = "0x1110 - clicintip Register"]
    pub clicintip_68: CLICINTIP_68,
    #[doc = "0x1111 - clicintie Register"]
    pub clicintie_68: CLICINTIE_68,
    #[doc = "0x1112 - clicintattr Register"]
    pub clicintattr_68: CLICINTATTR_68,
    #[doc = "0x1113 - clicintctl Register"]
    pub clicintctl_68: CLICINTCTL_68,
    #[doc = "0x1114 - clicintip Register"]
    pub clicintip_69: CLICINTIP_69,
    #[doc = "0x1115 - clicintie Register"]
    pub clicintie_69: CLICINTIE_69,
    #[doc = "0x1116 - clicintattr Register"]
    pub clicintattr_69: CLICINTATTR_69,
    #[doc = "0x1117 - clicintctl Register"]
    pub clicintctl_69: CLICINTCTL_69,
    #[doc = "0x1118 - clicintip Register"]
    pub clicintip_70: CLICINTIP_70,
    #[doc = "0x1119 - clicintie Register"]
    pub clicintie_70: CLICINTIE_70,
    #[doc = "0x111a - clicintattr Register"]
    pub clicintattr_70: CLICINTATTR_70,
    #[doc = "0x111b - clicintctl Register"]
    pub clicintctl_70: CLICINTCTL_70,
    #[doc = "0x111c - clicintip Register"]
    pub clicintip_71: CLICINTIP_71,
    #[doc = "0x111d - clicintie Register"]
    pub clicintie_71: CLICINTIE_71,
    #[doc = "0x111e - clicintattr Register"]
    pub clicintattr_71: CLICINTATTR_71,
    #[doc = "0x111f - clicintctl Register"]
    pub clicintctl_71: CLICINTCTL_71,
    #[doc = "0x1120 - clicintip Register"]
    pub clicintip_72: CLICINTIP_72,
    #[doc = "0x1121 - clicintie Register"]
    pub clicintie_72: CLICINTIE_72,
    #[doc = "0x1122 - clicintattr Register"]
    pub clicintattr_72: CLICINTATTR_72,
    #[doc = "0x1123 - clicintctl Register"]
    pub clicintctl_72: CLICINTCTL_72,
    #[doc = "0x1124 - clicintip Register"]
    pub clicintip_73: CLICINTIP_73,
    #[doc = "0x1125 - clicintie Register"]
    pub clicintie_73: CLICINTIE_73,
    #[doc = "0x1126 - clicintattr Register"]
    pub clicintattr_73: CLICINTATTR_73,
    #[doc = "0x1127 - clicintctl Register"]
    pub clicintctl_73: CLICINTCTL_73,
    #[doc = "0x1128 - clicintip Register"]
    pub clicintip_74: CLICINTIP_74,
    #[doc = "0x1129 - clicintie Register"]
    pub clicintie_74: CLICINTIE_74,
    #[doc = "0x112a - clicintattr Register"]
    pub clicintattr_74: CLICINTATTR_74,
    #[doc = "0x112b - clicintctl Register"]
    pub clicintctl_74: CLICINTCTL_74,
    #[doc = "0x112c - clicintip Register"]
    pub clicintip_75: CLICINTIP_75,
    #[doc = "0x112d - clicintie Register"]
    pub clicintie_75: CLICINTIE_75,
    #[doc = "0x112e - clicintattr Register"]
    pub clicintattr_75: CLICINTATTR_75,
    #[doc = "0x112f - clicintctl Register"]
    pub clicintctl_75: CLICINTCTL_75,
    #[doc = "0x1130 - clicintip Register"]
    pub clicintip_76: CLICINTIP_76,
    #[doc = "0x1131 - clicintie Register"]
    pub clicintie_76: CLICINTIE_76,
    #[doc = "0x1132 - clicintattr Register"]
    pub clicintattr_76: CLICINTATTR_76,
    #[doc = "0x1133 - clicintctl Register"]
    pub clicintctl_76: CLICINTCTL_76,
    #[doc = "0x1134 - clicintip Register"]
    pub clicintip_77: CLICINTIP_77,
    #[doc = "0x1135 - clicintie Register"]
    pub clicintie_77: CLICINTIE_77,
    #[doc = "0x1136 - clicintattr Register"]
    pub clicintattr_77: CLICINTATTR_77,
    #[doc = "0x1137 - clicintctl Register"]
    pub clicintctl_77: CLICINTCTL_77,
    #[doc = "0x1138 - clicintip Register"]
    pub clicintip_78: CLICINTIP_78,
    #[doc = "0x1139 - clicintie Register"]
    pub clicintie_78: CLICINTIE_78,
    #[doc = "0x113a - clicintattr Register"]
    pub clicintattr_78: CLICINTATTR_78,
    #[doc = "0x113b - clicintctl Register"]
    pub clicintctl_78: CLICINTCTL_78,
    #[doc = "0x113c - clicintip Register"]
    pub clicintip_79: CLICINTIP_79,
    #[doc = "0x113d - clicintie Register"]
    pub clicintie_79: CLICINTIE_79,
    #[doc = "0x113e - clicintattr Register"]
    pub clicintattr_79: CLICINTATTR_79,
    #[doc = "0x113f - clicintctl Register"]
    pub clicintctl_79: CLICINTCTL_79,
    #[doc = "0x1140 - clicintip Register"]
    pub clicintip_80: CLICINTIP_80,
    #[doc = "0x1141 - clicintie Register"]
    pub clicintie_80: CLICINTIE_80,
    #[doc = "0x1142 - clicintattr Register"]
    pub clicintattr_80: CLICINTATTR_80,
    #[doc = "0x1143 - clicintctl Register"]
    pub clicintctl_80: CLICINTCTL_80,
    #[doc = "0x1144 - clicintip Register"]
    pub clicintip_81: CLICINTIP_81,
    #[doc = "0x1145 - clicintie Register"]
    pub clicintie_81: CLICINTIE_81,
    #[doc = "0x1146 - clicintattr Register"]
    pub clicintattr_81: CLICINTATTR_81,
    #[doc = "0x1147 - clicintctl Register"]
    pub clicintctl_81: CLICINTCTL_81,
    #[doc = "0x1148 - clicintip Register"]
    pub clicintip_82: CLICINTIP_82,
    #[doc = "0x1149 - clicintie Register"]
    pub clicintie_82: CLICINTIE_82,
    #[doc = "0x114a - clicintattr Register"]
    pub clicintattr_82: CLICINTATTR_82,
    #[doc = "0x114b - clicintctl Register"]
    pub clicintctl_82: CLICINTCTL_82,
    #[doc = "0x114c - clicintip Register"]
    pub clicintip_83: CLICINTIP_83,
    #[doc = "0x114d - clicintie Register"]
    pub clicintie_83: CLICINTIE_83,
    #[doc = "0x114e - clicintattr Register"]
    pub clicintattr_83: CLICINTATTR_83,
    #[doc = "0x114f - clicintctl Register"]
    pub clicintctl_83: CLICINTCTL_83,
    #[doc = "0x1150 - clicintip Register"]
    pub clicintip_84: CLICINTIP_84,
    #[doc = "0x1151 - clicintie Register"]
    pub clicintie_84: CLICINTIE_84,
    #[doc = "0x1152 - clicintattr Register"]
    pub clicintattr_84: CLICINTATTR_84,
    #[doc = "0x1153 - clicintctl Register"]
    pub clicintctl_84: CLICINTCTL_84,
    _reserved3: [u8; 1usize],
    #[doc = "0x1155 - clicintie Register"]
    pub clicintie_85: CLICINTIE_85,
    #[doc = "0x1156 - clicintattr Register"]
    pub clicintattr_85: CLICINTATTR_85,
    #[doc = "0x1157 - clicintctl Register"]
    pub clicintctl_85: CLICINTCTL_85,
    #[doc = "0x1158 - clicintip Register"]
    pub clicintip_85: CLICINTIP_85,
    #[doc = "0x1159 - clicintie Register"]
    pub clicintie_86: CLICINTIE_86,
    #[doc = "0x115a - clicintattr Register"]
    pub clicintattr_86: CLICINTATTR_86,
    #[doc = "0x115b - clicintctl Register"]
    pub clicintctl_86: CLICINTCTL_86,
    #[doc = "0x115c - clicintip Register"]
    pub clicintip_86: CLICINTIP_86,
}
#[doc = "cliccfg Register"]
pub struct CLICCFG {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "cliccfg Register"]
pub mod cliccfg;
#[doc = "clicinfo Register"]
pub struct CLICINFO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "clicinfo Register"]
pub mod clicinfo;
#[doc = "MTH Register"]
pub struct MTH {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "MTH Register"]
pub mod mth;
#[doc = "clicintip Register"]
pub struct CLICINTIP_0 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_0;
#[doc = "clicintip Register"]
pub struct CLICINTIP_1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_1;
#[doc = "clicintip Register"]
pub struct CLICINTIP_2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_2;
#[doc = "clicintip Register"]
pub struct CLICINTIP_3 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_3;
#[doc = "clicintip Register"]
pub struct CLICINTIP_4 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_4;
#[doc = "clicintip Register"]
pub struct CLICINTIP_5 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_5;
#[doc = "clicintip Register"]
pub struct CLICINTIP_6 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_6;
#[doc = "clicintip Register"]
pub struct CLICINTIP_7 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_7;
#[doc = "clicintip Register"]
pub struct CLICINTIP_8 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_8;
#[doc = "clicintip Register"]
pub struct CLICINTIP_9 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_9;
#[doc = "clicintip Register"]
pub struct CLICINTIP_10 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_10;
#[doc = "clicintip Register"]
pub struct CLICINTIP_11 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_11;
#[doc = "clicintip Register"]
pub struct CLICINTIP_12 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_12;
#[doc = "clicintip Register"]
pub struct CLICINTIP_13 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_13;
#[doc = "clicintip Register"]
pub struct CLICINTIP_14 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_14;
#[doc = "clicintip Register"]
pub struct CLICINTIP_15 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_15;
#[doc = "clicintip Register"]
pub struct CLICINTIP_16 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_16;
#[doc = "clicintip Register"]
pub struct CLICINTIP_17 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_17;
#[doc = "clicintip Register"]
pub struct CLICINTIP_18 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_18;
#[doc = "clicintip Register"]
pub struct CLICINTIP_19 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_19;
#[doc = "clicintip Register"]
pub struct CLICINTIP_20 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_20;
#[doc = "clicintip Register"]
pub struct CLICINTIP_21 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_21;
#[doc = "clicintip Register"]
pub struct CLICINTIP_22 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_22;
#[doc = "clicintip Register"]
pub struct CLICINTIP_23 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_23;
#[doc = "clicintip Register"]
pub struct CLICINTIP_24 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_24;
#[doc = "clicintip Register"]
pub struct CLICINTIP_25 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_25;
#[doc = "clicintip Register"]
pub struct CLICINTIP_26 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_26;
#[doc = "clicintip Register"]
pub struct CLICINTIP_27 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_27;
#[doc = "clicintip Register"]
pub struct CLICINTIP_28 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_28;
#[doc = "clicintip Register"]
pub struct CLICINTIP_29 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_29;
#[doc = "clicintip Register"]
pub struct CLICINTIP_30 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_30;
#[doc = "clicintip Register"]
pub struct CLICINTIP_31 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_31;
#[doc = "clicintip Register"]
pub struct CLICINTIP_32 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_32;
#[doc = "clicintip Register"]
pub struct CLICINTIP_33 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_33;
#[doc = "clicintip Register"]
pub struct CLICINTIP_34 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_34;
#[doc = "clicintip Register"]
pub struct CLICINTIP_35 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_35;
#[doc = "clicintip Register"]
pub struct CLICINTIP_36 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_36;
#[doc = "clicintip Register"]
pub struct CLICINTIP_37 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_37;
#[doc = "clicintip Register"]
pub struct CLICINTIP_38 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_38;
#[doc = "clicintip Register"]
pub struct CLICINTIP_39 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_39;
#[doc = "clicintip Register"]
pub struct CLICINTIP_40 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_40;
#[doc = "clicintip Register"]
pub struct CLICINTIP_41 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_41;
#[doc = "clicintip Register"]
pub struct CLICINTIP_42 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_42;
#[doc = "clicintip Register"]
pub struct CLICINTIP_43 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_43;
#[doc = "clicintip Register"]
pub struct CLICINTIP_44 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_44;
#[doc = "clicintip Register"]
pub struct CLICINTIP_45 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_45;
#[doc = "clicintip Register"]
pub struct CLICINTIP_46 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_46;
#[doc = "clicintip Register"]
pub struct CLICINTIP_47 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_47;
#[doc = "clicintip Register"]
pub struct CLICINTIP_48 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_48;
#[doc = "clicintip Register"]
pub struct CLICINTIP_49 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_49;
#[doc = "clicintip Register"]
pub struct CLICINTIP_50 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_50;
#[doc = "clicintip Register"]
pub struct CLICINTIP_51 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_51;
#[doc = "clicintip Register"]
pub struct CLICINTIP_52 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_52;
#[doc = "clicintip Register"]
pub struct CLICINTIP_53 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_53;
#[doc = "clicintip Register"]
pub struct CLICINTIP_54 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_54;
#[doc = "clicintip Register"]
pub struct CLICINTIP_55 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_55;
#[doc = "clicintip Register"]
pub struct CLICINTIP_56 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_56;
#[doc = "clicintip Register"]
pub struct CLICINTIP_57 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_57;
#[doc = "clicintip Register"]
pub struct CLICINTIP_58 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_58;
#[doc = "clicintip Register"]
pub struct CLICINTIP_59 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_59;
#[doc = "clicintip Register"]
pub struct CLICINTIP_60 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_60;
#[doc = "clicintip Register"]
pub struct CLICINTIP_61 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_61;
#[doc = "clicintip Register"]
pub struct CLICINTIP_62 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_62;
#[doc = "clicintip Register"]
pub struct CLICINTIP_63 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_63;
#[doc = "clicintip Register"]
pub struct CLICINTIP_64 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_64;
#[doc = "clicintip Register"]
pub struct CLICINTIP_65 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_65;
#[doc = "clicintip Register"]
pub struct CLICINTIP_66 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_66;
#[doc = "clicintip Register"]
pub struct CLICINTIP_67 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_67;
#[doc = "clicintip Register"]
pub struct CLICINTIP_68 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_68;
#[doc = "clicintip Register"]
pub struct CLICINTIP_69 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_69;
#[doc = "clicintip Register"]
pub struct CLICINTIP_70 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_70;
#[doc = "clicintip Register"]
pub struct CLICINTIP_71 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_71;
#[doc = "clicintip Register"]
pub struct CLICINTIP_72 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_72;
#[doc = "clicintip Register"]
pub struct CLICINTIP_73 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_73;
#[doc = "clicintip Register"]
pub struct CLICINTIP_74 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_74;
#[doc = "clicintip Register"]
pub struct CLICINTIP_75 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_75;
#[doc = "clicintip Register"]
pub struct CLICINTIP_76 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_76;
#[doc = "clicintip Register"]
pub struct CLICINTIP_77 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_77;
#[doc = "clicintip Register"]
pub struct CLICINTIP_78 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_78;
#[doc = "clicintip Register"]
pub struct CLICINTIP_79 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_79;
#[doc = "clicintip Register"]
pub struct CLICINTIP_80 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_80;
#[doc = "clicintip Register"]
pub struct CLICINTIP_81 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_81;
#[doc = "clicintip Register"]
pub struct CLICINTIP_82 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_82;
#[doc = "clicintip Register"]
pub struct CLICINTIP_83 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_83;
#[doc = "clicintip Register"]
pub struct CLICINTIP_84 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_84;
#[doc = "clicintip Register"]
pub struct CLICINTIP_85 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_85;
#[doc = "clicintip Register"]
pub struct CLICINTIP_86 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintip Register"]
pub mod clicintip_86;
#[doc = "clicintie Register"]
pub struct CLICINTIE_0 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_0;
#[doc = "clicintie Register"]
pub struct CLICINTIE_1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_1;
#[doc = "clicintie Register"]
pub struct CLICINTIE_2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_2;
#[doc = "clicintie Register"]
pub struct CLICINTIE_3 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_3;
#[doc = "clicintie Register"]
pub struct CLICINTIE_4 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_4;
#[doc = "clicintie Register"]
pub struct CLICINTIE_5 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_5;
#[doc = "clicintie Register"]
pub struct CLICINTIE_6 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_6;
#[doc = "clicintie Register"]
pub struct CLICINTIE_7 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_7;
#[doc = "clicintie Register"]
pub struct CLICINTIE_8 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_8;
#[doc = "clicintie Register"]
pub struct CLICINTIE_9 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_9;
#[doc = "clicintie Register"]
pub struct CLICINTIE_10 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_10;
#[doc = "clicintie Register"]
pub struct CLICINTIE_11 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_11;
#[doc = "clicintie Register"]
pub struct CLICINTIE_12 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_12;
#[doc = "clicintie Register"]
pub struct CLICINTIE_13 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_13;
#[doc = "clicintie Register"]
pub struct CLICINTIE_14 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_14;
#[doc = "clicintie Register"]
pub struct CLICINTIE_15 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_15;
#[doc = "clicintie Register"]
pub struct CLICINTIE_16 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_16;
#[doc = "clicintie Register"]
pub struct CLICINTIE_17 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_17;
#[doc = "clicintie Register"]
pub struct CLICINTIE_18 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_18;
#[doc = "clicintie Register"]
pub struct CLICINTIE_19 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_19;
#[doc = "clicintie Register"]
pub struct CLICINTIE_20 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_20;
#[doc = "clicintie Register"]
pub struct CLICINTIE_21 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_21;
#[doc = "clicintie Register"]
pub struct CLICINTIE_22 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_22;
#[doc = "clicintie Register"]
pub struct CLICINTIE_23 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_23;
#[doc = "clicintie Register"]
pub struct CLICINTIE_24 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_24;
#[doc = "clicintie Register"]
pub struct CLICINTIE_25 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_25;
#[doc = "clicintie Register"]
pub struct CLICINTIE_26 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_26;
#[doc = "clicintie Register"]
pub struct CLICINTIE_27 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_27;
#[doc = "clicintie Register"]
pub struct CLICINTIE_28 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_28;
#[doc = "clicintie Register"]
pub struct CLICINTIE_29 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_29;
#[doc = "clicintie Register"]
pub struct CLICINTIE_30 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_30;
#[doc = "clicintie Register"]
pub struct CLICINTIE_31 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_31;
#[doc = "clicintie Register"]
pub struct CLICINTIE_32 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_32;
#[doc = "clicintie Register"]
pub struct CLICINTIE_33 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_33;
#[doc = "clicintie Register"]
pub struct CLICINTIE_34 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_34;
#[doc = "clicintie Register"]
pub struct CLICINTIE_35 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_35;
#[doc = "clicintie Register"]
pub struct CLICINTIE_36 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_36;
#[doc = "clicintie Register"]
pub struct CLICINTIE_37 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_37;
#[doc = "clicintie Register"]
pub struct CLICINTIE_38 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_38;
#[doc = "clicintie Register"]
pub struct CLICINTIE_39 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_39;
#[doc = "clicintie Register"]
pub struct CLICINTIE_40 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_40;
#[doc = "clicintie Register"]
pub struct CLICINTIE_41 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_41;
#[doc = "clicintie Register"]
pub struct CLICINTIE_42 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_42;
#[doc = "clicintie Register"]
pub struct CLICINTIE_43 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_43;
#[doc = "clicintie Register"]
pub struct CLICINTIE_44 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_44;
#[doc = "clicintie Register"]
pub struct CLICINTIE_45 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_45;
#[doc = "clicintie Register"]
pub struct CLICINTIE_46 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_46;
#[doc = "clicintie Register"]
pub struct CLICINTIE_47 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_47;
#[doc = "clicintie Register"]
pub struct CLICINTIE_48 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_48;
#[doc = "clicintie Register"]
pub struct CLICINTIE_49 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_49;
#[doc = "clicintie Register"]
pub struct CLICINTIE_50 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_50;
#[doc = "clicintie Register"]
pub struct CLICINTIE_51 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_51;
#[doc = "clicintie Register"]
pub struct CLICINTIE_52 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_52;
#[doc = "clicintie Register"]
pub struct CLICINTIE_53 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_53;
#[doc = "clicintie Register"]
pub struct CLICINTIE_54 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_54;
#[doc = "clicintie Register"]
pub struct CLICINTIE_55 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_55;
#[doc = "clicintie Register"]
pub struct CLICINTIE_56 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_56;
#[doc = "clicintie Register"]
pub struct CLICINTIE_57 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_57;
#[doc = "clicintie Register"]
pub struct CLICINTIE_58 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_58;
#[doc = "clicintie Register"]
pub struct CLICINTIE_59 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_59;
#[doc = "clicintie Register"]
pub struct CLICINTIE_60 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_60;
#[doc = "clicintie Register"]
pub struct CLICINTIE_61 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_61;
#[doc = "clicintie Register"]
pub struct CLICINTIE_62 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_62;
#[doc = "clicintie Register"]
pub struct CLICINTIE_63 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_63;
#[doc = "clicintie Register"]
pub struct CLICINTIE_64 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_64;
#[doc = "clicintie Register"]
pub struct CLICINTIE_65 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_65;
#[doc = "clicintie Register"]
pub struct CLICINTIE_66 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_66;
#[doc = "clicintie Register"]
pub struct CLICINTIE_67 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_67;
#[doc = "clicintie Register"]
pub struct CLICINTIE_68 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_68;
#[doc = "clicintie Register"]
pub struct CLICINTIE_69 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_69;
#[doc = "clicintie Register"]
pub struct CLICINTIE_70 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_70;
#[doc = "clicintie Register"]
pub struct CLICINTIE_71 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_71;
#[doc = "clicintie Register"]
pub struct CLICINTIE_72 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_72;
#[doc = "clicintie Register"]
pub struct CLICINTIE_73 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_73;
#[doc = "clicintie Register"]
pub struct CLICINTIE_74 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_74;
#[doc = "clicintie Register"]
pub struct CLICINTIE_75 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_75;
#[doc = "clicintie Register"]
pub struct CLICINTIE_76 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_76;
#[doc = "clicintie Register"]
pub struct CLICINTIE_77 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_77;
#[doc = "clicintie Register"]
pub struct CLICINTIE_78 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_78;
#[doc = "clicintie Register"]
pub struct CLICINTIE_79 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_79;
#[doc = "clicintie Register"]
pub struct CLICINTIE_80 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_80;
#[doc = "clicintie Register"]
pub struct CLICINTIE_81 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_81;
#[doc = "clicintie Register"]
pub struct CLICINTIE_82 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_82;
#[doc = "clicintie Register"]
pub struct CLICINTIE_83 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_83;
#[doc = "clicintie Register"]
pub struct CLICINTIE_84 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_84;
#[doc = "clicintie Register"]
pub struct CLICINTIE_85 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_85;
#[doc = "clicintie Register"]
pub struct CLICINTIE_86 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintie Register"]
pub mod clicintie_86;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_0 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_0;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_1;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_2;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_3 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_3;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_4 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_4;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_5 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_5;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_6 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_6;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_7 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_7;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_8 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_8;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_9 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_9;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_10 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_10;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_11 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_11;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_12 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_12;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_13 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_13;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_14 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_14;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_15 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_15;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_16 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_16;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_17 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_17;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_18 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_18;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_19 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_19;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_20 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_20;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_21 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_21;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_22 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_22;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_23 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_23;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_24 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_24;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_25 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_25;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_26 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_26;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_27 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_27;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_28 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_28;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_29 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_29;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_30 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_30;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_31 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_31;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_32 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_32;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_33 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_33;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_34 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_34;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_35 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_35;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_36 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_36;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_37 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_37;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_38 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_38;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_39 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_39;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_40 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_40;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_41 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_41;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_42 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_42;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_43 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_43;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_44 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_44;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_45 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_45;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_46 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_46;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_47 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_47;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_48 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_48;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_49 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_49;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_50 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_50;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_51 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_51;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_52 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_52;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_53 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_53;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_54 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_54;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_55 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_55;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_56 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_56;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_57 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_57;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_58 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_58;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_59 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_59;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_60 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_60;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_61 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_61;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_62 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_62;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_63 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_63;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_64 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_64;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_65 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_65;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_66 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_66;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_67 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_67;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_68 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_68;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_69 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_69;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_70 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_70;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_71 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_71;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_72 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_72;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_73 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_73;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_74 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_74;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_75 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_75;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_76 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_76;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_77 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_77;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_78 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_78;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_79 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_79;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_80 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_80;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_81 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_81;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_82 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_82;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_83 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_83;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_84 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_84;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_85 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_85;
#[doc = "clicintattr Register"]
pub struct CLICINTATTR_86 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintattr Register"]
pub mod clicintattr_86;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_0 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_0;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_1;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_2;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_3 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_3;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_4 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_4;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_5 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_5;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_6 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_6;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_7 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_7;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_8 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_8;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_9 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_9;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_10 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_10;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_11 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_11;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_12 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_12;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_13 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_13;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_14 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_14;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_15 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_15;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_16 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_16;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_17 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_17;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_18 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_18;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_19 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_19;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_20 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_20;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_21 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_21;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_22 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_22;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_23 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_23;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_24 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_24;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_25 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_25;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_26 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_26;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_27 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_27;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_28 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_28;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_29 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_29;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_30 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_30;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_31 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_31;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_32 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_32;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_33 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_33;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_34 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_34;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_35 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_35;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_36 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_36;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_37 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_37;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_38 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_38;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_39 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_39;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_40 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_40;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_41 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_41;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_42 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_42;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_43 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_43;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_44 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_44;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_45 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_45;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_46 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_46;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_47 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_47;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_48 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_48;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_49 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_49;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_50 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_50;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_51 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_51;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_52 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_52;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_53 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_53;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_54 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_54;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_55 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_55;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_56 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_56;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_57 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_57;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_58 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_58;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_59 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_59;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_60 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_60;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_61 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_61;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_62 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_62;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_63 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_63;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_64 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_64;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_65 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_65;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_66 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_66;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_67 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_67;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_68 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_68;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_69 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_69;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_70 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_70;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_71 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_71;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_72 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_72;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_73 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_73;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_74 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_74;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_75 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_75;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_76 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_76;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_77 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_77;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_78 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_78;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_79 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_79;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_80 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_80;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_81 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_81;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_82 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_82;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_83 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_83;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_84 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_84;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_85 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_85;
#[doc = "clicintctl Register"]
pub struct CLICINTCTL_86 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "clicintctl Register"]
pub mod clicintctl_86;
