use super::trucks_transmissions_data::scania::{
    SCANIA_R_2009_GA867, SCANIA_R_2009_GA867R, SCANIA_R_2009_GRSO_905, SCANIA_R_2009_GRSO_905R,
    SCANIA_R_2009_GRSO_925, SCANIA_R_2009_GRSO_925R, SCANIA_R_2009_GRS_905, SCANIA_R_2009_GRS_905R,
    SCANIA_R_GA867, SCANIA_R_GA867R, SCANIA_R_GRSO_905, SCANIA_R_GRSO_905R, SCANIA_R_GRSO_925,
    SCANIA_R_GRSO_925R, SCANIA_R_GRS_905, SCANIA_R_GRS_905R, SCANIA_STREAMLINE_GA867,
    SCANIA_STREAMLINE_GA867R, SCANIA_STREAMLINE_GRSO_905, SCANIA_STREAMLINE_GRSO_905R,
    SCANIA_STREAMLINE_GRSO_925, SCANIA_STREAMLINE_GRSO_925R, SCANIA_STREAMLINE_GRS_905,
    SCANIA_STREAMLINE_GRS_905R, SCANIA_S_GA867, SCANIA_S_GA867R, SCANIA_S_GRSO_905,
    SCANIA_S_GRSO_905R, SCANIA_S_GRSO_925, SCANIA_S_GRSO_925R, SCANIA_S_GRS_905, SCANIA_S_GRS_905R,
};
use super::trucks_transmissions_data::volvo::{
    VOLVO_FH_4500, VOLVO_FH_4500_R, VOLVO_FH_AT2812D, VOLVO_FH_AT2812D_R, VOLVO_FH_ATO3512D,
    VOLVO_FH_ATO3512D_R, VOLVO_FH_ATO3512F_ASO_ULC, VOLVO_FH_ATO3512F_R_ASO_ULC,
    VOLVO_FH_CLASSIC_4500, VOLVO_FH_CLASSIC_4500_R, VOLVO_FH_CLASSIC_AT2812D,
    VOLVO_FH_CLASSIC_AT2812D_R, VOLVO_FH_CLASSIC_ATO3512D, VOLVO_FH_CLASSIC_ATO3512D_R,
    VOLVO_FH_CLASSIC_ATO3512F_ASO_ULC, VOLVO_FH_CLASSIC_ATO3512F_R_ASO_ULC,
};

use super::trucks_transmissions_data::man::{
    MAN_TGX_ZF12, MAN_TGX_ZF12_O, MAN_TGX_ZF12_R, MAN_TGX_ZF12_RO, MAN_TGX_ALLISON, MAN_TGX_ALLISON_R,
    MAN_TGX_E6_ALLISON, MAN_TGX_E6_ALLISON_R, MAN_TGX_E6_DD, MAN_TGX_E6_DD_R, MAN_TGX_E6_OD, MAN_TGX_E6_OD_R,
    MAN_TGX_E6_ZF, MAN_TGX_E6_ZF_R
};

use super::trucks_transmissions_data::iveco::{
    IVECO_HIWAY_ZF12, IVECO_HIWAY_ZF12_O, IVECO_HIWAY_ZF12_R, IVECO_HIWAY_ZF12_RO, IVECO_HIWAY_ALLISON, 
    IVECO_HIWAY_ALLISON_R, IVECO_HIWAY_ZF16, IVECO_HIWAY_ZF16_R,IVECO_STRALIS_ZF12, IVECO_STRALIS_ZF12_O, 
    IVECO_STRALIS_ZF12_R, IVECO_STRALIS_ZF12_RO,IVECO_STRALIS_ALLISON,IVECO_STRALIS_ALLISON_R, IVECO_STRALIS_ZF16, 
    IVECO_STRALIS_ZF16_R,
};

