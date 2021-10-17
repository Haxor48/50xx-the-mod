#![allow(non_snake_case)]
#![allow(unused_imports)]

use smash::lib::{L2CValue, L2CAgent};
use skyline::nro::{self, NroInfo};
use smash::app::lua_bind::*;
use smash::hash40;
use smash::app::utility::*;
use smash::lua2cpp::{L2CFighterCommon, L2CFighterBase};
use smash::lib::lua_const::*;
use smash::phx::*;
use smashline::*;
use smash::app::*;
use crate::FIGHTER_CUTIN_MANAGER_ADDR;
use skyline::nn::ro::LookupSymbol;
use smash::app::GroundCorrectKind;
//use smash::params::*;


static mut LEDGE_POS: [Vector3f; 9] = [smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0}; 9];
static mut ECB_Y_OFFSETS: [f32; 9] = [0.0; 9];
static mut LOCKED: [bool; 9] = [false; 9];
static mut NOSPECIALFALL: [bool; 9] = [false; 9];
static mut RECENTSTATUSES: [[i32; 5]; 9] = [[0; 5]; 9];
static mut LAGCANCELED: [bool; 9] = [false; 9];
static mut DASHDIR: [f32; 9] = [0.0; 9];
static mut ISREVERSE: [bool; 9] = [false; 9];
static mut BREVERSE: [bool; 9] = [false; 9];
static mut ISFUNNY: [bool; 9] = [false; 9];
//Mewtwo disable globals
static mut DISABLESTART: [i32; 9] = [0; 9];
static mut ISDISABLED: [bool; 9] = [false; 9];
static mut DISABLEDMOVE: [i32; 9] = [0; 9];
//
static mut CANAIRDODGE: [bool; 9] = [true; 9];
static mut USEDUPB: [bool; 9] = [false; 9];
pub static mut GLOBALFRAMECOUNT: i32 = 1;
//Ivy meter shit
pub static mut AMOUNTSOLAR: [i32; 9] = [0; 9];
static mut SOLARSTART: [i32; 9] = [0; 9];
static mut CANHEAL: [bool; 9] = [true; 9];
//
static mut REGRABFRAMES: [u64; 9] = [1; 9];
static mut HITFALLBUFFER: [[bool; 6]; 9] = [[false; 6]; 9];
static mut DOUBLESWAP: [bool; 9] = [false; 9];
static mut REGAINUPB: [bool; 9] = [false; 9];
pub static mut CANPROJECTILE: [bool; 9] = [true; 9];
static mut CURRENTMOMENTUM: [f32; 9] = [0.0; 9];
static mut ISAUTOTURNAROUND: [bool; 9] = [true; 9];
static mut RISING: [[bool; 10]; 9] = [[false; 10]; 9];
static mut REVERSED: [i32; 9] = [0; 9];
static mut UPBCANCEL: [bool; 9] = [false; 9];
pub static mut LASTBARREL: [i32; 9] = [0; 9];
pub static mut CANBARREL: [bool; 9] = [true; 9];
static mut CANFLOAT: [bool; 9] = [true; 9];
static mut FLOATSTART: [i32; 9] = [0; 9];
static mut LASTFOXFRAME: [i32; 9] = [0; 9];
static mut LASTFRAME: i32 = 0;
pub static mut ZSSDAIR: [bool; 9] = [false; 9];
pub static mut OPPDASHSPEED: f32 = 0.0;
static mut HITSIDEB: [bool; 9] = [false; 9];
static mut TOGGELS: [[i32; 9]; 3] = [[0; 9]; 3];
static mut HITMOVE: [bool; 9] = [false; 9];
static mut ARSENEMETER: [f32; 9] = [15.0; 9];
static mut ISARSENE: [bool; 9] = [false; 9];
static mut MOVEDAMAGE: [f32; 9] = [0.0; 9];
static mut MOVE: [i32; 9] = [0; 9];
static mut LEECHSTART: [i32; 9] = [0; 9];
static mut NUMUPBS: [i32; 9] = [0; 9];
static mut ISATTACK: [bool; 9] = [false; 9];
static mut INCINSPEED: [f32; 9] = [1.0; 9];
static mut SPEEDSTART: [i32; 9] = [0; 9];
static mut CANDOWNB: [bool; 9] = [true; 9];
static mut FLASHSTART: [i32; 9] = [0; 9];
static mut LASTCOMBOHIT: [i32; 9] = [0; 9];
static mut TRUECOMBOCOUNT: [i32; 9] = [0; 9];
static mut COMBOCOUNT: [i32; 9] = [0; 9];
static mut ISSTUNNED: [bool; 9] = [false; 9];
pub static mut PUFFLEVELS: [[i32; 9]; 2] = [[0; 9]; 2];
static mut ROBINCANCEL: [bool; 9] = [false; 9];
static mut KBSTART: [i32; 9] = [0; 9];
static mut HORIZDJSTART: [i32; 9] = [0; 9];
//static mut BOMA: 0 as &mut BattleObjectModuleAccessor;
//pub static mut COMMON_PARAMS: *mut CommonParams = 0 as *mut _;

pub unsafe fn get_player_number(boma: &mut smash::app::BattleObjectModuleAccessor) -> usize {
    return WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
}

pub unsafe fn alt(boma: &mut smash::app::BattleObjectModuleAccessor, slot: i32) -> bool {
    return WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == slot;
}

pub unsafe fn abs(arg: f32) -> f32 {
    if arg < 0.0 {
        return arg * -1.0;
    }
    return arg;
}

pub unsafe fn returnSmall(arg1: f32, arg2: f32) -> f32{
    if arg1 < arg2 {
        return arg1;
    }
    else {
        return arg2;
    }
}

pub unsafe fn returnLarge(arg1: f32, arg2: f32) -> f32{
    if arg1 > arg2 {
        return arg1;
    }
    else {
        return arg2;
    }
}

pub unsafe fn isAttacking(status_kind: i32) -> bool {
    if [*FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_100, *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_HI4,
    *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_STATUS_KIND_AIR_LASSO].contains(&status_kind) {
        return true;
    }
    return false;
}

pub unsafe fn stick_y_flick_check(boma: &mut smash::app::BattleObjectModuleAccessor, flick_sensitivity: f32) -> bool {
    let stick_value_y = ControlModule::get_stick_y(boma);
    let cat2 = ControlModule::get_command_flag_cat(boma, 1);
    let stick_value_prev = ControlModule::get_stick_prev_y(boma);
    let flick_check = (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP) != 0;
    if flick_sensitivity < 0.0 && stick_value_y < flick_sensitivity && (stick_value_y < stick_value_prev || flick_check) {
        return true;
    }
    else if flick_sensitivity > 0.0 && stick_value_y > flick_sensitivity && (stick_value_y > stick_value_prev || flick_check) {
        return true;
    }
    return false;
}

pub unsafe fn stick_x_flick_check(boma: &mut smash::app::BattleObjectModuleAccessor, flick_sensitivity: f32) -> bool {
    let stick_value_x = ControlModule::get_stick_x(boma);
    let cat1 = ControlModule::get_command_flag_cat(boma, 0);
    let stick_value_prev = ControlModule::get_stick_prev_x(boma);
    let flick_check = (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN) != 0;
    if flick_sensitivity < 0.0 && stick_value_x < flick_sensitivity && (stick_value_x < stick_value_prev || flick_check) {
        return true;
    }
    else if flick_sensitivity > 0.0 && stick_value_x > flick_sensitivity && (stick_value_x > stick_value_prev || flick_check) {
        return true;
    }
    return false;
}

extern "C" {
    #[link_name = "\u{1}_ZN3app11FighterUtil30is_valid_just_shield_reflectorERNS_26BattleObjectModuleAccessorE"]
    fn is_valid_just_shield_reflector(boma: &mut smash::app::BattleObjectModuleAccessor) -> bool;
}

#[skyline::hook(replace=is_valid_just_shield_reflector)]
unsafe fn is_valid_just_shield_reflector_hook(boma: &mut smash::app::BattleObjectModuleAccessor) -> bool {
    return true;
}

/* pub static mut COMMON_PARAMS: *mut CommonParams = *mut smash::params::common::CommonParams(0);
fn params_main(params_info: &ParamsInfo<'_>) {
    if let Ok(common) = params_info.get::<CommonParams>() {
        unsafe {
            COMMON_PARAMS = *mut smash::params::common::CommonParams(common);
        }
    }
}
pub unsafe fn editParams (boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, fighter_kind: i32) {
    if COMMON_PARAMS != *mut smash::params::common::CommonParams(0) {
        let common = *COMMON_PARAMS;
        if status_kind == *FIGHTER_STATUS_KIND_THROW {
            if MotionModule::frame(boma) < 2.0 {
                let new_frames = REGRABFRAMES[get_player_number(boma)] + 15;
                println!("Added frames! New frames: {}", new_frames);
                REGRABFRAMES[get_player_number(boma)] = new_frames;
                common.capture_cut_frame = new_frames;
            }
        }
        if REGRABFRAMES[get_player_number(boma)] != 1 {
            let mut i = 0;
            let mut has_throw = false;
            while i < 5 {
                if RECENTSTATUSES[get_player_number(boma)][i] == *FIGHTER_STATUS_KIND_THROW {
                    has_throw = true;
                }
                i = i + 1;
            }
            if !has_throw {
                println!("Reset frames!");
                REGRABFRAMES[get_player_number(boma)] = 1;
                common.capture_cut_frame = 1;
            }
        }
    }
} */

/*
if COMMON_PARAMS != 0 {
    let common = *COMMON_PARAMS;
    common.shield_damage_mul = 100;
} */

/* #[skyline::hook(offset=0x4dae10)]
pub unsafe fn float_param_accessor_hook(boma: u64, param_type: u64, param_hash: u64) -> f32 {
    let module_accessor = &mut *(*((boma as *mut u64).offset(1)) as &mut BattleObjectModuleAccessor);
    let status_kind = StatusModule::status_kind(module_accessor);
    let stick_value_y = ControlModule::get_stick_y(module_accessor);
    let fighter_kind = get_kind(module_accessor);
    if param_hash == hash40("vl") {
        if param_type == hash40("shoot_angle") {
            if fighter_kind == *FIGHTER_KIND_SHIZUE {
                let test = 16.0 + (stick_value_y * 32.0);
                println!("new isabelle rod angle: {}", test);
                return test;
            }
        }
    }
    /* if param_hash == hash40("common") {
        if param_type == hash40("capture_cut_frame") {
            if status_kind == *FIGHTER_STATUS_KIND_THROW {
                if MotionModule::frame(boma) < 2.0 {
                    let new_frames = REGRABFRAMES[get_player_number(boma)] + 15;
                    println!("Added frames! New frames: {}", new_frames);
                    REGRABFRAMES[get_player_number(boma)] = new_frames;
                    return new_frames;
                }
            }
            if REGRABFRAMES[get_player_number(boma)] != 1 {
                let mut i = 0;
                let mut has_throw = false;
                while i < 5 {
                    if RECENTSTATUSES[get_player_number(boma)][i] == *FIGHTER_STATUS_KIND_THROW {
                        has_throw = true;
                    }
                    i = i + 1;
                }
                if !has_throw {
                    println!("Reset frames!");
                    REGRABFRAMES[get_player_number(boma)] = 1;
                    return 1;
                }
            }
        }
    } */
    original!()(boma, param_type, param_hash)
} */

