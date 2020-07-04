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
#include "UniversalTech.h"

using namespace app::lua_bind;
using namespace lib;

bool lcancelparams(u64& boma, u64& param_type, u64& param_hash){ 
    if(param_hash == 0){
        if(
            param_type == hash40("landing_attack_air_frame_n") ||
            param_type == hash40("landing_attack_air_frame_hi") ||
            param_type == hash40("landing_attack_air_frame_lw") ||
            param_type == hash40("landing_attack_air_frame_f") ||
            param_type == hash40("landing_attack_air_frame_b")
        ){
                return true;
        }
    }
    return false;
}

u64 get_param_float_prev = 0;
extern bool lagcanceled[8];
float get_param_float_replace(u64 boma, u64 param_type, u64 param_hash) { //weird for fighter_param's... check param_hash against 0 and param_type for "scale" or whatever param
    //prev replace
    float (*prev_replace)(u64, u64, u64) = (float (*)(u64, u64, u64)) get_param_float_prev;
    if (prev_replace)
        prev_replace(boma, param_type, param_hash);
    
    u64 work_module = load_module(boma, 0x50);
    float (*get_param_float)(u64, u64, u64) = (float (*)(u64, u64, u64)) load_module_impl(work_module, 0x240);
    u8 kind = (u8)(*(u32*)(boma + 8) >> 28);
    if (kind == BATTLE_OBJECT_CATEGORY_FIGHTER && lcancelparams(boma, param_type, param_hash)) {
        if (lagcanceled[get_player_number(boma)])
            return (int)(get_param_float(work_module, param_type, param_hash) / 1.5);
        else
            return get_param_float(work_module, param_type, param_hash);
    }
    return get_param_float(work_module, param_type, param_hash);
}
void get_param_replaces(){
    SaltySD_function_replace_sym_check_prev("_ZN3app8lua_bind32WorkModule__get_param_float_implEPNS_26BattleObjectModuleAccessorEmm", (u64)&get_param_float_replace, get_param_float_prev);
}