use super::trucks_transmissions_data::daf::{
    DAF_XF_ZF12, DAF_XF_ZF12_O, DAF_XF_ZF12_R, DAF_XF_ZF12_RO, DAF_XF_ALLISON, DAF_XF_ALLISON_R,DAF_XF_ZF16, DAF_XF_ZF16_R,
    DAF_XF_EURO6_ZF12, DAF_XF_EURO6_ZF12_O, DAF_XF_EURO6_ZF12_R, DAF_XF_EURO6_ZF12_RO, DAF_XF_EURO6_ALLISON, DAF_XF_EURO6_ALLISON_R,
    DAF_XF_EURO6_ZF_12TX, DAF_XF_EURO6_ZF_12TX_R, DAF_XF_EURO6_ZF_12T, DAF_XF_EURO6_ZF_12T_R, DAF_XF_EURO6_ZF_16A, DAF_XF_EURO6_ZF_16A_R,
    DAF_XF_EURO6_ZF16, DAF_XF_EURO6_ZF16_R,DAF_2021_ZF12T, DAF_2021_ZF12T_R, DAF_2021_ZF12, DAF_2021_ZF12_R, DAF_2021_ZF16, DAF_2021_ZF16_R,
};

use super::trucks_transmissions_data::mercedes::{
    MERCEDES_ACTROS_G12, MERCEDES_ACTROS_G12_O, MERCEDES_ACTROS_G12_R, MERCEDES_ACTROS_G12_RO, MERCEDES_ACTROS_ALLISON, MERCEDES_ACTROS_ALLISON_R,
    MERCEDES_ACTROS_G16, MERCEDES_ACTROS_G16_R, MERCEDES_NEW_ACTROS_G12, MERCEDES_NEW_ACTROS_G12_O, MERCEDES_NEW_ACTROS_G12_R, MERCEDES_NEW_ACTROS_G12_RO,
    MERCEDES_NEW_ACTROS_ALLISON, MERCEDES_NEW_ACTROS_ALLISON_R, MERCEDES_NEW_ACTROS_G16, MERCEDES_NEW_ACTROS_G16_R,
};

use super::trucks_transmissions_data::renault::{
    RENAULT_PREMIUM_A12, RENAULT_PREMIUM_A12_O, RENAULT_PREMIUM_AT12_R, RENAULT_PREMIUM_AT12_RO, RENAULT_PREMIUM_ALLISON, RENAULT_PREMIUM_ALLISON_R,
    RENAULT_PREMIUM_ZF16, RENAULT_PREMIUM_ZF16_R, RENAULT_T_ALLISON, RENAULT_T_ALLISON_R, RENAULT_T_AT12, RENAULT_T_AT12_R, RENAULT_T_ATO12,
    RENAULT_T_AT14, RENAULT_T_AT14_R, RENAULT_MAGNUM_AT12, RENAULT_MAGNUM_AT12_O, RENAULT_MAGNUM_AT12_R, RENAULT_MAGNUM_AT12_RO,
    RENAULT_MAGNUM_ALLISON, RENAULT_MAGNUM_ALLISON_R, RENAULT_MAGNUM_ZF16, RENAULT_MAGNUM_ZF16_R,
};

use crate::structs::vec_trucks::TransmissionStruct;

pub const SCANIA_R_TRANSMISSION: [TransmissionStruct; 8] = [
    SCANIA_R_GA867,
    SCANIA_R_GA867R,
    SCANIA_R_GRSO_905,
    SCANIA_R_GRSO_905R,
    SCANIA_R_GRSO_925,
    SCANIA_R_GRSO_925R,
    SCANIA_R_GRS_905,
    SCANIA_R_GRS_905R,
];

pub const SCANIA_S_TRANSMISSION: [TransmissionStruct; 8] = [
    SCANIA_S_GA867,
    SCANIA_S_GA867R,
    SCANIA_S_GRSO_905,
    SCANIA_S_GRSO_905R,
    SCANIA_S_GRSO_925,
    SCANIA_S_GRSO_925R,
    SCANIA_S_GRS_905,
    SCANIA_S_GRS_905R,
];