#[skyline::hook(offset=0x4e4070)]
pub unsafe fn get_param_int_hook(module_accessor: u64, param_type: u64, param_hash: u64) -> i32 {
    let boma = &mut *(*((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let fighter_kind = get_kind(boma);
    original!()(module_accessor, param_type, param_hash)
}

/*#[skyline::hook(offset=INT_OFFSET + 0x20)]
pub unsafe fn get_param_int64_hook(x0: u64, param_type: u64, param_hash: u64) -> u64 {
    let mut boma = *((x0 as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor;
    let fighter_kind = get_kind(&mut *boma);
    let status_kind = StatusModule::status_kind(&mut *boma);
    if param_hash == hash40("common") {
        if param_type == hash40("capture_cut_frame") {
            if status_kind == *FIGHTER_STATUS_KIND_THROW {
                if MotionModule::frame(&mut *boma) < 2.0 {
                    let new_frames = REGRABFRAMES[get_player_number(&mut *boma)] + 15;
                    println!("Added frames! New frames: {}", new_frames);
                    REGRABFRAMES[get_player_number(&mut *boma)] = new_frames;
                    return new_frames;
                }
            }
            if REGRABFRAMES[get_player_number(&mut *boma)] != 1 {
                let mut i = 0;
                let mut has_throw = false;
                while i < 5 {
                    if RECENTSTATUSES[get_player_number(&mut *boma)][i] == *FIGHTER_STATUS_KIND_THROW {
                        has_throw = true;
                    }
                    i = i + 1;
                }
                if !has_throw {
                    println!("Reset frames!");
                    REGRABFRAMES[get_player_number(&mut *boma)] = 1;
                    return 1;
                }
            }
        }
    }
    original!()(x0, param_type, param_hash)
} */

#[skyline::hook(offset=0x4e40b0)]
pub unsafe fn get_param_float_hook(module_accessor: u64, param_type: u64, param_hash: u64) -> f32 {
    let boma = &mut *(*((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let fighter_kind = get_kind(boma);
    let status_kind = StatusModule::status_kind(boma);
    let stick_value_y = ControlModule::get_stick_y(boma);

    if param_hash == 0 {
        if [hash40("landing_attack_air_frame_n"), hash40("landing_attack_air_frame_hi"), hash40("landing_attack_air_frame_lw"), 
        hash40("landing_attack_air_frame_f"), hash40("landing_attack_air_frame_b")].contains(&param_type) {
            let origLandingLag = original!()(module_accessor, param_type, param_hash);
            let mut newLandingLag = 0;
            if LAGCANCELED[get_player_number(boma)] {
                newLandingLag = (origLandingLag / 1.25) as i32;
            }
            else {
                newLandingLag = (origLandingLag * 1.25) as i32;
            }
            return newLandingLag as f32;
        }
    }

    if fighter_kind == *FIGHTER_KIND_PZENIGAME {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
            if param_hash == 0 {
                if param_type == hash40("ground_brake") {
                    println!("Traction!");
                }
                if param_type == hash40("run_speed_max") {
                    println!("Max run speed!");
                }
            }
        }
    }

    if param_hash == hash40("vl") {
        if param_type == hash40("shoot_angle") {
            if fighter_kind == *FIGHTER_KIND_SHIZUE {
                let test = 16.0 + (stick_value_y * 32.0);
                println!("new isabelle rod angle: {}", test);
                return test;
            }
        }
    }

    original!()(module_accessor, param_type, param_hash)
}

#[skyline::hook(replace = smash::app::lua_bind::GroundModule::entry_cliff)]

pub unsafe fn entry_cliff_hook(boma: &mut smash::app::BattleObjectModuleAccessor) -> u64 {
    let entry_id = get_player_number(boma);
    LEDGE_POS[entry_id] = GroundModule::hang_cliff_pos_3f(boma);
    original!()(boma)
}

#[skyline::hook(replace = smash::app::lua_bind::GroundModule::leave_cliff)]

pub unsafe fn leave_cliff_hook(boma: &mut smash::app::BattleObjectModuleAccessor) -> u64 {
    let entry_id = get_player_number(boma);
    LEDGE_POS[entry_id] = smash::phx::Vector3f { x: 0.0, y: 0.0, z:0.0 };
    original!()(boma)
}

#[skyline::hook(replace = smash::app::lua_bind::GroundModule::can_entry_cliff)]

pub unsafe fn can_entry_cliff_hook(boma: &mut smash::app::BattleObjectModuleAccessor) -> u64 {
    let pos = GroundModule::hang_cliff_pos_3f(boma);
    let entry_id = get_player_number(boma);
    let status_kind = StatusModule::status_kind(boma);
    let fighter_kind = get_kind(boma);
    let motion_kind = MotionModule::motion_kind(boma);
    if fighter_kind == *FIGHTER_KIND_KOOPAJR {
        if KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) > WorkModule::get_param_float(boma, hash40("air_speed_y_stable"), 0) {
            return 0;
        }
    }
    if KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) > 0.0 { //Melee style sweetspots
        if ![*FIGHTER_KIND_PFUSHIGISOU, *FIGHTER_KIND_TANTAN, *FIGHTER_KIND_MASTER].contains(&fighter_kind) && status_kind != *FIGHTER_STATUS_KIND_AIR_LASSO && status_kind != 248 &&
        (fighter_kind != *FIGHTER_KIND_JACK || ![*FIGHTER_JACK_STATUS_KIND_SPECIAL_HI_CUT, *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI_THROW, *FIGHTER_STATUS_KIND_SPECIAL_HI].contains(&status_kind)) &&
        (fighter_kind != *FIGHTER_KIND_SHIZUE || ![*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_START, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_THROW].contains(&status_kind)) &&
        (![*FIGHTER_KIND_SIMON, *FIGHTER_KIND_RICHTER].contains(&fighter_kind) || status_kind != *FIGHTER_STATUS_KIND_ATTACK_AIR)  {
            return 0 as u64;
        }
    }
    if (status_kind != *FIGHTER_STATUS_KIND_FALL_AERIAL && status_kind != *FIGHTER_STATUS_KIND_JUMP_AERIAL && status_kind != *FIGHTER_STATUS_KIND_FALL && 
    status_kind != *FIGHTER_STATUS_KIND_FLY && status_kind != *FIGHTER_STATUS_KIND_AIR_LASSO && ![*FIGHTER_KIND_PFUSHIGISOU, *FIGHTER_KIND_MASTER, *FIGHTER_KIND_TANTAN].contains(&fighter_kind) && (fighter_kind != *FIGHTER_KIND_JACK ||  
        ![*FIGHTER_JACK_STATUS_KIND_SPECIAL_HI_CUT, *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI_THROW, *FIGHTER_STATUS_KIND_SPECIAL_HI].contains(&status_kind)) &&
        (![*FIGHTER_KIND_SIMON, *FIGHTER_KIND_RICHTER].contains(&fighter_kind) || status_kind != *FIGHTER_STATUS_KIND_ATTACK_AIR)) || motion_kind == 61345827621 { //Edgehog/trump
        for i in 0..9 {
            i as usize;
            if i == entry_id || LEDGE_POS[i].x == 0.0 {
                continue;
            }

            if pos.x == LEDGE_POS[i].x && pos.y == LEDGE_POS[i].y {
                return 0 as u64;
            }
        }
    }
    original!()(boma)
}

#[skyline::hook(replace = smash::app::lua_bind::WorkModule::is_enable_transition_term)]

pub unsafe fn is_enable_transition_term_hook(boma: &mut smash::app::BattleObjectModuleAccessor, flag: i32) -> bool {
    let status_kind = StatusModule::status_kind(boma);
    let fighter_kind = get_kind(boma);
    let motion_kind = MotionModule::motion_kind(boma);
    if [*FIGHTER_STATUS_KIND_DAMAGE_FALL, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR].contains(&status_kind) {
        if (flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR) {
            return false;
        }
        if [*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S, 
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI].contains(&flag) {
            if [*FIGHTER_KIND_FOX, *FIGHTER_KIND_FALCO, *FIGHTER_KIND_WOLF].contains(&fighter_kind) {
                if KBSTART[get_player_number(boma)] > 0 {
                    return false;
                }
            }
        }
    } 
    if flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI {
        if fighter_kind == *FIGHTER_KIND_PIT {
            if NUMUPBS[get_player_number(boma)] >= 2 {
                return false;
            }
        }
    }
    if flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW {
        if fighter_kind == *FIGHTER_KIND_GAOGAEN {
            return CANDOWNB[get_player_number(boma)];
        }
    }
    if flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR {
        if !CANAIRDODGE[get_player_number(boma)] {
            return false;
        }
    }
    if flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI {
        if USEDUPB[get_player_number(boma)] {
            if [*FIGHTER_KIND_PLIZARDON, *FIGHTER_KIND_CLOUD, *FIGHTER_KIND_PALUTENA, *FIGHTER_KIND_WIIFIT].contains(&fighter_kind) {
                return false;
            }
        }
    }
    if flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S {
        if fighter_kind == *FIGHTER_KIND_LITTLEMAC {
            return original!()(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR) || original!()(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK);
        }
    }
    if flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL {
        return false;
    }
    if fighter_kind == *FIGHTER_KIND_CAPTAIN {
        if motion_kind == hash40("special_s_end") {
            if MotionModule::frame(boma) < 19.0 {
                return false;
            }
        }
    }
    if DISABLEDMOVE[get_player_number(boma)] == flag && ISDISABLED[get_player_number(boma)] {
        return false;
    }

    original!()(boma, flag)
}

/* #[skyline::hook(replace = smash::app::lua_bind::CancelModule::is_enable_cancel)]
pub unsafe fn is_enable_cancel_hook (boma: &mut smash::app::BattleObjectModuleAccessor) -> bool {
    let motion_kind = MotionModule::motion_kind(boma);
    let status_kind = StatusModule::status_kind(boma);
    if [*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_100, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_HI4, 
    *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_LW3].contains(&status_kind) {
        if LAGCANCELED[get_player_number(boma)] {
            if MotionModule::frame(boma) >= (FighterMotionModuleImpl::get_cancel_frame(boma, Hash40{hash: motion_kind}, false) as f32 / 1.1) {
                return true;
            }
            else {
                return false;
            }
        }
        else {
            if MotionModule::frame(boma) >= (FighterMotionModuleImpl::get_cancel_frame(boma, Hash40{hash: motion_kind}, false) as f32 * 1.1) {
                return true;
            }
            else {
                return false;
            }
        }
    }
    original!()(boma)
} */

/* #[skyline::hook(replace = smash::app::lua_bind::ControlModule::check_button_on)]
pub unsafe fn check_button_on_hook(boma: &mut smash::app::BattleObjectModuleAccessor, flag: i32) -> bool {
    if flag == *CONTROL_PAD_BUTTON_CSTICK_ON {
        return ControlModule::check_button_trigger(boma, flag);
    }
    original!()(boma, flag)
}
#[skyline::hook(replace = smash::app::lua_bind::ControlModule::check_button_off)]
pub unsafe fn check_button_off_hook(boma: &mut smash::app::BattleObjectModuleAccessor, flag: i32) -> bool {
    if flag == *CONTROL_PAD_BUTTON_CSTICK_ON {
        return !ControlModule::check_button_trigger(boma, flag);
    }
    original!()(boma, flag)
} */

#[skyline::hook(replace = smash::app::lua_bind::WorkModule::get_int)]

pub unsafe fn get_int_hook(boma: &mut smash::app::BattleObjectModuleAccessor, instance: i32) -> i32 {
    let fighter_kind = get_kind(boma);
    if instance == *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT {
        if [*FIGHTER_KIND_CAPTAIN, *FIGHTER_KIND_GANON].contains(&fighter_kind) {
            if REGAINUPB[get_player_number(boma)] {
                if original!()(boma, instance) == 2 {
                    REGAINUPB[get_player_number(boma)] = false;
                    return 1;
                }
            }
        }
        else if fighter_kind == *FIGHTER_KIND_MEWTWO {
            if UPBCANCEL[get_player_number(boma)] {
                if original!()(boma, instance) == 1 {
                    return 2;
                }
            }
        }
    }
    original!()(boma, instance)
}

#[skyline::hook(replace = smash::app::lua_bind::WorkModule::get_float)]

pub unsafe fn get_float_hook(boma: &mut smash::app::BattleObjectModuleAccessor, instance: i32) -> f32 {
    let status_kind = StatusModule::status_kind(boma);
    /*if isAttacking(status_kind) {
        if instance == *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLOAT_REVENGE_RATE {
            return 1.0;
        }
    } */
    original!()(boma, instance)
}

#[skyline::hook(replace=StatusModule::init_settings)]
unsafe fn init_settings_hook(boma: &mut BattleObjectModuleAccessor, situation: smash::app::SituationKind, arg3: i32, arg4: u32,
                             ground_cliff_check_kind: smash::app::GroundCliffCheckKind, arg6: bool,
                             arg7: i32, arg8: i32, arg9: i32, arg10: i32) -> u64 {
    let category = get_category(boma);
    let fighter_kind = get_kind(boma);
    let status_kind = StatusModule::status_kind(boma);
    let situation_kind = StatusModule::situation_kind(boma);
    //
    // Call edge_slippoffs init_settings
    let fix = init_settings_edges(boma, situation, arg3, arg4, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10);

    original!()(boma, situation, arg3, fix, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10)
}

#[skyline::hook(replace=GroundModule::correct)]
unsafe fn correct_hook(boma: &mut BattleObjectModuleAccessor, kind: GroundCorrectKind) -> u64 {
    let status_kind = StatusModule::status_kind(boma);
    let situation_kind = StatusModule::situation_kind(boma);
    let fighter_kind = get_kind(boma);
    let category = get_category(boma);

    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_DASH,
        *FIGHTER_STATUS_KIND_TURN_RUN, *FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, *FIGHTER_STATUS_KIND_WAIT].contains(&status_kind) {
            return original!()(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }
        if situation_kind == *SITUATION_KIND_GROUND {
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH {
                if [*FIGHTER_KIND_BUDDY, *FIGHTER_KIND_DONKEY, *FIGHTER_KIND_KIRBY].contains(&fighter_kind) {
                    return original!()(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                }
            }
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S { //Side b's
                if [*FIGHTER_KIND_INKLING, *FIGHTER_KIND_LITTLEMAC, *FIGHTER_KIND_BAYONETTA, *FIGHTER_KIND_DIDDY, *FIGHTER_KIND_SHIZUE, *FIGHTER_KIND_MARTH, *FIGHTER_KIND_JACK, *FIGHTER_KIND_GANON].contains(&fighter_kind) {
                    return original!()(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                }
            }
            if fighter_kind == *FIGHTER_KIND_JACK { // Joker gun edge cancels
                if [*FIGHTER_JACK_STATUS_KIND_SPECIAL_N_ESCAPE, *FIGHTER_JACK_STATUS_KIND_SPECIAL_N_LANDING, *FIGHTER_JACK_STATUS_KIND_SPECIAL_N_BARRAGE_END].contains(&status_kind) {
                    return original!()(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                }
            }
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI { //Up specials
                if [*FIGHTER_KIND_CLOUD, *FIGHTER_KIND_PIKACHU, *FIGHTER_KIND_GEKKOUGA, *FIGHTER_KIND_INKLING, *FIGHTER_KIND_PICHU].contains(&fighter_kind) {
                    return original!()(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                }
            }
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N { //Neutral Specials
                if [*FIGHTER_KIND_JACK, *FIGHTER_KIND_MARTH, *FIGHTER_KIND_PLIZARDON].contains(&fighter_kind) {
                    return original!()(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                }
            }
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW { //Down Special edgecancels
                if [*FIGHTER_KIND_CAPTAIN, *FIGHTER_KIND_NESS, *FIGHTER_KIND_INKLING, *FIGHTER_KIND_GANON].contains(&fighter_kind) {
                    return original!()(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                }
            }
            if fighter_kind == *FIGHTER_KIND_CAPTAIN {
                if [*FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_WALL_END].contains(&status_kind) {
                    return original!()(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                }
            }
        }
    }
    original!()(boma, kind)
}

#[skyline::hook(replace = smash::app::lua_bind::StatusModule::change_status_request_from_script)]
pub unsafe fn change_status_request_from_script_hook(boma: &mut smash::app::BattleObjectModuleAccessor, status: i32, unk: bool) -> u64 {
    let fighter_kind = get_kind(boma);
    let situation_kind = StatusModule::situation_kind(boma);
    let cat1 = ControlModule::get_command_flag_cat(boma, 0);
    let cat2 = ControlModule::get_command_flag_cat(boma, 1);
    let status_kind = StatusModule::status_kind(boma);
    let stick_value_x = ControlModule::get_stick_x(boma);
    let stick_value_y = ControlModule::get_stick_y(boma);
    if [*FIGHTER_KIND_FOX, *FIGHTER_KIND_WOLF].contains(&fighter_kind) {
        if status == 475 {
            return 0;
        }
    }

    if [*FIGHTER_STATUS_KIND_DOWN_STAND_FB, *FIGHTER_STATUS_KIND_DOWN_STAND_ATTACK].contains(&status) {
        if LOCKED[get_player_number(boma)] {
            return 0;
        }
    }

    if fighter_kind == *FIGHTER_KIND_PIKACHU {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            if status == *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_END {
                return 0;
            }
        }
    }

    if [*FIGHTER_KIND_RYU, *FIGHTER_KIND_KEN, *FIGHTER_KIND_DOLLY].contains(&fighter_kind) {
        if !ISAUTOTURNAROUND[get_player_number(boma)] {
            if [*FIGHTER_RYU_STATUS_KIND_TURN_AUTO, *FIGHTER_RYU_STATUS_KIND_SQUAT_TURN_AUTO, *FIGHTER_DOLLY_STATUS_KIND_TURN_AUTO, *FIGHTER_DOLLY_STATUS_KIND_SQUAT_TURN_AUTO].contains(&status) {
                return 0;
            }
        }
    } 

    if [*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_OVER_TURN, *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_OVER_TURN_START].contains(&status) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
            return 0;
        }
    }
    
    if [*FIGHTER_KIND_FOX, *FIGHTER_KIND_FALCO, *FIGHTER_KIND_WOLF].contains(&fighter_kind) {
        if [*FIGHTER_STATUS_KIND_DAMAGE_FALL, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        *FIGHTER_STATUS_KIND_DAMAGE_SONG, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_FALL, *FIGHTER_STATUS_KIND_DAMAGE_SONG_START, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_START,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U, *FIGHTER_STATUS_KIND_DAMAGE].contains(&status_kind) {
            if [*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_HIT,
            *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_LOOP, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_LOOP].contains(&status) {
                if KBSTART[get_player_number(boma)] > 0 {
                    return 0;
                }
            }
        }
    }

    /*if situation_kind == *SITUATION_KIND_AIR && status == *FIGHTER_STATUS_KIND_ESCAPE_AIR { 
        if ![*FIGHTER_KIND_SAMUSD, *FIGHTER_KIND_LUCAS, *FIGHTER_KIND_LUIGI, *FIGHTER_KIND_SAMUS, *FIGHTER_KIND_YOUNGLINK, *FIGHTER_KIND_TOONLINK, 
        *FIGHTER_KIND_SZEROSUIT].contains(&fighter_kind) {
            if !ItemModule::is_have_item(boma, 0) {
                if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CATCH) || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH) != 0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, true);
                    if stick_value_x * PostureModule::lr(boma) > 0.4 {
                        MotionModule::change_motion(boma, Hash40{hash: hash40("attack_air_f")}, 0.0, 1.0, false, 0.0, false, false);
                    }
                    else if stick_value_x * PostureModule::lr(boma) < -0.4 {
                        MotionModule::change_motion(boma, Hash40{hash: hash40("attack_air_b")}, 0.0, 1.0, false, 0.0, false, false);
                    }
                    else if stick_value_y > 0.4 {
                        MotionModule::change_motion(boma, Hash40{hash: hash40("attack_air_hi")}, 0.0, 1.0, false, 0.0, false, false);
                    }
                    else if stick_value_y < -0.4 {
                        MotionModule::change_motion(boma, Hash40{hash: hash40("attack_air_lw")}, 0.0, 1.0, false, 0.0, false, false);
                    }
                }
            }
        }
    }*/

    if fighter_kind == *FIGHTER_KIND_SNAKE {
        if status == *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_LW_EXPLODING {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
                original!()(boma, *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_LW_PRODUCE, unk);
            }
        }
    }

    if [*FIGHTER_KIND_PLIZARDON, *FIGHTER_KIND_PZENIGAME, *FIGHTER_KIND_PFUSHIGISOU].contains(&fighter_kind) {
        if status_kind == *FIGHTER_STATUS_KIND_APPEAL {
            if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R) != 0 || (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L) != 0 {
                if fighter_kind == *FIGHTER_KIND_PZENIGAME {
                    original!()(boma, *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_LW_OUT, unk);
                }
                else if fighter_kind == *FIGHTER_KIND_PFUSHIGISOU {
                    original!()(boma, *FIGHTER_PFUSHIGISOU_STATUS_KIND_SPECIAL_LW_OUT, unk);
                }
                else if fighter_kind == *FIGHTER_KIND_PLIZARDON {
                    original!()(boma, *FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_LW_OUT, unk);
                }
            }
        }
        else if [*FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_LW_OUT, *FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_LW_OUT, *FIGHTER_PFUSHIGISOU_STATUS_KIND_SPECIAL_LW_OUT].contains(&status_kind) {
            if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 {
                original!()(boma, *FIGHTER_STATUS_KIND_APPEAL, unk);
            }
        }
    }
    /*if fighter_kind == *FIGHTER_KIND_MARIO {
        if alt(boma, 4) || alt(boma, 5) {
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
                if situation_kind == *SITUATION_KIND_GROUND {
                    MotionModule::change_motion(boma, Hash40{hash: hash40("ladder_catch_air_r")}, 0.0, 1.0, false, 0.0, false, false);
                    return 0;
                }
                else if situation_kind == *SITUATION_KIND_AIR {
                    MotionModule::change_motion(boma, Hash40{hash: hash40("ladder_catch_air_l")}, 0.0, 1.0, false, 0.0, false, false);
                    return 0;
                }
            }
        }
    } */
    original!()(boma, status, unk)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_Jump_sub)]
pub unsafe fn status_jump_sub_hook(fighter: &mut L2CFighterCommon, param_2: L2CValue, param_3: L2CValue) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let mut l2c_agent = L2CAgent::new(fighter.lua_state_agent);

    l2c_agent.clear_lua_stack();
    l2c_agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_CONTROL as u64));
    l2c_agent.push_lua_stack(&mut L2CValue::new_num(calcMomentum(boma)));
    smash::app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);


    original!()(fighter, param_2, param_3)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_attack_air_common)]
pub unsafe fn status_attack_air_hook(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let mut l2c_agent = L2CAgent::new(fighter.lua_state_agent);
    let is_speed_backward = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) * PostureModule::lr(boma) < 0.0;
    let prev_status_check = [*FIGHTER_STATUS_KIND_FALL, *FIGHTER_STATUS_KIND_JUMP, *FIGHTER_STATUS_KIND_JUMP_SQUAT].contains(&StatusModule::prev_status_kind(boma, 0));    
    let mut new_speed = CURRENTMOMENTUM[get_player_number(boma)];


        /*      Shorthop aerial macro and "bair stick flick" fix     */
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR) <= 1 && 
        StatusModule::prev_status_kind(boma, 1) == *FIGHTER_STATUS_KIND_JUMP_SQUAT && !is_speed_backward { //if you used the shorthop aerial macro
        new_speed = calcMomentum(boma);
    }

    if prev_status_check {
        l2c_agent.clear_lua_stack();
        l2c_agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_CONTROL as u64));
        l2c_agent.push_lua_stack(&mut L2CValue::new_num(new_speed));
        smash::app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
    }

    original!()(fighter, param_1)
}

#[skyline::hook(replace = KineticModule::change_kinetic)]
pub unsafe fn change_kinetic_hook(boma: &mut smash::app::BattleObjectModuleAccessor, kinetic_type: i32) -> Option<i32> { //spacie laser momentum conservation
    let mut kinetic_type_new = kinetic_type;
    let status_kind = StatusModule::status_kind(boma);
    let fighter_kind = get_kind(boma);
    let mut should_change_kinetic = false;

    if [*FIGHTER_KIND_FALCO, *FIGHTER_KIND_FOX, *FIGHTER_KIND_WOLF].contains(&fighter_kind) && status_kind == 446 /* laser status */ { 
        should_change_kinetic = true;
    }

    if [*FIGHTER_KINETIC_TYPE_FALL].contains(&kinetic_type_new) && should_change_kinetic {
        kinetic_type_new = -1;
    }

    original!()(boma, kinetic_type_new)
}

pub unsafe fn additionalTransfer(lua_state: u64, l2c_agent: &mut L2CAgent, boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, fighter_kind: i32) {
    if [*FIGHTER_STATUS_KIND_JUMP_SQUAT, *FIGHTER_STATUS_KIND_JUMP, *FIGHTER_STATUS_KIND_FALL].contains(&status_kind) {
        CURRENTMOMENTUM[get_player_number(boma)] = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN); 
    }

            /*      ADDITIONAL MOVES THAT SHOULD CONSERVE MOMENTUM       */
    let mut should_conserve_momentum = false;
    
    if situation_kind == *SITUATION_KIND_AIR && MotionModule::frame(boma) <= 1.0 {

        if [*FIGHTER_KIND_CAPTAIN, *FIGHTER_KIND_MARIO, *FIGHTER_KIND_LUIGI, *FIGHTER_KIND_FOX, *FIGHTER_KIND_FALCO, *FIGHTER_KIND_WOLF]
            .contains(&fighter_kind) && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N { //put any fighter here whose neutral special should conserve momentum
                should_conserve_momentum = true; 
        }

        if should_conserve_momentum && KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN).abs() > 0.1 {
            l2c_agent.clear_lua_stack();
            l2c_agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_CONTROL as u64));
            l2c_agent.push_lua_stack(&mut L2CValue::new_num(CURRENTMOMENTUM[get_player_number(boma)]));
            smash::app::sv_kinetic_energy::set_speed(lua_state);
        }

    }
}

pub unsafe fn jumpCancels(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, fighter_kind: i32) { //Jump cancel shine & etc
    if fighter_kind == *FIGHTER_KIND_FOX {
        if [*FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_LOOP, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_HIT, 
        *FIGHTER_STATUS_KIND_SPECIAL_LW].contains(&status_kind) {
            if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_JUMP) || (ControlModule::is_enable_flick_jump(boma) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_FLICK_JUMP)) {
                if situation_kind == *SITUATION_KIND_AIR{
                    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX){
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
                    }
                }
                else if situation_kind == *SITUATION_KIND_GROUND{
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                }
            }
        }
    }
    if fighter_kind == *FIGHTER_KIND_WOLF {
        if MotionModule::frame(boma) >= 4.0 {
            if [*FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_LOOP, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_HIT, 
            *FIGHTER_STATUS_KIND_SPECIAL_LW].contains(&status_kind) {
                if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) || (ControlModule::is_enable_flick_jump(boma) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_FLICK_JUMP)) {
                    if situation_kind == *SITUATION_KIND_AIR{
                        if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX){
                            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
                        }
                    }
                    else if situation_kind == *SITUATION_KIND_GROUND{
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                    }
                }
            }
        }
    }
    if fighter_kind == *FIGHTER_KIND_FALCO {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
            if MotionModule::frame(boma) >= 2.0 {
                if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_JUMP) || (ControlModule::is_enable_flick_jump(boma) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_FLICK_JUMP)) {
                    if situation_kind == *SITUATION_KIND_AIR {
                        if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
                            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
                        }
                    }
                    else if situation_kind == *SITUATION_KIND_GROUND{
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                    }
                }
            }
        }
    }
    if fighter_kind == *FIGHTER_KIND_KROOL {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
            if MotionModule::frame(boma) >= 4.0 {
                if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_JUMP) || (ControlModule::is_enable_flick_jump(boma) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_FLICK_JUMP)) {
                    if situation_kind == *SITUATION_KIND_AIR {
                        if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
                            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
                        }
                    }
                    else if situation_kind == *SITUATION_KIND_GROUND{
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                    }
                }
            }
        }
    }
    if fighter_kind == *FIGHTER_KIND_NESS {
        if [*FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HOLD, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HIT, 
        *FIGHTER_STATUS_KIND_SPECIAL_LW].contains(&status_kind) {
            if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_JUMP) || (ControlModule::is_enable_flick_jump(boma) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_FLICK_JUMP)) {
                if situation_kind == *SITUATION_KIND_AIR{
                    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX){
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
                    }
                }
                else if situation_kind == *SITUATION_KIND_GROUND{
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                }
            }
        }
    }
    if fighter_kind == *FIGHTER_KIND_IKE {
        if [*FIGHTER_IKE_STATUS_KIND_SPECIAL_S_DASH, *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_END, *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_ATTACK].contains(&status_kind) {
            if situation_kind == *SITUATION_KIND_GROUND {
                if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_JUMP) || (ControlModule::is_enable_flick_jump(boma) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_FLICK_JUMP)) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);   
                }
            }
        }
    }
    if fighter_kind == *FIGHTER_KIND_EDGE {
        if status_kind == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH {
            if situation_kind == *SITUATION_KIND_GROUND {
                if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_JUMP) || (ControlModule::is_enable_flick_jump(boma) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_FLICK_JUMP)) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);   
                }
            }
        }
    }
    if fighter_kind == *FIGHTER_KIND_CLOUD {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_JUMP) || (ControlModule::is_enable_flick_jump(boma) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_FLICK_JUMP)) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);   
                }
            }
        }
    }
    if fighter_kind == *FIGHTER_KIND_KAMUI {
        if status_kind == *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_HOLD {
            if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_JUMP) || (ControlModule::is_enable_flick_jump(boma) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_FLICK_JUMP)) {
                if situation_kind == *SITUATION_KIND_AIR{
                    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX){
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
                    }
                }
                else if situation_kind == *SITUATION_KIND_GROUND{
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                }
            }
            if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_GUARD) {
                if situation_kind == *SITUATION_KIND_AIR {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
                }
            }
        }
    }
    if fighter_kind == *FIGHTER_KIND_INKLING {
        if [*FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_WALK, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_DASH, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_RUN, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_WALK_TURN,
        *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_RUN_TURN].contains(&status_kind) {
            if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_JUMP) || (ControlModule::is_enable_flick_jump(boma) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_FLICK_JUMP)) {
                if situation_kind == *SITUATION_KIND_AIR{
                    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX){
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
                    }
                }
                else if situation_kind == *SITUATION_KIND_GROUND{
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                }
            }
        }
    }
}

