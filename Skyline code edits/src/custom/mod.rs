use smash::app::lua_bind::*;
use smash::hash40;
use smash::app::utility::get_kind;
use smash::lua2cpp::{L2CFighterCommon, L2CFighterBase};
use smash::lib::lua_const::*;
use smash::phx::*;
use acmd;

static mut LEDGE_POS: [Vector3f; 8] = [smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0}, smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0}, smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0}, 
smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0}, smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0}, smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0}, 
smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0}, smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0}];

#[skyline::hook(replace = smash::app::lua_bind::WorkModule::is_enable_transition_term)]

pub unsafe fn get_param_float_hook(boma: &mut smash::app::BattleObjectModuleAccessor, param_type: u64, param_hash: u64) -> f32 {
    let status_kind = StatusModule::status_kind(boma);
    if param_hash == 0 {
        if [hash40("landing_attack_air_frame_n"), hash40("landing_attack_air_frame_hi"), hash40("landing_attack_air_frame_lw"), 
        hash40("landing_attack_air_frame_f"), hash40("landing_attack_air_frame_b")].contains(&param_type) {
            if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) {
                if [*FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR].contains(&status_kind) {
                    return (original!()(boma, param_type, param_hash) / 1.5) % 100.0;
                }
            }
        }
    }
    original!()(boma, param_type, param_hash)
}
#[skyline::hook(replace = smash::app::lua_bind::GroundModule::entry_cliff)]

pub unsafe fn entry_cliff_hook(boma: &mut smash::app::BattleObjectModuleAccessor) -> u64 {
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    let a = entry_id as usize;
    LEDGE_POS[a] = GroundModule::hang_cliff_pos_3f(boma);
    original!()(boma)
}

#[skyline::hook(replace = smash::app::lua_bind::GroundModule::leave_cliff)]

pub unsafe fn leave_cliff_hook(boma: &mut smash::app::BattleObjectModuleAccessor) -> u64 {
    let entry_id = WorkModule::get_int(boma , *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    let a = entry_id as usize;
    LEDGE_POS[a] = smash::phx::Vector3f { x: 0.0, y: 0.0, z:0.0 };
    original!()(boma)
}

/*#[skyline::hook(replace = smash::app::lua_bind::GroundModule::can_entry_cliff)]

pub unsafe fn can_entry_cliff_hook(boma: &mut smash::app::BattleObjectModuleAccessor) -> u64 {
    let pos = GroundModule::hang_cliff_pos_3f(boma);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    let status_kind = StatusModule::status_kind(boma);
    let fighter_kind = get_kind(boma);
    let motion_kind = MotionModule::motion_kind(boma);
    if status_kind != *FIGHTER_STATUS_KIND_FALL_AERIAL && status_kind != *FIGHTER_STATUS_KIND_JUMP_AERIAL && status_kind != *FIGHTER_STATUS_KIND_FALL && 
    status_kind != *FIGHTER_STATUS_KIND_FLY && (status_kind != *FIGHTER_STATUS_KIND_AIR_LASSO || motion_kind == hash40("air_catch_hit")) && ((fighter_kind != *FIGHTER_KIND_PFUSHIGISOU /*&& fighter_kind != *FIGHTER_KIND_MASTER &&
    fighter_kind != *FIGHTER_KIND_TANTAN*/) || (status_kind != *FIGHTER_STATUS_KIND_SPECIAL_HI || motion_kind == hash40("air_catch_hit"))) && (fighter_kind != *FIGHTER_KIND_JACK && (status_kind == *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI2_RUSH ||
    status_kind != *FIGHTER_STATUS_KIND_SPECIAL_HI || motion_kind == hash40("air_catch_hit"))) { 
        let mut i = 0;
        while i < 8 {
            let r = i as usize;
            if i == entry_id || LEDGE_POS[r].x == 0.0 {
                continue;
            }

            if pos.x == LEDGE_POS[r].x && pos.y == LEDGE_POS[r].y {
                return 0;
            }
            i += 1;
        }
    }
    original!()(boma)
} */

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
        let motion_kind = MotionModule::motion_kind(boma);
        let kinetic_type = KineticModule::get_kinetic_type(boma);

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
                println!("status_works!");
                if MotionModule::frame(boma) > (5.0) {
                    println!("timer works!");
                    if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_JUMP) {
                        if situation_kind == *SITUATION_KIND_AIR{
                            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX){
                                println!("air jump");
                                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
                            }
                        }
                        else if situation_kind == *SITUATION_KIND_GROUND{
                            println!("ground jump");
                            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                        }
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
                if MotionModule::frame(boma) >= 11.0 {
                    if StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR && situation_kind == *SITUATION_KIND_GROUND {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
                    }
                }
            }
        }
        if fighter_kind == *FIGHTER_KIND_FALCO {
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
                if MotionModule::frame(boma) >= 8.0 {
                    if StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR && situation_kind == *SITUATION_KIND_GROUND {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
                    }
                }
            }
        }
        if fighter_kind == *FIGHTER_KIND_WOLF {
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
                if MotionModule::frame(boma) >= 16.0 {
                    if StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR && situation_kind == *SITUATION_KIND_GROUND {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
                    }
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

        if status_kind == *FIGHTER_STATUS_KIND_DASH || status_kind == *FIGHTER_STATUS_KIND_RUN { //Taunt Slides
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_HI) ||
            ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_LW) ||
            ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_L) ||
            ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
                let x_vel = PostureModule::lr(boma) * 2.0 * KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
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

        if [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_TURN_RUN].contains(&status_kind) { //Dash Platform Drop
            if stick_value_y <= -0.75 {
                if GroundModule::is_passable_ground(boma) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_PASS, true);
                }
            }
        }

        if status_kind == *FIGHTER_STATUS_KIND_GUARD || status_kind == *FIGHTER_STATUS_KIND_GUARD_ON { //Shield Drop
            if (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_GUARD_TO_PASS) != 0 {
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
                    if (PostureModule::lr(boma) < 0.0 && stick_value_x >= 0.2) || (PostureModule::lr(boma) > 0.0 && stick_value_x <= -0.2) {
                        PostureModule::reverse_lr(boma);
                    }
                }
            }
            if situation_kind == *SITUATION_KIND_AIR { //Boost side b
                if status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL {
                    let speed_vector = smash::phx::Vector3f { x: 0.0, y: 2.0, z: 0.0 };
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
                            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
                        }
                    }
                }
            }
        }

        if [*FIGHTER_STATUS_KIND_ATTACK_AIR].contains(&status_kind) { //edge cancels
            if GroundModule::is_ottotto(boma, 0.76) {
                GroundModule::set_correct(boma, smash::cpp::root::app::GroundCorrectKind{_address: *GROUND_CORRECT_KIND_AIR as u8});
            }
        }

        /* if StatusModule::is_situation_changed(boma) {
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
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter_base.lua_state_agent);
        let frame = smash::app::lua_bind::MotionModule::frame(module_accessor) as i32;

        if frame % 10 == 0 {
            println!("[Weapon Hook] Frame : {}", frame);
        }
    }
}

pub fn install() {
    acmd::add_custom_hooks!(once_per_fighter_frame);
    acmd::add_custom_weapon_hooks!(once_per_weapon_frame);
    skyline::install_hook!(get_param_float_hook);
    skyline::install_hook!(entry_cliff_hook);
    skyline::install_hook!(leave_cliff_hook);
    //skyline::install_hook!(can_entry_cliff_hook);
}