pub const SCANIA_R_2009_TRANSMISSION: [TransmissionStruct; 8] = [
    SCANIA_R_2009_GA867,
    SCANIA_R_2009_GA867R,
    SCANIA_R_2009_GRSO_905,
    SCANIA_R_2009_GRSO_905R,
    SCANIA_R_2009_GRSO_925,
    SCANIA_R_2009_GRSO_925R,
    SCANIA_R_2009_GRS_905,
    SCANIA_R_2009_GRS_905R,
];

pub const SCANIA_STREAMLINE_TRANSMISSION: [TransmissionStruct; 8] = [
    SCANIA_STREAMLINE_GA867,
    SCANIA_STREAMLINE_GA867R,
    SCANIA_STREAMLINE_GRSO_905,
    SCANIA_STREAMLINE_GRSO_905R,
    SCANIA_STREAMLINE_GRSO_925,
    SCANIA_STREAMLINE_GRSO_925R,
    SCANIA_STREAMLINE_GRS_905,
    SCANIA_STREAMLINE_GRS_905R,
];

pub const VOLVO_FH_TRANSMISSION: [TransmissionStruct; 8] = [
    VOLVO_FH_AT2812D,
    VOLVO_FH_AT2812D_R,
    VOLVO_FH_ATO3512D,
    VOLVO_FH_ATO3512D_R,
    VOLVO_FH_ATO3512F_ASO_ULC,
    VOLVO_FH_ATO3512F_R_ASO_ULC,
    VOLVO_FH_4500,
    VOLVO_FH_4500_R,
];

pub const VOLVO_FH_CLASSIC_TRANSMISSION: [TransmissionStruct; 8] = [
    VOLVO_FH_CLASSIC_AT2812D,
    VOLVO_FH_CLASSIC_AT2812D_R,
    VOLVO_FH_CLASSIC_ATO3512D,
    VOLVO_FH_CLASSIC_ATO3512D_R,
    VOLVO_FH_CLASSIC_ATO3512F_ASO_ULC,
    VOLVO_FH_CLASSIC_ATO3512F_R_ASO_ULC,
    VOLVO_FH_CLASSIC_4500,
    VOLVO_FH_CLASSIC_4500_R,
];

pub const MAN_TGX_TRANSMISSION: [TransmissionStruct; 6] = [
    MAN_TGX_ZF12, 
    MAN_TGX_ZF12_O, 
    MAN_TGX_ZF12_R, 
    MAN_TGX_ZF12_RO, 
    MAN_TGX_ALLISON, 
    MAN_TGX_ALLISON_R,
];


pub const MAN_TGX_EU6_TRANSMISSION: [TransmissionStruct; 8] = [
    MAN_TGX_E6_ALLISON, 
    MAN_TGX_E6_ALLISON_R, 
    MAN_TGX_E6_DD, 
    MAN_TGX_E6_DD_R, 
    MAN_TGX_E6_OD, 
    MAN_TGX_E6_OD_R,
    MAN_TGX_E6_ZF, 
    MAN_TGX_E6_ZF_R,
];

pub const IVECO_HIWAY_TRANSMISSION: [TransmissionStruct; 8] = [
    IVECO_HIWAY_ZF12, 
    IVECO_HIWAY_ZF12_O, 
    IVECO_HIWAY_ZF12_R, 
    IVECO_HIWAY_ZF12_RO, 
    IVECO_HIWAY_ALLISON, 
    IVECO_HIWAY_ALLISON_R,
    IVECO_HIWAY_ZF16, 
    IVECO_HIWAY_ZF16_R,
];

pub const IVECO_STRALIS_TRANSMISSION: [TransmissionStruct; 8] = [
    IVECO_STRALIS_ZF12, 
    IVECO_STRALIS_ZF12_O, 
    IVECO_STRALIS_ZF12_R, 
    IVECO_STRALIS_ZF12_RO,
    IVECO_STRALIS_ALLISON,
    IVECO_STRALIS_ALLISON_R, 
    IVECO_STRALIS_ZF16, 
    IVECO_STRALIS_ZF16_R,

];

