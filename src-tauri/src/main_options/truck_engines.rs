use super::trucks_engines_data::scania::{
    SCANIA_R_2009_360, SCANIA_R_2009_380, SCANIA_R_2009_400, SCANIA_R_2009_420, SCANIA_R_2009_440,
    SCANIA_R_2009_440_2, SCANIA_R_2009_480, SCANIA_R_2009_480_2, SCANIA_R_2009_500,
    SCANIA_R_2009_560, SCANIA_R_2009_620, SCANIA_R_2009_730, SCANIA_R_370, SCANIA_R_410,
    SCANIA_R_450, SCANIA_R_500, SCANIA_R_520, SCANIA_R_580, SCANIA_R_650, SCANIA_R_730,
    SCANIA_STREAMLINE_360, SCANIA_STREAMLINE_370, SCANIA_STREAMLINE_380, SCANIA_STREAMLINE_400,
    SCANIA_STREAMLINE_410, SCANIA_STREAMLINE_420, SCANIA_STREAMLINE_440, SCANIA_STREAMLINE_440_2,
    SCANIA_STREAMLINE_450, SCANIA_STREAMLINE_480, SCANIA_STREAMLINE_480_2, SCANIA_STREAMLINE_490,
    SCANIA_STREAMLINE_500, SCANIA_STREAMLINE_520, SCANIA_STREAMLINE_560, SCANIA_STREAMLINE_580,
    SCANIA_STREAMLINE_620, SCANIA_STREAMLINE_730, SCANIA_STREAMLINE_730_2, SCANIA_S_370,
    SCANIA_S_410, SCANIA_S_450, SCANIA_S_500, SCANIA_S_520, SCANIA_S_580, SCANIA_S_650,
    SCANIA_S_730,
};
use super::trucks_engines_data::volvo::{
    VOLVO_FH_2012_D13C_420, VOLVO_FH_2012_D13C_460, VOLVO_FH_2012_D13C_500, VOLVO_FH_2012_D13C_540,
    VOLVO_FH_2012_D13K_460, VOLVO_FH_2012_D16G_540, VOLVO_FH_2012_D16G_600, VOLVO_FH_2012_D16G_700,
    VOLVO_FH_2012_D16G_750, VOLVO_FH_CLASSIC_D13C_420, VOLVO_FH_CLASSIC_D13C_460,
    VOLVO_FH_CLASSIC_D13C_500, VOLVO_FH_CLASSIC_D13C_540, VOLVO_FH_CLASSIC_D16G_540,
    VOLVO_FH_CLASSIC_D16G_600, VOLVO_FH_CLASSIC_D16G_700, VOLVO_FH_CLASSIC_D16G_750,
};
use crate::structs::vec_trucks::EngineStruct;

pub const SCANIA_R_ENGINES: [EngineStruct<'static>; 8] = [
    SCANIA_R_370,
    SCANIA_R_410,
    SCANIA_R_450,
    SCANIA_R_500,
    SCANIA_R_520,
    SCANIA_R_580,
    SCANIA_R_650,
    SCANIA_R_730,
];

pub const SCANIA_S_ENGINES: [EngineStruct<'static>; 8] = [
    SCANIA_S_370,
    SCANIA_S_410,
    SCANIA_S_450,
    SCANIA_S_500,
    SCANIA_S_520,
    SCANIA_S_580,
    SCANIA_S_650,
    SCANIA_S_730,
];

pub const SCANIA_R_2009_ENGINES: [EngineStruct<'static>; 14] = [
    SCANIA_R_2009_360,
    SCANIA_R_2009_380,
    SCANIA_R_2009_400,
    SCANIA_R_2009_420,
    SCANIA_R_2009_440,
    SCANIA_R_2009_440_2,
    SCANIA_R_2009_480,
    SCANIA_R_2009_480_2,
    SCANIA_R_2009_500,
    SCANIA_R_2009_560,
    SCANIA_R_2009_620,
    SCANIA_R_2009_730,
    SCANIA_R_2009_730,
    SCANIA_R_2009_730,
];

pub const SCANIA_STREAMLINE_ENGINES: [EngineStruct<'static>; 19] = [
    SCANIA_STREAMLINE_360,
    SCANIA_STREAMLINE_370,
    SCANIA_STREAMLINE_380,
    SCANIA_STREAMLINE_400,
    SCANIA_STREAMLINE_410,
    SCANIA_STREAMLINE_420,
    SCANIA_STREAMLINE_440,
    SCANIA_STREAMLINE_440_2,
    SCANIA_STREAMLINE_450,
    SCANIA_STREAMLINE_480,
    SCANIA_STREAMLINE_480_2,
    SCANIA_STREAMLINE_490,
    SCANIA_STREAMLINE_500,
    SCANIA_STREAMLINE_520,
    SCANIA_STREAMLINE_560,
    SCANIA_STREAMLINE_580,
    SCANIA_STREAMLINE_620,
    SCANIA_STREAMLINE_730,
    SCANIA_STREAMLINE_730_2,
];

pub const VOLVO_FH_CLASSIC: [EngineStruct<'static>; 9] = [
    VOLVO_FH_2012_D13C_420,
    VOLVO_FH_2012_D13C_460,
    VOLVO_FH_2012_D13C_500,
    VOLVO_FH_2012_D13C_540,
    VOLVO_FH_2012_D13K_460,
    VOLVO_FH_2012_D16G_540,
    VOLVO_FH_2012_D16G_600,
    VOLVO_FH_2012_D16G_700,
    VOLVO_FH_2012_D16G_750,
];

pub const VOLVO_FH: [EngineStruct<'static>; 8] = [
    VOLVO_FH_CLASSIC_D13C_420,
    VOLVO_FH_CLASSIC_D13C_460,
    VOLVO_FH_CLASSIC_D13C_500,
    VOLVO_FH_CLASSIC_D13C_540,
    VOLVO_FH_CLASSIC_D16G_540,
    VOLVO_FH_CLASSIC_D16G_600,
    VOLVO_FH_CLASSIC_D16G_700,
    VOLVO_FH_CLASSIC_D16G_750,
];
