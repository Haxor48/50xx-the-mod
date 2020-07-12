#include <switch_min.h>
#include <string.h>
#include <cmath>
#include <stdio.h>
#include <stdarg.h>
#include <vector>

#include "saltysd/saltysd_core.h"
#include "saltysd/saltysd_ipc.h"
#include "saltysd/saltysd_dynamic.h"
#include "saltysd/saltysd_helper.h"

#include "useful/const_value_table.h"
#include "useful/useful.h"
#include "useful/crc32.h"
#include "app/lua_bind.h"
#include "acmd_wrapper.h"

#include "useful/raygun_printer.h"

using namespace lib;
using namespace app::lua_bind;
using namespace app::sv_animcmd;

namespace app {
    namespace utility {
        extern u64 get_kind(u64) asm("_ZN3app7utility8get_kindEPKNS_26BattleObjectModuleAccessorE") LINKABLE;
    }
}
int GLOBAL_FRAME_COUNT = 0;
int currframe = 0;
u64 prev_get_command_flag_cat = 0;
u64 init_settings_prev = 0;
u64 prev_is_enable_transition_term = 0;
bool lagcanceled[8] = {false, false, false, false, false, false, false, false};
Vector3f ledge_pos[8];
int get_player_number(u64 boma){
		return WorkModule::get_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
}
bool jump_checker_buffer(u64 boma, int cat){
    bool tapJumpOn = ControlModule::is_enable_flick_jump(boma);
    if((cat & FIGHTER_PAD_CMD_CAT1_FLAG_JUMP) && tapJumpOn){
        return true;
    }
    
    if(cat & FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON){
        return true;
    }
    return false;
}
u64 __entry_cliff(u64 boma) {
    // Flag this cliff as occupied
    int entry_id = WorkModule::get_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);    
    ledge_pos[entry_id] = *(Vector3f*)GroundModule::hang_cliff_pos_3f(boma);
    
    // Call base func
    u64 ground_module = load_module(boma,0x58);
    return ((int (*)(u64))(load_module_impl(ground_module, 0x250)))(ground_module);
}

u64 __can_entry_cliff(u64 boma) {
    // Get player pos and check if another player is 
    // occupying a ledge within our grab range
    Vector3f pos = *(Vector3f*)GroundModule::hang_cliff_pos_3f(boma);
    int entry_id = WorkModule::get_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    u64 status_kind = StatusModule::status_kind(boma);
    int fighter_kind = app::utility::get_kind(boma);
    if (status_kind != FIGHTER_STATUS_KIND_FALL_AERIAL && status_kind != FIGHTER_STATUS_KIND_JUMP_AERIAL && status_kind != FIGHTER_STATUS_KIND_FALL && 
    status_kind != FIGHTER_STATUS_KIND_FLY && status_kind != FIGHTER_STATUS_KIND_AIR_LASSO_REACH && status_kind != FIGHTER_STATUS_KIND_AIR_LASSO_HANG) {
        for (int i=0;i<8;i++) {
            if (i == entry_id || ledge_pos[i].x == 0) {
                continue;
            }

            if(pos.x == ledge_pos[i].x && pos.y == ledge_pos[i].y) {
                return false;
            }
        }
    }
    // Call base func
    u64 ground_module = load_module(boma, 0x58);
    return ((int (*)(u64))(load_module_impl(ground_module, 0x260)))(ground_module);
}

u64 __is_grab(u64 boma, int unk1) {
    print_string(boma, "is_grab %llx", unk1);

    u64 grab_module = load_module(boma, 0x158);
    return ((int (*)(u64, int))(load_module_impl(grab_module, 0x78)))(grab_module, unk1);
}