pub const DAF_XF_TRANSMISSION: [TransmissionStruct; 8] = [
    DAF_XF_ZF12, 
    DAF_XF_ZF12_O, 
    DAF_XF_ZF12_R, 
    DAF_XF_ZF12_RO, 
    DAF_XF_ALLISON, 
    DAF_XF_ALLISON_R,
    DAF_XF_ZF16, 
    DAF_XF_ZF16_R,
];

pub const DAF_XF_EU6_TRANSMISSION: [TransmissionStruct; 14] = [
    DAF_XF_EURO6_ZF12, 
    DAF_XF_EURO6_ZF12_O, 
    DAF_XF_EURO6_ZF12_R, 
    DAF_XF_EURO6_ZF12_RO, 
    DAF_XF_EURO6_ALLISON, 
    DAF_XF_EURO6_ALLISON_R,
    DAF_XF_EURO6_ZF_12TX, 
    DAF_XF_EURO6_ZF_12TX_R, 
    DAF_XF_EURO6_ZF_12T, 
    DAF_XF_EURO6_ZF_12T_R, 
    DAF_XF_EURO6_ZF_16A, 
    DAF_XF_EURO6_ZF_16A_R,
    DAF_XF_EURO6_ZF16, 
    DAF_XF_EURO6_ZF16_R,
];

pub const DAF_2021_TRANSMISSION: [TransmissionStruct; 6] = [
    DAF_2021_ZF12T, 
    DAF_2021_ZF12T_R, 
    DAF_2021_ZF12, 
    DAF_2021_ZF12_R, 
    DAF_2021_ZF16, 
    DAF_2021_ZF16_R,
];

pub const MERCEDES_ACTROS_TRANSMISSION: [TransmissionStruct; 8] = [
    MERCEDES_ACTROS_G12, 
    MERCEDES_ACTROS_G12_O, 
    MERCEDES_ACTROS_G12_R, 
    MERCEDES_ACTROS_G12_RO, 
    MERCEDES_ACTROS_ALLISON, 
    MERCEDES_ACTROS_ALLISON_R,
    MERCEDES_ACTROS_G16, 
    MERCEDES_ACTROS_G16_R, 
];

pub const MERCEDES_NEW_ACTROS_TRANSMISSION: [TransmissionStruct; 8] = [
    MERCEDES_NEW_ACTROS_G12, 
    MERCEDES_NEW_ACTROS_G12_O, 
    MERCEDES_NEW_ACTROS_G12_R, 
    MERCEDES_NEW_ACTROS_G12_RO,
    MERCEDES_NEW_ACTROS_ALLISON, 
    MERCEDES_NEW_ACTROS_ALLISON_R, 
    MERCEDES_NEW_ACTROS_G16, 
    MERCEDES_NEW_ACTROS_G16_R,
];

pub const RENAULT_PREMIUM_TRANSMISSION: [TransmissionStruct; 8] = [
    RENAULT_PREMIUM_A12, 
    RENAULT_PREMIUM_A12_O, 
    RENAULT_PREMIUM_AT12_R, 
    RENAULT_PREMIUM_AT12_RO, 
    RENAULT_PREMIUM_ALLISON,
    RENAULT_PREMIUM_ALLISON_R,
    RENAULT_PREMIUM_ZF16, 
    RENAULT_PREMIUM_ZF16_R, 
];

pub const RENAULT_T_TRANSMISSION: [TransmissionStruct; 7] = [
    RENAULT_T_ALLISON, 
    RENAULT_T_ALLISON_R, 
    RENAULT_T_AT12, 
    RENAULT_T_AT12_R, 
    RENAULT_T_ATO12,
    RENAULT_T_AT14, 
    RENAULT_T_AT14_R, 
];

pub const RENAULT_MAGNUM_TRANSMISSION: [TransmissionStruct; 8] = [
    RENAULT_MAGNUM_AT12, 
    RENAULT_MAGNUM_AT12_O, 
    RENAULT_MAGNUM_AT12_R, 
    RENAULT_MAGNUM_AT12_RO,
    RENAULT_MAGNUM_ALLISON, 
    RENAULT_MAGNUM_ALLISON_R, 
    RENAULT_MAGNUM_ZF16, 
    RENAULT_MAGNUM_ZF16_R, 
];