pub unsafe fn landCancels(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, fighter_kind: i32) { //Land Cancels
    if [*FIGHTER_KIND_FOX, *FIGHTER_KIND_FALCO].contains(&fighter_kind) {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
            if StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR && situation_kind == *SITUATION_KIND_GROUND {
                CancelModule::enable_cancel(boma);
            }
        }
    }
    if fighter_kind == *FIGHTER_KIND_EDGE {
        if status_kind == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_S_SHOOT {
            if StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR && situation_kind == *SITUATION_KIND_GROUND {
                CancelModule::enable_cancel(boma);
            }
        }
    }
    if fighter_kind == *FIGHTER_KIND_INKLING {
        if [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_N_END, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_N_SHOOT].contains(&status_kind) {
            if StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR && situation_kind == *SITUATION_KIND_GROUND {
                CancelModule::enable_cancel(boma);
            }
        }
    }
    if fighter_kind == *FIGHTER_KIND_KOOPAJR {
        if [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_N_SHOOT, *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_N_HOLD].contains(&status_kind) {
            if StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR && situation_kind == *SITUATION_KIND_GROUND {
                CancelModule::enable_cancel(boma);
            }
        }
    }
}

pub unsafe fn dacus(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, cat1: i32, stick_value_y: f32) { //Dacus
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH {
        if MotionModule::frame(boma) < (10.0){
            if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 || (stick_value_y >= 0.7 && (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH) != 0) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, true);
            }
            if MotionModule::frame(boma) > (2.0){
                if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, true);
                }
            }
        }
    }
}

pub unsafe fn jumpCancelMove(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, cat1: i32, stick_value_y: f32) { //Jump cancel grab, usmash, etc.
    if status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT {
        if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_CATCH) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_CATCH, true);
        }
        else if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_GUARD) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
        }
        if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 && MotionModule::frame(boma) >= 1.0 {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, true);  
        }
        if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);  
        }
    }
}

pub unsafe fn perfectPivots(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, stick_value_x: f32) { //Perfect Pivots and microdashes
    if [*FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_DASH].contains(&status_kind) {
        if MotionModule::frame(boma) < (3.0) && stick_value_x < 0.5 && stick_value_x > -0.5 {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
        }
    }
}

pub unsafe fn glideTossing(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat3: i32, stick_value_x: f32) { //Glide Tossing
    if [*FIGHTER_STATUS_KIND_ESCAPE_B, *FIGHTER_STATUS_KIND_ESCAPE_F].contains(&status_kind) && situation_kind == *SITUATION_KIND_GROUND {
        if ItemModule::is_have_item(boma, 0) {
            if (cat3 & *FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_ALL) != 0 {
                if (stick_value_x * PostureModule::lr(boma)) < 0.0 {
                    PostureModule::reverse_lr(boma);
                }
                let x_vel = PostureModule::lr(boma) * 2.0 * KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                let speed_vector = smash::phx::Vector3f { x: x_vel, y: 0.0, z: 0.0 };
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ITEM_THROW, true);
                KineticModule::add_speed(boma, &speed_vector);
            }
        }
    }
}

pub unsafe fn tauntSliding(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, cat2: i32) { //Taunt Slides
    if [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_RUN_BRAKE, *FIGHTER_STATUS_KIND_TURN_RUN, *FIGHTER_STATUS_KIND_TURN_DASH].contains(&status_kind) {
        if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI) != 0 || (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW) != 0  ||
        (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L) != 0 || (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R) != 0 {
            let x_vel = PostureModule::lr(boma) * 2.5 * KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            let speed_vector = smash::phx::Vector3f { x: x_vel, y: 0.0, z: 0.0 };
            CancelModule::enable_cancel(boma);
            KineticModule::add_speed(boma, &speed_vector);
        }
    }
    else if status_kind == *FIGHTER_STATUS_KIND_APPEAL {
        if MotionModule::frame(boma) > 4.0 {
            CancelModule::enable_cancel(boma);
        }
    }
}

pub unsafe fn ditcit (boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, cat1: i32, cat3: i32) { //Ditcit
    if status_kind == *FIGHTER_STATUS_KIND_ITEM_THROW_DASH {
        if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 || 
            (cat3 & *FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_HI) != 0 || (cat3 & *FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_HI4) != 0 {
            if MotionModule::frame(boma) < (7.0) {
                let x_vel = PostureModule::lr(boma) * 3.0 * KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                let speed_vector = smash::phx::Vector3f { x: x_vel, y: 0.0, z: 0.0 };
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ITEM_THROW, true);
                KineticModule::add_speed(boma, &speed_vector);
            }
        }
    }
}

pub unsafe fn shieldStops(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, cat1: i32) { //Shield Stop
    if status_kind == *FIGHTER_STATUS_KIND_DASH || status_kind == *FIGHTER_STATUS_KIND_TURN_DASH {
        if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE) != 0 || ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_GUARD) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD_ON, true);
        }
    }
}

pub unsafe fn dashPlatformDrop(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, stick_value_y: f32) { //Dash Platform Drop
    if [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_TURN_RUN, 
    *FIGHTER_STATUS_KIND_ESCAPE_AIR, 22].contains(&status_kind) {
        if situation_kind == SITUATION_KIND_GROUND {
            if stick_value_y <= -0.8 {
                if GroundModule::is_passable_ground(boma) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_PASS, true);
                }
            }
        }
    }
    if status_kind == *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR {
        if LAGCANCELED[get_player_number(boma)] {
            if stick_value_y <= -0.8 {
                if GroundModule::is_passable_ground(boma) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_PASS, true);
                }
            }
        }
    }
}

pub unsafe fn shieldDrops(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, cat2: i32) {
    if status_kind == *FIGHTER_STATUS_KIND_GUARD || status_kind == *FIGHTER_STATUS_KIND_GUARD_ON { //Shield Drop
        if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_GUARD_TO_PASS) != 0 || (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI) != 0 || (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW) != 0  ||
        (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L) != 0 || (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R) != 0 {
            if GroundModule::is_passable_ground(boma) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_PASS, true);
            }
        }
    }
}

pub unsafe fn jabCancels(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, motion_kind: u64, fighter_kind: i32, cat1: i32, stick_value_x: f32, stick_value_y: f32) { //Jab Cancels
    if (![*FIGHTER_KIND_CHROM, *FIGHTER_KIND_ROY, *FIGHTER_KIND_GANON, *FIGHTER_KIND_SHIZUE, *FIGHTER_KIND_PIKACHU, *FIGHTER_KIND_PICHU, *FIGHTER_KIND_METAKNIGHT, *FIGHTER_KIND_PICKEL].contains(&fighter_kind) && motion_kind == hash40("attack_11")) ||
        (![*FIGHTER_KIND_DONKEY, *FIGHTER_KIND_MARTH, *FIGHTER_KIND_LUCINA, *FIGHTER_KIND_KOOPA, *FIGHTER_KIND_DAISY, *FIGHTER_KIND_PEACH, *FIGHTER_KIND_SAMUS, *FIGHTER_KIND_SAMUSD, *FIGHTER_KIND_PURIN, *FIGHTER_KIND_NANA, *FIGHTER_KIND_POPO, *FIGHTER_KIND_YOSHI, *FIGHTER_KIND_PIKMIN].contains(&fighter_kind) && motion_kind == hash40("attack_12")) || 
        ([*FIGHTER_KIND_METAKNIGHT, *FIGHTER_KIND_PACKUN, *FIGHTER_KIND_SNAKE].contains(&fighter_kind) && motion_kind == hash40("attack_s3_s")) ||
        (fighter_kind == *FIGHTER_KIND_METAKNIGHT && motion_kind == hash40("attack_s3_s2")) {
        if MotionModule::frame(boma) > (10.0) {
            if stick_value_y <= -0.66 {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SQUAT_F, true);
            }
            if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 {
                CancelModule::enable_cancel(boma);
            }
            if stick_value_x <= -0.6 {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_TURN, true);
            }
            if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE) != 0 || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD_ON, true);
            }
        }
    }
    if fighter_kind == *FIGHTER_KIND_METAKNIGHT {
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_100 {
            if MotionModule::frame(boma) > (15.0) {
                if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4, true);
                }
                else if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3, true);
                }
            }
        }
    }
    if fighter_kind == *FIGHTER_KIND_GAOGAEN {
        if [hash40("attack_11"), hash40("attack_12")].contains(&motion_kind) {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 {
                    CancelModule::enable_cancel(boma);
                }
            }
        }
    }
    /*if fighter_kind == *FIGHTER_KIND_CLOUD {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                if (stick_value_x * PostureModule::lr(boma) < 0.4 || ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_SPECIAL))  {
                    CancelModule::enable_cancel(boma);
                }
            }
        }
    }*/
}

pub unsafe fn pteStuff(boma: &mut smash::app::BattleObjectModuleAccessor, fighter_kind: i32, status_kind: i32, situation_kind: i32, motion_kind: u64, cat1: i32, stick_value_x: f32) { //Pte stuff
    if fighter_kind == *FIGHTER_KIND_SHIZUE || fighter_kind == *FIGHTER_KIND_MURABITO {
        if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR) { // Midair dash dance
            if situation_kind == *SITUATION_KIND_AIR {
                if ((cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN) != 0) && stick_value_x * PostureModule::lr(boma) < 0.0 {
                    PostureModule::reverse_lr(boma);
                    PostureModule::update_rot_y_lr(boma);
                }
            }
        }
        if motion_kind == hash40("attack_air_b") { //Bair = fair
            PostureModule::reverse_lr(boma);
            PostureModule::update_rot_y_lr(boma);
            MotionModule::change_motion(boma, Hash40{hash: hash40("attack_air_f")}, 0.0, 1.0, false, 0.0, false, false);
        }
    }
    if fighter_kind == *FIGHTER_KIND_SHIZUE {
        if [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_RUN].contains(&status_kind) || ![*FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4_START].contains(&status_kind) { //Get direction during dash
            DASHDIR[get_player_number(boma)] = PostureModule::lr(boma);
        }
        if [*FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4_START].contains(&status_kind) { //F smash flip check
            if (DASHDIR[get_player_number(boma)] * stick_value_x) < 0.0 {
                ISREVERSE[get_player_number(boma)] = true;
            }
        }
        else {
            ISREVERSE[get_player_number(boma)] = false;
        }
        if ISREVERSE[get_player_number(boma)] {
            if [*FIGHTER_STATUS_KIND_ATTACK_S4].contains(&status_kind) { //F smash flip
                if MotionModule::frame(boma) > 15.0 {
                    PostureModule::reverse_lr(boma);
                    PostureModule::update_rot_y_lr(boma);
                    ISREVERSE[get_player_number(boma)] = false;
                }
            }
        }
        if motion_kind == hash40("appeal_s_l") || motion_kind == hash40("appeal_s_r") { //Set funny thing
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CATCH) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                ISFUNNY[get_player_number(boma)] = !ISFUNNY[get_player_number(boma)];
            }
        }
        if situation_kind == *SITUATION_KIND_AIR { //Boost side b
            if status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL {
                let speed_vector = smash::phx::Vector3f { x: 0.0, y: 1.2, z: 0.0 };
                if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S, true);
                    KineticModule::add_speed(boma, &speed_vector);
                }
            }
        }
        if ISFUNNY[get_player_number(boma)] { //Do the funny
            if MotionModule::frame(boma) == 1.0 {
                let rng1 = smash::app::sv_math::randf(0, 6.0);
                let rng2 = smash::app::sv_math::randf(0, 6.0);
                let speed_vector = smash::phx::Vector3f { x: (rng1 - 3.0), y: (rng2 - 3.0), z: 0.0 };
                KineticModule::add_speed(boma, &speed_vector);
                //MotionModule::set_rate(boma, -1.0);
            }
        }
    }
}

pub unsafe fn djcs (lua_state: u64, l2c_agent: &mut L2CAgent, boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, fighter_kind: i32, kinetic_type: i32) {
    if [*FIGHTER_KIND_NESS, *FIGHTER_KIND_LUCAS, *FIGHTER_KIND_MEWTWO].contains(&fighter_kind){ //DJCs
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
            if kinetic_type == *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION_2ND {
                if MotionModule::frame(boma) < (7.0) {
                    if !(ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP)) {
                        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
                    }
                }
            }
        }
    }
    if [*FIGHTER_KIND_PEACH, *FIGHTER_KIND_YOSHI, *FIGHTER_KIND_SAMUSD, *FIGHTER_KIND_KOOPAJR].contains(&fighter_kind) {
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
            if StatusModule::prev_status_kind(boma, 0) == *FIGHTER_STATUS_KIND_JUMP_AERIAL {
                if MotionModule::frame(boma) < (7.0) {
                    if !(ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP)) {
                        l2c_agent.clear_lua_stack();
                        l2c_agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY as u64));
                        l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.0));
                        smash::app::sv_kinetic_energy::set_speed(lua_state);
                    }
                }
            }
        }
    }
}

pub unsafe fn init_settings_edges(boma: &mut BattleObjectModuleAccessor, situation: smash::app::SituationKind, arg3: i32, arg4: u32,
    ground_cliff_check_kind: smash::app::GroundCliffCheckKind, arg6: bool,
    arg7: i32, arg8: i32, arg9: i32, arg10: i32) -> u32 {
    /* "fix" forces GroundModule::correct to be called for the statuses we need */
    let mut fix = arg4;
    let category = get_category(boma);
    let fighter_kind = get_kind(boma);
    let status_kind = StatusModule::status_kind(boma);
    let situation_kind = StatusModule::situation_kind(boma);

    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if [*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN, *FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_SQUAT, *FIGHTER_STATUS_KIND_SQUAT_WAIT, 
        *FIGHTER_STATUS_KIND_SQUAT_F, *FIGHTER_STATUS_KIND_SQUAT_B, *FIGHTER_STATUS_KIND_SQUAT_RV, *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_LANDING_LIGHT, 
        *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, *FIGHTER_STATUS_KIND_LANDING_DAMAGE_LIGHT, *FIGHTER_STATUS_KIND_GUARD_DAMAGE,
        *FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE, *FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_TURN_RUN,
        *FIGHTER_STATUS_KIND_APPEAL].contains(&status_kind) {
            fix = *GROUND_CORRECT_KIND_GROUND as u32;
        }
        if situation_kind == *SITUATION_KIND_GROUND {
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH {
                if [*FIGHTER_KIND_BUDDY, *FIGHTER_KIND_DONKEY, *FIGHTER_KIND_KIRBY].contains(&fighter_kind) {
                    fix = *GROUND_CORRECT_KIND_GROUND as u32;
                }
            }
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S { //Side b's
                if [*FIGHTER_KIND_CAPTAIN, *FIGHTER_KIND_INKLING, *FIGHTER_KIND_BAYONETTA, *FIGHTER_KIND_DIDDY, *FIGHTER_KIND_SHIZUE, *FIGHTER_KIND_MARTH].contains(&fighter_kind) {
                    fix = *GROUND_CORRECT_KIND_GROUND as u32;
                }
                if fighter_kind == *FIGHTER_KIND_JACK {
                    fix = *GROUND_CORRECT_KIND_GROUND as u32;
                }
            }
            if fighter_kind == *FIGHTER_KIND_JACK { // Joker gun edge cancels
                if [*FIGHTER_JACK_STATUS_KIND_SPECIAL_N_ESCAPE, *FIGHTER_JACK_STATUS_KIND_SPECIAL_N_LANDING, *FIGHTER_JACK_STATUS_KIND_SPECIAL_N_BARRAGE_END].contains(&status_kind) {
                    fix = *GROUND_CORRECT_KIND_GROUND as u32;
                }
            }
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI { //Up specials
                if [*FIGHTER_KIND_CLOUD, *FIGHTER_KIND_PIKACHU, *FIGHTER_KIND_GEKKOUGA, *FIGHTER_KIND_INKLING].contains(&fighter_kind) {
                    fix = *GROUND_CORRECT_KIND_GROUND as u32;
                }
            }
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N { //Neutral Specials
                if [*FIGHTER_KIND_JACK, *FIGHTER_KIND_MARTH, *FIGHTER_KIND_PLIZARDON].contains(&fighter_kind) {
                    fix = *GROUND_CORRECT_KIND_GROUND as u32;
                }
            }
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW { //Down Special edgecancels
                if [*FIGHTER_KIND_CAPTAIN, *FIGHTER_KIND_NESS, *FIGHTER_KIND_INKLING].contains(&fighter_kind) {
                    fix = *GROUND_CORRECT_KIND_GROUND as u32;
                }
            }
            if fighter_kind == *FIGHTER_KIND_CAPTAIN {
                if [*FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_WALL_END].contains(&status_kind) {
                    fix = *GROUND_CORRECT_KIND_GROUND as u32;
                }
            }
        }
    }
    return fix;
}

pub unsafe fn airDodgeZair(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, fighter_kind: i32, cat2: i32, cat3: i32, stick_value_x: f32) { //Air dodge cancel zair
    if [*FIGHTER_STATUS_KIND_ESCAPE_AIR].contains(&status_kind) && situation_kind == *SITUATION_KIND_AIR {
        if ItemModule::is_have_item(boma, 0) {
            if (cat3 & *FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_ALL) != 0 {
                if (stick_value_x * PostureModule::lr(boma)) < 0.0 {
                    PostureModule::reverse_lr(boma);
                }
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ITEM_THROW, true);
            }
        }
        else if [*FIGHTER_KIND_SAMUSD, *FIGHTER_KIND_LUCAS, *FIGHTER_KIND_LUIGI, *FIGHTER_KIND_SAMUS, *FIGHTER_KIND_YOUNGLINK, *FIGHTER_KIND_TOONLINK, 
        *FIGHTER_KIND_SZEROSUIT].contains(&fighter_kind) {
            if MotionModule::frame(boma) < (7.0) {
                if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_AIR_LASSO) != 0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_AIR_LASSO, true);
                }
            }
        }
    }
}

pub unsafe fn wallJumpSpecial(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, fighter_kind: i32, cat1: i32) { //Walljump during special
    if fighter_kind == *FIGHTER_KIND_MARIO {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            if situation_kind == *SITUATION_KIND_AIR {
                if GroundModule::is_wall_touch_line(boma, 0) {
                    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_WALL_JUMP_LEFT) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_WALL_JUMP_RIGHT) != 0 {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WALL_JUMP, true);
                    }
                }
            }
        }
    }
}

pub unsafe fn sm4shJabLocks(lua_state: u64, l2c_agent: &mut L2CAgent, boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32) {
    if status_kind == *FIGHTER_STATUS_KIND_DOWN_DAMAGE { //Sm4sh jab locks
        LOCKED[get_player_number(boma)] = true;
    }
    else if [*FIGHTER_STATUS_KIND_DOWN_STAND_FB, *FIGHTER_STATUS_KIND_DOWN_STAND_ATTACK].contains(&status_kind) {
        if LOCKED[get_player_number(boma)] {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DOWN_STAND, true);
            LOCKED[get_player_number(boma)] = false;
            l2c_agent.clear_lua_stack();
            l2c_agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_CONTROL as u64));
            l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.0));
            smash::app::sv_kinetic_energy::set_speed(lua_state);
        }
    }
    else if ![*FIGHTER_STATUS_KIND_DOWN_CONTINUE, *FIGHTER_STATUS_KIND_DOWN_WAIT, *FIGHTER_STATUS_KIND_DOWN, *FIGHTER_STATUS_KIND_DOWN_WAIT_CONTINUE].contains(&status_kind) {
        LOCKED[get_player_number(boma)] = false;
    }
    if status_kind == *FIGHTER_STATUS_KIND_DOWN_STAND {
        l2c_agent.clear_lua_stack();
        l2c_agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_CONTROL as u64));
        l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.0));
        smash::app::sv_kinetic_energy::set_speed(lua_state);
    }
}

