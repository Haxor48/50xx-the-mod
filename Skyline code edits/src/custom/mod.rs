use smash::app::lua_bind::*;
use smash::hash40;
use smash::app::utility::get_kind;
use smash::lua2cpp::{L2CFighterCommon, L2CFighterBase};
use smash::lib::lua_const::*;
use smash::phx::*;
use acmd;

static mut LEDGE_POS: [Vector3f; 9] = [smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0}, smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0}, smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0},
smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0}, smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0}, smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0}, smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0},
smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0}, smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0}];

static mut ECB_Y_OFFSETS: [f32; 9] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
static mut LOCKED: [bool; 9] = [false, false, false, false, false, false, false, false, false];
static mut NOSPECIALFALL: [bool; 9] = [false, false, false, false, false, false, false, false, false];
static mut RECENTSTATUSES: [[i32; 5]; 9] = [[0, 0, 0, 0, 0], [0, 0, 0, 0, 0], [0, 0, 0, 0, 0], [0, 0, 0, 0, 0], [0, 0, 0, 0, 0], [0, 0, 0, 0, 0], [0, 0, 0, 0, 0], [0, 0, 0, 0, 0], [0, 0, 0, 0, 0]];

pub unsafe fn get_player_number(boma: &mut smash::app::BattleObjectModuleAccessor) -> usize {
    return WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
}

#[skyline::hook(replace = smash::app::lua_bind::WorkModule::get_param_float)]

pub unsafe fn get_param_float_hook(boma: &mut smash::app::BattleObjectModuleAccessor, param_type: u64, param_hash: u64) -> f32 {
    let status_kind = StatusModule::status_kind(boma);
    if [hash40("landing_attack_air_frame_n"), hash40("landing_attack_air_frame_hi"), hash40("landing_attack_air_frame_lw"), 
    hash40("landing_attack_air_frame_f"), hash40("landing_attack_air_frame_b")].contains(&param_type) {
        if param_hash == 0 {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                if [*FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR].contains(&status_kind) {
                    println!("edited landing lag");
                    return (original!()(boma, param_type, param_hash) / 1.5) % 100.0;
                }
            }
        }
    }
    original!()(boma, param_type, param_hash)
}

#[skyline::hook(replace = smash::app::lua_bind::WorkModule::get_param_int64)]