u64 __leave_cliff(u64 boma) {
    // clear occupancy when leaving edge
    int entry_id = WorkModule::get_int(boma , FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    ledge_pos[entry_id] = Vector3f { .x =0, .y=0, .z=0 };

    // Call base func
    u64 ground_module = load_module(boma, 0x58);    
    return ((int (*)(u64))(load_module_impl(ground_module, 0x278)))(ground_module);
}

void jabCancels (u64 &boma, int &cat1, u64 &status_kind, float stick_value_x, float stick_value_y) {
    int fighter_kind = app::utility::get_kind(boma);
    u64 motion_kind = MotionModule::motion_kind(boma);
    if (motion_kind == hash40("attack_11")) {
        if (fighter_kind != FIGHTER_KIND_CHROM && fighter_kind != FIGHTER_KIND_CHROM && fighter_kind != FIGHTER_KIND_GANON && fighter_kind != FIGHTER_KIND_SHIZUE &&
        fighter_kind != FIGHTER_KIND_PIKACHU && fighter_kind != FIGHTER_KIND_PICHU && fighter_kind != FIGHTER_KIND_METAKNIGHT) {
            if(MotionModule::frame(boma) > (10)) {
                if (stick_value_y <= -0.66) {
                    (u64)StatusModule::change_status_request_from_script(boma, (u64)FIGHTER_STATUS_KIND_SQUAT_F,0x1);
                }
                if (stick_value_x <= -0.6) {
                    (u64)StatusModule::change_status_request_from_script(boma, (u64)FIGHTER_STATUS_KIND_TURN,0x1);
                }
                if (cat1 & FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE) {
                    (u64)StatusModule::change_status_request_from_script(boma, (u64)FIGHTER_STATUS_KIND_GUARD_ON,0x1);
                }
            }
        }
    }
    if (motion_kind == hash40("attack_12")) {
        if (fighter_kind != FIGHTER_KIND_DONKEY && fighter_kind != FIGHTER_KIND_MARTH && fighter_kind != FIGHTER_KIND_LUCINA && fighter_kind != FIGHTER_KIND_KOOPA &&
        fighter_kind != FIGHTER_KIND_DAISY && fighter_kind != FIGHTER_KIND_PEACH && fighter_kind != FIGHTER_KIND_SAMUS && fighter_kind != FIGHTER_KIND_SAMUSD &&
        fighter_kind != FIGHTER_KIND_PURIN && fighter_kind != FIGHTER_KIND_NANA && fighter_kind != FIGHTER_KIND_POPO && fighter_kind != FIGHTER_KIND_YOSHI &&
        fighter_kind != FIGHTER_KIND_PIKMIN) {
            if(MotionModule::frame(boma) > (10)) {
                if (stick_value_y <= -0.66) {
                    (u64)StatusModule::change_status_request_from_script(boma, (u64)FIGHTER_STATUS_KIND_SQUAT_F,0x1);
                }
                if (stick_value_x <= -0.6) {
                    (u64)StatusModule::change_status_request_from_script(boma, (u64)FIGHTER_STATUS_KIND_TURN,0x1);
                }
                if (cat1 & FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE) {
                    (u64)StatusModule::change_status_request_from_script(boma, (u64)FIGHTER_STATUS_KIND_GUARD_ON,0x1);
                }
            }
        }
    }
    if (status_kind == FIGHTER_STATUS_KIND_ATTACK_S3) {
        if (fighter_kind == FIGHTER_KIND_METAKNIGHT || fighter_kind == FIGHTER_KIND_PACKUN || fighter_kind == FIGHTER_KIND_SNAKE) {
            if(MotionModule::frame(boma) > (10)) {
                if (stick_value_y <= -0.66) {
                    (u64)StatusModule::change_status_request_from_script(boma, (u64)FIGHTER_STATUS_KIND_SQUAT_F,0x1);
                }
                if (stick_value_x <= -0.6) {
                    (u64)StatusModule::change_status_request_from_script(boma, (u64)FIGHTER_STATUS_KIND_TURN,0x1);
                }
                if (cat1 & FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE) {
                    (u64)StatusModule::change_status_request_from_script(boma, (u64)FIGHTER_STATUS_KIND_GUARD_ON,0x1);
                }
            }
        }
    }
    if (motion_kind == hash40("special_s1") || motion_kind == hash40("special_s2") || motion_kind == hash40("special_s2_hi") || motion_kind == hash40("special_s2_lw") ||
    motion_kind == hash40("special_s3") || motion_kind == hash40("special_s3_hi") || motion_kind == hash40("special_s3_lw")) {
        if (fighter_kind == FIGHTER_KIND_MARTH || fighter_kind == FIGHTER_KIND_CHROM || fighter_kind == FIGHTER_KIND_ROY || fighter_kind == FIGHTER_KIND_LUCINA) {
            if(MotionModule::frame(boma) > (10)) {
                if (stick_value_y <= -0.66) {
                    (u64)StatusModule::change_status_request_from_script(boma, (u64)FIGHTER_STATUS_KIND_SQUAT_F,0x1);
                }
                if (stick_value_x <= -0.6) {
                    (u64)StatusModule::change_status_request_from_script(boma, (u64)FIGHTER_STATUS_KIND_TURN,0x1);
                }
                if (cat1 & FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE) {
                    (u64)StatusModule::change_status_request_from_script(boma, (u64)FIGHTER_STATUS_KIND_GUARD_ON,0x1);
                }
            }
        }
    }
}

void shield_drop(u64 boma, int category, u64 status_kind){    
    u64 control_module = load_module(boma, 0x48);
    int (*get_command_flag_cat)(u64,int) = (int (*)(u64,int))load_module_impl(control_module, 0x350);
    int cat = get_command_flag_cat(control_module, 1);
    if(status_kind == FIGHTER_STATUS_KIND_GUARD || status_kind == FIGHTER_STATUS_KIND_GUARD_ON){
        if(cat & FIGHTER_PAD_CMD_CAT2_FLAG_GUARD_TO_PASS){ // Check if shield drop input has been taken
            if(GroundModule::is_passable_ground(boma)){
                StatusModule::change_status_request_from_script(boma, FIGHTER_STATUS_KIND_PASS, 0x1);
            }
        }
    }
}

void dashPlatformDrop (u64 boma, u64 &status_kind, float stick_value_y) {
    if (status_kind == FIGHTER_STATUS_KIND_DASH || status_kind == FIGHTER_STATUS_KIND_TURN_DASH || status_kind == FIGHTER_STATUS_KIND_RUN || status_kind == FIGHTER_STATUS_KIND_TURN_RUN) {
        if (stick_value_y <= -0.75) {
            if(GroundModule::is_passable_ground(boma)){
                StatusModule::change_status_request_from_script(boma, FIGHTER_STATUS_KIND_PASS, 0x1);
            }
        }
    }
}

void shieldStopping (u64 boma, int &cat1, int status_kind) {
    if (status_kind == FIGHTER_STATUS_KIND_DASH || status_kind == FIGHTER_STATUS_KIND_TURN_DASH) {
        if (cat1 & FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE) {
            StatusModule::change_status_request_from_script(boma, FIGHTER_STATUS_KIND_GUARD_ON, 0x1);
        }
    }
}

u64 __is_valid_just_shield_reflector(u64 boma){
    return true;
}

void tauntCanceling (u64 boma, int status_kind) {
    u64 motion_kind = MotionModule::motion_kind(boma);
    if (status_kind == FIGHTER_STATUS_KIND_DASH) {
            if (ControlModule::check_button_on(boma, CONTROL_PAD_BUTTON_APPEAL_HI) ||
            ControlModule::check_button_on(boma, CONTROL_PAD_BUTTON_APPEAL_LW) ||
            ControlModule::check_button_on(boma, CONTROL_PAD_BUTTON_APPEAL_S_L) ||
            ControlModule::check_button_on(boma, CONTROL_PAD_BUTTON_APPEAL_S_R)) {
                CancelModule::enable_cancel(boma);
                Vector3f new_speed = {.x = 1.0, .y = 0.0, .z = 0.0};
                KineticModule::add_speed(boma, &new_speed);
            }
    }
    /* if (motion_kind == hash40("appeal_hi_l") || motion_kind == hash40("appeal_hi_r") || motion_kind == hash40("appeal_lw_l") ||
    motion_kind == hash40("appeal_lw_r") || motion_kind == hash40("appeal_s_l") || motion_kind == hash40("appeal_s_r")) {
        CancelModule::enable_cancel(boma);
    } */
}

/*void fixDjs (u64 boma, int status_kind) {
    int fighter_kind = app::utility::get_kind(boma);
    if (fighter_kind == FIGHTER_KIND_RIDLEY || fighter_kind == FIGHTER_KIND_PLIZARDON || fighter_kind == FIGHTER_KIND_PIT) {
        if (status_kind == FIGHTER_STATUS_KIND_JUMP_AERIAL) {
            if (WorkModule::get_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) == WorkModule::get_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) - 1) {
                MotionModule::change_motion_kind(boma, hash40("jump_aerial_f2"));
            }
        }
    }
} */

/*void removeAirDodgeTumble (u64 boma, int status_kind, int situation_kind) {
    if (status_kind == FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL && situation_kind == SITUATION_KIND_AIR) {
        if (ControlModule::check_button_on(boma, CONTROL_PAD_BUTTON_GUARD) || ControlModule::check_button_on(boma, CONTROL_PAD_BUTTON_GUARD_HOLD) || ControlModule::check_button_on(boma, CONTROL_PAD_BUTTON_CATCH)) {
            (u64)StatusModule::change_status_request_from_script(boma, (u64)FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,0x1);  
        }
    }

} */

int __add_motion_2nd(u64 boma, u64 motion_kind, float param3, float param4, bool param5, float param6){
    
    u8 fighter_category = (u8)(*(u32*)(boma + 8) >> 28);
    int fighter_kind = app::utility::get_kind(boma);
    if (fighter_category == BATTLE_OBJECT_CATEGORY_FIGHTER){
        if(fighter_kind == FIGHTER_KIND_NESS || fighter_kind == FIGHTER_KIND_LUCAS || fighter_kind == FIGHTER_KIND_MEWTWO){
            if(motion_kind == hash40("jump_aerial_f") || motion_kind == hash40("jump_aerial_b") || motion_kind == hash40("jump_aerial_hi") || motion_kind == hash40("jump_aerial_low")){
                if (MotionModule::frame(boma) < (5)) {
                    if ( !(ControlModule::check_button_on(boma, CONTROL_PAD_BUTTON_JUMP)) ) {
                        return 0;
                    }
                }
            }
        }
        /* if (fighter_kind == FIGHTER_KIND_PEACH || fighter_kind == FIGHTER_KIND_YOSHI) {
            if(motion_kind == hash40("jump_aerial_f") || motion_kind == hash40("jump_aerial_b") || motion_kind == hash40("jump_aerial_hi") || motion_kind == hash40("jump_aerial_low")) {
                if (!(ControlModule::check_button_on(boma, CONTROL_PAD_BUTTON_JUMP)) ) {
                    return 0;
                }
            }   
        } */
    }
    u64 motion_module = load_module(boma, 0x88);
    int (*add_motion_2nd)(u64, u64, float, float, bool, float) = (int (*)(u64, u64, float, float, bool, float))load_module_impl(motion_module, 0x108);
    int add_motion_2nd_Original = add_motion_2nd(motion_module, motion_kind, param3, param4, param5, param6);
        
    return add_motion_2nd_Original;
    return 0;
}

int __change_kinetic (u64 boma, int kinetic_type) {
    int kinetic_type_new = kinetic_type;
    u8 fighter_category = (u8)(*(u32*)(boma + 8) >> 28);
    int fighter_kind = app::utility::get_kind(boma);
    u64 situation_kind = StatusModule::situation_kind(boma);
    if (fighter_category == BATTLE_OBJECT_CATEGORY_FIGHTER){
        if(fighter_kind == FIGHTER_KIND_NESS || fighter_kind == FIGHTER_KIND_LUCAS || fighter_kind == FIGHTER_KIND_MEWTWO){
            if(StatusModule::status_kind(boma) == FIGHTER_STATUS_KIND_ATTACK_AIR){
                if(kinetic_type == FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION_2ND){
                    if (MotionModule::frame(boma) < (5)) {
                        if (!(ControlModule::check_button_on(boma, CONTROL_PAD_BUTTON_JUMP)) ) {
                            kinetic_type_new = FIGHTER_KINETIC_TYPE_MOTION_FALL;
                        }
                    }
                }
            }
        }
        /* if (fighter_kind == FIGHTER_KIND_PEACH || fighter_kind == FIGHTER_KIND_YOSHI) {
            if(StatusModule::status_kind(boma) == FIGHTER_STATUS_KIND_ATTACK_AIR && StatusModule::prev_status_kind(boma, boma) == FIGHTER_STATUS_KIND_JUMP_AERIAL){
                if (MotionModule::frame(boma) < (3)) {
                    if (!(ControlModule::check_button_on(boma, CONTROL_PAD_BUTTON_JUMP)) ) {
                        u64 controller_energy_control = KineticModule::get_energy(boma, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                        Vector3f new_speed;
                        new_speed.y = -1.0 * KineticModule::get_sum_speed_y(boma, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                        KineticModule::add_speed_outside(boma, FIGHTER_KINETIC_ENERGY_ID_CONTROL, &new_speed);
                    }
                }
            }
        } */
    }
    
    u64 kinetic_module = load_module(boma, 0x68);
    int (*change_kinetic)(u64, int) = (int (*)(u64, int))load_module_impl(kinetic_module, 0x118);
    int change_kinetic_Original = change_kinetic(kinetic_module, kinetic_type_new);
    return change_kinetic_Original;
}

void perfectPivots (u64 boma, u64 &status_kind, float stick_value_x) {
    if (status_kind == FIGHTER_STATUS_KIND_TURN_DASH) {
        if (MotionModule::frame(boma) < (2) && stick_value_x < 0.6 && stick_value_x > -0.6) {
            (u64)StatusModule::change_status_request_from_script(boma, (u64)FIGHTER_STATUS_KIND_WAIT,0x1);
        }
    }
}

void ditcit (u64 boma, int &cat1, u64 &status_kind, int stick_value_y) {
    if (status_kind == FIGHTER_STATUS_KIND_ITEM_THROW_DASH) {
        if (cat1 & FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 || cat1 & FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 || (stick_value_y >= 0.7 && cat1 & FIGHTER_PAD_CMD_CAT1_FLAG_CATCH)) {
            if (MotionModule::frame(boma) < (7)) {
                CancelModule::enable_cancel(boma);
                Vector3f new_speed = {.x = 0.5, .y = 0.0, .z = 0.0};
                KineticModule::add_speed(boma, &new_speed);
            }
        }
    }
}

void jumpCancelGrab (u64 &boma, int &cat1, u64 &status_kind, float stick_value_y) {
    int fighter_kind = app::utility::get_kind(boma);
    if (status_kind == FIGHTER_STATUS_KIND_JUMP_SQUAT) {
        if (ControlModule::check_button_on(boma, CONTROL_PAD_BUTTON_CATCH)) {
            (u64)StatusModule::change_status_request_from_script(boma, (u64)FIGHTER_STATUS_KIND_CATCH,0x1);
        }
        else if (ControlModule::check_button_on(boma, CONTROL_PAD_BUTTON_GUARD)) {
            (u64)StatusModule::change_status_request_from_script(boma, (u64)FIGHTER_STATUS_KIND_ESCAPE_AIR,0x1);
        }
        if (cat1 & FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 && MotionModule::frame(boma) > (1)) {
            (u64)StatusModule::change_status_request_from_script(boma, (u64)FIGHTER_STATUS_KIND_ATTACK_HI4,0x1);  
        }
        else if (stick_value_y >= 0.66 && (ControlModule::check_button_on(boma, CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, CONTROL_PAD_BUTTON_SPECIAL_RAW))) {
            (u64)StatusModule::change_status_request_from_script(boma, (u64)FIGHTER_STATUS_KIND_SPECIAL_HI,0x1);  
        }
        /* if (fighter_kind == FIGHTER_KIND_PITB) {
            if (cat1 & FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) {
                (u64)StatusModule::change_status_request_from_script(boma, (u64)FIGHTER_STATUS_KIND_SPECIAL_S,0x1);
            }
        } */
    }
}
void otherCancels (u64 &boma, int &cat1, u64 &status_kind, float stick_value_x, float stick_value_y) {
    int fighter_kind = app::utility::get_kind(boma);
}

void glideTossing (u64 &boma, int &cat1, u64 &status_kind, u64 &situation_kind, float stick_value_y, float stick_value_x) {
    if ((status_kind == FIGHTER_STATUS_KIND_ESCAPE_B || status_kind == FIGHTER_STATUS_KIND_ESCAPE_F) && situation_kind == SITUATION_KIND_GROUND) {
        if (MotionModule::frame(boma) < (7)) {
            if (cat1 & FIGHTER_PAD_CMD_CAT1_FLAG_CATCH || ControlModule::get_pad_flag(boma) & FIGHTER_PAD_FLAG_ATTACK_TRIGGER || ControlModule::check_button_on(boma, CONTROL_PAD_BUTTON_CATCH)) {
                CancelModule::enable_cancel(boma);
                Vector3f new_speed = {.x = 0.3, .y = 0.0, .z = 0.0};
                KineticModule::add_speed(boma, &new_speed);
            }
        }
    }
}

void dacus(u64 &boma, int &cat1, u64 &status_kind, float stick_value_y){
    if(status_kind == FIGHTER_STATUS_KIND_ATTACK_DASH){
        if(MotionModule::frame(boma) < (10)){
            if ((cat1 & FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) ||
                (stick_value_y >= 0.7 && (cat1 & FIGHTER_PAD_CMD_CAT1_FLAG_CATCH)) ){
                (u64)StatusModule::change_status_request_from_script(boma, (u64)FIGHTER_STATUS_KIND_ATTACK_HI4_START,0x1);
            }
            // Adjust input window for utilt to remove accidental usmashes
            if(MotionModule::frame(boma) > (2.0)){
                if(cat1 & FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3){
                    (u64)StatusModule::change_status_request_from_script(boma, (u64)FIGHTER_STATUS_KIND_ATTACK_HI4_START,0x1);
                }
            }
        }
    }
}
void landingLagCancel(u64 &boma, int &cat1, u64 &status_kind) {
    u8 fighter_category = (u8)(*(u32*)(boma + 8) >> 28);
    if (fighter_category == BATTLE_OBJECT_CATEGORY_FIGHTER) {
        if (AttackModule::is_infliction(boma, COLLISION_KIND_MASK_HIT)) {
            if (status_kind == FIGHTER_STATUS_KIND_ATTACK_AIR || status_kind == FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR) {
                lagcanceled[get_player_number(boma)] = true;
            }
        }
    }
}
void jumpCancels(u64 boma, int category, u64 status_kind, u64 situation_kind){
    u8 fighter_category = (u8)(*(u32*)(boma + 8) >> 28);
    int fighter_kind = app::utility::get_kind(boma);
    u64 control_module = load_module(boma, 0x48);
    int (*get_command_flag_cat)(u64,int) = (int (*)(u64,int))load_module_impl(control_module, 0x350);
    int cat = get_command_flag_cat(control_module, 0);
    if (fighter_category == BATTLE_OBJECT_CATEGORY_FIGHTER && fighter_kind == FIGHTER_KIND_FOX){ //Fox
        if (status_kind == FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_HIT || status_kind == FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_END || status_kind == FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_LOOP) {
            if (jump_checker_buffer(boma, cat)){// (ControlModule::get_pad_flag(boma) & FIGHTER_PAD_FLAG_JUMP_TRIGGER){
                if (situation_kind == SITUATION_KIND_AIR){
                    if (WorkModule::get_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX)){
                        StatusModule::change_status_request_from_script(boma, FIGHTER_STATUS_KIND_JUMP_AERIAL,0x1);
                    }
                }
                else if (situation_kind == SITUATION_KIND_GROUND){
                        StatusModule::change_status_request_from_script(boma, FIGHTER_STATUS_KIND_JUMP_SQUAT,0x1);
                }
            }
        }
    }
    if (fighter_category == BATTLE_OBJECT_CATEGORY_FIGHTER && fighter_kind == FIGHTER_KIND_FALCO){ //Falco
        if (status_kind == FIGHTER_STATUS_KIND_SPECIAL_LW) {
            if (jump_checker_buffer(boma, cat)){// (ControlModule::get_pad_flag(boma) & FIGHTER_PAD_FLAG_JUMP_TRIGGER){
                if (situation_kind == SITUATION_KIND_AIR){
                    if (WorkModule::get_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX)){
                        StatusModule::change_status_request_from_script(boma, FIGHTER_STATUS_KIND_JUMP_AERIAL,0x1);
                    }
                }
                else if (situation_kind == SITUATION_KIND_GROUND){
                        StatusModule::change_status_request_from_script(boma, FIGHTER_STATUS_KIND_JUMP_SQUAT,0x1);
                }
            }
        }
    }
    if (fighter_category == BATTLE_OBJECT_CATEGORY_FIGHTER && fighter_kind == FIGHTER_KIND_WOLF){ //Wolf
        if (status_kind == FIGHTER_STATUS_KIND_SPECIAL_LW) {
            if (jump_checker_buffer(boma, cat)){// (ControlModule::get_pad_flag(boma) & FIGHTER_PAD_FLAG_JUMP_TRIGGER){
                if (situation_kind == SITUATION_KIND_AIR){
                    if (WorkModule::get_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX)){
                        StatusModule::change_status_request_from_script(boma, FIGHTER_STATUS_KIND_JUMP_AERIAL,0x1);
                    }
                }
                else if (situation_kind == SITUATION_KIND_GROUND){
                        StatusModule::change_status_request_from_script(boma, FIGHTER_STATUS_KIND_JUMP_SQUAT,0x1);
                }
            }
        }
    }
    if (fighter_category == BATTLE_OBJECT_CATEGORY_FIGHTER && fighter_kind == FIGHTER_KIND_PZENIGAME){ //Squirtle
        if (AttackModule::is_infliction(boma, COLLISION_KIND_MASK_HIT)) {
            if (status_kind == FIGHTER_STATUS_KIND_SPECIAL_S) {
				StatusModule::change_status_request_from_script(boma, FIGHTER_STATUS_KIND_FALL,0x1);
            }
        }
    }
}
void landCancels (u64 boma, int &cat1, int status_kind, int situation_kind) {
    u8 fighter_category = (u8)(*(u32*)(boma + 8) >> 28);
    int fighter_kind = app::utility::get_kind(boma);
    if (fighter_category == BATTLE_OBJECT_CATEGORY_FIGHTER) {
        if (fighter_kind == FIGHTER_KIND_FOX) {
            if (status_kind == FIGHTER_STATUS_KIND_SPECIAL_N) {
                if (MotionModule::frame(boma) >= 11) {
                    if (StatusModule::prev_situation_kind(boma) == SITUATION_KIND_AIR && situation_kind == SITUATION_KIND_GROUND) {
                        StatusModule::change_status_request_from_script(boma, FIGHTER_STATUS_KIND_WAIT,0x1);
                    }
                }
            }
        }
        if (fighter_kind == FIGHTER_KIND_FALCO) {
            if (status_kind == FIGHTER_STATUS_KIND_SPECIAL_N) {
                if (MotionModule::frame(boma) >= 8) {
                    if (StatusModule::prev_situation_kind(boma) == SITUATION_KIND_AIR && situation_kind == SITUATION_KIND_GROUND) {
                        StatusModule::change_status_request_from_script(boma, FIGHTER_STATUS_KIND_WAIT,0x1);
                    }
                }
            }
        }
        if (fighter_kind == FIGHTER_KIND_WOLF) {
            if (status_kind == FIGHTER_STATUS_KIND_SPECIAL_N) {
                if (MotionModule::frame(boma) >= 16) {
                    if (StatusModule::prev_situation_kind(boma) == SITUATION_KIND_AIR && situation_kind == SITUATION_KIND_GROUND) {
                        StatusModule::change_status_request_from_script(boma, FIGHTER_STATUS_KIND_WAIT,0x1);
                    }
                }
            }
        }
    }
}
int get_command_flag_cat_replace(u64 boma, int category) {
    int (*prev_replace)(u64, int) = (int (*)(u64, int)) prev_get_command_flag_cat;
    if (prev_replace)
        prev_replace(boma, category);

    u64 control_module = load_module(boma, 0x48);
    int (*get_command_flag_cat)(u64, int) = (int (*)(u64, int)) load_module_impl(control_module, 0x350);
    int cat = get_command_flag_cat(control_module, category);
    u64 status_kind = StatusModule::status_kind(boma);
    u8 BOcategory = (u8)(*(u32*)(boma + 8) >> 28);
    if (BOcategory == BATTLE_OBJECT_CATEGORY_FIGHTER) {
        if((int)MotionModule::frame(boma) != currframe){
            currframe = (int)MotionModule::frame(boma);
            GLOBAL_FRAME_COUNT += 1;
        }
    }
    float stick_y = ControlModule::get_stick_y(boma);
    u64 situation_kind = StatusModule::situation_kind(boma);
    float stick_x = ControlModule::get_stick_x(boma);
    // int kinetic_type = KineticModule::get_kinetic_type(boma);
    dacus(boma, cat, status_kind, stick_y);
    landingLagCancel(boma, cat, status_kind);
    jumpCancels(boma, cat, status_kind, situation_kind);
    jabCancels(boma, cat, status_kind, stick_x, stick_y);
    //jumpCancelGrab (boma, cat, status_kind, stick_y);
    //landCancels(boma, cat, status_kind, situation_kind);
    //otherCancels(boma, cat, status_kind, stick_x, stick_y);
    shield_drop(boma, cat, status_kind);
    glideTossing(boma, cat, status_kind, situation_kind, stick_y, stick_x);
    dashPlatformDrop(boma, status_kind, stick_y);
    shieldStopping(boma, cat, status_kind);
    perfectPivots(boma, status_kind, stick_x);
    ditcit(boma, cat, status_kind, stick_y);
    tauntCanceling(boma, status_kind);
    //fixDjs(boma, status_kind);
    //removeAirDodgeTumble(boma, status_kind, situation_kind);
    return cat;
}
bool ground_fix = true; //whether or not to use ground_correct_kind fix for calc's mods
//init_settings is called at the very beginning of a transition to a new status
u64 __init_settings(u64 boma, u64 situation_kind, int param_3, u64 param_4, u64 param_5,bool param_6,int param_7,int param_8,int param_9,int param_10) {
    // Call other function replacement code if this function has been replaced
    u64 (*prev_replace)(u64, u64, int, u64, u64, bool, int, int, int, int) = (u64 (*)(u64, u64, int, u64, u64, bool, int, int, int, int)) init_settings_prev;
    if (prev_replace){
        prev_replace(boma, situation_kind, param_3, param_4, param_5, param_6, param_7, param_8, param_9, param_10);
    }

  	u64 status_module = load_module(boma, 0x40);
  	u64 (*init_settings)(u64, u64, int, u64, u64, bool, int, int, int, int) =
	  (u64 (*)(u64, u64, int, u64, u64, bool, int, int, int, int))(load_module_impl(status_module, 0x1c8));
	u8 category = (u8)(*(u32*)(boma + 8) >> 28);
    int status_kind = StatusModule::status_kind(boma);

	if (category == BATTLE_OBJECT_CATEGORY_FIGHTER) {
        //ground_correct_kind fix (for calc's ecb mod/HDR)
        if(ground_fix){
            u64 fix = param_4;
            switch (status_kind) {
                case 0x0:
                case 0x3:
                case 0x6:
                case 0x7:
                case 0x11:
                case 0x12:
                case 0x13:
                case 0x14:
                case 0x15:
                case 0x16:
                case 0x17:
                case 0x18:
                case 0x19:
                case 0x1a:
                case 0x1b:
                case 0x1c:
                case 0x1e:
                case 0x22:
                case 0x23:
                case 0x7e:
                case 0x7f:
                        fix = 1;
                        break;
            }
            param_4 = fix;
        } 
        if(status_kind == FIGHTER_STATUS_KIND_ENTRY){ //reset variables on match start
            GLOBAL_FRAME_COUNT = 0;
            currframe = 0;
            for(int i = 0; i < 8; i++){
                lagcanceled[i] = false;
            }
        }

        //L-Cancel variable resets
        if(status_kind != FIGHTER_STATUS_KIND_ATTACK_AIR && status_kind != FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR){
            lagcanceled[get_player_number(boma)] = false;
        }
	}
  //ORIGINAL CALL
  return init_settings(status_module, situation_kind, param_3,
  					   param_4, param_5, param_6, param_7, param_8, param_9, param_10);
}
bool is_enable_transition_term_replace(u64 boma, int flag) {
    // Call other function replacement code if this function has been replaced
    bool (*prev_replace)(u64, int) = (bool (*)(u64, int)) prev_is_enable_transition_term;
    if (prev_replace){
        prev_replace(boma, flag);
    }
    // Continue with our current function replacement
    
    
    u64 work_module = load_module(boma, 0x50);
    bool(*is_enable_transition_term)(u64, int) = (bool(*)(u64, int)) (load_module_impl(work_module, 0x180));
    bool is_enable_transition_term_Original = is_enable_transition_term(work_module, flag);
    return is_enable_transition_term_Original;
}
void UniversalTech(){
    SaltySD_function_replace_sym_check_prev("_ZN3app8lua_bind40ControlModule__get_command_flag_cat_implEPNS_26BattleObjectModuleAccessorEi", (u64)&get_command_flag_cat_replace, prev_get_command_flag_cat);
    SaltySD_function_replace_sym_check_prev("_ZN3app8lua_bind32StatusModule__init_settings_implEPNS_26BattleObjectModuleAccessorENS_13SituationKindEijNS_20GroundCliffCheckKindEbiiii", (u64) &__init_settings, init_settings_prev);
    SaltySD_function_replace_sym_check_prev("_ZN3app8lua_bind42WorkModule__is_enable_transition_term_implEPNS_26BattleObjectModuleAccessorEi", (u64)&is_enable_transition_term_replace, prev_is_enable_transition_term);
    SaltySD_function_replace_sym("_ZN3app8lua_bind30GroundModule__entry_cliff_implEPNS_26BattleObjectModuleAccessorE",      (u64)&__entry_cliff);
    SaltySD_function_replace_sym("_ZN3app8lua_bind34GroundModule__can_entry_cliff_implEPNS_26BattleObjectModuleAccessorE",  (u64)&__can_entry_cliff);
    SaltySD_function_replace_sym("_ZN3app8lua_bind30GroundModule__leave_cliff_implEPNS_26BattleObjectModuleAccessorE",      (u64)&__leave_cliff);
    SaltySD_function_replace_sym("_ZN3app11FighterUtil30is_valid_just_shield_reflectorERNS_26BattleObjectModuleAccessorE", (u64) &__is_valid_just_shield_reflector);
    SaltySD_function_replace_sym("_ZN3app8lua_bind34KineticModule__change_kinetic_implEPNS_26BattleObjectModuleAccessorEi", (u64) &__change_kinetic);
    SaltySD_function_replace_sym("_ZN3app8lua_bind33MotionModule__add_motion_2nd_implEPNS_26BattleObjectModuleAccessorEN3phx6Hash40Effbf", (u64) &__add_motion_2nd);
    SaltySD_function_replace_sym("_ZN3app8lua_bind24GrabModule__is_grab_implEPNS_26BattleObjectModuleAccessorEi", (u64)__is_grab);
}