pub unsafe fn noSpecialFall(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, fighter_kind: i32) {
    if [*FIGHTER_KIND_WOLF].contains(&fighter_kind) { //No freefall on hit
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                NOSPECIALFALL[get_player_number(boma)] = true;
            }
        }
        if NOSPECIALFALL[get_player_number(boma)] {
            if status_kind == *FIGHTER_STATUS_KIND_FALL_SPECIAL {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
                NOSPECIALFALL[get_player_number(boma)] = false;
            }
        }
        if ![*FIGHTER_STATUS_KIND_FALL_SPECIAL, *FIGHTER_STATUS_KIND_SPECIAL_S].contains(&status_kind) {
            if NOSPECIALFALL[get_player_number(boma)] {
                NOSPECIALFALL[get_player_number(boma)] = false;
            }
        }
    }

    if [*FIGHTER_KIND_FOX, *FIGHTER_KIND_FALCO].contains(&fighter_kind) {
        if situation_kind == *SITUATION_KIND_AIR {
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
                if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                    NOSPECIALFALL[get_player_number(boma)] = true;
                }
            }
            if NOSPECIALFALL[get_player_number(boma)] {
                if status_kind == *FIGHTER_STATUS_KIND_FALL || MotionModule::frame(boma) > 22.0 || CancelModule::is_enable_cancel(boma) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
                    NOSPECIALFALL[get_player_number(boma)] = false;
                }
            }
            if ![*FIGHTER_STATUS_KIND_FALL, *FIGHTER_STATUS_KIND_SPECIAL_S].contains(&status_kind) {
                if NOSPECIALFALL[get_player_number(boma)] {
                    NOSPECIALFALL[get_player_number(boma)] = false;
                }
            }
        }
        else {
            if NOSPECIALFALL[get_player_number(boma)] {
                NOSPECIALFALL[get_player_number(boma)] = false;
            }
        }
    }

    if [*FIGHTER_KIND_EDGE].contains(&fighter_kind) { //No freefall on hit
        if status_kind == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_CHARGED_RUSH {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                NOSPECIALFALL[get_player_number(boma)] = true;
            }
        }
        if NOSPECIALFALL[get_player_number(boma)] {
            if status_kind == *FIGHTER_STATUS_KIND_FALL_SPECIAL {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
                NOSPECIALFALL[get_player_number(boma)] = false;
            }
        }
        if ![*FIGHTER_STATUS_KIND_FALL_SPECIAL, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_CHARGED_RUSH, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_END].contains(&status_kind) {
            if NOSPECIALFALL[get_player_number(boma)] {
                NOSPECIALFALL[get_player_number(boma)] = false;
            }
        }
    }

    if [*FIGHTER_KIND_PLIZARDON, *FIGHTER_KIND_CLOUD].contains(&fighter_kind) { //No freefall on hit
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            USEDUPB[get_player_number(boma)] = true;
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                NOSPECIALFALL[get_player_number(boma)] = true;
            }
        }
        if NOSPECIALFALL[get_player_number(boma)] {
            if status_kind == *FIGHTER_STATUS_KIND_FALL_SPECIAL || MotionModule::frame(boma) > 45.0 {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
                NOSPECIALFALL[get_player_number(boma)] = false;
            }
        }
        if ![*FIGHTER_STATUS_KIND_FALL_SPECIAL, *FIGHTER_STATUS_KIND_SPECIAL_HI].contains(&status_kind) {
            if NOSPECIALFALL[get_player_number(boma)] {
                NOSPECIALFALL[get_player_number(boma)] = false;
            }
        }
        if situation_kind != *SITUATION_KIND_AIR {
            USEDUPB[get_player_number(boma)] = false;
        }
        if [*FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL].contains(&status_kind) {
            USEDUPB[get_player_number(boma)] = false;
        }
    }
    if fighter_kind == *FIGHTER_KIND_WIIFIT {
        if [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_HI_JUMP, *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_HI_JUMP].contains(&status_kind) {
            USEDUPB[get_player_number(boma)] = true;
            if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_GUARD) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
            }
        }
        if situation_kind != *SITUATION_KIND_AIR || [*FIGHTER_STATUS_KIND_DAMAGE_FALL, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        *FIGHTER_STATUS_KIND_DAMAGE_SONG, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_FALL, *FIGHTER_STATUS_KIND_DAMAGE_SONG_START, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_START,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U, *FIGHTER_STATUS_KIND_DAMAGE].contains(&status_kind) {
            USEDUPB[get_player_number(boma)] = false;
        }
    }
}

pub unsafe fn regainDJ(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, fighter_kind: i32, situation_kind: i32) {
    if [*FIGHTER_KIND_GANON, *FIGHTER_KIND_CAPTAIN].contains(&fighter_kind) { //Falcon return dj after down b
        if situation_kind == *SITUATION_KIND_AIR {
            if [*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_WALL_END,
            *FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_WALL_END].contains(&status_kind) {
                if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) >= WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
                    REGAINUPB[get_player_number(boma)] = true;
                }
            }
        }
        else {
            REGAINUPB[get_player_number(boma)] = false;
        }
    }
}

pub unsafe fn wiggleHitstun(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, cat1: i32) { // Wiggle out of tumble
    if status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY {
        if CancelModule::is_enable_cancel(boma) {
            if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH) != 0 {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
            }
        }
    }
}

pub unsafe fn shortens(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, fighter_kind: i32, motion_kind: u64, cat1: i32) { //Shortens
    if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
        if fighter_kind == *FIGHTER_KIND_FOX || fighter_kind == *FIGHTER_KIND_WOLF {
            WorkModule::on_flag(boma, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_RUSH_FORCE_END);
        }
        else if fighter_kind == *FIGHTER_KIND_FALCO {
            WorkModule::on_flag(boma, *FIGHTER_FALCO_ILLUSION_STATUS_WORK_ID_FLAG_RUSH_FORCE_END);
        }
    }
    if fighter_kind == *FIGHTER_KIND_MEWTWO {
        if status_kind == *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_2 {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3, true);
            }
        }
    }
    /*if fighter_kind == *FIGHTER_KIND_METAKNIGHT {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
            if ![hash40("special_air_lw_start"), hash40("special_lw_start")].contains(&motion_kind) || MotionModule::frame(boma) >= 9.0 {
                if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
                    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_ATTACK, true);
                    }
                    else {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_END, true);
                    }
                }
            }
        }
    } */
}

pub unsafe fn ivyHeals(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, fighter_kind: i32, motion_kind: u64) { //Ivysaur healing and meter gain
    if fighter_kind == *FIGHTER_KIND_PFUSHIGISOU {
        if [hash40("special_n_end"), hash40("special_air_n_end"), hash40("special_n"), hash40("special_air_n")].contains(&motion_kind) || status_kind == *FIGHTER_PFUSHIGISOU_STATUS_SPECIAL_N_INT_LOOP {
            CancelModule::enable_cancel(boma);
        }
        if motion_kind == hash40("attack_air_hi") {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                if CANHEAL[get_player_number(boma)] {
                    DamageModule::add_damage(boma, -3.5, 0);
                    CANHEAL[get_player_number(boma)] = false;
                    /*if AMOUNTSOLAR[get_player_number(boma)] < 200 {
                        AMOUNTSOLAR[get_player_number(boma)] += 8;
                        println!("increased meter by 8, new meter: {}", AMOUNTSOLAR[get_player_number(boma)]);
                    } */
                }
            }
        }
        if motion_kind == hash40("attack_air_lw") {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                if CANHEAL[get_player_number(boma)] {
                    DamageModule::add_damage(boma, -5.0, 0);
                    CANHEAL[get_player_number(boma)] = false;
                    /*if AMOUNTSOLAR[get_player_number(boma)] < 200 {
                        AMOUNTSOLAR[get_player_number(boma)] += 10;
                        println!("increased meter by 10, new meter: {}", AMOUNTSOLAR[get_player_number(boma)]);
                    }*/
                }
            }
        }
        if motion_kind == hash40("throw_hi") {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                if CANHEAL[get_player_number(boma)] {
                    DamageModule::add_damage(boma, -7.0, 0);
                    CANHEAL[get_player_number(boma)] = false;
                    /*if AMOUNTSOLAR[get_player_number(boma)] < 200 {
                        AMOUNTSOLAR[get_player_number(boma)] += 10;
                        println!("increased meter by 10, new meter: {}", AMOUNTSOLAR[get_player_number(boma)]);
                    }*/
                }
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_CATCH_ATTACK {
            if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) {
                if CANHEAL[get_player_number(boma)] {
                    DamageModule::add_damage(boma, -1.0, 0);
                    CANHEAL[get_player_number(boma)] = false;
                    /*if AMOUNTSOLAR[get_player_number(boma)] < 200 { 
                        AMOUNTSOLAR[get_player_number(boma)] += 3;
                        println!("increased meter by 3, new meter: {}", AMOUNTSOLAR[get_player_number(boma)]);
                    } */
                }
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4 {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                if CANHEAL[get_player_number(boma)] {
                    DamageModule::add_damage(boma, -6.0, 0);
                    CANHEAL[get_player_number(boma)] = false;
                    /*if AMOUNTSOLAR[get_player_number(boma)] < 200 { 
                        AMOUNTSOLAR[get_player_number(boma)] += 14;
                        println!("increased meter by 14, new meter: {}", AMOUNTSOLAR[get_player_number(boma)]);
                    }*/
                }
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
            if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) {
                if LEECHSTART[get_player_number(boma)] == 0 {
                    LEECHSTART[get_player_number(boma)] = GLOBALFRAMECOUNT;
                }
            }
        }
        if LEECHSTART[get_player_number(boma)] != 0 {
            if (GLOBALFRAMECOUNT - LEECHSTART[get_player_number(boma)]) % 60 == 0 {
                DamageModule::add_damage(boma, -0.6, 0);
            }
            if GLOBALFRAMECOUNT - LEECHSTART[get_player_number(boma)] >= 900 {
                LEECHSTART[get_player_number(boma)] = 0;
            }
        }
        /*if AMOUNTSOLAR[get_player_number(boma)] >= 200 { //meter start and end
            AMOUNTSOLAR[get_player_number(boma)] = 200;
            if SOLARSTART[get_player_number(boma)] == 0 {
                println!("start meter");
                SOLARSTART[get_player_number(boma)] = GLOBALFRAMECOUNT;
            }
            if (GLOBALFRAMECOUNT - SOLARSTART[get_player_number(boma)]) == 1800 { //Too lazy to make the global frame counter and solar start a float lollllll
                AMOUNTSOLAR[get_player_number(boma)] = 0;
                SOLARSTART[get_player_number(boma)] = 0;
                println!("no more meter!");
            }
            let pos = smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
            let rot = smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
            let idk = smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
            //smash::app::sv_animcmd::EFFECT_FOLLOW(0x16f1e4733f_u64, hash40("flower"), 0, 0, 0, 0, 0, 0, 1, true);
            EffectModule::req_on_joint(boma, Hash40{hash: hash40("finptrainer_solar_beam")}, Hash40{hash: hash40("flower")}, &pos, &rot, 1.0, &idk, &idk, false, 0, 1, 0);
        }
        else if AMOUNTSOLAR[get_player_number(boma)] == 0 {
            EffectModule::kill_kind(boma, Hash40{hash: hash40("finptrainer_solar_beam")}, false, false);
        }
        else {
            let pos = smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
            let rot = smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
            let idk = smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
            //smash::app::sv_animcmd::EFFECT_FOLLOW(0x16f1e4733f_u64, hash40("flower"), 0, 0, 0, 0, 0, 0, 1, true);
            EffectModule::req_on_joint(boma, Hash40{hash: hash40("finptrainer_solar_beam")}, Hash40{hash: hash40("flower")}, &pos, &rot, (0.005 * AMOUNTSOLAR[get_player_number(boma)] as f32), &idk, &idk, false, 0, 1, 0);
        }*/
        if MotionModule::frame(boma) < 2.0 || (![hash40("attack_air_lw"), hash40("attack_air_hi"), hash40("throw_hi")].contains(&motion_kind) && ![*FIGHTER_STATUS_KIND_CATCH_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_HI4].contains(&status_kind)) {
            CANHEAL[get_player_number(boma)] = true;
        }
    }
}

pub unsafe fn setStatuses(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32) { //Set Statuses
    if status_kind != RECENTSTATUSES[get_player_number(boma)][0] {
        let mut i = 4;
        while i > 0 {
            RECENTSTATUSES[get_player_number(boma)][i] = RECENTSTATUSES[get_player_number(boma)][i-1];
            i = i - 1;
        }
        RECENTSTATUSES[get_player_number(boma)][0] = status_kind;
    }
}

pub unsafe fn meleeSmashStick(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32) { //Melee-Style Smash Stick
    if TOGGELS[1][get_player_number(boma)] == 1 {
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CSTICK_ON) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4, true);
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CSTICK_ON) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4, true);
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CSTICK_ON) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4, true);
            }
        }
    }
}

pub unsafe fn grabIsAerial(lua_state: u64, l2c_agent: &mut L2CAgent, boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32) { //Grab will now do aerials in the air
    if situation_kind == *SITUATION_KIND_AIR && status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR { 
        if !ItemModule::is_have_item(boma, 0) {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CATCH) || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH) != 0 {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, true);
                l2c_agent.clear_lua_stack();
                l2c_agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_CONTROL as u64));
                l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.0));
                smash::app::sv_kinetic_energy::set_speed(lua_state);
            }
        }
    }
}

pub unsafe fn slipTechs(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, cat1: i32, stick_value_x: f32) { //Banana deletes on hit
    if status_kind == *FIGHTER_STATUS_KIND_SLIP {
        if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE) != 0 || ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_GUARD) {
            if MotionModule::frame(boma) < 6.0 {
                if stick_value_x < 0.4 && stick_value_x > -0.4 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SLIP_STAND, true);
                }
                else if stick_value_x * PostureModule::lr(boma) > 0.0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SLIP_STAND_F, true);
                }
                else {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SLIP_STAND_B, true);
                }
            }
        }
    }
}

pub unsafe fn meleeECBs(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, fighter_kind: i32) {
    let mut offset = smash::phx::Vector2f { x: 0.0, y: 0.0};
        let mut max_offset = 0.0;
        let vanilla_ecb =     [*FIGHTER_STATUS_KIND_CAPTURE_PULLED, *FIGHTER_STATUS_KIND_CAPTURE_WAIT, *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE, *FIGHTER_STATUS_KIND_CAPTURE_CUT,
                               *FIGHTER_STATUS_KIND_THROWN].contains(&StatusModule::prev_status_kind(boma, 0)) ||
                               [*FIGHTER_STATUS_KIND_CAPTURE_PULLED, *FIGHTER_STATUS_KIND_CAPTURE_WAIT, *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE, *FIGHTER_STATUS_KIND_CAPTURE_CUT,
                                *FIGHTER_STATUS_KIND_ENTRY, *FIGHTER_STATUS_KIND_THROWN, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, 
                                *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
                                *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D, *FIGHTER_STATUS_KIND_DAMAGE_FALL, *FIGHTER_STATUS_KIND_TREAD_DAMAGE_AIR, *FIGHTER_STATUS_KIND_BURY,
                                *FIGHTER_STATUS_KIND_BURY_WAIT].contains(&status_kind);
                                
        let air_trans = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR) < 10;
                
            
        if  [*FIGHTER_KIND_KIRBY, *FIGHTER_KIND_PIKACHU, *FIGHTER_KIND_NESS, *FIGHTER_KIND_PURIN, *FIGHTER_KIND_GAMEWATCH, *FIGHTER_KIND_POPO, *FIGHTER_KIND_NANA, 
            *FIGHTER_KIND_PICHU, *FIGHTER_KIND_METAKNIGHT, *FIGHTER_KIND_WARIO, *FIGHTER_KIND_PZENIGAME, *FIGHTER_KIND_PFUSHIGISOU, *FIGHTER_KIND_LUCAS, 
            *FIGHTER_KIND_PIKMIN, *FIGHTER_KIND_TOONLINK, *FIGHTER_KIND_DUCKHUNT, *FIGHTER_KIND_MURABITO, *FIGHTER_KIND_INKLING, *FIGHTER_KIND_SHIZUE].contains(&fighter_kind) {
                max_offset = 2.0;
            }
            
        if  [*FIGHTER_KIND_MARIO, *FIGHTER_KIND_YOSHI, *FIGHTER_KIND_LUIGI, *FIGHTER_KIND_MARIOD, *FIGHTER_KIND_YOUNGLINK, *FIGHTER_KIND_PLIZARDON, *FIGHTER_KIND_DIDDY, 
            *FIGHTER_KIND_DEDEDE, *FIGHTER_KIND_ROCKMAN, *FIGHTER_KIND_GEKKOUGA, *FIGHTER_KIND_PACMAN, *FIGHTER_KIND_KOOPAJR, *FIGHTER_KIND_PACKUN, *FIGHTER_KIND_MIIFIGHTER, 
            *FIGHTER_KIND_MIISWORDSMAN, *FIGHTER_KIND_MIIGUNNER, *FIGHTER_KIND_PACKUN, *FIGHTER_KIND_BUDDY].contains(&fighter_kind) {
                max_offset = 3.5;
            }
            
        if  [*FIGHTER_KIND_FOX, *FIGHTER_KIND_FALCO, *FIGHTER_KIND_DAISY, *FIGHTER_KIND_MEWTWO, *FIGHTER_KIND_PIT, *FIGHTER_KIND_PITB, *FIGHTER_KIND_SONIC, *FIGHTER_KIND_LUCARIO, 
            *FIGHTER_KIND_ROBOT, *FIGHTER_KIND_WOLF, *FIGHTER_KIND_LITTLEMAC, *FIGHTER_KIND_KROOL, *FIGHTER_KIND_GAOGAEN, *FIGHTER_KIND_TANTAN, *FIGHTER_KIND_PICKEL].contains(&fighter_kind) {
                max_offset = 4.0;
            }
            
        if  [*FIGHTER_KIND_DONKEY, *FIGHTER_KIND_LINK, *FIGHTER_KIND_SAMUS, *FIGHTER_KIND_SAMUSD, *FIGHTER_KIND_CAPTAIN, *FIGHTER_KIND_PEACH, *FIGHTER_KIND_KOOPA, 
            *FIGHTER_KIND_SHEIK, *FIGHTER_KIND_ZELDA, *FIGHTER_KIND_MARTH, *FIGHTER_KIND_LUCINA, *FIGHTER_KIND_GANON, *FIGHTER_KIND_ROY, *FIGHTER_KIND_CHROM, 
            *FIGHTER_KIND_SZEROSUIT, *FIGHTER_KIND_SNAKE, *FIGHTER_KIND_IKE, *FIGHTER_KIND_WIIFIT, *FIGHTER_KIND_ROSETTA, *FIGHTER_KIND_PALUTENA, 
            *FIGHTER_KIND_REFLET, *FIGHTER_KIND_SHULK, *FIGHTER_KIND_RYU, *FIGHTER_KIND_KEN, *FIGHTER_KIND_CLOUD, *FIGHTER_KIND_KAMUI, *FIGHTER_KIND_BAYONETTA, 
            *FIGHTER_KIND_RIDLEY, *FIGHTER_KIND_SIMON, *FIGHTER_KIND_RICHTER, *FIGHTER_KIND_JACK, *FIGHTER_KIND_BRAVE, *FIGHTER_KIND_DOLLY, *FIGHTER_KIND_MASTER, *FIGHTER_KIND_EDGE].contains(&fighter_kind) {
                max_offset = 5.0;
            }
            
        if status_kind == *FIGHTER_STATUS_KIND_ENTRY {
                max_offset = 0.0;
            }
            
        if (StatusModule::prev_status_kind(boma, 0) == *FIGHTER_STATUS_KIND_PASS) && MotionModule::frame(boma) < 10.0 {
                max_offset = 0.0;
            }
            
        if situation_kind == *SITUATION_KIND_AIR {// || status_kind == FIGHTER_STATUS_KIND_JUMP || status_kind == FIGHTER_STATUS_KIND_JUMP_AERIAL || status_kind == FIGHTER_STATUS_KIND_FALL || status_kind == FIGHTER_STATUS_KIND_FALL_AERIAL || status_kind == FIGHTER_STATUS_KIND_FALL_SPECIAL){
    
            if ECB_Y_OFFSETS[get_player_number(boma)] < max_offset {
                ECB_Y_OFFSETS[get_player_number(boma)] += 0.05;
            }
            else {
                ECB_Y_OFFSETS[get_player_number(boma)] = max_offset;
            }
                
            //ecb_y_offsets[WorkModule::get_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID)] = max_offset;
                
            offset.x = 0.0;
            offset.y = ECB_Y_OFFSETS[get_player_number(boma)];

            if !(vanilla_ecb || air_trans) {
                GroundModule::set_rhombus_offset(boma, &offset);
            }
        }
            
        else if situation_kind == *SITUATION_KIND_GROUND {
            max_offset = 0.0;
            //ecb_y_offsets[nx::utils::get_player_number(boma)] = max_offset;
                
            offset.x = 0.0;
            //offset.y = ecb_y_offsets[nx::utils::get_player_number(boma)];
            offset.y = 0.0;
            if !vanilla_ecb {
                GroundModule::set_rhombus_offset(boma, &offset);
            }
                
        }
            
        else{
            ECB_Y_OFFSETS[get_player_number(boma)] = 0.0;
            offset.x = 0.0;
            offset.y = ECB_Y_OFFSETS[get_player_number(boma)];
                
            if !vanilla_ecb {
                GroundModule::set_rhombus_offset(boma, &offset);
            }
        }
}

pub unsafe fn calcMomentum(boma: &mut smash::app::BattleObjectModuleAccessor) -> f32 {
    let jump_speed_x = WorkModule::get_param_float(boma, hash40("jump_speed_x"), 0);
    let jump_speed_x_mul = WorkModule::get_param_float(boma, hash40("jump_speed_x_mul"), 0);
    let stick_x = ControlModule::get_stick_x(boma);
    let x_vel = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let jump_speed_x_max = WorkModule::get_param_float(boma, hash40("jump_speed_x_max"), 0);
    let calcJumpSpeed = (jump_speed_x * stick_x) + (jump_speed_x_mul * x_vel);
    let mut jumpSpeedClamped = 0.0;
    if x_vel < 0.0 {
        jumpSpeedClamped = returnLarge(calcJumpSpeed, -1.0 * jump_speed_x_max);
    }
    else {
        jumpSpeedClamped = returnSmall(calcJumpSpeed, jump_speed_x_max);
    }
    jumpSpeedClamped
}

