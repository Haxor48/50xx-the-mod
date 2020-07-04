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
    return ((int (*)(u64))(load_module_impl(ground_module, 0x248)))(ground_module);
}

u64 __can_entry_cliff(u64 boma, u64 &status_kind) {
    // Get player pos and check if another player is 
    // occupying a ledge within our grab range
    Vector3f pos = *(Vector3f*)GroundModule::hang_cliff_pos_3f(boma);
    int entry_id = WorkModule::get_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    if (status_kind != FIGHTER_STATUS_KIND_SPECIAL_HI && status_kind != FIGHTER_STATUS_KIND_FALL_SPECIAL) {
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
    return ((int (*)(u64))(load_module_impl(ground_module, 0x258)))(ground_module);
}

u64 __leave_cliff(u64 boma) {
    // clear occupancy when leaving edge
    int entry_id = WorkModule::get_int(boma , FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    ledge_pos[entry_id] = Vector3f { .x =0, .y=0, .z=0 };

    // Call base func
    u64 ground_module = load_module(boma, 0x58);    
    return ((int (*)(u64))(load_module_impl(ground_module, 0x270)))(ground_module);
}
u64 __is_grab(u64 boma, int unk1) {
    print_string(boma, "is_grab %llx", unk1);

    u64 grab_module = load_module(boma, 0x158);
    return ((int (*)(u64, int))(load_module_impl(grab_module, 0x78)))(grab_module, unk1);
}

void jabCancels (u64 &boma, int &cat1, u64 &status_kind, float stick_value_x, float stick_value_y) {
    int fighter_kind = app::utility::get_kind(boma);
    if (status_kind == FIGHTER_STATUS_KIND_ATTACK) {
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
    if (status_kind == FIGHTER_STATUS_KIND_ATTACK_S3) {
        if (fighter_kind == FIGHTER_KIND_METAKNIGHT || fighter_kind == FIGHTER_KIND_PACKUN) {
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

void jumpCancelGrab (u64 &boma, int &cat1, u64 &status_kind, float stick_value_y) {
    int fighter_kind = app::utility::get_kind(boma);
    if (status_kind == FIGHTER_STATUS_KIND_JUMP_SQUAT) {
        if (stick_value_y > -0.66) {
            if (cat1 & FIGHTER_PAD_CMD_CAT1_FLAG_CATCH) {
                (u64)StatusModule::change_status_request_from_script(boma, (u64)FIGHTER_STATUS_KIND_CATCH,0x1);
            }
        }
        if (fighter_kind == FIGHTER_KIND_PITB) {
            if (cat1 & FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) {
                (u64)StatusModule::change_status_request_from_script(boma, (u64)FIGHTER_STATUS_KIND_SPECIAL_S,0x1);
            }
        }
    }
}
void otherCancels (u64 &boma, int &cat1, u64 &status_kind, float stick_value_x, float stick_value_y) {
    int fighter_kind = app::utility::get_kind(boma);
    if (fighter_kind == FIGHTER_KIND_METAKNIGHT) {
        if (status_kind == FIGHTER_STATUS_KIND_ATTACK) {
            if (cat1 & FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) {
                (u64)StatusModule::change_status_request_from_script(boma, (u64)FIGHTER_STATUS_KIND_ATTACK_LW3,0x1);
            }
            if (cat1 & FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) {
                (u64)StatusModule::change_status_request_from_script(boma, (u64)FIGHTER_STATUS_KIND_ATTACK_LW4,0x1);
            }
        }
    }
}

void glideTossing (u64 &boma, int &cat1, u64 &status_kind, u64 &situation_kind, float stick_value_y, float stick_value_x) {
    if ((status_kind == FIGHTER_STATUS_KIND_ESCAPE_B || status_kind == FIGHTER_STATUS_KIND_ESCAPE_F) && situation_kind == SITUATION_KIND_GROUND) {
        if (MotionModule::frame(boma) < (7)) {
            if ((stick_value_x >= 0.6 && ((cat1 & FIGHTER_PAD_CMD_CAT1_FLAG_CATCH) || (cat1 & FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) || (cat1 & FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4))) {
                (u64)StatusModule::change_status_request_from_script(boma, (u64)FIGHTER_STATUS_KIND_ITEM_THROW,0x1);
            }
            if ((stick_value_y >= 0.6 && ((cat1 & FIGHTER_PAD_CMD_CAT1_FLAG_CATCH) || (cat1 & FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) || (cat1 & FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4))) {
                (u64)StatusModule::change_status_request_from_script(boma, (u64)FIGHTER_STATUS_KIND_ITEM_THROW,0x1);
            }
            if ((stick_value_x <= -0.6 && ((cat1 & FIGHTER_PAD_CMD_CAT1_FLAG_CATCH) || (cat1 & FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) || (cat1 & FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4))) {
                (u64)StatusModule::change_status_request_from_script(boma, (u64)FIGHTER_STATUS_KIND_ITEM_THROW,0x1);
            }
            if ((stick_value_y <= -0.6 && ((cat1 & FIGHTER_PAD_CMD_CAT1_FLAG_CATCH) || (cat1 & FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) || (cat1 & FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4))) {
                (u64)StatusModule::change_status_request_from_script(boma, (u64)FIGHTER_STATUS_KIND_ITEM_THROW,0x1);
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
        if (AttackModule::is_infliction_status(boma, COLLISION_KIND_MASK_HIT)) {
            if (status_kind == FIGHTER_STATUS_KIND_ATTACK_AIR) {
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
        if (AttackModule::is_infliction_status(boma, COLLISION_KIND_MASK_HIT)) {
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
        if (fighter_kind == FIGHTER_KIND_FALCO || fighter_kind == FIGHTER_KIND_FOX || fighter_kind == FIGHTER_KIND_WOLF) {
            if (status_kind == FIGHTER_STATUS_KIND_SPECIAL_N) {
                if (StatusModule::prev_situation_kind(boma) == SITUATION_KIND_AIR && situation_kind == SITUATION_KIND_GROUND) {
					StatusModule::change_status_request_from_script(boma, FIGHTER_STATUS_KIND_WAIT,0x1);
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
    dacus(boma, cat, status_kind, stick_y);
    landingLagCancel(boma, cat, status_kind);
    jumpCancels(boma, cat, status_kind, situation_kind);
    jabCancels(boma, cat, status_kind, stick_x, stick_y);
    jumpCancelGrab (boma, cat, status_kind, stick_y);
    landCancels(boma, cat, status_kind, situation_kind);
    otherCancels(boma, cat, status_kind, stick_x, stick_y);
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
    SaltySD_function_replace_sym("_ZN3app8lua_bind24GrabModule__is_grab_implEPNS_26BattleObjectModuleAccessorEi", (u64)__is_grab);
}