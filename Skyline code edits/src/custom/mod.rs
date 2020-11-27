#![warn(non_snake_case)]

use smash::lib::{L2CValue, L2CAgent};
use smash::app::lua_bind::*;
use smash::hash40;
use smash::app::utility::get_kind;
use smash::lua2cpp::{L2CFighterCommon, L2CFighterBase};
use smash::lib::lua_const::*;
use smash::phx::*;
use acmd;
use crate::FIGHTER_CUTIN_MANAGER_ADDR;
use skyline::nn::ro::LookupSymbol;
//use smash::params::*;


static mut LEDGE_POS: [Vector3f; 9] = [smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0}; 9];
static mut ECB_Y_OFFSETS: [f32; 9] = [0.0; 9];
static mut LOCKED: [bool; 9] = [false; 9];
static mut NOSPECIALFALL: [bool; 9] = [false; 9];
static mut RECENTSTATUSES: [[i32; 5]; 9] = [[0; 5]; 9];
static mut GRNDVELOS: [f32; 9] = [0.0; 9];
static mut LAGCANCELED: [bool; 9] = [false; 9];
static mut DASHDIR: [f32; 9] = [0.0; 9];
static mut ISREVERSE: [bool; 9] = [false; 9];
static mut BREVERSE: [bool; 9] = [false; 9];
static mut ISFUNNY: [bool; 9] = [false; 9];
//Mewtwo disable globals
static mut PASTATTACKS: [i32; 9] = [0; 9];
static mut ISDISABLED: [bool; 9] = [false; 9];
static mut DISABLEDMOVE: [i32; 9] = [0; 9];
//
static mut CANAIRDODGE: [bool; 9] = [true; 9];
static mut USEDUPB: [bool; 9] = [false; 9];
static mut GLOBALFRAMECOUNT: i32 = 0;
//Ivy meter shit
pub static mut AMOUNTSOLAR: [i32; 9] = [0; 9];
static mut SOLARSTART: [i32; 9] = [0; 9];
static mut CANHEAL: [bool; 9] = [true; 9];
//
static mut REGRABFRAMES: [i32; 9] = [1; 9];
static mut HITFALLBUFFER: [[bool; 6]; 9] = [[false; 6]; 9];
static mut CANMOONWALK: [bool; 9] = [false; 9];
static mut DOUBLESWAP: [bool; 9] = [false; 9];
static mut REGAINUPB: [bool; 9] = [false; 9];
pub static mut CANPROJECTILE: [bool; 9] = [true; 9];
//pub static mut COMMON_PARAMS: *mut CommonParams = 0 as *mut _;

pub unsafe fn get_player_number(boma: &mut smash::app::BattleObjectModuleAccessor) -> usize {
    return WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
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

/*#[skyline::hook(offset=0x4dae10)]
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
    /*if param_hash == hash40("common") {
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

#[skyline::hook(replace = smash::app::lua_bind::WorkModule::get_param_float)]

pub unsafe fn get_param_float_hook(boma: &mut smash::app::BattleObjectModuleAccessor, param_type: u64, param_hash: u64) -> f32 {
    let status_kind = StatusModule::status_kind(boma);
    let stick_value_y = ControlModule::get_stick_y(boma);
    let fighter_kind = get_kind(boma);
    if param_hash == 0 {
        if [hash40("landing_attack_air_frame_n"), hash40("landing_attack_air_frame_hi"), hash40("landing_attack_air_frame_lw"), 
        hash40("landing_attack_air_frame_f"), hash40("landing_attack_air_frame_b")].contains(&param_type) {
            let origLandingLag = original!()(boma, param_type, param_hash);
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
    original!()(boma, param_type, param_hash)
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
    if KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) > 0.0 { //Melee style sweetspots
        if ![*FIGHTER_KIND_PFUSHIGISOU, *FIGHTER_KIND_TANTAN, *FIGHTER_KIND_MASTER].contains(&fighter_kind) && status_kind != *FIGHTER_STATUS_KIND_AIR_LASSO && status_kind != 248 &&
        (fighter_kind != *FIGHTER_KIND_JACK || ![*FIGHTER_JACK_STATUS_KIND_SPECIAL_HI_CUT, *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI_THROW, *FIGHTER_STATUS_KIND_SPECIAL_HI].contains(&status_kind)) &&
        (fighter_kind != *FIGHTER_KIND_SHIZUE || ![*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_START, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_THROW].contains(&status_kind))  {
            return 0 as u64;
        }
    }
    if status_kind != *FIGHTER_STATUS_KIND_FALL_AERIAL && status_kind != *FIGHTER_STATUS_KIND_JUMP_AERIAL && status_kind != *FIGHTER_STATUS_KIND_FALL && 
    status_kind != *FIGHTER_STATUS_KIND_FLY && status_kind != *FIGHTER_STATUS_KIND_AIR_LASSO && ![*FIGHTER_KIND_PFUSHIGISOU, *FIGHTER_KIND_MASTER, *FIGHTER_KIND_TANTAN].contains(&fighter_kind) && (fighter_kind != *FIGHTER_KIND_JACK ||  
        ![*FIGHTER_JACK_STATUS_KIND_SPECIAL_HI_CUT, *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI_THROW, *FIGHTER_STATUS_KIND_SPECIAL_HI].contains(&status_kind)) { //Edgehog/trump
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
    if (flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR) && [*FIGHTER_STATUS_KIND_DAMAGE_FALL, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, 
    *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR].contains(&status_kind) {
        return false;
    } 
    if flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR {
        if !CANAIRDODGE[get_player_number(boma)] {
            return false;
        }
    }
    if flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI {
        if USEDUPB[get_player_number(boma)] {
            if [*FIGHTER_KIND_PLIZARDON, *FIGHTER_KIND_CLOUD].contains(&fighter_kind) {
                return false;
            }
        }
    }
    /* if [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_ATTACK_S3, 
    *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_100, *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_HI4,
    *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_STATUS_KIND_AIR_LASSO].contains(&flag) {
        if DISABLEDMOVE[get_player_number(boma)] == flag && ISDISABLED[get_player_number(boma)] == true {
            return false;
        }
    } */

    original!()(boma, flag)
}