pub unsafe fn get_param_int64_hook(boma: &mut smash::app::BattleObjectModuleAccessor, param_type: u64, param_hash: u64) -> u64 {
    let status_kind = StatusModule::status_kind(boma);
    if param_hash == hash40("common") {
        if param_type == hash40("invalid_capture_frame") {
            if status_kind == *FIGHTER_STATUS_KIND_THROW {
                if MotionModule::frame(boma) == 1.0 {
                    let new_frames = original!()(boma, param_type, param_hash) + 15;
                    println!("Added frames! New frames: {}", new_frames);
                    return new_frames;
                }
            }
            if original!()(boma, param_type, param_hash) != 1 {
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
                    return 1;
                }
            }
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
    if status_kind != *FIGHTER_STATUS_KIND_FALL_AERIAL && status_kind != *FIGHTER_STATUS_KIND_JUMP_AERIAL && status_kind != *FIGHTER_STATUS_KIND_FALL && 
    status_kind != *FIGHTER_STATUS_KIND_FLY && (status_kind != *FIGHTER_STATUS_KIND_AIR_LASSO || motion_kind == hash40("air_catch_hit")) && ((fighter_kind != *FIGHTER_KIND_PFUSHIGISOU /*&& fighter_kind != *FIGHTER_KIND_MASTER &&
    fighter_kind != *FIGHTER_KIND_TANTAN*/) || (status_kind != *FIGHTER_STATUS_KIND_SPECIAL_HI || motion_kind == hash40("air_catch_hit"))) && (fighter_kind != *FIGHTER_KIND_JACK && (status_kind == *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI2_RUSH ||
    status_kind != *FIGHTER_STATUS_KIND_SPECIAL_HI || motion_kind == hash40("air_catch_hit"))) { 
        let mut i = 0 as usize;
        while i < 8 {
            if i == entry_id || LEDGE_POS[i].x == 0.0 {
            }

            if pos.x == LEDGE_POS[i].x && pos.y == LEDGE_POS[i].y {
                println!("hogged");
                return 0 as u64;
            }
            i = i + 1;
        }
    }
    original!()(boma)
}

#[skyline::hook(replace = smash::app::lua_bind::WorkModule::is_enable_transition_term)]

pub unsafe fn is_enable_transition_term_hook(boma: &mut smash::app::BattleObjectModuleAccessor, flag: i32) -> bool {
    let status_kind = StatusModule::status_kind(boma);
    if (flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR) && [*FIGHTER_STATUS_KIND_DAMAGE_FALL, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, 
    *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR].contains(&status_kind) {
        return false;
    } 
    if flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR {
        if !(original!()(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL)) {
            return false;
        }
    }

    original!()(boma, flag)
}

// Use this for general per-frame fighter-level hooks
pub fn once_per_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = StatusModule::status_kind(boma);
        let situation_kind = StatusModule::situation_kind(boma);
        let fighter_kind = get_kind(boma);
        let cat1 = ControlModule::get_command_flag_cat(boma, 0);
        let cat2 = ControlModule::get_command_flag_cat(boma, 1);
        let stick_value_y = ControlModule::get_stick_y(boma);
        let stick_value_x = ControlModule::get_stick_x(boma);
        let cstick_value_x = ControlModule::get_sub_stick_x(boma);
        let cstick_value_y = ControlModule::get_sub_stick_y(boma);
        let motion_kind = MotionModule::motion_kind(boma);
        let kinetic_type = KineticModule::get_kinetic_type(boma);
        let flick_x_dir = ControlModule::get_flick_x_dir(boma);

        if fighter_kind == *FIGHTER_KIND_FOX { //Jump Cancel Move
            if [*FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_LOOP, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_HIT, 
            *FIGHTER_STATUS_KIND_SPECIAL_LW].contains(&status_kind) {
                if MotionModule::frame(boma) > (3.0) {
                    if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_JUMP) {
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
        if fighter_kind == *FIGHTER_KIND_WOLF {
            if [*FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_LOOP, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_HIT, 
            *FIGHTER_STATUS_KIND_SPECIAL_LW].contains(&status_kind) {
                if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) {
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
                if MotionModule::frame(boma) > (4.0) {
                    if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_JUMP) {
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

        if fighter_kind == *FIGHTER_KIND_FOX { //Land Cancels
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
                if StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR && situation_kind == *SITUATION_KIND_GROUND {
                    CancelModule::enable_cancel(boma);
                }
            }
        }
        if fighter_kind == *FIGHTER_KIND_FALCO {
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
                if StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR && situation_kind == *SITUATION_KIND_GROUND {
                    CancelModule::enable_cancel(boma);
                }
            }
        }
        if fighter_kind == *FIGHTER_KIND_WOLF {
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
                if StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR && situation_kind == *SITUATION_KIND_GROUND {
                    CancelModule::enable_cancel(boma);
                }
            }
        }

        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH { //Dacus
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

        if status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT { //Jump Cancel grabs and etc
            if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_CATCH) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_CATCH, true);
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

        if status_kind == *FIGHTER_STATUS_KIND_TURN_DASH { //Perfect Pivots
            if MotionModule::frame(boma) < (2.0) && stick_value_x < 0.6 && stick_value_x > -0.6 {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
            }
        }

        if [*FIGHTER_STATUS_KIND_ESCAPE_B, *FIGHTER_STATUS_KIND_ESCAPE_F].contains(&status_kind) && situation_kind == *SITUATION_KIND_GROUND { //Glide Tossing
            if MotionModule::frame(boma) < (7.0) {
                if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_CATCH) {
                    let x_vel = PostureModule::lr(boma) * 2.0 * KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                    let speed_vector = smash::phx::Vector3f { x: x_vel, y: 0.0, z: 0.0 };
                    CancelModule::enable_cancel(boma);
                    KineticModule::add_speed(boma, &speed_vector);
                }
            }
        }

        if [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_RUN_BRAKE].contains(&status_kind) { //Taunt Slides
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_HI) ||
            ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_LW) ||
            ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_L) ||
            ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
                let x_vel = PostureModule::lr(boma) * 2.5 * KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                let speed_vector = smash::phx::Vector3f { x: x_vel, y: 0.0, z: 0.0 };
                CancelModule::enable_cancel(boma);
                KineticModule::add_speed(boma, &speed_vector);
            }
        }

        if status_kind == *FIGHTER_STATUS_KIND_ITEM_THROW_DASH { //Ditcit
            if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 || 
                (stick_value_y >= 0.7 && ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_CATCH)) {
                if MotionModule::frame(boma) < (7.0) {
                    let x_vel = PostureModule::lr(boma) * 3.0 * KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                    let speed_vector = smash::phx::Vector3f { x: x_vel, y: 0.0, z: 0.0 };
                    CancelModule::enable_cancel(boma);
                    KineticModule::add_speed(boma, &speed_vector);
                }
            }
        }

        if status_kind == *FIGHTER_STATUS_KIND_DASH || status_kind == *FIGHTER_STATUS_KIND_TURN_DASH { //Shield Stop
            if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE) != 0 {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD_ON, true);
            }
        }

        if [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_TURN_RUN, 
        *FIGHTER_STATUS_KIND_ESCAPE_AIR].contains(&status_kind) { //Dash Platform Drop
            if situation_kind == SITUATION_KIND_GROUND {
                if stick_value_y <= -0.75 {
                    if GroundModule::is_passable_ground(boma) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_PASS, true);
                    }
                }
            }
        }

        if status_kind == *FIGHTER_STATUS_KIND_GUARD || status_kind == *FIGHTER_STATUS_KIND_GUARD_ON { //Shield Drop
            if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_GUARD_TO_PASS) != 0 || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_HI) ||
            ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_LW) ||
            ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_L) ||
            ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
                if GroundModule::is_passable_ground(boma) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_PASS, true);
                }
            }
        }

        if motion_kind == hash40("attack_11") { //Jab Cancels
            if ![*FIGHTER_KIND_CHROM, *FIGHTER_KIND_CHROM, *FIGHTER_KIND_GANON, *FIGHTER_KIND_SHIZUE, *FIGHTER_KIND_PIKACHU, *FIGHTER_KIND_PICHU, *FIGHTER_KIND_METAKNIGHT].contains(&fighter_kind) {
                if MotionModule::frame(boma) > (10.0) {
                    if stick_value_y <= -0.66 {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SQUAT_F, true);
                    }
                    if stick_value_x <= -0.6 {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_TURN, true);
                    }
                    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE) != 0 {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD_ON, true);
                    }
                }
            }
        }
        if motion_kind == hash40("attack_12") {
            if ![*FIGHTER_KIND_DONKEY, *FIGHTER_KIND_MARTH, *FIGHTER_KIND_LUCINA, *FIGHTER_KIND_KOOPA, *FIGHTER_KIND_DAISY, *FIGHTER_KIND_PEACH, *FIGHTER_KIND_SAMUS, 
            *FIGHTER_KIND_SAMUSD, *FIGHTER_KIND_PURIN, *FIGHTER_KIND_NANA, *FIGHTER_KIND_POPO, *FIGHTER_KIND_YOSHI, *FIGHTER_KIND_PIKMIN].contains(&fighter_kind) {
                if MotionModule::frame(boma) > (10.0) {
                    if stick_value_y <= -0.66 {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SQUAT_F, true);
                    }
                    if stick_value_x <= -0.6 {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_TURN, true);
                    }
                    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE) != 0 {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD_ON, true);
                    }
                }
            }
        }
        if motion_kind == hash40("attack_s3") {
            if [*FIGHTER_KIND_METAKNIGHT, *FIGHTER_KIND_PACKUN, *FIGHTER_KIND_SNAKE].contains(&fighter_kind) {
                if MotionModule::frame(boma) > (10.0) {
                    if stick_value_y <= -0.66 {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SQUAT_F, true);
                    }
                    if stick_value_x <= -0.6 {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_TURN, true);
                    }
                    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE) != 0 {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD_ON, true);
                    }
                }
            }
        }
        if motion_kind == hash40("attack_s3_s2") {
            if fighter_kind == *FIGHTER_KIND_METAKNIGHT {
                if MotionModule::frame(boma) > (10.0) {
                    if stick_value_y <= -0.66 {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SQUAT_F, true);
                    }
                    if stick_value_x <= -0.6 {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_TURN, true);
                    }
                    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE) != 0 {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD_ON, true);
                    }
                }
            }
        }
        if [hash40("special_s1"), hash40("special_s2"), hash40("special_s2_hi"), hash40("special_s2_lw"), hash40("special_s3"), hash40("special_s3_hi"), hash40("special_s3_lw")].contains(&motion_kind) {
            if [*FIGHTER_KIND_MARTH, *FIGHTER_KIND_CHROM, *FIGHTER_KIND_ROY].contains(&fighter_kind) {
                if MotionModule::frame(boma) > (10.0) {
                    if stick_value_y <= -0.66 {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SQUAT_F, true);
                    }
                    if stick_value_x <= -0.6 {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_TURN, true);
                    }
                    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE) != 0 {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD_ON, true);
                    }
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

        if fighter_kind == *FIGHTER_KIND_SHIZUE { //Pte stuff
            if ![*FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_STATUS_KIND_SPECIAL_S].contains(&status_kind) { // Midair dash dance
                if situation_kind == *SITUATION_KIND_AIR {
                    if (PostureModule::lr(boma) < 0.0 && flick_x_dir > 0) || (PostureModule::lr(boma) > 0.0 && flick_x_dir < 0) {
                        PostureModule::reverse_lr(boma);
                    }
                }
            }
            if situation_kind == *SITUATION_KIND_AIR { //Boost side b
                if status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL {
                    let speed_vector = smash::phx::Vector3f { x: 0.0, y: 1.5, z: 0.0 };
                    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0 {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S, true);
                        KineticModule::add_speed(boma, &speed_vector);
                    }
                }
            }
        }

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

        if [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_TURN_RUN,  
        *FIGHTER_STATUS_KIND_FALL, *FIGHTER_STATUS_KIND_FALL_AERIAL, *FIGHTER_STATUS_KIND_FALL_SPECIAL, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, 
        *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_LANDING_LIGHT, *FIGHTER_STATUS_KIND_APPEAL, 
        *FIGHTER_STATUS_KIND_GUARD_DAMAGE, *FIGHTER_STATUS_KIND_ESCAPE_AIR].contains(&status_kind) { //edge cancels
            if GroundModule::is_ottotto(boma, 0.76) {
                GroundModule::set_correct(boma, smash::cpp::root::app::GroundCorrectKind{_address: *GROUND_CORRECT_KIND_AIR as u8});
            }
        }

        if [*FIGHTER_STATUS_KIND_ESCAPE_AIR].contains(&status_kind) && situation_kind == *SITUATION_KIND_AIR { //Air dodge cancel zair
            if [*FIGHTER_KIND_SAMUSD, *FIGHTER_KIND_LUCAS, *FIGHTER_KIND_LUIGI, *FIGHTER_KIND_SAMUS, *FIGHTER_KIND_YOUNGLINK, *FIGHTER_KIND_TOONLINK, 
            *FIGHTER_KIND_SZEROSUIT].contains(&fighter_kind) {
                if MotionModule::frame(boma) < (7.0) {
                    if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_CATCH) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_AIR_LASSO, true);
                    }
                }
            }
        }

        if fighter_kind == *FIGHTER_KIND_MARIO { //Walljump during special
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
                if situation_kind == *SITUATION_KIND_AIR {
                    if PostureModule::pos_x(boma) == GroundModule::hang_cliff_pos_3f(boma).x {
                        if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_WALL_JUMP_LEFT) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_WALL_JUMP_RIGHT) != 0 {
                            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WALL_JUMP, true);
                        }
                    }
                }
            }
        }

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
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
                if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                    NOSPECIALFALL[get_player_number(boma)] = true;
                }
            }
            if !NOSPECIALFALL[get_player_number(boma)] {
                if status_kind == *FIGHTER_STATUS_KIND_FALL {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
                    NOSPECIALFALL[get_player_number(boma)] = true;
                }
            }
            if ![*FIGHTER_STATUS_KIND_FALL, *FIGHTER_STATUS_KIND_SPECIAL_S].contains(&status_kind) {
                if NOSPECIALFALL[get_player_number(boma)] {
                    NOSPECIALFALL[get_player_number(boma)] = true;
                }
            }
        }

        if fighter_kind == *FIGHTER_KIND_CAPTAIN { //Falcon return dj after down b
            if StatusModule::prev_status_kind(boma, 0) == *FIGHTER_STATUS_KIND_SPECIAL_LW && situation_kind == *SITUATION_KIND_AIR {
                WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
            }
        }

        if status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY { // Wiggle out of tumble
            if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH) != 0 {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
            }
        }

        if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR { //Turnaround during wavedash
            if situation_kind == *SITUATION_KIND_GROUND {
                if (PostureModule::lr(boma) < 0.0 && flick_x_dir > 0) || (PostureModule::lr(boma) > 0.0 && flick_x_dir < 0) {
                    PostureModule::reverse_lr(boma);
                }
            }
        }

        if fighter_kind == *FIGHTER_KIND_FALCO { //Shortens
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

        if fighter_kind == *FIGHTER_KIND_PLIZARDON { //Charizard cancels
            if [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_S_RUSH, 
            *FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_S_END].contains(&status_kind) {
                if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                    CancelModule::enable_cancel(boma);
                }
            }
        }

        if fighter_kind == *FIGHTER_KIND_PFUSHIGISOU { //Ivysaur healing and meter gain
            if motion_kind == hash40("attack_air_hi") {
                if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                    println!("heal");
                    DamageModule::add_damage(boma, -3.0, 0);
                }
            }
            if motion_kind == hash40("attack_air_lw") {
                if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                    println!("heal");
                    DamageModule::add_damage(boma, -2.0, 0);
                }
            }
            if status_kind == *FIGHTER_STATUS_KIND_CATCH_ATTACK {
                if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                    println!("heal");
                    DamageModule::add_damage(boma, -1.0, 0);
                }
            }
        }

        if status_kind != RECENTSTATUSES[get_player_number(boma)][0] { //Set Statuses
            let mut i = 4;
            while i > 0 {
                RECENTSTATUSES[get_player_number(boma)][i] = RECENTSTATUSES[get_player_number(boma)][i-1];
                i = i - 1;
            }
            RECENTSTATUSES[get_player_number(boma)][0] = status_kind;
        }

        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD { //Melee-Style Smash Stick
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

        if situation_kind == *SITUATION_KIND_AIR { //CStick aerial fix
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CSTICK_ON) {
                if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR) {
                    if PostureModule::lr(boma) < 0.0 {
                        if cstick_value_x > 0.4 {
                            MotionModule::change_motion(boma, Hash40{hash: hash40("attack_air_b")}, 0.0, 1.0, false, 0.0, false, false);
                        }
                        else if cstick_value_x < -0.4 {
                            MotionModule::change_motion(boma, Hash40{hash: hash40("attack_air_f")}, 0.0, 1.0, false, 0.0, false, false);
                        }
                    }
                    else if PostureModule::lr(boma) > 0.0 {
                        if cstick_value_x > 0.4 {
                            MotionModule::change_motion(boma, Hash40{hash: hash40("attack_air_f")}, 0.0, 1.0, false, 0.0, false, false);
                        }
                        else if cstick_value_x < -0.4 {
                            MotionModule::change_motion(boma, Hash40{hash: hash40("attack_air_b")}, 0.0, 1.0, false, 0.0, false, false);
                        }
                    }
                    if cstick_value_y > 0.4 {
                        MotionModule::change_motion(boma, Hash40{hash: hash40("attack_air_hi")}, 0.0, 1.0, false, 0.0, false, false);
                    }
                    else if cstick_value_y < 0.4 {
                        MotionModule::change_motion(boma, Hash40{hash: hash40("attack_air_lw")}, 0.0, 1.0, false, 0.0, false, false);
                    }
                }
            }
        }

        //ECBs
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
    skyline::install_hook!(get_param_int64_hook);
}