pub unsafe fn lagCanceled(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32) {
    if [*FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR].contains(&status_kind) {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            LAGCANCELED[get_player_number(boma)] = true;
        }
    }
    else {
        LAGCANCELED[get_player_number(boma)] = false;
    }
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR && MotionModule::frame(boma) < 1.0 {
        LAGCANCELED[get_player_number(boma)] = false;
    }
}

pub unsafe fn removeSHMacro(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32) {
    if TOGGELS[0][get_player_number(boma)] == 0 {
        if status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT || StatusModule::prev_status_kind(boma, 0) == *FIGHTER_STATUS_KIND_JUMP_SQUAT 
        || StatusModule::prev_status_kind(boma, 1) == *FIGHTER_STATUS_KIND_JUMP_SQUAT {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) || (ControlModule::is_enable_flick_jump(boma) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_FLICK_JUMP)) {
                WorkModule::off_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
            }
            else {
                WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_JUMP {
            if MotionModule::frame(boma) < 2.0 {
                if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) || (ControlModule::is_enable_flick_jump(boma) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_FLICK_JUMP)) {
                    WorkModule::off_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
                }
                else {
                    WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
                }
            }
        }
    }
}

pub unsafe fn fixExtraJumps (boma: &mut smash::app::BattleObjectModuleAccessor, situation_kind: i32, motion_kind: u64, fighter_kind: i32) {
    if [*FIGHTER_KIND_RIDLEY, *FIGHTER_KIND_PIT].contains(&fighter_kind) {
        if [hash40("jump_aerial_f3"), hash40("jump_aerial_f4")].contains(&motion_kind) {
            MotionModule::change_motion(boma, Hash40{hash: hash40("jump_aerial_f2")}, 0.0, 1.0, false, 0.0, false, false);
        }
    }
    if fighter_kind == *FIGHTER_KIND_GEKKOUGA {
        if motion_kind == 0 {
            if PostureModule::lr(boma) < 0.0 {
                MotionModule::change_motion(boma, Hash40{hash: hash40("appeal_lw_l")}, 0.0, 1.0, false, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(boma, Hash40{hash: hash40("appeal_lw_r")}, 0.0, 1.0, false, 0.0, false, false);
            }
        }
        else if [82155291285, 67420653553, 100249898067].contains(&motion_kind) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_N_MAX_SHOT, true);
        }
    }
}

pub unsafe fn disableStuff (boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, fighter_kind: i32) {
    if [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_ATTACK_S3, 
        *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_100, *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_HI4,
        *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_STATUS_KIND_AIR_LASSO].contains(&status_kind) { //Change last used move
        if !ISDISABLED[get_player_number(boma)] {
            if DISABLEDMOVE[get_player_number(boma)] != status_kind {
                DISABLEDMOVE[get_player_number(boma)] = status_kind;
            }
        }
    }
    if status_kind == *FIGHTER_STATUS_KIND_BIND {
        if MotionModule::frame(boma) < 1.0 {
            if !ISDISABLED[get_player_number(boma)] {
                ISDISABLED[get_player_number(boma)] = true;
                DISABLESTART[get_player_number(boma)] = GLOBALFRAMECOUNT;
            }
        }
    }
    if ISDISABLED[get_player_number(boma)] {
        if GLOBALFRAMECOUNT - DISABLESTART[get_player_number(boma)] <= 1200 {
            ISDISABLED[get_player_number(boma)] = false;
            DISABLESTART[get_player_number(boma)] = 0;
        }
    }
}

pub unsafe fn regainAirDodge(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    if situation_kind != *SITUATION_KIND_AIR {
        CANAIRDODGE[get_player_number(boma)] = true;
    }
    if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
        CANAIRDODGE[get_player_number(boma)] = false;
    }
}

pub unsafe fn deathStuff (boma: &mut smash::app::BattleObjectModuleAccessor) {
    if DamageModule::damage(boma, 0) >= 225.0 {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DEAD, true);
        DamageModule::add_damage(boma, -1.0 * DamageModule::damage(boma, 0), 0);
    }
}

pub unsafe fn squirtleSideB (boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, fighter_kind: i32, motion_kind: u64, cat1: i32) {
    if fighter_kind == *FIGHTER_KIND_PZENIGAME {
        /*if motion_kind == hash40("special_air_s_hit") {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP, true);
        }*/
        if status_kind == *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_HIT {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP, true);
        }
        if status_kind == *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_LOOP {
            if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0 || ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_END, true);
            }
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) || (ControlModule::is_enable_flick_jump(boma) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_FLICK_JUMP)) {
                if situation_kind == *SITUATION_KIND_AIR{
                    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX){
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
                    }
                }
                else if situation_kind == *SITUATION_KIND_GROUND{
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                }
            }
        }
    }
}

pub unsafe fn airDodgeCancels (boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, fighter_kind: i32, cat1: i32) {
    if situation_kind == *SITUATION_KIND_AIR {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE) != 0 {
            if fighter_kind == *FIGHTER_KIND_WOLF {
                if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
                    if MotionModule::frame(boma) > 16.0 {
                        CancelModule::enable_cancel(boma);
                    }
                }
            }
            else if fighter_kind == *FIGHTER_KIND_GEKKOUGA {
                if status_kind == *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_N_HOLD {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
                }
            }
            else if fighter_kind == *FIGHTER_KIND_DIDDY {
                if [*FIGHTER_DIDDY_STATUS_KIND_SPECIAL_N_SHOOT, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_N_CHARGE, *FIGHTER_STATUS_KIND_SPECIAL_N].contains(&status_kind) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
                }
            }
            else if fighter_kind == *FIGHTER_KIND_BAYONETTA {
                if status_kind == *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_N_CHARGE {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
                }
            }
            else if fighter_kind == *FIGHTER_KIND_EDGE {
                if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
                }
            }
        }
    }
    if fighter_kind == *FIGHTER_KIND_DIDDY {
        if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_JUMP) || (ControlModule::is_enable_flick_jump(boma) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_FLICK_JUMP)) {
            if [*FIGHTER_DIDDY_STATUS_KIND_SPECIAL_N_CHARGE, *FIGHTER_STATUS_KIND_SPECIAL_N].contains(&status_kind) {
                if situation_kind == *SITUATION_KIND_AIR{
                    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX){
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
                    }
                }
                else if situation_kind == *SITUATION_KIND_GROUND{
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                }
            }
        }
    }
}

pub unsafe fn hitfalling (boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    if situation_kind == *SITUATION_KIND_AIR {
        if [*FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_AIR_LASSO].contains(&status_kind) {
            for i in 5..1 {
                HITFALLBUFFER[i][get_player_number(boma)] = HITFALLBUFFER[i-1][get_player_number(boma)];
            }
            HITFALLBUFFER[0][get_player_number(boma)] = AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT);
            if stick_y_flick_check(boma, -0.3) && (HITFALLBUFFER[5][get_player_number(boma)] || HITFALLBUFFER[4][get_player_number(boma)] || HITFALLBUFFER[3][get_player_number(boma)] ||
                HITFALLBUFFER[2][get_player_number(boma)] || HITFALLBUFFER[1][get_player_number(boma)] || HITFALLBUFFER[0][get_player_number(boma)]) {
                    WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
            }
        }
    }
}

pub unsafe fn fastfallShit (boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, fighter_kind: i32) {
    if situation_kind == *SITUATION_KIND_AIR {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
            if [*FIGHTER_KIND_GEKKOUGA, *FIGHTER_KIND_WOLF, *FIGHTER_KIND_FALCO, *FIGHTER_KIND_FOX, *FIGHTER_KIND_MARIO, *FIGHTER_KIND_DIDDY, *FIGHTER_KIND_CLOUD, *FIGHTER_KIND_PICHU, *FIGHTER_KIND_KEN].contains(&fighter_kind) {
                if stick_y_flick_check(boma, -0.3) && KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) <= 0.0 {
                    WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
                }
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
            if [*FIGHTER_KIND_PFUSHIGISOU, *FIGHTER_KIND_NESS, *FIGHTER_KIND_JACK].contains(&fighter_kind) {
                if stick_y_flick_check(boma, -0.3) && KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) <= 0.0 {
                    WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
                }
            }
        }
        if status_kind == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_S_SHOOT {
            if fighter_kind == *FIGHTER_KIND_EDGE {
                if stick_y_flick_check(boma, -0.3) && KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) <= 0.0 {
                    WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
                }
            }
        }
    }
}

pub unsafe fn pkFlashCancels (boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, fighter_kind: i32, cat1: i32) {
    if fighter_kind == *FIGHTER_KIND_NESS {
        if [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_NESS_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_NESS_STATUS_KIND_SPECIAL_N_FIRE, *FIGHTER_NESS_STATUS_KIND_SPECIAL_N_END].contains(&status_kind) {
            if MotionModule::frame(boma) > 4.0 {
                CancelModule::enable_cancel(boma);
            }
        }
        else if [*FIGHTER_STATUS_KIND_JUMP, *FIGHTER_STATUS_KIND_JUMP_AERIAL].contains(&status_kind) {
            if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 {
                CancelModule::enable_cancel(boma);
            }
        }
        else {
            if MotionModule::frame(boma) > MotionModule::end_frame(boma) - 6.0 {
                if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0 {
                    CancelModule::enable_cancel(boma);
                }
            }
        }
    }
}

pub unsafe fn quickAttackCancels (boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, fighter_kind: i32, stick_value_y: f32) {
    if [*FIGHTER_KIND_PICHU, *FIGHTER_KIND_GEKKOUGA].contains(&fighter_kind) {
        if [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_WARP, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_END,
        *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_HI_LOOP, *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_HI_END].contains(&status_kind) {
            if situation_kind == *SITUATION_KIND_GROUND {
                if (GroundModule::is_passable_ground(boma) && stick_value_y <= -0.75) || ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_JUMP) 
                || (ControlModule::is_enable_flick_jump(boma) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_FLICK_JUMP)) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
                }
            }
        }
    }
    if status_kind == *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_HI_WALL_DAMAGE {
        if fighter_kind == *FIGHTER_KIND_GEKKOUGA {
            CancelModule::enable_cancel(boma);
        }
    }
    else if fighter_kind == *FIGHTER_KIND_PIKACHU {
        if [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_WARP, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_END].contains(&status_kind) {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                HITMOVE[get_player_number(boma)] = true;
            }
        }
        if [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_WARP, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_END].contains(&status_kind) {
            if HITMOVE[get_player_number(boma)] {
                if situation_kind == *SITUATION_KIND_GROUND {
                    if (GroundModule::is_passable_ground(boma) && stick_value_y <= -0.75) || ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_JUMP) 
                    || (ControlModule::is_enable_flick_jump(boma) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_FLICK_JUMP)) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
                    }
                }
            }
        }
        else {
            HITMOVE[get_player_number(boma)] = false;
        }
    }
}

pub unsafe fn moonwalking (boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, stick_value_x: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_DASH {
        if stick_value_x * PostureModule::lr(boma) < -0.3 {
            //WorkModule::get_param_float(boma, 0, hash40("dash_speed"))
            let reverse_speed = Vector3f{x: 0.45 * stick_value_x * PostureModule::lr(boma), y: 0.0, z: 0.0};
            KineticModule::add_speed(boma, &reverse_speed);
        }
    }
}

pub unsafe fn upbCancels (lua_state: u64, l2c_agent: &mut L2CAgent, boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, fighter_kind: i32, cat1: i32) {
    if (fighter_kind == *FIGHTER_KIND_CHROM && status_kind == *FIGHTER_IKE_STATUS_KIND_SPECIAL_HI_3)  || (fighter_kind == *FIGHTER_KIND_IKE && status_kind == *FIGHTER_IKE_STATUS_KIND_SPECIAL_HI_2) {
        if KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) <= 0.0 && MotionModule::frame(boma) > 22.0 {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE) != 0 {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
            }
        }
    }
    if fighter_kind == *FIGHTER_KIND_GAOGAEN {
        if [*FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_FALL, *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_BOUND, *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_LOOP].contains(&status_kind) {
            if KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) <= 0.0 {
                if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE) != 0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
                    l2c_agent.clear_lua_stack();
                    l2c_agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY as u64));
                    l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.0));
                    smash::app::sv_kinetic_energy::set_speed(lua_state);
                }
            }
        }
    }
}

pub unsafe fn ptSwaps (boma: &mut smash::app::BattleObjectModuleAccessor, fighter_kind: i32, cat1: i32, cat2: i32) {
    if [*FIGHTER_KIND_PZENIGAME, *FIGHTER_KIND_PFUSHIGISOU, *FIGHTER_KIND_PLIZARDON].contains(&fighter_kind) {
        if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L) != 0 {
            if CancelModule::is_enable_cancel(boma) {
                if fighter_kind == *FIGHTER_KIND_PZENIGAME {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_LW_OUT, true);
                }
                else if fighter_kind == *FIGHTER_KIND_PFUSHIGISOU {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_PFUSHIGISOU_STATUS_KIND_SPECIAL_LW_OUT, true);
                }
                else if fighter_kind == *FIGHTER_KIND_PLIZARDON {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_LW_OUT, true);
                }
            }
        }
        else if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R) != 0 {
            if CancelModule::is_enable_cancel(boma) {
                if fighter_kind == *FIGHTER_KIND_PZENIGAME {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_LW_OUT, true);
                }
                else if fighter_kind == *FIGHTER_KIND_PFUSHIGISOU {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_PFUSHIGISOU_STATUS_KIND_SPECIAL_LW_OUT, true);
                }
                else if fighter_kind == *FIGHTER_KIND_PLIZARDON {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_LW_OUT, true);
                }
                DOUBLESWAP[get_player_number(boma)] = true;
            }
        }
        if StatusModule::prev_status_kind(boma, 0) == 470 {
            if DOUBLESWAP[get_player_number(boma)] {
                StatusModule::change_status_request_from_script(boma, 470, true);
                DOUBLESWAP[get_player_number(boma)] = false;
            }
        }
        if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 {
            if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_S) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_APPEAL, true);
                MotionModule::change_motion(boma, Hash40{hash: hash40("appeal_s_r")}, 0.0, 1.0, false, 0.0, false, false);
            }
        }
    }
}

pub unsafe fn driftDi (boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, stick_value_x: f32) {
    if [*FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FLY].contains(&status_kind) {
        let speed_vector = smash::phx::Vector3f { x: KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) * stick_value_x * 0.2, y: 0.0, z:0.0 };
        KineticModule::add_speed(boma, &speed_vector);
    }
}

pub unsafe fn grenShurikenFix (boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, fighter_kind: i32) {
    if fighter_kind == *FIGHTER_KIND_GEKKOUGA {
        if status_kind == *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_N_SHOT {
            if situation_kind == *SITUATION_KIND_AIR {
                if KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) > WorkModule::get_param_float(boma, 0, hash40("air_speed_y_stable")) {
                    let speed_vector = smash::phx::Vector3f {x: 0.0, y: WorkModule::get_param_float(boma, 0, hash40("air_speed_y_stable")), z: 0.0 };
                    KineticModule::add_speed(boma, &speed_vector);
                }
            }
        }
    }
}

pub unsafe fn animPortFix (lua_state: u64, l2c_agent: &mut L2CAgent, boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, fighter_kind: i32, motion_kind: u64, stick_value_y: f32) {
    if fighter_kind == *FIGHTER_KIND_MARIO {
        if motion_kind == hash40("attack_hi3") {
            if MotionModule::frame(boma) == MotionModule::end_frame(boma) - 1.0 {
                MotionModule::change_motion(boma, Hash40{hash: hash40("wait")}, 0.0, 1.0, false, 0.0, false, false);
            }
        }
        if StatusModule::prev_status_kind(boma, 0) == *FIGHTER_STATUS_KIND_ATTACK_HI3 {
            l2c_agent.clear_lua_stack();
            l2c_agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_CONTROL as u64));
            l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.0));
            smash::app::sv_kinetic_energy::set_speed(lua_state);
        }
    }
    else if fighter_kind == *FIGHTER_KIND_CAPTAIN {
        if [hash40("attack_hi3"), hash40("catch_dash")].contains(&motion_kind) {
            if MotionModule::frame(boma) == MotionModule::end_frame(boma) - 2.0 {
                MotionModule::change_motion(boma, Hash40{hash: hash40("wait")}, 0.0, 1.0, false, 0.0, false, false);
            }
        }
        if StatusModule::prev_status_kind(boma, 0) == *FIGHTER_STATUS_KIND_ATTACK_HI3 && MotionModule::frame(boma) < 1.0 {
            l2c_agent.clear_lua_stack();
            l2c_agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_CONTROL as u64));
            l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.0));
            smash::app::sv_kinetic_energy::set_speed(lua_state);
        }
    }
    else if fighter_kind == *FIGHTER_KIND_MARTH {
        if [hash40("throw_f"), hash40("special_s3_lw"), hash40("special_s1"), hash40("special_s2_lw"), hash40("special_s2_hi"), hash40("special_s3_hi"), hash40("special_s3_s"),
        hash40("special_s4_s"), hash40("special_s4_hi")].contains(&motion_kind) {
            if MotionModule::frame(boma) == MotionModule::end_frame(boma) - 2.0 {
                MotionModule::change_motion(boma, Hash40{hash: hash40("wait")}, 0.0, 1.0, false, 0.0, false, false);
                l2c_agent.clear_lua_stack();
                l2c_agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_CONTROL as u64));
                l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.0));
                smash::app::sv_kinetic_energy::set_speed(lua_state);
            }
        }
        if [*FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S3, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S4].contains(&StatusModule::prev_status_kind(boma, 0)) && MotionModule::frame(boma) < 1.0 {
            l2c_agent.clear_lua_stack();
            l2c_agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_CONTROL as u64));
            l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.0));
            smash::app::sv_kinetic_energy::set_speed(lua_state);
        }
    }
    else if fighter_kind == *FIGHTER_KIND_FALCO {
        if [hash40("attack_hi4"), hash40("escape_n")].contains(&motion_kind) {
            if MotionModule::frame(boma) == MotionModule::end_frame(boma) - 2.0 {
                MotionModule::change_motion(boma, Hash40{hash: hash40("wait")}, 0.0, 1.0, false, 0.0, false, false);
            }
        }
        if StatusModule::prev_status_kind(boma, 0) == *FIGHTER_STATUS_KIND_ATTACK_HI4 && MotionModule::frame(boma) < 1.0 {
            l2c_agent.clear_lua_stack();
            l2c_agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_CONTROL as u64));
            l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.0));
            smash::app::sv_kinetic_energy::set_speed(lua_state);
        }
        if [hash40("special_lw"), hash40("special_lw_r"), hash40("special_air_lw"), hash40("special_air_lw_r")].contains(&motion_kind) {
            if MotionModule::frame(boma) > 1.0 {
                if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_GUARD) {
                    if situation_kind == *SITUATION_KIND_GROUND {
                        if PostureModule::lr(boma) < 0.0 {
                            MotionModule::change_motion(boma, Hash40{hash: hash40("ladder_catch_l")}, 0.0, 1.0, false, 0.0, false, false);
                        }
                        else {
                            MotionModule::change_motion(boma, Hash40{hash: hash40("ladder_catch_r")}, 0.0, 1.0, false, 0.0, false, false);
                        }
                    }
                    else {
                        if PostureModule::lr(boma) < 0.0 {
                            MotionModule::change_motion(boma, Hash40{hash: hash40("ladder_catch_air_l")}, 0.0, 1.0, false, 0.0, false, false);
                        }
                        else {
                            MotionModule::change_motion(boma, Hash40{hash: hash40("ladder_catch_air_r")}, 0.0, 1.0, false, 0.0, false, false);
                        }
                    }
                }
            }
        }
    }
    else if fighter_kind == *FIGHTER_KIND_CHROM {
        if motion_kind == hash40("attack_s3_s") {
            if MotionModule::frame(boma) == MotionModule::end_frame(boma) - 2.0 {
                MotionModule::change_motion(boma, Hash40{hash: hash40("wait")}, 0.0, 1.0, false, 0.0, false, false);
                l2c_agent.clear_lua_stack();
                l2c_agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_CONTROL as u64));
                l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.0));
                smash::app::sv_kinetic_energy::set_speed(lua_state);
            }
            else if MotionModule::frame(boma) < 2.0 {
                if stick_value_y > 0.25 {
                    MotionModule::change_motion(boma, Hash40{hash: hash40("ladder_catch_end_l")}, 0.0, 1.0, false, 0.0, false, false);
                }
                else if stick_value_y < -0.25 {
                    MotionModule::change_motion(boma, Hash40{hash: hash40("ladder_catch_end_r")}, 0.0, 1.0, false, 0.0, false, false);
                }
            }
        }
        else if [hash40("ladder_catch_end_l"), hash40("ladder_catch_end_r")].contains(&motion_kind) {
            StatusModule::set_situation_kind(boma, smash::cpp::root::app::SituationKind(*SITUATION_KIND_GROUND), false);
        }
        else if [hash40("special_air_s2_hi"), hash40("special_air_s2_lw")].contains(&motion_kind) {
            if MotionModule::frame(boma) > 18.0 {
                CancelModule::enable_cancel(boma);
            }
        }
        if StatusModule::prev_status_kind(boma, 0) == *FIGHTER_STATUS_KIND_ATTACK_S3 && MotionModule::frame(boma) < 1.0 {
            l2c_agent.clear_lua_stack();
            l2c_agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_CONTROL as u64));
            l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.0));
            smash::app::sv_kinetic_energy::set_speed(lua_state);
        }
    }
    else if fighter_kind == *FIGHTER_KIND_PALUTENA {
        if [hash40("special_n"), hash40("special_air_n")].contains(&motion_kind) {
            if MotionModule::frame(boma) >= 40.0 {
                CancelModule::enable_cancel(boma);
            }
        }
    }
}