#[skyline::hook(replace = smash::app::lua_bind::ControlModule::check_button_on)]

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
}

#[skyline::hook(replace = smash::app::lua_bind::WorkModule::get_int)]

pub unsafe fn get_int_hook(boma: &mut smash::app::BattleObjectModuleAccessor, instance: i32) -> i32 {
    let fighter_kind = get_kind(boma);
    if instance == *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT {
        if fighter_kind == *FIGHTER_KIND_CAPTAIN {
            if REGAINUPB[get_player_number(boma)] {
                if original!()(boma, instance) == 2 {
                    REGAINUPB[get_player_number(boma)] = false;
                    return 1;
                }
            }
        }
    }
    original!()(boma, instance)
}

#[skyline::hook(replace = smash::app::lua_bind::StatusModule::change_status_request_from_script)]

pub unsafe fn change_status_request_from_script_hook(boma: &mut smash::app::BattleObjectModuleAccessor, status: i32, unk: bool) -> u64 {
    let fighter_kind = get_kind(boma);
    let situation_kind = StatusModule::situation_kind(boma);
    let cat1 = ControlModule::get_command_flag_cat(boma, 0);
    let cat2 = ControlModule::get_command_flag_cat(boma, 1);
    if [*FIGHTER_KIND_PZENIGAME, *FIGHTER_KIND_PLIZARDON, *FIGHTER_KIND_PFUSHIGISOU].contains(&fighter_kind) {
        if status == *FIGHTER_STATUS_KIND_SPECIAL_LW {
            if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 {
                if situation_kind == *SITUATION_KIND_AIR {
                    original!()(boma, *FIGHTER_STATUS_KIND_FALL, unk);
                }
                else {
                    if PostureModule::lr(boma) < 0.0 {
                        MotionModule::change_motion(boma, Hash40{hash: hash40("appeal_s_l")}, 0.0, 1.0, false, 0.0, false, false);
                    }
                    else {
                        MotionModule::change_motion(boma, Hash40{hash: hash40("appeal_s_r")}, 0.0, 1.0, false, 0.0, false, false);
                    }
                    original!()(boma, *FIGHTER_STATUS_KIND_APPEAL, unk);
                }
            }
        }
        else if status == *FIGHTER_STATUS_KIND_APPEAL {
            if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L) != 0 || (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R) != 0 || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_L) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
                if WorkModule::is_enable_transition_term(boma, *FIGHTER_POKEMON_STATUS_TRANSITION_TERM_ID_PRE_START) {
                    original!()(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, unk);
                }
            }
        }
    }
    original!()(boma, status, unk)
}

pub unsafe fn jumpCancels(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, fighter_kind: i32) { //Jump cancel shine & etc
    if fighter_kind == *FIGHTER_KIND_FOX {
        if [*FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_LOOP, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_HIT, 
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
    if fighter_kind == *FIGHTER_KIND_WOLF {
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
    if fighter_kind == *FIGHTER_KIND_FALCO {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) || (ControlModule::is_enable_flick_jump(boma) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_FLICK_JUMP)) {
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
    if fighter_kind == *FIGHTER_KIND_NESS {
        if [*FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HOLD, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HIT, 
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
    if fighter_kind == *FIGHTER_KIND_IKE {
        if [*FIGHTER_IKE_STATUS_KIND_SPECIAL_S_DASH, *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_END, *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_ATTACK].contains(&status_kind) {
            if situation_kind == *SITUATION_KIND_GROUND {
                if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) || (ControlModule::is_enable_flick_jump(boma) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_FLICK_JUMP)) {
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
            if KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) > WorkModule::get_param_float(boma, hash40("walk_speed_max"), hash40("fighter_param")) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_CATCH_DASH, true);
            }
            else {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_CATCH, true);
            }
        }
        else if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_GUARD) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
        }
        if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 && MotionModule::frame(boma) > (1.0) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4, true);  
        }
        else if stick_value_y >= 0.66 && ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
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
        if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE) != 0 {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD_ON, true);
        }
    }
}

pub unsafe fn dashPlatformDrop(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, stick_value_y: f32) { //Dash Platform Drop
    if [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_TURN_RUN, 
        *FIGHTER_STATUS_KIND_ESCAPE_AIR].contains(&status_kind) {
            if situation_kind == SITUATION_KIND_GROUND {
                if stick_value_y <= -0.75 {
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
    if (![*FIGHTER_KIND_CHROM, *FIGHTER_KIND_ROY, *FIGHTER_KIND_GANON, *FIGHTER_KIND_SHIZUE, *FIGHTER_KIND_PIKACHU, *FIGHTER_KIND_PICHU, *FIGHTER_KIND_METAKNIGHT].contains(&fighter_kind) && motion_kind == hash40("attack_11")) ||
        (![*FIGHTER_KIND_DONKEY, *FIGHTER_KIND_MARTH, *FIGHTER_KIND_LUCINA, *FIGHTER_KIND_KOOPA, *FIGHTER_KIND_DAISY, *FIGHTER_KIND_PEACH, *FIGHTER_KIND_SAMUS, *FIGHTER_KIND_SAMUSD, *FIGHTER_KIND_PURIN, *FIGHTER_KIND_NANA, *FIGHTER_KIND_POPO, *FIGHTER_KIND_YOSHI, *FIGHTER_KIND_PIKMIN].contains(&fighter_kind) && motion_kind == hash40("attack_12")) || 
        ([*FIGHTER_KIND_METAKNIGHT, *FIGHTER_KIND_PACKUN, *FIGHTER_KIND_SNAKE].contains(&fighter_kind) && motion_kind == hash40("attack_s3s")) ||
        (fighter_kind == *FIGHTER_KIND_METAKNIGHT && motion_kind == hash40("attack_s3_s2")) || 
        ([*FIGHTER_KIND_MARTH, *FIGHTER_KIND_CHROM, *FIGHTER_KIND_ROY].contains(&fighter_kind) && [hash40("special_s1"), hash40("special_s2"), hash40("special_s2_hi"), hash40("special_s2_lw"), hash40("special_s3"), hash40("special_s3_hi"), hash40("special_s3_lw")].contains(&motion_kind)) {
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
            if MotionModule::frame(boma) > (10.0) {
                if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4, true);
                }
                if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3, true);
                }
            }
        }
    }
}