pub unsafe fn bReverses (boma: &mut smash::app::BattleObjectModuleAccessor, motion_kind: u64, fighter_kind: i32) {
    if fighter_kind == FIGHTER_KIND_CAPTAIN {
        if motion_kind == hash40("special_air_lw") {
            if MotionModule::frame(boma) < 6.0 {
                if ControlModule::get_stick_x(boma) * PostureModule::lr(boma) <= -0.50 {
                    BREVERSE[get_player_number(boma)] = true;;
                }
            }
            else if MotionModule::frame(boma) >= 6.0 && MotionModule::frame(boma) < 7.0 {
                if BREVERSE[get_player_number(boma)] {
                    PostureModule::reverse_lr(boma);
                    PostureModule::update_rot_y_lr(boma);
                    KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
                }                
            }
            else if MotionModule::frame(boma) >= 16.0 {
                if BREVERSE[get_player_number(boma)] {
                    let reverse_mul = Vector3f{x: -1.0, y: 1.0, z: 1.0};
                    KineticModule::mul_speed(boma, &reverse_mul, *FIGHTER_KINETIC_ENERGY_ID_NONE);
                    MotionModule::update_trans_move_speed(boma);
                }
            }
        }
        else {
            BREVERSE[get_player_number(boma)] = false;
        }
    }
    if fighter_kind == *FIGHTER_KIND_PZENIGAME {
        if motion_kind == hash40("special_air_n") {
            if MotionModule::frame(boma) < 6.0 {
                if ControlModule::get_stick_x(boma) * PostureModule::lr(boma) <= -0.50 {
                    BREVERSE[get_player_number(boma)] = true;;
                }
            }
            else if MotionModule::frame(boma) < 7.0 {
                if BREVERSE[get_player_number(boma)] {
                    PostureModule::reverse_lr(boma);
                    PostureModule::update_rot_y_lr(boma);
                    KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
                    let reverse_mul = Vector3f{x: -1.0, y: 1.0, z: 1.0};
                    KineticModule::mul_speed(boma, &reverse_mul, *FIGHTER_KINETIC_ENERGY_ID_NONE);
                    MotionModule::update_trans_move_speed(boma);
                }
            }
        }
        else {
            BREVERSE[get_player_number(boma)] = false;
        }
    }
}

pub unsafe fn fixProjectiles (boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, fighter_kind: i32) {
    if !CANPROJECTILE[get_player_number(boma)] {
        if [*FIGHTER_KIND_PFUSHIGISOU, *FIGHTER_KIND_DONKEY].contains(&status_kind) {
            if status_kind != *FIGHTER_STATUS_KIND_SPECIAL_S || MotionModule::frame(boma) < 1.0 {
                CANPROJECTILE[get_player_number(boma)] = true;
            }
        }
        else if [*FIGHTER_KIND_PICHU, *FIGHTER_KIND_EDGE, *FIGHTER_KIND_MARIO].contains(&fighter_kind) {
            if [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_N_SHOOT].contains(&fighter_kind) || MotionModule::frame(boma) < 1.0 {
                CANPROJECTILE[get_player_number(boma)] = true;
            }
        }
        else if fighter_kind == *FIGHTER_KIND_GEKKOUGA {
            if status_kind != *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_N_MAX_SHOT || MotionModule::frame(boma) < 1.0 {
                CANPROJECTILE[get_player_number(boma)] = true;
            }
        }
    }
}

pub unsafe fn magicSeries (boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, motion_kind: u64, fighter_kind: i32, stick_value_x: f32, cat1: i32) {
    if fighter_kind == *FIGHTER_KIND_KEN {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            if [*FIGHTER_STATUS_KIND_ATTACK_100, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_STATUS_KIND_ATTACK].contains(&status_kind) { //Jab/da cancels
                if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 ||
                (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0 ||
                (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 {
                    CancelModule::enable_cancel(boma);
                }
            }
            if [*FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND1, *FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND2, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_S3].contains(&status_kind) { //tilts cancels
                if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) != 0 ||
                (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0 {
                    CancelModule::enable_cancel(boma);
                }
            }
            if [*FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_AIR].contains(&status_kind) { //Smashes/aerials into specials
                if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 {
                    CancelModule::enable_cancel(boma);
                }
            }
            if motion_kind == hash40("attack_air_n") { //nair cancels
                if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 ||
                (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) != 0 {
                    CancelModule::enable_cancel(boma);
                }
            }
            if motion_kind == hash40("attack_air_hi") { //up air cancels
                if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) != 0 {
                    CancelModule::enable_cancel(boma);
                }
            }
            if motion_kind == hash40("attack_air_f") || motion_kind == hash40("attack_air_lw") { //dair/fair cancels
                if ((cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) != 0) && PostureModule::lr(boma) * stick_value_x < 0.0 {
                    CancelModule::enable_cancel(boma);
                }
            }
            if [*FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND, *FIGHTER_STATUS_KIND_SPECIAL_S, 459, 475, 476].contains(&status_kind) { //tatsu cancels into tilts/aerials
                if situation_kind == *SITUATION_KIND_AIR {
                    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 ||
                    (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, true);
                    }
                }
                else if situation_kind == *SITUATION_KIND_GROUND {
                    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3, true);
                    }
                    else if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3, true);
                    }
                    else if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) != 0 {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
                    }
                }
            }
        }
    }
}

pub unsafe fn inklingStuff (lua_state: u64, l2c_agent: &mut L2CAgent, boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, fighter_kind: i32, cat1: i32, stick_value_x: f32) {
    if fighter_kind == *FIGHTER_KIND_INKLING {
        if [*FIGHTER_INKLING_STATUS_KIND_CHARGE_INK, *FIGHTER_INKLING_STATUS_KIND_CHARGE_INK_START, *FIGHTER_INKLING_STATUS_KIND_CHARGE_INK_END].contains(&status_kind) {
            let speed_vector = smash::phx::Vector3f {x: WorkModule::get_param_float(boma, 0, hash40("walk_speed_max")) * stick_value_x, y: 0.0, z: 0.0 };
            KineticModule::add_speed(boma, &speed_vector);
            if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 {
                CancelModule::enable_cancel(boma);
            }
        }
    }
}

pub unsafe fn toggleAutoTurnaround (boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, motion_kind: u64, fighter_kind: i32, stick_value_x: f32) {
    if fighter_kind == *FIGHTER_KIND_RYU || fighter_kind == *FIGHTER_KIND_KEN || fighter_kind == *FIGHTER_KIND_DOLLY {
        if (motion_kind == hash40("appeal_hi_l") || motion_kind == hash40("appeal_hi_r")) && MotionModule::frame(boma) < 1.0 {
            ISAUTOTURNAROUND[get_player_number(boma)] = !ISAUTOTURNAROUND[get_player_number(boma)];
        }
        if [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_WALK].contains(&status_kind) {
            if !ISAUTOTURNAROUND[get_player_number(boma)] {
                if stick_value_x * PostureModule::lr(boma) < 0.0 {
                    PostureModule::reverse_lr(boma);
                }
            }
        }
    }
}

pub unsafe fn mewtwoTail (boma: &mut smash::app::BattleObjectModuleAccessor, fighter_kind: i32) {
    if fighter_kind == *FIGHTER_KIND_DONKEY {
        HitModule::set_status_joint(boma, Hash40{hash: hash40("necktie")}, HitStatus(*HIT_STATUS_XLU),0);
        HitModule::set_status_joint(boma, Hash40{hash: hash40("s_necktie1")}, HitStatus(*HIT_STATUS_XLU),0);
        HitModule::set_status_joint(boma, Hash40{hash: hash40("s_necktie2")}, HitStatus(*HIT_STATUS_XLU),0);
        HitModule::set_status_joint(boma, Hash40{hash: hash40("s_necktie3")}, HitStatus(*HIT_STATUS_XLU),0);
        HitModule::set_status_joint(boma, Hash40{hash: hash40("s_necktie4")}, HitStatus(*HIT_STATUS_XLU),0);
        HitModule::set_status_joint(boma, Hash40{hash: hash40("s_necktie5_null")}, HitStatus(*HIT_STATUS_XLU),0);
    }
    else if fighter_kind == *FIGHTER_KIND_PLIZARDON {
        HitModule::set_status_joint(boma, Hash40{hash: hash40("tail1")}, HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(boma, Hash40{hash: hash40("tail2")}, HitStatus(*HIT_STATUS_XLU),0);
        HitModule::set_status_joint(boma, Hash40{hash: hash40("tail3")}, HitStatus(*HIT_STATUS_XLU),0);
        HitModule::set_status_joint(boma, Hash40{hash: hash40("tail4")}, HitStatus(*HIT_STATUS_XLU),0);
    }
}

pub unsafe fn teleportCancels(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, fighter_kind: i32) {
    if fighter_kind == *FIGHTER_KIND_MEWTWO {
        if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
            if status_kind == *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3 {
                if [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3].contains(&StatusModule::prev_status_kind(boma, 0)) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
                    UPBCANCEL[get_player_number(boma)] = true;
                }
            }
        }
        if situation_kind != *SITUATION_KIND_AIR {
            UPBCANCEL[get_player_number(boma)] = false;
        }
    }
    if fighter_kind == *FIGHTER_KIND_PALUTENA {
        if [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_3].contains(&status_kind) {
            USEDUPB[get_player_number(boma)] = true;
        }
        if status_kind == *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_3 {
            if [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_3].contains(&StatusModule::prev_status_kind(boma, 0)) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
            }
        }
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            USEDUPB[get_player_number(boma)] = false;
        }
        if situation_kind != *SITUATION_KIND_AIR || [*FIGHTER_STATUS_KIND_DAMAGE_FALL, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        *FIGHTER_STATUS_KIND_DAMAGE_SONG, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_FALL, *FIGHTER_STATUS_KIND_DAMAGE_SONG_START, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_START,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U, *FIGHTER_STATUS_KIND_DAMAGE].contains(&status_kind) {
            USEDUPB[get_player_number(boma)] = false;
        }
    }
}

pub unsafe fn canDkBarrel(boma: &mut smash::app::BattleObjectModuleAccessor, fighter_kind: i32) {
    if fighter_kind == *FIGHTER_KIND_DONKEY {
        if !CANBARREL[get_player_number(boma)] {
            if GLOBALFRAMECOUNT - LASTBARREL[get_player_number(boma)] == 420 {
                CANBARREL[get_player_number(boma)] = true;
            }
        }
    }
}

/* pub unsafe fn noImpactLandingLag (boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32) {
    for i in 9..1 {
        RISING[i][get_player_number(boma)] = RISING[i-1][get_player_number(boma)];
    }
    RISING[0][get_player_number(boma)] = (KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) > 0.0);
    if status_kind == *FIGHTER_STATUS_KIND_LANDING {
        let mut wasRising = false;
        for i in 0..9 {
            if RISING[i][get_player_number(boma)] == true {
                wasRising = true;
            }
        }
        if wasRising {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
        }
    }
} */

pub unsafe fn shineStalls (lua_state: u64, l2c_agent: &mut L2CAgent, boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, fighter_kind: i32) {
    if situation_kind == *SITUATION_KIND_AIR {
        if (fighter_kind == *FIGHTER_KIND_FOX && [*FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_LOOP, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_STATUS_KIND_SPECIAL_LW].contains(&status_kind)) || 
        (fighter_kind == *FIGHTER_KIND_NESS && [*FIGHTER_STATUS_KIND_SPECIAL_LW].contains(&status_kind)) || 
        (fighter_kind == *FIGHTER_KIND_FALCO && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW && MotionModule::frame(boma) < 10.0) {
            l2c_agent.clear_lua_stack();
            l2c_agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY as u64));
            l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.0));
            smash::app::sv_kinetic_energy::set_speed(lua_state);
        }
    }
}

pub unsafe fn floats (lua_state: u64, l2c_agent: &mut L2CAgent, boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, fighter_kind: i32, stick_value_y: f32) {
    if [*FIGHTER_KIND_GANON, *FIGHTER_KIND_BUDDY].contains(&fighter_kind) {
        if CANFLOAT[get_player_number(boma)] {
            if (status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR || WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR)) {
                if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) && (stick_value_y < -0.5 || KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) < 0.0) {
                    if FLOATSTART[get_player_number(boma)] == 0 {
                        FLOATSTART[get_player_number(boma)] = GLOBALFRAMECOUNT;
                    }
                    if status_kind != *FIGHTER_STATUS_KIND_ATTACK_AIR {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
                    }
                    l2c_agent.clear_lua_stack();
                    l2c_agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY as u64));
                    l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.0));
                    smash::app::sv_kinetic_energy::set_speed(lua_state);
                }
            }
            if (GLOBALFRAMECOUNT - FLOATSTART[get_player_number(boma)]) >= 120 && FLOATSTART[get_player_number(boma)] != 0 {
                CANFLOAT[get_player_number(boma)] = false;
                FLOATSTART[get_player_number(boma)] = 0;
            }
            if FLOATSTART[get_player_number(boma)] != 0 && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_JUMP) {
                CANFLOAT[get_player_number(boma)] = false;
                FLOATSTART[get_player_number(boma)] = 0;
            }
        }
        if situation_kind != *SITUATION_KIND_AIR {
            CANFLOAT[get_player_number(boma)] = true;
        }
    }
}

pub unsafe fn superJumps (boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, fighter_kind: i32, cat1: i32) {
    if [*FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_100, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_LW4].contains(&status_kind) {
        if fighter_kind == *FIGHTER_KIND_KEN {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                /*let colorflashvec1 = smash::phx::Vector4f{ /* Red */ x: 221.0, /* Green */ y: 160.0, /* Blue */ z: 221.0, /* Alpha? */ w: 0.1}; // setting this and the next vector's .w to 1 seems to cause a ghostly effect
                let colorflashvec2 = smash::phx::Vector4f{ /* Red */ x: 221.0, /* Green */ y: 160.0, /* Blue */ z: 221.0, /* Alpha? */ w: 0.1};
                ColorBlendModule::set_main_color(boma, &colorflashvec1, &colorflashvec2, 0.7, 0.2, 25, true); //int here is opacity */
                if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) || (ControlModule::is_enable_flick_jump(boma) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_FLICK_JUMP)) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                }
            }
        }
    }
}

pub unsafe fn bayoCancels (boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, fighter_kind: i32, cat1: i32) {
    if fighter_kind == *FIGHTER_KIND_BAYONETTA {
        if [*FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F].contains(&status_kind) {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                if (*FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S & cat1) != 0 || (*FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI & cat1) != 0 {
                    CancelModule::enable_cancel(boma);
                }
            }
        }
        if [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_HI_JUMP, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_U, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D, 
        *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D_HIT, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_S_HOLD_END, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D_LANDING, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_WALL_END].contains(&status_kind) {
            if situation_kind == *SITUATION_KIND_AIR {
                if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                    if (*FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 & cat1) != 0 || (*FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 & cat1) != 0 || (*FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3 & cat1) != 0 || 
                    (*FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4 & cat1) != 0 || (*FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 & cat1) != 0 || (*FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4 & cat1) != 0 || 
                    (*FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N & cat1) != 0 {
                        CancelModule::enable_cancel(boma);
                    }
                }
            }
        }
    }
}

pub unsafe fn ganonFix (lua_state: u64, l2c_agent: &mut L2CAgent, boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, fighter_kind: i32) {
    if fighter_kind == *FIGHTER_KIND_GANON {
        if ![*FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, 
        *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD,*FIGHTER_STATUS_KIND_ATTACK_AIR,
        *FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, *FIGHTER_STATUS_KIND_APPEAL].contains(&status_kind) {
            ArticleModule::remove_exist(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, smash::cpp::root::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
        if status_kind == *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_FALL {
            if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_GUARD) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_THROW, true);
            } 
        }
        if situation_kind == *SITUATION_KIND_AIR {
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
                if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP, true);
                }
            }
        }
    }
}

pub unsafe fn sephirothEdits (boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, fighter_kind: i32, motion_kind: u64, stick_value_x: f32, stick_value_y: f32) {
    if fighter_kind == *FIGHTER_KIND_EDGE {
        if [hash40("special_lw"), hash40("special_air_lw")].contains(&motion_kind) {
            if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_LW_HIT, true);
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH, true);
                let speed = Vector3f{x: 2.0 * stick_value_x, y: 2.0 * stick_value_y, z: 0.0};
                KineticModule::add_speed(boma, &speed);
            }
        }
    }
}

pub unsafe fn nessEffects (boma: &mut smash::app::BattleObjectModuleAccessor) {
    if ArticleModule::is_exist(boma, *FIGHTER_NESS_GENERATE_ARTICLE_PK_FLASH) {
        let jointPos = ArticleModule::get_joint_pos(boma, *FIGHTER_NESS_GENERATE_ARTICLE_PK_FLASH, Hash40{hash: hash40("top")}, smash::cpp::root::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        let rot = smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
        let idk = smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
        //smash::app::sv_animcmd::EFFECT_FOLLOW(0x16f1e4733f_u64, hash40("flower"), 0, 0, 0, 0, 0, 0, 1, true);
        EffectModule::req_on_joint(boma, Hash40{hash: 0x0d0da6e3c0}, Hash40{hash: hash40("top")}, &jointPos, &rot, 1.0, &idk, &idk, false, 0, 1, 0);
    }
}

pub unsafe fn zssDair(lua_state: u64, l2c_agent: &mut L2CAgent, boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, motion_kind: u64, fighter_kind: i32, cat1: i32) {
    if fighter_kind == *FIGHTER_KIND_SZEROSUIT {
        if motion_kind == hash40("attack_air_lw") {
            if MotionModule::frame(boma) < 5.0 && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                ZSSDAIR[get_player_number(boma)] = true;
            }
            /*if MotionModule::frame(boma) < 1.0 {
                if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 {
                    ZSSDAIR[get_player_number(boma)] = true;
                }
                else if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 {
                    ZSSDAIR[get_player_number(boma)] = false;
                }
            }*/
            if ZSSDAIR[get_player_number(boma)] {
                if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP, true);
                }
                if MotionModule::frame(boma) > 34.0 {
                    CancelModule::enable_cancel(boma);
                }
            }
        }
        else {
            ZSSDAIR[get_player_number(boma)] = false;
        }
        if situation_kind == *SITUATION_KIND_AIR {
            if CancelModule::is_enable_cancel(boma) || [*FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_AIR_LASSO].contains(&status_kind) {
                if stick_y_flick_check(boma, -0.3) && KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) <= 0.0 {
                    if KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) > 0.0 {
                        l2c_agent.clear_lua_stack();
                        l2c_agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY as u64));
                        l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.0));
                        smash::app::sv_kinetic_energy::set_speed(lua_state);
                    }
                    WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
                }
            }
        }
    }
}

/*pub unsafe fn docFixes(boma: &mut smash::app::BattleObjectModuleAccessor, fighter_kind: i32, motion_kind: u64) {
    if fighter_kind == *FIGHTER_KIND_MARIO {
        if alt(boma, 4) || alt(boma, 5) {
            if motion_kind == hash40("attack_air_lw") {
                MotionModule::change_motion(boma, Hash40{hash: hash40("ladder_catch_r")}, 0.0, 1.0, false, 0.0, false, false);
            }
            else if motion_kind == hash40("attack_air_b") {
                MotionModule::change_motion(boma, Hash40{hash: hash40("ladder_catch_l")}, 0.0, 1.0, false, 0.0, false, false);
            }
        }
    }
}*/

pub unsafe fn fireFox(boma: &mut smash::app::BattleObjectModuleAccessor, motion_kind: u64, fighter_kind: i32) {
    if [*FIGHTER_KIND_FOX, *FIGHTER_KIND_FALCO, *FIGHTER_KIND_WOLF].contains(&fighter_kind) {
        if [hash40("special_hi_hold_air"), hash40("special_hi_hold")].contains(&motion_kind) {
            if CANAIRDODGE[get_player_number(boma)] {
                if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_GUARD) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
                }
            }
        }
    }
}
pub unsafe fn snakeC4Thing(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, fighter_kind: i32, motion_kind: u64) {
    if fighter_kind == *FIGHTER_KIND_SNAKE {
        if status_kind == *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_LW_EXPLODING {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_LW_PRODUCE, true);
                if ArticleModule::is_exist(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4) {
                    ArticleModule::remove_exist(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, smash::cpp::root::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
                }
            }
        }
        if ArticleModule::is_exist(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4) && ArticleModule::motion_kind(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, smash::cpp::root::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL)) != hash40("explosion") {
            let c4pos = ArticleModule::get_joint_pos(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, Hash40{hash: hash40("trans")}, smash::cpp::root::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            if ArticleModule::is_exist(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE) {
                let temp = ArticleModule::get_joint_pos(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE, Hash40{hash: hash40("trans")}, smash::cpp::root::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
                if abs(c4pos.x - temp.x) <= 10.0 {
                    ArticleModule::change_status(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, *WEAPON_SNAKE_C4_STATUS_KIND_EXPLOSION, smash::cpp::root::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
                    //WorkModule::on_flag(boma, *FIGHTER_SNAKE_STATUS_SPECIAL_LW_EXPLODING_FLAG_C4_STARTUP);
                }
            }
            if ArticleModule::is_exist(boma, *WEAPON_SNAKE_TRENCHMORTAR_GENERATE_ARTICLE_BULLET) {
                let temp = ArticleModule::get_joint_pos(boma, *WEAPON_SNAKE_TRENCHMORTAR_GENERATE_ARTICLE_BULLET, Hash40{hash: hash40("trans")}, smash::cpp::root::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
                if abs(c4pos.x - temp.x) <= 10.0 {
                    //ArticleModule::change_motion(boma, *WEAPON_SNAKE_TRENCHMORTAR_GENERATE_ARTICLE_BULLET, Hash40{hash: hash40("impact")}, false, 0.0);
                    ArticleModule::change_status(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, *WEAPON_SNAKE_C4_STATUS_KIND_EXPLOSION, smash::cpp::root::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
                    ArticleModule::change_status(boma, *WEAPON_SNAKE_TRENCHMORTAR_GENERATE_ARTICLE_BULLET, *WEAPON_SNAKE_TRENCHMORTAR_BULLET_STATUS_KIND_IMPACT, smash::cpp::root::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
                    //WorkModule::on_flag(boma, *FIGHTER_SNAKE_STATUS_SPECIAL_LW_EXPLODING_FLAG_C4_STARTUP);
                }
            }
            if ArticleModule::is_exist(boma, *WEAPON_SNAKE_NIKITA_GENERATE_ARTICLE_MISSILE) {
                let temp = ArticleModule::get_joint_pos(boma, *WEAPON_SNAKE_NIKITA_GENERATE_ARTICLE_MISSILE, Hash40{hash: hash40("trans")}, smash::cpp::root::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
                if abs(c4pos.x - temp.x) <= 10.0 {
                    //ArticleModule::change_motion(boma, *WEAPON_SNAKE_NIKITA_GENERATE_ARTICLE_MISSILE, Hash40{hash: hash40("explosion")}, false, 0.0);
                    ArticleModule::change_status(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, *WEAPON_SNAKE_C4_STATUS_KIND_EXPLOSION, smash::cpp::root::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
                    ArticleModule::change_status(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_MISSILE, *WEAPON_SNAKE_MISSILE_STATUS_KIND_EXPLOSION, smash::cpp::root::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
                    //WorkModule::on_flag(boma, *FIGHTER_SNAKE_STATUS_SPECIAL_LW_EXPLODING_FLAG_C4_STARTUP);
                }
            }
            if ArticleModule::is_exist(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_RPG7) {
                let temp = ArticleModule::get_joint_pos(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_RPG7, Hash40{hash: hash40("trans")}, smash::cpp::root::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
                if abs(c4pos.x - temp.x) <= 10.0 {
                    //WorkModule::on_flag(boma, *FIGHTER_SNAKE_STATUS_SPECIAL_LW_EXPLODING_FLAG_C4_STARTUP);
                    ArticleModule::change_status(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, *WEAPON_SNAKE_C4_STATUS_KIND_EXPLOSION, smash::cpp::root::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
                }
            }
        }
    }
}

unsafe fn bjrShit(lua_state: u64, l2c_agent: &mut L2CAgent, boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, fighter_kind: i32, stick_value_y: f32, cat1: i32) {
    if fighter_kind == *FIGHTER_KIND_KOOPAJR {
        if [*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_S_JUMP].contains(&status_kind) {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE) != 0 {
                if CANAIRDODGE[get_player_number(boma)] {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
                }
            }
        }
        if [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_SHOOT].contains(&status_kind) {
            if stick_value_y < -0.5 {
                l2c_agent.clear_lua_stack();
                l2c_agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY as u64));
                l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.0));
                smash::app::sv_kinetic_energy::set_speed(lua_state);
            }
        }
        if status_kind == *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_S_JUMP {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                if stick_value_y < -0.5 {
                    l2c_agent.clear_lua_stack();
                    l2c_agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY as u64));
                    l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.0));
                    smash::app::sv_kinetic_energy::set_speed(lua_state);
                }
            }
        }
        if stick_value_y < -0.5 {
            if status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL {
                if MotionModule::frame(boma) < (7.0) {
                    if !(ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP)) {
                        l2c_agent.clear_lua_stack();
                        l2c_agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY as u64));
                        l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.0));
                        smash::app::sv_kinetic_energy::set_speed(lua_state);
                    }
                }
            }
        }
    }
}

pub unsafe fn otherCancels(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, fighter_kind: i32, motion_kind: u64, cat1: i32) {
    if fighter_kind == *FIGHTER_KIND_WIIFIT {
        if [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_HI_JUMP].contains(&status_kind) {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                CancelModule::enable_cancel(boma);
            }
        }
    }
    if fighter_kind == *FIGHTER_KIND_GEKKOUGA {
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
            if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0 {
                if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                    CancelModule::enable_cancel(boma);
                }
            }
        }
    }
    if fighter_kind == *FIGHTER_KIND_SNAKE {
        if [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_HI_HANG].contains(&status_kind) {
            CancelModule::enable_cancel(boma);
        }
    }
    if [*FIGHTER_KIND_PICHU, *FIGHTER_KIND_PIKACHU].contains(&fighter_kind) {
        if status_kind == *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_HOLD {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_ATTACK, true);
        }
    }
    if fighter_kind == *FIGHTER_KIND_LUCARIO {
        if isAttacking(status_kind) && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 {
                CancelModule::enable_cancel(boma);
            }
        }
    }
    if fighter_kind == *FIGHTER_KIND_PACKUN {
        if isAttacking(status_kind) && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 {
                CancelModule::enable_cancel(boma);
            }
        }
    }
    if fighter_kind == *FIGHTER_KIND_BUDDY {
        if motion_kind == hash40("attack_air_lw") {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP, true);
            }
        }
        if [*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_LW_SHOOT, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_AIR,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_END, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_FALL, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_JUMP, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_TURN,
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_START, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_WALK_B, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_WALK_F, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_LANDING, 
        *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_AIR_TURN, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_JUMP_SQUAT, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_N_SHOOT_JUMP_AERIAL].contains(&status_kind) {
            if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_GUARD) {
                CancelModule::enable_cancel(boma);
            }
        }
    }
    if fighter_kind == *FIGHTER_KIND_WIIFIT {
        if status_kind == *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_N_HOLD {
            if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_ATTACK) || ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_GUARD)
            || ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_CATCH) || ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_CSTICK_ON) || ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_JUMP)
            || ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_FLICK_JUMP) {
                CancelModule::enable_cancel(boma);
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_FALL_SPECIAL {
            if [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_S_JUMP, *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_S_HEADING].contains(&StatusModule::prev_status_kind(boma, 0)) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
            }
        }
    }
}

pub unsafe fn incinSpeed(lua_state: u64, l2c_agent: &mut L2CAgent, boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, fighter_kind: i32, cat1: i32, stick_value_x: f32) {
    if fighter_kind == *FIGHTER_KIND_GAOGAEN {
        let mut momentum = 0.0;
        if stick_value_x < 0.0 {
            momentum = -1.0
        }
        else {
            momentum = 1.0;
        }
        if INCINSPEED[get_player_number(boma)] > 1.0 {
            CANDOWNB[get_player_number(boma)] = false;
        }
        else if ![*FIGHTER_STATUS_KIND_FALL, *FIGHTER_STATUS_KIND_WAIT].contains(&status_kind) {
            CANDOWNB[get_player_number(boma)] = true;
        }
        if WorkModule::get_float(boma, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLOAT_REVENGE_RATE) > 1.0 {
            if SPEEDSTART[get_player_number(boma)] == 0 {
                INCINSPEED[get_player_number(boma)] = WorkModule::get_float(boma, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLOAT_REVENGE_RATE);
                SPEEDSTART[get_player_number(boma)] = GLOBALFRAMECOUNT;
            }
            WorkModule::set_flag(boma, false, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_REVENGE);
            WorkModule::set_float(boma, 1.0, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLOAT_REVENGE_RATE);
        }
        if GLOBALFRAMECOUNT - SPEEDSTART[get_player_number(boma)] >= 1200 && SPEEDSTART[get_player_number(boma)] != 0 {
            INCINSPEED[get_player_number(boma)] = 1.0;
            SPEEDSTART[get_player_number(boma)] = 0;
        }
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            if INCINSPEED[get_player_number(boma)] > 1.0 {
                if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 {
                    if situation_kind == *SITUATION_KIND_AIR {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
                    }
                    else if situation_kind == *SITUATION_KIND_GROUND {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
                    }
                    WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
                    INCINSPEED[get_player_number(boma)] = 1.0;
                    SPEEDSTART[get_player_number(boma)] = 0;
                }
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_DASH {
            if abs(KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)) < 1.54 * (1.0 + ((INCINSPEED[get_player_number(boma)] - 1.0) / 3.0)) {
                if abs(KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)) >= 1.54 {
                    l2c_agent.clear_lua_stack();
                    l2c_agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_CONTROL as u64));
                    l2c_agent.push_lua_stack(&mut L2CValue::new_num(momentum * 1.54 * (1.0 + ((INCINSPEED[get_player_number(boma)] - 1.0) / 3.0))));
                    smash::app::sv_kinetic_energy::set_speed(lua_state);
                }
                else {
                    let reverse_mul = Vector3f{x: 1.0 + ((INCINSPEED[get_player_number(boma)] - 1.0) / 3.0), y: 1.0, z: 1.0};
                    KineticModule::mul_speed(boma, &reverse_mul, *FIGHTER_KINETIC_ENERGY_ID_NONE);
                }
            }
        }
        else if status_kind == *FIGHTER_STATUS_KIND_RUN {
            if abs(KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)) < 1.36 * (1.0 + ((INCINSPEED[get_player_number(boma)] - 1.0) / 3.0)) {
                if abs(KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)) >= 1.36 {
                    l2c_agent.clear_lua_stack();
                    l2c_agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_CONTROL as u64));
                    l2c_agent.push_lua_stack(&mut L2CValue::new_num(momentum * 1.36 * (1.0 + ((INCINSPEED[get_player_number(boma)] - 1.0) / 3.0))));
                    smash::app::sv_kinetic_energy::set_speed(lua_state);
                }
                else {
                    let reverse_mul = Vector3f{x: 1.0 + ((INCINSPEED[get_player_number(boma)] - 1.0) / 3.0), y: 1.0, z: 1.0};
                    KineticModule::mul_speed(boma, &reverse_mul, *FIGHTER_KINETIC_ENERGY_ID_NONE);
                }
            }
        }
        else if status_kind == *FIGHTER_STATUS_KIND_WALK {
            if abs(KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)) < 0.62 * (1.0 + ((INCINSPEED[get_player_number(boma)] - 1.0) / 3.0)) {
                if abs(KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)) >= 0.62 {
                    l2c_agent.clear_lua_stack();
                    l2c_agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_CONTROL as u64));
                    l2c_agent.push_lua_stack(&mut L2CValue::new_num(momentum * 0.62 * (1.0 + ((INCINSPEED[get_player_number(boma)] - 1.0) / 3.0))));
                    smash::app::sv_kinetic_energy::set_speed(lua_state);
                }
                else {
                    let reverse_mul = Vector3f{x: 1.0 + ((INCINSPEED[get_player_number(boma)] - 1.0) / 3.0), y: 1.0, z: 1.0};
                    KineticModule::mul_speed(boma, &reverse_mul, *FIGHTER_KINETIC_ENERGY_ID_NONE);
                }
            }
        }
        else if [*FIGHTER_STATUS_KIND_JUMP, *FIGHTER_STATUS_KIND_JUMP_AERIAL, *FIGHTER_STATUS_KIND_FALL, *FIGHTER_STATUS_KIND_FALL_AERIAL, *FIGHTER_STATUS_KIND_FALL_SPECIAL].contains(&status_kind) {
            if abs(KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)) < 0.95 * (1.0 + ((INCINSPEED[get_player_number(boma)] - 1.0) / 3.0)) {
                if abs(KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)) >= 0.95 && stick_value_x * KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) > 0.0 {
                    l2c_agent.clear_lua_stack();
                    l2c_agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_CONTROL as u64));
                    l2c_agent.push_lua_stack(&mut L2CValue::new_num(momentum * 0.95 * (1.0 + ((INCINSPEED[get_player_number(boma)] - 1.0) / 3.0))));
                    smash::app::sv_kinetic_energy::set_speed(lua_state);
                }
                else if stick_value_x * KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) > 0.0 {
                    let reverse_mul = Vector3f{x: 1.0 + ((INCINSPEED[get_player_number(boma)] - 1.0) / 3.0), y: 1.0, z: 1.0};
                    KineticModule::mul_speed(boma, &reverse_mul, *FIGHTER_KINETIC_ENERGY_ID_NONE);
                }
            }
        }
    }
}

/*pub unsafe fn jmoker (boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, fighter_kind: i32) {
    LookupSymbol(
        &mut FIGHTER_MANAGER,
        "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\u{0}"
        .as_bytes()
        .as_ptr(),
    );
    let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
    if fighter_kind == *FIGHTER_KIND_JACK {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
            if ISARSENE[get_player_number(boma)] {
                ISARSENE[get_player_number(boma)] = false;
            }
            else {
                ISARSENE[get_player_number(boma)] = true;
            }
            if situation_kind == *SITUATION_KIND_GROUND {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
            }
            else if situation_kind == *SITUATION_KIND_AIR {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
            }
            WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
        }
        if !ISARSENE[get_player_number(boma)] {
            if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) {
                ARSENEMETER[get_player_number(boma)] += MOVEDAMAGE[get_player_number(boma)];
            }
            if GLOBALFRAMECOUNT % 30 == 0 {
                ARSENEMETER[get_player_number(boma)] += 0.3;
            }
        }
        else {
            if isAttacking[get_player_number(boma)] {
                ARSENEMETER[get_player_number(boma)] -= 0.6 * MOVEDAMAGE[get_player_number(boma)];
            }
            if GLOBALFRAMECOUNT % 30 == 0 {
                ARSENEMETER[get_player_number(boma)] -= 0.15;
            }
            if *FIGHTER_STATUS_KIND_DAMAGE_FALL, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR {
                let mut i = 0;
                while i < 9 {
                    if i != get_player_number(boma) && isAttacking[i] {
                        ARSENEMETER[get_player_number(boma)] -= 0.4 * MOVEDAMAGE[i];
                        i = 9;
                    }
                    i ++;
                }
            }
        }
    }
}*/

pub unsafe fn pitZoom(lua_state: u64, l2c_agent: &mut L2CAgent, boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, fighter_kind: i32) {
    if fighter_kind == *FIGHTER_KIND_PIT {
        if [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_PIT_STATUS_KIND_SPECIAL_S_END].contains(&status_kind) {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_JUMP) || (ControlModule::is_enable_flick_jump(boma) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_FLICK_JUMP)) { 
                    if situation_kind == *SITUATION_KIND_AIR{
                        if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX){
                            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
                        }
                    }
                    else if situation_kind == *SITUATION_KIND_GROUND{
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                    }
                }
            }
        }
        if situation_kind != *SITUATION_KIND_AIR {
            NUMUPBS[get_player_number(boma)] = 0;
        }
        if status_kind == *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH {
            if MotionModule::frame(boma) < 1.0 {
                NUMUPBS[get_player_number(boma)] += 1;
            }
            if MotionModule::frame(boma) > 3.0 {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
                l2c_agent.clear_lua_stack();
                l2c_agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY as u64));
                l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.0));
                smash::app::sv_kinetic_energy::set_speed(lua_state);
            }
        }
        if [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END].contains(&status_kind) {
            CancelModule::enable_cancel(boma);
        }
        if NUMUPBS[get_player_number(boma)] >= 2 {
            WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
        }
    }
}

pub unsafe fn macStuff(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, fighter_kind: i32) {
    if fighter_kind == *FIGHTER_KIND_LITTLEMAC {
        if status_kind == *FIGHTER_STATUS_KIND_DASH {
            CancelModule::enable_cancel(boma);
        }
        if isAttacking(status_kind) || [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_N].contains(&status_kind) {
            for i in 0..9 {
                if i != get_player_number(boma) && ISATTACK[i] {
                    AttackModule::set_power(boma, 0, (*AttackModule::attack_data(boma, 0, false)).power * 1.1, false);
                }
            }
        }
    }
}

pub unsafe fn toggelStuff(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, cat2: i32) {
    if status_kind == *FIGHTER_STATUS_KIND_GUARD {
        if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L) != 0 && FLASHSTART[get_player_number(boma)] == 0 {
            TOGGELS[0][get_player_number(boma)] = abs((1 - TOGGELS[0][get_player_number(boma)]) as f32) as i32;
            println!("SH macro is now: {}", TOGGELS[0][get_player_number(boma)]);
            if TOGGELS[0][get_player_number(boma)] == 0 {
                let colorflashvec1 = smash::phx::Vector4f{ /* Red */ x: 255.0, /* Green */ y: 170.0, /* Blue */ z: 255.0, /* Alpha? */ w: 0.1}; // setting this and the next vector's .w to 1 seems to cause a ghostly effect
                let colorflashvec2 = smash::phx::Vector4f{ /* Red */ x: 255.0, /* Green */ y: 170.0, /* Blue */ z: 255.0, /* Alpha? */ w: 0.1};
                ColorBlendModule::set_main_color(boma, &colorflashvec1, &colorflashvec2, 0.7, 0.2, 25, true); //int here is opacity
                FLASHSTART[get_player_number(boma)] = GLOBALFRAMECOUNT;
            }
            else {
                let colorflashvec1 = smash::phx::Vector4f{ /* Red */ x: 255.0, /* Green */ y: 0.0, /* Blue */ z: 0.0, /* Alpha? */ w: 0.1}; // setting this and the next vector's .w to 1 seems to cause a ghostly effect
                let colorflashvec2 = smash::phx::Vector4f{ /* Red */ x: 255.0, /* Green */ y: 0.0, /* Blue */ z: 0.0, /* Alpha? */ w: 0.1};
                ColorBlendModule::set_main_color(boma, &colorflashvec1, &colorflashvec2, 0.7, 0.2, 25, true); //int here is opacity
                FLASHSTART[get_player_number(boma)] = GLOBALFRAMECOUNT;
            }
        }
    }
    if status_kind == *FIGHTER_STATUS_KIND_GUARD {
        if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI) != 0 && FLASHSTART[get_player_number(boma)] == 0 {
            TOGGELS[1][get_player_number(boma)] = abs((1 - TOGGELS[1][get_player_number(boma)]) as f32) as i32;
            println!("Smash stick is now: {}", TOGGELS[1][get_player_number(boma)]);
            if TOGGELS[0][get_player_number(boma)] == 0 {
                let colorflashvec1 = smash::phx::Vector4f{ /* Red */ x: 255.0, /* Green */ y: 170.0, /* Blue */ z: 255.0, /* Alpha? */ w: 0.1}; // setting this and the next vector's .w to 1 seems to cause a ghostly effect
                let colorflashvec2 = smash::phx::Vector4f{ /* Red */ x: 255.0, /* Green */ y: 170.0, /* Blue */ z: 255.0, /* Alpha? */ w: 0.1};
                ColorBlendModule::set_main_color(boma, &colorflashvec1, &colorflashvec2, 0.7, 0.2, 25, true); //int here is opacity
                FLASHSTART[get_player_number(boma)] = GLOBALFRAMECOUNT;
            }
            else {
                let colorflashvec1 = smash::phx::Vector4f{ /* Red */ x: 255.0, /* Green */ y: 0.0, /* Blue */ z: 0.0, /* Alpha? */ w: 0.1}; // setting this and the next vector's .w to 1 seems to cause a ghostly effect
                let colorflashvec2 = smash::phx::Vector4f{ /* Red */ x: 255.0, /* Green */ y: 0.0, /* Blue */ z: 0.0, /* Alpha? */ w: 0.1};
                ColorBlendModule::set_main_color(boma, &colorflashvec1, &colorflashvec2, 0.7, 0.2, 25, true); //int here is opacity
                FLASHSTART[get_player_number(boma)] = GLOBALFRAMECOUNT;
            }
        }
    }
    if FLASHSTART[get_player_number(boma)] != 0 && GLOBALFRAMECOUNT - FLASHSTART[get_player_number(boma)] == 6 {
        ColorBlendModule::cancel_main_color(boma, 0);
        FLASHSTART[get_player_number(boma)] = 0;
    }
}