pub unsafe fn pteStuff(boma: &mut smash::app::BattleObjectModuleAccessor, fighter_kind: i32, status_kind: i32, situation_kind: i32, motion_kind: u64, cat1: i32, stick_value_x: f32) { //Pte stuff
    if fighter_kind == *FIGHTER_KIND_SHIZUE {
        if [*FIGHTER_STATUS_KIND_JUMP, *FIGHTER_STATUS_KIND_JUMP_AERIAL, *FIGHTER_STATUS_KIND_FALL].contains(&status_kind) { // Midair dash dance
            if situation_kind == *SITUATION_KIND_AIR {
                if ((cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN) != 0) && stick_value_x * PostureModule::lr(boma) < 0.0 {
                    PostureModule::reverse_lr(boma);
                    PostureModule::update_rot_y_lr(boma);
                }
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
        if motion_kind == hash40("attack_air_b") { //Bair = fair
            PostureModule::reverse_lr(boma);
            PostureModule::update_rot_y_lr(boma);
            MotionModule::change_motion(boma, Hash40{hash: hash40("attack_air_f")}, 0.0, 1.0, false, 0.0, false, false);
        }
        if [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_RUN].contains(&status_kind) { //Get direction during dash
            DASHDIR[get_player_number(boma)] = PostureModule::lr(boma);
        }
        if [*FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4_START].contains(&status_kind) { //F smash flip check
            if [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_RUN].contains(&StatusModule::prev_status_kind(boma, 0)) {
                if (DASHDIR[get_player_number(boma)] * stick_value_x) < 0.0 {
                    PostureModule::reverse_lr(boma);
                    ISREVERSE[get_player_number(boma)] = true;
                }
            }
        }
        else {
            ISREVERSE[get_player_number(boma)] = false;
        }
        if ISREVERSE[get_player_number(boma)] {
            if [*FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4_START].contains(&status_kind) { //F smash flip
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

pub unsafe fn djcs (boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, fighter_kind: i32, kinetic_type: i32) {
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
    if [*FIGHTER_KIND_PEACH, *FIGHTER_KIND_YOSHI].contains(&fighter_kind) {
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
            if kinetic_type == *FIGHTER_KINETIC_TYPE_JUMP_AERIAL {
                if MotionModule::frame(boma) < (7.0) {
                    if !(ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP)) {
                        println!("djc");
                        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
                    }
                }
            }
        }
    }
}

pub unsafe fn edgeCancels(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, fighter_kind: i32, situation_kind: i32) { //Edge cancels
    if [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_TURN_RUN,  
    *FIGHTER_STATUS_KIND_FALL, *FIGHTER_STATUS_KIND_FALL_AERIAL, *FIGHTER_STATUS_KIND_FALL_SPECIAL, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, 
    *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_LANDING_LIGHT, *FIGHTER_STATUS_KIND_APPEAL, 
    *FIGHTER_STATUS_KIND_GUARD_DAMAGE, *FIGHTER_STATUS_KIND_ESCAPE_AIR].contains(&status_kind) {
        if GroundModule::is_ottotto(boma, 0.76) {
            GroundModule::set_correct(boma, smash::cpp::root::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
    }
    //Character specific edge cancels
    if situation_kind == *SITUATION_KIND_GROUND {
        if StatusModule::prev_situation_kind(boma) != *SITUATION_KIND_AIR {
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S { //Side b's
                if [*FIGHTER_KIND_CAPTAIN, *FIGHTER_KIND_INKLING, *FIGHTER_KIND_BAYONETTA, *FIGHTER_KIND_DIDDY, *FIGHTER_KIND_SHIZUE, *FIGHTER_KIND_MARTH].contains(&fighter_kind) {
                    if GroundModule::is_ottotto(boma, 0.76) {
                        GroundModule::set_correct(boma, smash::cpp::root::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                    }
                }
                if fighter_kind == *FIGHTER_KIND_JACK {
                    if GroundModule::is_ottotto(boma, 0.76) {
                        GroundModule::set_correct(boma, smash::cpp::root::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
                    }
                }
            }
            if fighter_kind == *FIGHTER_KIND_JACK { // Joker gun edge cancels
                if [*FIGHTER_JACK_STATUS_KIND_SPECIAL_N_ESCAPE, *FIGHTER_JACK_STATUS_KIND_SPECIAL_N_LANDING, *FIGHTER_JACK_STATUS_KIND_SPECIAL_N_BARRAGE_END].contains(&status_kind) {
                    if GroundModule::is_ottotto(boma, 0.76) {
                        GroundModule::set_correct(boma, smash::cpp::root::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                    }
                }
            }
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI { //Up specials
                if [*FIGHTER_KIND_CLOUD, *FIGHTER_KIND_PIKACHU, *FIGHTER_KIND_GEKKOUGA, *FIGHTER_KIND_INKLING].contains(&fighter_kind) {
                    if GroundModule::is_ottotto(boma, 0.76) {
                        GroundModule::set_correct(boma, smash::cpp::root::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                    }
                }
            }
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N { //Neutral Specials
                if [*FIGHTER_KIND_JACK, *FIGHTER_KIND_MARTH, *FIGHTER_KIND_PLIZARDON].contains(&fighter_kind) {
                    if GroundModule::is_ottotto(boma, 0.76) {
                        GroundModule::set_correct(boma, smash::cpp::root::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                    }
                }
            }
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW { //Down Special edgecancels
                if [*FIGHTER_KIND_CAPTAIN].contains(&fighter_kind) {
                    if GroundModule::is_ottotto(boma, 0.76) {
                        GroundModule::set_correct(boma, smash::cpp::root::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
                    }
                }
            }
        }
    }
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

pub unsafe fn sm4shJabLocks(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32) {
    if status_kind == *FIGHTER_STATUS_KIND_DOWN_DAMAGE { //Sm4sh jab locks
        LOCKED[get_player_number(boma)] = true;
    }
    else if [*FIGHTER_STATUS_KIND_DOWN_STAND_FB, *FIGHTER_STATUS_KIND_DOWN_STAND_ATTACK].contains(&status_kind) {
        if LOCKED[get_player_number(boma)] {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DOWN_STAND, true);
            LOCKED[get_player_number(boma)] = false;
        }
    }
    else if ![*FIGHTER_STATUS_KIND_DOWN_CONTINUE, *FIGHTER_STATUS_KIND_DOWN_WAIT, *FIGHTER_STATUS_KIND_DOWN, *FIGHTER_STATUS_KIND_DOWN_WAIT_CONTINUE].contains(&status_kind) {
        LOCKED[get_player_number(boma)] = false;
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
    }
}

pub unsafe fn regainDJ(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, fighter_kind: i32, situation_kind: i32) {
    if fighter_kind == *FIGHTER_KIND_CAPTAIN { //Falcon return dj after down b
        if situation_kind == *SITUATION_KIND_AIR {
            if [*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_WALL_END].contains(&status_kind) {
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

pub unsafe fn shortens(boma: &mut smash::app::BattleObjectModuleAccessor, fighter_kind: i32, motion_kind: u64) { //Shortens
    if fighter_kind == *FIGHTER_KIND_FALCO {
        if motion_kind == hash40("special_air_s") {
            if MotionModule::frame(boma) > 2.0 && MotionModule::frame(boma) < 6.0 {
                if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                    MotionModule::change_motion(boma, Hash40{hash: hash40("special_air_s_end")}, 0.0, 1.0, false, 0.0, false, false);
                }
            }
        }
        if motion_kind == hash40("special_s") {
            if MotionModule::frame(boma) > 2.0 && MotionModule::frame(boma) < 6.0 {
                if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                    MotionModule::change_motion(boma, Hash40{hash: hash40("special_s_end")}, 0.0, 1.0, false, 0.0, false, false);
                }
            }
        }
    }
    if fighter_kind == *FIGHTER_KIND_FOX {
        if motion_kind == hash40("special_air_s") {
            if MotionModule::frame(boma) > 0.0 && MotionModule::frame(boma) < 4.0 {
                if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                    MotionModule::change_motion(boma, Hash40{hash: hash40("special_air_s_end")}, 0.0, 1.0, false, 0.0, false, false);
                }
            }
        }
        if motion_kind == hash40("special_s") {
            if MotionModule::frame(boma) > 0.0 && MotionModule::frame(boma) < 4.0 {
                if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                    MotionModule::change_motion(boma, Hash40{hash: hash40("special_s_end")}, 0.0, 1.0, false, 0.0, false, false);
                }
            }
        }
    }
    if fighter_kind == *FIGHTER_KIND_WOLF {
        if motion_kind == hash40("special_air_s") {
            if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                MotionModule::change_motion(boma, Hash40{hash: hash40("special_air_s_end")}, 0.0, 1.0, false, 0.0, false, false);
            }
        }
        if motion_kind == hash40("special_s") {
            if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                MotionModule::change_motion(boma, Hash40{hash: hash40("special_s_end")}, 0.0, 1.0, false, 0.0, false, false);
            }
        }
    }
}

pub unsafe fn ivyHeals(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, fighter_kind: i32, motion_kind: u64) { //Ivysaur healing and meter gain
    if fighter_kind == *FIGHTER_KIND_PFUSHIGISOU {
        if [hash40("special_n_end"), hash40("special_air_n_end")].contains(&motion_kind) {
            if AMOUNTSOLAR[get_player_number(boma)] < 200 {
                CancelModule::enable_cancel(boma);
            }
        }
        if motion_kind == hash40("attack_air_hi") {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                if CANHEAL[get_player_number(boma)] {
                    DamageModule::add_damage(boma, -2.0, 0);
                    CANHEAL[get_player_number(boma)] = false;
                    if AMOUNTSOLAR[get_player_number(boma)] < 200 {
                        AMOUNTSOLAR[get_player_number(boma)] += 8;
                        println!("increased meter by 8, new meter: {}", AMOUNTSOLAR[get_player_number(boma)]);
                    }
                }
            }
        }
        if motion_kind == hash40("attack_air_lw") {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                if CANHEAL[get_player_number(boma)] {
                    DamageModule::add_damage(boma, -3.0, 0);
                    CANHEAL[get_player_number(boma)] = false;
                    if AMOUNTSOLAR[get_player_number(boma)] < 200 {
                        AMOUNTSOLAR[get_player_number(boma)] += 10;
                        println!("increased meter by 10, new meter: {}", AMOUNTSOLAR[get_player_number(boma)]);
                    }
                }
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_CATCH_ATTACK {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                if CANHEAL[get_player_number(boma)] {
                    DamageModule::add_damage(boma, -1.0, 0);
                    CANHEAL[get_player_number(boma)] = false;
                    if AMOUNTSOLAR[get_player_number(boma)] < 200 { 
                        AMOUNTSOLAR[get_player_number(boma)] += 3;
                        println!("increased meter by 3, new meter: {}", AMOUNTSOLAR[get_player_number(boma)]);
                    }
                }
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4 {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                if CANHEAL[get_player_number(boma)] {
                    DamageModule::add_damage(boma, -3.5, 0);
                    CANHEAL[get_player_number(boma)] = false;
                    if AMOUNTSOLAR[get_player_number(boma)] < 200 { 
                        AMOUNTSOLAR[get_player_number(boma)] += 14;
                        println!("increased meter by 14, new meter: {}", AMOUNTSOLAR[get_player_number(boma)]);
                    }
                }
            }
        }
        if AMOUNTSOLAR[get_player_number(boma)] >= 200 { //meter start and end
            AMOUNTSOLAR[get_player_number(boma)] = 200;
            if SOLARSTART[get_player_number(boma)] == 0 {
                println!("start meter");
                SOLARSTART[get_player_number(boma)] = GLOBALFRAMECOUNT;
            }
            if (GLOBALFRAMECOUNT - SOLARSTART[get_player_number(boma)]) == (1800 * 8) { //Too lazy to make the global frame counter and solar start a float lollllll
                AMOUNTSOLAR[get_player_number(boma)] = 0;
                SOLARSTART[get_player_number(boma)] = 0;
                println!("no more meter!");
            }
            let pos = smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
            let rot = smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
            let idk = smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
            EffectModule::req_on_joint(boma, Hash40{hash: hash40("pfushigisou_tmuchi_flash")}, Hash40{hash: hash40("flower")}, &pos, &rot, 1.5, &idk, &idk, false, 0, 0, 0);
        }
        else if AMOUNTSOLAR[get_player_number(boma)] == 0 {
            EffectModule::kill_kind(boma, Hash40{hash: hash40("pfushigisou_tmuchi_flash")}, false, false);
        }
        else {
            let pos = smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
            let rot = smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
            let idk = smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
            EffectModule::req_on_joint(boma, Hash40{hash: hash40("pfushigisou_tmuchi_flash")}, Hash40{hash: hash40("flower")}, &pos, &rot, (0.0075 * AMOUNTSOLAR[get_player_number(boma)] as f32), &idk, &idk, false, 0, 0, 0);
        }
        if MotionModule::frame(boma) < 2.0 || (![hash40("attack_air_lw"), hash40("attack_air_hi")].contains(&motion_kind) && ![*FIGHTER_STATUS_KIND_CATCH_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_HI4].contains(&status_kind)) {
            CANHEAL[get_player_number(boma)] = true;
        }
    }
    else {
        AMOUNTSOLAR[get_player_number(boma)] = 0;
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

pub unsafe fn grabIsAerial(boma: &mut smash::app::BattleObjectModuleAccessor, situation_kind: i32) { //Grab will now do aerials in the air
    if situation_kind == *SITUATION_KIND_AIR { 
        if !ItemModule::is_have_item(boma, 0) {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CATCH) {
                if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, true);
                }
            }
        }
    }
}

pub unsafe fn deleteExtraBanan(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32) { //Banana deletes on hit
    if status_kind == *FIGHTER_STATUS_KIND_SLIP {
        if ArticleModule::is_exist(boma, *ITEM_KIND_BANANA) {
            ItemModule::remove_item(boma, *ITEM_KIND_BANANA);
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
            *FIGHTER_KIND_ROBOT, *FIGHTER_KIND_WOLF, *FIGHTER_KIND_LITTLEMAC, *FIGHTER_KIND_KROOL, *FIGHTER_KIND_GAOGAEN, *FIGHTER_KIND_TANTAN].contains(&fighter_kind) {
                max_offset = 4.0;
            }
            
        if  [*FIGHTER_KIND_DONKEY, *FIGHTER_KIND_LINK, *FIGHTER_KIND_SAMUS, *FIGHTER_KIND_SAMUSD, *FIGHTER_KIND_CAPTAIN, *FIGHTER_KIND_PEACH, *FIGHTER_KIND_KOOPA, 
            *FIGHTER_KIND_SHEIK, *FIGHTER_KIND_ZELDA, *FIGHTER_KIND_MARTH, *FIGHTER_KIND_LUCINA, *FIGHTER_KIND_GANON, *FIGHTER_KIND_ROY, *FIGHTER_KIND_CHROM, 
            *FIGHTER_KIND_SZEROSUIT, *FIGHTER_KIND_SNAKE, *FIGHTER_KIND_IKE, *FIGHTER_KIND_WIIFIT, *FIGHTER_KIND_ROSETTA, *FIGHTER_KIND_PALUTENA, 
            *FIGHTER_KIND_REFLET, *FIGHTER_KIND_SHULK, *FIGHTER_KIND_RYU, *FIGHTER_KIND_KEN, *FIGHTER_KIND_CLOUD, *FIGHTER_KIND_KAMUI, *FIGHTER_KIND_BAYONETTA, 
            *FIGHTER_KIND_RIDLEY, *FIGHTER_KIND_SIMON, *FIGHTER_KIND_RICHTER, *FIGHTER_KIND_JACK, *FIGHTER_KIND_BRAVE, *FIGHTER_KIND_DOLLY, *FIGHTER_KIND_MASTER].contains(&fighter_kind) {
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

pub unsafe fn setGrndVelo(boma: &mut smash::app::BattleObjectModuleAccessor, fighter: &mut L2CFighterCommon, status_kind: i32, stick_value_x: f32) {
    if [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_TURN_RUN, *FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_TURN_RUN_BRAKE, 
    *FIGHTER_STATUS_KIND_WALK, *FIGHTER_STATUS_KIND_WALK_BRAKE, *FIGHTER_STATUS_KIND_RUN_BRAKE].contains(&status_kind) {
        GRNDVELOS[get_player_number(boma)] = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    if status_kind == *FIGHTER_STATUS_KIND_JUMP {
        if StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_GROUND {
            let newspeed = returnLarge(returnLarge((WorkModule::get_param_float(boma, 0, hash40("jump_speed_x")) * stick_value_x + GRNDVELOS[get_player_number(boma)] * WorkModule::get_param_float(boma, 0, hash40("jump_speed_x_mul"))), WorkModule::get_param_float(boma, 0, hash40("jump_speed_x_max"))), WorkModule::get_param_float(boma, 0, hash40("air_speed_x_stable")));
        }
    }
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
}

pub unsafe fn removeSHMacro(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32) {
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

pub unsafe fn fixExtraJumps (boma: &mut smash::app::BattleObjectModuleAccessor, motion_kind: u64, fighter_kind: i32) {
    if [*FIGHTER_KIND_RIDLEY, *FIGHTER_KIND_PIT].contains(&fighter_kind) {
        if motion_kind == hash40("jump_aerial_f3") {
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
    }
}

pub unsafe fn disableStuff (boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, fighter_kind: i32) {
    if fighter_kind != *FIGHTER_KIND_MEWTWO {
        if [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_ATTACK_S3, 
        *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_100, *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_HI4,
        *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_STATUS_KIND_AIR_LASSO].contains(&status_kind) { //Change last used move
            if status_kind != PASTATTACKS[get_player_number(boma)] && (status_kind != DISABLEDMOVE[get_player_number(boma)] || !ISDISABLED[get_player_number(boma)]) {
                if PASTATTACKS[get_player_number(boma)] != DISABLEDMOVE[get_player_number(boma)] {
                    ISDISABLED[get_player_number(boma)] = false;
                    DISABLEDMOVE[get_player_number(boma)] = 0;
                }
                PASTATTACKS[get_player_number(boma)] = status_kind;
            }
        }
    }
    else {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                let mut i = 0 as usize;
                while i < 10 {
                    if i != get_player_number(boma) && DISABLEDMOVE[i] != 0 {
                        DISABLEDMOVE[i] = PASTATTACKS[i];
                        ISDISABLED[i] = true;
                    }
                }
            }
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
    if DamageModule::damage(boma, 0) >= 200.0 {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DEAD, true);
        DamageModule::add_damage(boma, -1.0 * DamageModule::damage(boma, 0), 0);
    }
}

pub unsafe fn squirtleSideB (boma: &mut smash::app::BattleObjectModuleAccessor, fighter_kind: i32, motion_kind: u64) {
    if fighter_kind == *FIGHTER_KIND_PZENIGAME {
        if motion_kind == hash40("special_air_s_hit") {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP, true);
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
                if [*FIGHTER_DIDDY_STATUS_KIND_SPECIAL_N_SHOOT, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_N_CHARGE].contains(&status_kind) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
                }
            }
            else if fighter_kind == *FIGHTER_KIND_BAYONETTA {
                if status_kind == *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_N_CHARGE {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
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
            HITFALLBUFFER[0][get_player_number(boma)] = AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT);
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
            if [*FIGHTER_KIND_GEKKOUGA, *FIGHTER_KIND_WOLF, *FIGHTER_KIND_FALCO, *FIGHTER_KIND_FOX, *FIGHTER_KIND_MARIO, *FIGHTER_KIND_DIDDY, *FIGHTER_KIND_CLOUD, *FIGHTER_KIND_PICHU].contains(&fighter_kind) {
                if stick_y_flick_check(boma, -0.3) {
                    WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
                }
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
            if [*FIGHTER_KIND_PFUSHIGISOU, *FIGHTER_KIND_NESS, *FIGHTER_KIND_JACK].contains(&fighter_kind) {
                if stick_y_flick_check(boma, -0.3) {
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
        if [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL].contains(&status_kind) {
            if situation_kind == *SITUATION_KIND_GROUND {
                if (GroundModule::is_passable_ground(boma) && stick_value_y <= -0.75) || ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_JUMP) 
                || (ControlModule::is_enable_flick_jump(boma) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_FLICK_JUMP)) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
                }
            }
        }
    }
}

pub unsafe fn moonwalking (boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, stick_value_x: f32, stick_value_y: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_DASH {
        if stick_value_y > -0.33 && stick_value_x > -0.33 && stick_value_x < 0.33 {
            CANMOONWALK[get_player_number(boma)] = false;
        }
        else if (PostureModule::lr(boma) < 0.0 && stick_value_x > 0.5) || (PostureModule::lr(boma) > 0.0 && stick_value_x < -0.5) {
            let reverse_mul = Vector3f{x: -2.0, y: 1.0, z: 1.0};
            KineticModule::mul_speed(boma, &reverse_mul, *FIGHTER_KINETIC_ENERGY_ID_NONE);
            MotionModule::update_trans_move_speed(boma);
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_DASH);
        }
    }
    else {
        CANMOONWALK[get_player_number(boma)] = true;
    }
}

pub unsafe fn upbCancels (boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, fighter_kind: i32, cat1: i32) {
    if [*FIGHTER_KIND_CHROM, *FIGHTER_KIND_IKE].contains(&fighter_kind) {
        if [*FIGHTER_IKE_STATUS_KIND_SPECIAL_HI_3].contains(&status_kind) && MotionModule::frame(boma) > 16.0 {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE) != 0 {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
            }
        }
    }
}

pub unsafe fn ptSwaps (boma: &mut smash::app::BattleObjectModuleAccessor, fighter_kind: i32, cat1: i32, cat2: i32) {
    if [*FIGHTER_KIND_PZENIGAME, *FIGHTER_KIND_PFUSHIGISOU, *FIGHTER_KIND_PLIZARDON].contains(&fighter_kind) {
        if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L) != 0 {
            if WorkModule::is_enable_transition_term(boma, *FIGHTER_POKEMON_STATUS_TRANSITION_TERM_ID_PRE_START) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
            }
        }
        else if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R) != 0 {
            if WorkModule::is_enable_transition_term(boma, *FIGHTER_POKEMON_STATUS_TRANSITION_TERM_ID_PRE_START) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
                DOUBLESWAP[get_player_number(boma)] = true;
            }
        }
        if StatusModule::prev_status_kind(boma, 0) == 455 || StatusModule::prev_status_kind(boma, 1) == 455 {
            if DOUBLESWAP[get_player_number(boma)] {
                StatusModule::change_status_request_from_script(boma, 455, true);
                DOUBLESWAP[get_player_number(boma)] = false;
            }
        }
        if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 {
            if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_S) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_APPEAL, true);
                if PostureModule::lr(boma) < 0.0 {
                    MotionModule::change_motion(boma, Hash40{hash: hash40("appeal_s_l")}, 0.0, 1.0, false, 0.0, false, false);
                }
                else {
                    MotionModule::change_motion(boma, Hash40{hash: hash40("appeal_s_r")}, 0.0, 1.0, false, 0.0, false, false);
                }
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

pub unsafe fn animPortFix (boma: &mut smash::app::BattleObjectModuleAccessor, fighter_kind: i32, motion_kind: u64) {
    if fighter_kind == *FIGHTER_KIND_MARIO {
        if motion_kind == hash40("attack_hi3") {
            if MotionModule::frame(boma) == MotionModule::end_frame(boma) - 1.0 {
                MotionModule::change_motion(boma, Hash40{hash: hash40("wait")}, 0.0, 1.0, false, 0.0, false, false);
            }
        }
    }
    else if fighter_kind == *FIGHTER_KIND_CAPTAIN {
        if [hash40("attack_hi3"), hash40("attack_s4_charge"), hash40("attack_s4_s"), hash40("attack_s4_hi"), hash40("attack_s4_lw")].contains(&motion_kind) {
            if MotionModule::frame(boma) == MotionModule::end_frame(boma) - 2.0 {
                MotionModule::change_motion(boma, Hash40{hash: hash40("wait")}, 0.0, 1.0, false, 0.0, false, false);
            }
        }
    }
    else if fighter_kind == *FIGHTER_KIND_MARTH {
        if motion_kind == hash40("throw_f") {
            if MotionModule::frame(boma) == MotionModule::end_frame(boma) - 1.0 {
                MotionModule::change_motion(boma, Hash40{hash: hash40("wait")}, 0.0, 1.0, false, 0.0, false, false);
            }
        }
    }
    else if fighter_kind == *FIGHTER_KIND_FALCO {
        if [hash40("attack_hi3"), hash40("attack_hi4"), hash40("escape_n")].contains(&motion_kind) {
            if MotionModule::frame(boma) == MotionModule::end_frame(boma) - 2.0 {
                MotionModule::change_motion(boma, Hash40{hash: hash40("wait")}, 0.0, 1.0, false, 0.0, false, false);
            }
        }
    }
    else if fighter_kind == *FIGHTER_KIND_CHROM {
        if motion_kind == hash40("attack_hi4") {
            if MotionModule::frame(boma) == MotionModule::end_frame(boma) - 2.0 {
                MotionModule::change_motion(boma, Hash40{hash: hash40("wait")}, 0.0, 1.0, false, 0.0, false, false);
            }
        }
    }
}

pub unsafe fn bReverses (boma: &mut smash::app::BattleObjectModuleAccessor, motion_kind: u64, fighter_kind: i32) {
    if fighter_kind == FIGHTER_KIND_CAPTAIN || fighter_kind == FIGHTER_KIND_GANON {
        if motion_kind == hash40("special_air_lw") {
            if MotionModule::frame(boma) < 6.0 {
                if ControlModule::get_stick_x(boma) * PostureModule::lr(boma) <= -0.66 {
                    BREVERSE[get_player_number(boma)];
                }
            }
            else if MotionModule::frame(boma) >= 6.0 && MotionModule::frame(boma) < 7.0 {
                if BREVERSE[get_player_number(boma)] {
                    PostureModule::reverse_lr(boma);
                    PostureModule::update_rot_y_lr(boma);
                    KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
                }                
            }
            else if MotionModule::frame(boma) >= 16.0 && MotionModule::frame(boma) < 17.0 {
                if BREVERSE[get_player_number(boma)] {
                    let reverse_mul = Vector3f{x: -1.0, y: 1.0, z: 1.0};
                    KineticModule::mul_speed(boma, &reverse_mul, *FIGHTER_KINETIC_ENERGY_ID_NONE);
                    MotionModule::update_trans_move_speed(boma);
                }
            }
        }
        else {
            BREVERSE[get_player_number(boma)];
        }
    }
    if fighter_kind == *FIGHTER_KIND_PZENIGAME {
        if motion_kind == hash40("special_air_n") {
            if MotionModule::frame(boma) < 6.0 {
                if ControlModule::get_stick_x(boma) * PostureModule::lr(boma) <= -0.66 {
                    BREVERSE[get_player_number(boma)];
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
    }
}

pub unsafe fn fixProjectiles (boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, fighter_kind: i32) {
    if !CANPROJECTILE[get_player_number(boma)] {
        if fighter_kind == *FIGHTER_KIND_PFUSHIGISOU {
            if status_kind != *FIGHTER_STATUS_KIND_SPECIAL_S {
                CANPROJECTILE[get_player_number(boma)] = true;
            }
        }
        else if fighter_kind == *FIGHTER_KIND_PICHU {
            if status_kind != *FIGHTER_STATUS_KIND_SPECIAL_N {
                CANPROJECTILE[get_player_number(boma)] = true;
            }
        }
    }
}

pub unsafe fn magicSeries (boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, motion_kind: u64, fighter_kind: i32, stick_value_x: f32, cat1: i32) {
    if fighter_kind == *FIGHTER_KIND_KEN {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            if [*FIGHTER_STATUS_KIND_ATTACK_100, *FIGHTER_STATUS_KIND_ATTACK_DASH].contains(&status_kind) {
                if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 ||
                (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0 ||
                (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 {
                    CancelModule::enable_cancel(boma);
                }
            }
            if [*FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND1, *FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND2, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_S3].contains(&status_kind) {
                if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) != 0 ||
                (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0 {
                    CancelModule::enable_cancel(boma);
                }
            }
            if [*FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_AIR].contains(&status_kind) {
                if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 {
                    CancelModule::enable_cancel(boma);
                }
            }
            if motion_kind == hash40("attack_air_n") {
                if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 ||
                (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) != 0 {
                    CancelModule::enable_cancel(boma);
                }
            }
            if motion_kind == hash40("attack_air_hi") {
                if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) != 0 {
                    CancelModule::enable_cancel(boma);
                }
            }
            if motion_kind == hash40("attack_air_f") || motion_kind == hash40("attack_air_lw") {
                if ((cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) != 0) && PostureModule::lr(boma) * stick_value_x < 0.0 {
                    CancelModule::enable_cancel(boma);
                }
            }
            if [*FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND, *FIGHTER_STATUS_KIND_SPECIAL_S].contains(&status_kind) {
                if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 {
                    CancelModule::enable_cancel(boma);
                }
            }
        }
    }
}

/* pub unsafe fn dashgrabSlide (boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32) {
    if status_kind == *FIGHTER_STATUS_KIND_CATCH_DASH {
        if StatusModule::prev_status_kind(boma, 0) == *FIGHTER_STATUS_KIND_DASH {
            let speed_vector = smash::phx::Vector3f {x: WorkModule::get_param_float(boma, 0, hash40("dash_speed")) * 0.2, y: 0.0, z: 0.0 };
            KineticModule::add_speed(boma, &speed_vector);
        }
        else {
            let speed_vector = smash::phx::Vector3f {x: WorkModule::get_param_float(boma, 0, hash40("run_speed_max")) * 0.2, y: 0.0, z: 0.0 };
            KineticModule::add_speed(boma, &speed_vector);
        }
    }
} */

// Use this for general per-frame fighter-level hooks
pub fn once_per_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
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

        /* if [*FIGHTER_KIND_PFUSHIGISOU, *FIGHTER_KIND_PZENIGAME, *FIGHTER_KIND_PLIZARDON].contains(&fighter_kind) {
            if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 {
                println!("status: {}, Ptrainer our: {}, Ptrainer standby: {}", status_kind, *FIGHTER_POKEMON_STATUS_KIND_SPECIAL_LW_OUT, *FIGHTER_POKEMON_STATUS_KIND_SPECIAL_LW_STANDBY);
                println!("Ivy out: {}, Ivy standby: {}, Squirtle out: {}", *FIGHTER_PFUSHIGISOU_STATUS_KIND_SPECIAL_LW_OUT, *FIGHTER_PFUSHIGISOU_STATUS_KIND_SPECIAL_LW_STANDBY, *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_LW_OUT);
                println!("Squirtle standby: {}, Zard out: {}, Zard standby: {}", *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_LW_STANDBY, *FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_LW_OUT, *FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_LW_STANDBY);
                println!("Down B status: {}", *FIGHTER_STATUS_KIND_SPECIAL_LW);
            }
        } */

        //let mut globals = *fighter.globals_mut();

        GLOBALFRAMECOUNT += 1; //Increase the frame counter by 1 each frame

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
        djcs(boma, status_kind, fighter_kind, kinetic_type);
        edgeCancels(boma, status_kind, fighter_kind, situation_kind);
        airDodgeZair(boma, status_kind, situation_kind, fighter_kind, cat2, cat3, stick_value_x);
        wallJumpSpecial(boma, status_kind, situation_kind, fighter_kind, cat1);
        sm4shJabLocks(boma, status_kind);
        noSpecialFall(boma, status_kind, situation_kind, fighter_kind);  
        regainDJ(boma, status_kind, fighter_kind, situation_kind);
        wiggleHitstun(boma, status_kind, cat1);
        shortens(boma, fighter_kind, motion_kind);
        ivyHeals(boma, status_kind, fighter_kind, motion_kind);
        setStatuses(boma, status_kind);
        meleeSmashStick(boma, status_kind);
        grabIsAerial(boma, situation_kind);
        deleteExtraBanan(boma, status_kind);
        meleeECBs(boma, status_kind, situation_kind, fighter_kind);
        setGrndVelo(boma, fighter, status_kind, stick_value_x);
        lagCanceled(boma, status_kind);
        removeSHMacro(boma, status_kind);
        fixExtraJumps(boma, motion_kind, fighter_kind);
        disableStuff(boma, status_kind, fighter_kind);
        regainAirDodge(boma, status_kind, situation_kind);
        deathStuff(boma);
        squirtleSideB(boma, fighter_kind, motion_kind);
        airDodgeCancels(boma, status_kind, situation_kind, fighter_kind, cat1);
        hitfalling(boma, status_kind, situation_kind);
        fastfallShit(boma, status_kind, situation_kind, fighter_kind);
        pkFlashCancels(boma, status_kind, fighter_kind, cat1);
        quickAttackCancels(boma, status_kind, situation_kind, fighter_kind, stick_value_y);
        moonwalking(boma, status_kind, stick_value_x, stick_value_y);
        upbCancels(boma, status_kind, fighter_kind, cat1);
        ptSwaps(boma, fighter_kind, cat1, cat2);
        driftDi(boma, status_kind, stick_value_x);
        grenShurikenFix(boma, status_kind, situation_kind, fighter_kind);
        animPortFix(boma, fighter_kind, motion_kind);
        bReverses(boma, motion_kind, fighter_kind);
        fixProjectiles(boma, status_kind, fighter_kind);
        magicSeries(boma, status_kind, motion_kind, fighter_kind, stick_value_x, cat1);
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
pub fn once_per_weapon_frame(fighter_base : &mut L2CFighterBase) {
    unsafe {
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

pub fn install() {
    acmd::add_custom_hooks!(once_per_fighter_frame);
    acmd::add_custom_weapon_hooks!(once_per_weapon_frame);
    skyline::install_hook!(get_param_float_hook);
    skyline::install_hook!(entry_cliff_hook);
    skyline::install_hook!(leave_cliff_hook);
    skyline::install_hook!(can_entry_cliff_hook);
    skyline::install_hook!(is_enable_transition_term_hook);
    skyline::install_hook!(check_button_on_hook);
    skyline::install_hook!(check_button_off_hook);
    skyline::install_hook!(is_valid_just_shield_reflector_hook);
    skyline::install_hook!(change_status_request_from_script_hook);
    skyline::install_hook!(get_int_hook);
    //skyline::install_hook!(float_param_accessor_hook);
    //add_hook(params_main).unwrap();
}