pub unsafe fn comboCounter(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32) {
    if LASTCOMBOHIT[get_player_number(boma)] != 0 && GLOBALFRAMECOUNT - LASTCOMBOHIT[get_player_number(boma)] >= 90 && CancelModule::is_enable_cancel(boma) {
        COMBOCOUNT[get_player_number(boma)] = 0;
        TRUECOMBOCOUNT[get_player_number(boma)] = 0;
        LASTCOMBOHIT[get_player_number(boma)] = 0;
    }
    if [*FIGHTER_STATUS_KIND_DAMAGE_FALL, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
    *FIGHTER_STATUS_KIND_DAMAGE_SONG, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_FALL, *FIGHTER_STATUS_KIND_DAMAGE_SONG_START, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_START,
    *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U, *FIGHTER_STATUS_KIND_DAMAGE].contains(&status_kind) && MotionModule::frame(boma) == 1.0
    /*KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) == 0.0 && KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) == 0.0 && !ISSTUNNED[get_player_number(boma)]*/ {
        COMBOCOUNT[get_player_number(boma)] += 1;
        TRUECOMBOCOUNT[get_player_number(boma)] += 1;
        if LASTCOMBOHIT[get_player_number(boma)] == 0 {
            LASTCOMBOHIT[get_player_number(boma)] = GLOBALFRAMECOUNT;
        }
        if COMBOCOUNT[get_player_number(boma)] > 1 {
            if COMBOCOUNT[get_player_number(boma)] > TRUECOMBOCOUNT[get_player_number(boma)] {
                println!("{} hit combo!", COMBOCOUNT[get_player_number(boma)]);
            }
            else {
                println!("{} hit true combo!", TRUECOMBOCOUNT[get_player_number(boma)]);
            }
        }
    }
    if CancelModule::is_enable_cancel(boma) {
        TRUECOMBOCOUNT[get_player_number(boma)] = 0;
    }
    ISSTUNNED[get_player_number(boma)] = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) == 0.0 && KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) == 0.0;
}

pub unsafe fn puffLevels(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, fighter_kind: i32) {
    if fighter_kind == *FIGHTER_KIND_PURIN {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            if FLASHSTART[get_player_number(boma)] == 0 {
                if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_GUARD) && PUFFLEVELS[0][get_player_number(boma)] < 3 {
                    PUFFLEVELS[0][get_player_number(boma)] += 1;
                    let colorflashvec1 = smash::phx::Vector4f{ /* Red */ x: 0.0, /* Green */ y: 0.0, /* Blue */ z: 1.0, /* Alpha? */ w: 0.1}; // setting this and the next vector's .w to 1 seems to cause a ghostly effect
                    let colorflashvec2 = smash::phx::Vector4f{ /* Red */ x: 0.0, /* Green */ y: 0.0, /* Blue */ z: 1.0, /* Alpha? */ w: 0.1};
                    ColorBlendModule::set_main_color(boma, &colorflashvec1, &colorflashvec2, 0.7, 0.2, 25, true); //int here is opacity
                    FLASHSTART[get_player_number(boma)] = GLOBALFRAMECOUNT;
                    println!("Puff rest level is now: {}", PUFFLEVELS[0][get_player_number(boma)]);
                }
                if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_CATCH) {
                    PUFFLEVELS[0][get_player_number(boma)] = 0;
                    let colorflashvec1 = smash::phx::Vector4f{ /* Red */ x: 1.0, /* Green */ y: 0.0, /* Blue */ z: 0.0, /* Alpha? */ w: 0.1}; // setting this and the next vector's .w to 1 seems to cause a ghostly effect
                    let colorflashvec2 = smash::phx::Vector4f{ /* Red */ x: 1.0, /* Green */ y: 0.0, /* Blue */ z: 0.0, /* Alpha? */ w: 0.1};
                    ColorBlendModule::set_main_color(boma, &colorflashvec1, &colorflashvec2, 0.7, 0.2, 25, true); //int here is opacity
                    FLASHSTART[get_player_number(boma)] = GLOBALFRAMECOUNT;
                    println!("Puff rest level is now: {}", PUFFLEVELS[0][get_player_number(boma)]);
                }
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_ENTRY {
            PUFFLEVELS[0][get_player_number(boma)] = 0;
        }
        if [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_END, *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_ROLL, *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_TURN, 
        *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_HIT_END, *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_HOLD_MAX, *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_ROLL_AIR].contains(&status_kind) {
            if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) {
                PUFFLEVELS[1][get_player_number(boma)] += 1;
                println!("Puff rollout level is now: {}", PUFFLEVELS[1][get_player_number(boma)]);
            }
        }
    }
}

pub unsafe fn villagerShit(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, fighter_kind: i32, cat1: i32) {
    if fighter_kind == *FIGHTER_KIND_MURABITO {
        if [*FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_TURN, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_FLAP, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_WAIT].contains(&status_kind) {
            if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 ||
            (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
            CancelModule::enable_cancel(boma);
        }
    }
}

pub unsafe fn robinCancels(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, fighter_kind: i32) {
    if fighter_kind == *FIGHTER_KIND_REFLET {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            let temp = WorkModule::get_int(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT);
            if temp > 0 {
                if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_JUMP) || (ControlModule::is_enable_flick_jump(boma) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_FLICK_JUMP)) {
                    if situation_kind == *SITUATION_KIND_AIR {
                        if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX){
                            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
                        }
                    }
                    else if situation_kind == *SITUATION_KIND_GROUND{
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                    }
                }
                if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_GUARD) {
                    CancelModule::enable_cancel(boma);
                }
                WorkModule::set_int(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT, (temp - 1));
            }
        }
        if [*FIGHTER_REFLET_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_STATUS_KIND_SPECIAL_HI].contains(&status_kind) {
            if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_GUARD) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
            }
        }
    }
}

pub unsafe fn horizDjs (lua_state: u64, l2c_agent: &mut L2CAgent, boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    if situation_kind == *SITUATION_KIND_AIR {
        if MotionModule::frame(boma) > 0.0 && (WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL) || status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR) {
            if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_APPEAL_S_L) || ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_APPEAL_S_R) ||
            ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_APPEAL_LW) || ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_APPEAL_HI) {
                if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
                    if HORIZDJSTART[get_player_number(boma)] == 0 {
                        HORIZDJSTART[get_player_number(boma)] = GLOBALFRAMECOUNT;
                        WorkModule::set_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT, (WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) + 1));
                    }
                }
            }
            if GLOBALFRAMECOUNT - HORIZDJSTART[get_player_number(boma)] < 4 {
                l2c_agent.clear_lua_stack();
                l2c_agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY as u64));
                l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.0));
                smash::app::sv_kinetic_energy::set_speed(lua_state);
                /*let reverse_speed = Vector3f{x: 1.0, y: 0.0, z: 0.0};
                KineticModule::add_speed(boma, &reverse_speed); */
                l2c_agent.clear_lua_stack();
                l2c_agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_CONTROL as u64));
                l2c_agent.push_lua_stack(&mut L2CValue::new_num(2.2 * PostureModule::lr(boma)));
                smash::app::sv_kinetic_energy::set_speed(lua_state);
            }
            else {
                HORIZDJSTART[get_player_number(boma)] = 0;
            }
        }
    }
}

// Use this for general per-frame fighter-level hooks
#[smashline::fighter_frame_callback]
pub fn global_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
        //BOMA = boma;
        let mut l2c_agent = L2CAgent::new(lua_state);
        //let fighter_category = get_category(boma);
        let status_kind = StatusModule::status_kind(boma);
        let situation_kind = StatusModule::situation_kind(boma);
        let fighter_kind = get_kind(boma);
        let cat1 = ControlModule::get_command_flag_cat(boma, 0);
        let cat2 = ControlModule::get_command_flag_cat(boma, 1);
        let cat3 = ControlModule::get_command_flag_cat(boma, 2);
        let stick_value_y = ControlModule::get_stick_y(boma);
        let stick_value_x = ControlModule::get_stick_x(boma);
        let motion_kind = MotionModule::motion_kind(boma);
        let kinetic_type = KineticModule::get_kinetic_type(boma);
        let flick_x_dir = ControlModule::get_flick_x_dir(boma);
        JostleModule::set_team(boma, 0);

        /*if fighter_kind == *FIGHTER_KIND_MASTER {
            if status_kind == *FIGHTER_STATUS_KIND_APPEAL && MotionModule::frame(boma) < 1.0 {
                if WEAPONMODE[get_player_number(boma)] < 4 {
                    WEAPONMODE[get_player_number(boma)] += 1;
                }
                else {
                    WEAPONMODE[get_player_number(boma)] = 0;
                }
            }
        } */

        //let mut globals = *fighter.globals_mut();

        ISATTACK[get_player_number(boma)] = isAttacking(status_kind);

        if fighter_kind == *FIGHTER_KIND_PZENIGAME {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                println!("Status: {}, out: {}, standby: {}", status_kind, *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_LW_OUT, *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_LW_STANDBY);
            }
        }

        if motion_kind == hash40("just_shield_off") {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
        }

        if [*FIGHTER_STATUS_KIND_DAMAGE_FALL, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        *FIGHTER_STATUS_KIND_DAMAGE_SONG, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_FALL, *FIGHTER_STATUS_KIND_DAMAGE_SONG_START, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_START,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U, *FIGHTER_STATUS_KIND_DAMAGE].contains(&status_kind) {
            if GLOBALFRAMECOUNT - KBSTART[get_player_number(boma)] == 30 {
                KBSTART[get_player_number(boma)] = -1;
            }
            if KBSTART[get_player_number(boma)] == 0 {
                KBSTART[get_player_number(boma)] = GLOBALFRAMECOUNT;
            }
        }
        else {
            KBSTART[get_player_number(boma)] = 0;
        }

        /*if isAttacking(status_kind) && AttackModule::get_power(boma, 0, 0.0, false, 0.0) > MOVEDAMAGE[get_player_number(boma)] || MOVE[get_player_number(boma)] != status_kind {
            MOVEDAMAGE[get_player_number(boma)] = AttackModule::get_power(boma, 0, 0.0, false, 0.0);
            MOVE[get_player_number(boma)] = status_kind;
        } */

        if fighter_kind == *FIGHTER_KIND_PZENIGAME {
            if [*FIGHTER_STATUS_KIND_TURN_RUN, *FIGHTER_STATUS_KIND_TURN_DASH].contains(&status_kind) {
                if MotionModule::frame(boma) < 1.0 {
                    let reverse_speed = Vector3f{x: -0.5 * PostureModule::lr(boma), y: 0.0, z: 0.0};
                    KineticModule::add_speed(boma, &reverse_speed);
                }
            }
        }

        /*if fighter_kind == *FIGHTER_KIND_PIT {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) {
                println!("Motion: {}", motion_kind);
            }
        }*/

        /*if fighter_kind == *FIGHTER_KIND_JACK {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
                println!("arsene health: {}", WorkModule)
            }
        }*/

        if OPPDASHSPEED > WorkModule::get_param_float(boma, 0, hash40("run_speed_max")) && fighter_kind != *FIGHTER_KIND_PICHU {
            OPPDASHSPEED = WorkModule::get_param_float(boma, 0, hash40("run_speed_max"));
        }

        if get_player_number(boma) == 0 {
            if LASTFRAME != MotionModule::frame(boma) as i32 {
                GLOBALFRAMECOUNT += 1;
                LASTFRAME = MotionModule::frame(boma) as i32;
                //println!("Next frame! Global Frame: {}, Frame: {}", GLOBALFRAMECOUNT, LASTFRAME);
            }
        }

        //edit damage ratio for ffas
        /*let mut hitbox_params: [smash::lib::L2CValue; 36] = [smash::lib::L2CValue::new_void(); 36];
        if FighterManager::entry_count(blanklol) > 1 && !smash::app::smashball::is_training_mode() {
            for i in 0..35 {
                hitbox_params[i as usize] = l2c_agent.pop_lua_stack(i+1);
            }
            l2c_agent.clear_lua_stack();
            for i in 0..35 {
                if (i == 3) {
                    l2c_agent.push_lua_stack(&mut smash::lib::L2CValue::new_num(get_num(hitbox_params[i as usize])) * 0.9);
                } else {
                    l2c_agent.push_lua_stack(&mut hitbox_params[i as usize]);
                }    
            }
        } */

        jumpCancels(boma, status_kind, situation_kind, fighter_kind);
        landCancels(boma, status_kind, situation_kind, fighter_kind);
        dacus(boma, status_kind, cat1, stick_value_y);
        jumpCancelMove(boma, status_kind, cat1, stick_value_y);
        perfectPivots(boma, status_kind, stick_value_x);
        glideTossing(boma, status_kind, situation_kind, cat3, stick_value_x);
        tauntSliding(boma, status_kind, cat2);
        ditcit(boma, status_kind, cat1, cat3);
        shieldStops(boma, status_kind, cat1);
        dashPlatformDrop(boma, status_kind, situation_kind, stick_value_y);
        shieldDrops(boma, status_kind, cat2);
        jabCancels(boma, status_kind, motion_kind, fighter_kind, cat1, stick_value_x, stick_value_y);
        pteStuff(boma, fighter_kind, status_kind, situation_kind, motion_kind, cat1, stick_value_x);
        djcs(lua_state, &mut l2c_agent, boma, status_kind, fighter_kind, kinetic_type);
        airDodgeZair(boma, status_kind, situation_kind, fighter_kind, cat2, cat3, stick_value_x);
        wallJumpSpecial(boma, status_kind, situation_kind, fighter_kind, cat1);
        sm4shJabLocks(lua_state, &mut l2c_agent, boma, status_kind);
        noSpecialFall(boma, status_kind, situation_kind, fighter_kind);  
        regainDJ(boma, status_kind, fighter_kind, situation_kind);
        wiggleHitstun(boma, status_kind, cat1);
        shortens(boma, status_kind, fighter_kind, motion_kind, cat1);
        ivyHeals(boma, status_kind, fighter_kind, motion_kind);
        setStatuses(boma, status_kind);
        meleeSmashStick(boma, status_kind);
        //grabIsAerial(lua_state, &mut l2c_agent, boma, status_kind, situation_kind, cat1);
        slipTechs(boma, status_kind, cat1, stick_value_x);
        meleeECBs(boma, status_kind, situation_kind, fighter_kind);
        lagCanceled(boma, status_kind);
        removeSHMacro(boma, status_kind);
        fixExtraJumps(boma, situation_kind, motion_kind, fighter_kind);
        disableStuff(boma, status_kind, fighter_kind);
        regainAirDodge(boma, status_kind, situation_kind);
        //deathStuff(boma);
        squirtleSideB(boma, status_kind, situation_kind, fighter_kind, motion_kind, cat1);
        airDodgeCancels(boma, status_kind, situation_kind, fighter_kind, cat1);
        hitfalling(boma, status_kind, situation_kind);
        fastfallShit(boma, status_kind, situation_kind, fighter_kind);
        pkFlashCancels(boma, status_kind, fighter_kind, cat1);
        quickAttackCancels(boma, status_kind, situation_kind, fighter_kind, stick_value_y);
        moonwalking(boma, status_kind, stick_value_x);
        upbCancels(lua_state, &mut l2c_agent, boma, status_kind, fighter_kind, cat1);
        //ptSwaps(boma, fighter_kind, cat1, cat2);
        driftDi(boma, status_kind, stick_value_x);
        grenShurikenFix(boma, status_kind, situation_kind, fighter_kind);
        animPortFix(lua_state, &mut l2c_agent, boma, status_kind, situation_kind, fighter_kind, motion_kind, stick_value_y);
        bReverses(boma, motion_kind, fighter_kind);
        fixProjectiles(boma, status_kind, fighter_kind);
        magicSeries(boma, status_kind, situation_kind, motion_kind, fighter_kind, stick_value_x, cat1);
        additionalTransfer(lua_state, &mut l2c_agent, boma, status_kind, situation_kind, fighter_kind);
        inklingStuff(lua_state, &mut l2c_agent, boma, status_kind, fighter_kind, cat1, stick_value_x);
        toggleAutoTurnaround(boma, status_kind, motion_kind, fighter_kind, stick_value_x);
        //noImpactLandingLag(boma, status_kind);
        shineStalls(lua_state, &mut l2c_agent, boma, status_kind, situation_kind, fighter_kind);
        mewtwoTail(boma, fighter_kind);
        teleportCancels(boma, status_kind, situation_kind, fighter_kind);
        canDkBarrel(boma, fighter_kind);
        superJumps(boma, status_kind, fighter_kind, cat1);
        bayoCancels(boma, status_kind, situation_kind, fighter_kind, cat1);
        floats(lua_state, &mut l2c_agent, boma, status_kind, situation_kind, fighter_kind, stick_value_y);
        ganonFix(lua_state, &mut l2c_agent, boma, status_kind, situation_kind, fighter_kind);
        sephirothEdits(boma, status_kind, fighter_kind, motion_kind, stick_value_x, stick_value_y);
        zssDair(lua_state, &mut l2c_agent, boma, status_kind, situation_kind, motion_kind, fighter_kind, cat1);
        //docFixes(boma, fighter_kind, motion_kind);
        fireFox(boma, motion_kind, fighter_kind);
        snakeC4Thing(boma, status_kind, fighter_kind, motion_kind);
        bjrShit(lua_state, &mut l2c_agent, boma, status_kind, fighter_kind, stick_value_y, cat1);
        otherCancels(boma, status_kind, situation_kind, fighter_kind, motion_kind, cat1);
        incinSpeed(lua_state, &mut l2c_agent, boma, status_kind, situation_kind, fighter_kind, cat1, stick_value_x);
        pitZoom(lua_state, &mut l2c_agent, boma, status_kind, situation_kind, fighter_kind);
        macStuff(boma, status_kind, fighter_kind);
        toggelStuff(boma, status_kind, cat2);
        comboCounter(boma, status_kind);
        puffLevels(boma, status_kind, fighter_kind);
        villagerShit(boma, status_kind, fighter_kind, cat1);
        robinCancels(boma, status_kind, situation_kind, fighter_kind);
        horizDjs(lua_state, &mut l2c_agent, boma, status_kind, situation_kind);
        //nessEffects(boma);
        //editParams(boma, status_kind, fighter_kind);
        //dashgrabSlide(boma, status_kind);

        LookupSymbol(
            &mut FIGHTER_CUTIN_MANAGER_ADDR,
            "_ZN3lib9SingletonIN3app19FighterCutInManagerEE9instance_E\u{0}"
                .as_bytes()
                .as_ptr(),
        );

        /*if StatusModule::is_situation_changed(boma) {
            let situation_kind = &format!("{}", StatusModule::situation_kind(boma));
            println!(
                "[Fighter Hook]\nFighter Kind: {}\nStatus Kind: {}\nSituation Kind: {}",
                get_kind(boma),
                StatusModule::status_kind(boma),
                match StatusModule::situation_kind(boma) {
                    0 => "SITUATION_KIND_GROUND",
                    1 => "SITUATION_KIND_CLIFF",
                    2 => "SITUATION_KIND_AIR",
                    _ => situation_kind
                }
            );
        } */
    }
}

// Use this for general per-frame weapon-level hooks
#[smashline::weapon_frame_callback]
pub fn global_weapon_frame(weapon : &mut L2CFighterBase) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(weapon.lua_state_agent);
        let owner_boma = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        let weapon_kind = get_kind(boma);
        let fighter_kind = get_kind(&mut *owner_boma);

        if weapon_kind == *FIGHTER_FALCO_GENERATE_ARTICLE_BLASTER_BULLET {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                println!("Laser hit");
            }
        }
        //let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter_base.lua_state_agent);
        //let frame = smash::app::lua_bind::MotionModule::frame(module_accessor) as i32;

        /* if frame % 10 == 0 {
            println!("[Weapon Hook] Frame : {}", frame);
        } */

        /* LookupSymbol(
            &mut FIGHTER_CUTIN_MANAGER_ADDR,
            "_ZN3lib9SingletonIN3app19FighterCutInManagerEE9instance_E\u{0}"
                .as_bytes()
                .as_ptr(),
        ); */
    }
}

fn nro_main(nro: &NroInfo) {
    match nro.name {
        "common" => {
            skyline::install_hooks!(
                status_jump_sub_hook,
                status_attack_air_hook
            );
        }
        _ => (),
    }
}

pub fn installCustom() {
    smashline::install_agent_frame_callbacks!(
        global_fighter_frame,
        global_weapon_frame
    );
    skyline::install_hooks!(
        get_param_float_hook,
        entry_cliff_hook,
        leave_cliff_hook,
        can_entry_cliff_hook,
        is_enable_transition_term_hook,
        change_status_request_from_script_hook,
        is_valid_just_shield_reflector_hook,
        change_kinetic_hook,
        get_int_hook,
        init_settings_hook,
        correct_hook,
        get_float_hook,
        get_param_int_hook
    );
    nro::add_hook(nro_main).unwrap();
}
