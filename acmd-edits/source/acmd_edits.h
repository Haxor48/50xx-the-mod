#include <switch_min.h>

#include <stdint.h>

#include "useful/const_value_table.h"
#include "useful/raygun_printer.h"
#include "useful/useful.h"

#include "acmd_wrapper.h"

using namespace lib;
using namespace app::sv_animcmd;
using namespace app::sv_math;
using namespace app::sv_kinetic_energy;
using namespace app::sv_battle_object;
using namespace app::sv_module_access;
using namespace app::FL_sv_module_access;
using namespace app::sv_system;
using namespace app::lua_bind;

ACMD acmd_objs[] = {
   
	//Falco
	ACMD("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_FALCO", "attack_lw3", "game_attacklw3", [](ACMD* acmd) ->void {

	}),
	ACMD("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_FALCO", "attack_air_lw", "game_attackairlw", [](ACMD* acmd) ->void {

	}),
	ACMD("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_FALCO", "attack_air_hi", "game_attackairhi", [](ACMD* acmd) ->void {

	}),
	ACMD("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_FALCO", "attack_air_b", "game_attackairb", [](ACMD* acmd) ->void {

	}),
	ACMD("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_FALCO", "landing_air_f", "game_landingairf", [](ACMD* acmd) ->void {

	}),
	//Falco 
	// Mario
    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_MARIO", "attack_hi_3", "game_attackhi3", [] (ACMD* acmd) ->void {
        acmd->frame(5);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("head"), /*Damage*/ 5.5, /*Angle*/ 96, /*KBG*/ 130, /*FKB*/ 0, /*BKB*/ 42, /*Size*/ 3.5, /*X*/ -0.5, /*Y*/ -0.8, /*Z*/ 0.2, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_PUNCH, /*Type*/ ATTACK_REGION_PUNCH);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("arml"), /*Damage*/ 5.5, /*Angle*/ 96, /*KBG*/ 130, /*FKB*/ 0, /*BKB*/ 42, /*Size*/ 4.2, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_PUNCH, /*Type*/ ATTACK_REGION_PUNCH);
            acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("arml"), /*Damage*/ 5.5, /*Angle*/ 96, /*KBG*/ 130, /*FKB*/ 0, /*BKB*/ 42, /*Size*/ 5.0, /*X*/ 4.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_PUNCH, /*Type*/ ATTACK_REGION_PUNCH);
        }
        acmd->wait(7);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
    }),

    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_MARIO", "attack_s_4_hi" "game_attacks4hi", [] (ACMD* acmd) -> void {
        acmd->frame(15);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("arml"), /*Damage*/ 14.7, /*Angle*/ 361, /*KBG*/ 110, /*FKB*/ 0, /*BKB*/ 33, /*Size*/ 2.0, /*X*/ -1.0, /*Y*/ 0.7, /*Z*/ 0.0, /*X2*/ -3.0, /*Y2*/ 1.0, /*Z2*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ hash40("false"), /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ hash40("false"), /*Absorbable*/ hash40("false"), /*Flinchless*/ hash40("false"), /*DisableHitlag*/ hash40("false"), /*Direct_Hitbox*/ hash40("true"), /*Ground_or_Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ hash40("false"), /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_PUNCH, /*Type*/ ATTACK_REGION_PUNCH);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("arml"), /*Damage*/ 17.799999, /*Angle*/ 361, /*KBG*/ 107, /*FKB*/ 0, /*BKB*/ 33, /*Size*/ 5.0, /*X*/ 5.4, /*Y*/ 0.0, /*Z*/ -1.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ hash40("false"), /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ hash40("false"), /*Absorbable*/ hash40("false"), /*Flinchless*/ hash40("false"), /*DisableHitlag*/ hash40("false"), /*Direct_Hitbox*/ hash40("true"), /*Ground_or_Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ hash40("false"), /*Effect*/ hash40("collision_attr_fire"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_FIRE, /*Type*/ ATTACK_REGION_PUNCH);
        }
        acmd->wait(3);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
    }),

    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_MARIO", "attack_s_4", "game_attacks4", [] (ACMD* acmd) -> void {
        acmd->frame(6);
        if (acmd->is_excute()) 
            WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        }
        acmd->frame(15);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("arml"), /*Damage*/ 14.7, /*Angle*/ 361, /*KBG*/ 110, /*FKB*/ 0, /*BKB*/ 33, /*Size*/ 2.0, /*X*/ -1.0, /*Y*/ 0.7, /*Z*/ 0.0, /*X2*/ -3.0, /*Y2*/ 1.0, /*Z2*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ hash40("false"), /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ hash40("false"), /*Absorbable*/ hash40("false"), /*Flinchless*/ hash40("false"), /*DisableHitlag*/ hash40("false"), /*Direct_Hitbox*/ hash40("true"), /*Ground_or_Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ hash40("false"), /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_PUNCH, /*Type*/ ATTACK_REGION_PUNCH);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("arml"), /*Damage*/ 17.799999, /*Angle*/ 361, /*KBG*/ 105, /*FKB*/ 0, /*BKB*/ 33, /*Size*/ 5.0, /*X*/ 5.4, /*Y*/ 0.0, /*Z*/ -1.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ hash40("false"), /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ hash40("false"), /*Absorbable*/ hash40("false"), /*Flinchless*/ hash40("false"), /*DisableHitlag*/ hash40("false"), /*Direct_Hitbox*/ hash40("true"), /*Ground_or_Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ hash40("false"), /*Effect*/ hash40("collision_attr_fire"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_FIRE, /*Type*/ ATTACK_REGION_PUNCH);
        }
        acmd->wait(3);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
    }),

    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_MARIO", "attack_s_4_lw", "game_attacks4lw", [] (ACMD* acmd) -> void {
        acmd->frame(6);
        if (acmd->is_excute()) 
            WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        }
        acmd->frame(15);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("arml"), /*Damage*/ 14.7, /*Angle*/ 361, /*KBG*/ 110, /*FKB*/ 0, /*BKB*/ 33, /*Size*/ 2.0, /*X*/ -1.0, /*Y*/ 0.7, /*Z*/ 0.0, /*X2*/ -3.0, /*Y2*/ 1.0, /*Z2*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ hash40("false"), /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ hash40("false"), /*Absorbable*/ hash40("false"), /*Flinchless*/ hash40("false"), /*DisableHitlag*/ hash40("false"), /*Direct_Hitbox*/ hash40("true"), /*Ground_or_Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ hash40("false"), /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_PUNCH, /*Type*/ ATTACK_REGION_PUNCH);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("arml"), /*Damage*/ 17.799999, /*Angle*/ 361, /*KBG*/ 103, /*FKB*/ 0, /*BKB*/ 33, /*Size*/ 5.0, /*X*/ 5.4, /*Y*/ 0.0, /*Z*/ -1.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ hash40("false"), /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ hash40("false"), /*Absorbable*/ hash40("false"), /*Flinchless*/ hash40("false"), /*DisableHitlag*/ hash40("false"), /*Direct_Hitbox*/ hash40("true"), /*Ground_or_Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ hash40("false"), /*Effect*/ hash40("collision_attr_fire"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_FIRE, /*Type*/ ATTACK_REGION_PUNCH);
        }
        acmd->wait(3);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
    }),

    ACMD("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_MARIO", "attack_lw_4", "game_attacklw4", [] (ACMD* acmd) -> void {
        acmd->frame(3);
        if (acmd->is_excute()) {
        WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        }
        acmd->frame(5);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 10.0, /*Angle*/ 45, /*KBG*/ 105, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 3.6, /*Z*/ 12.5, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ hash40("false"), /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ hash40("false"), /*Absorbable*/ hash40("false"), /*Flinchless*/ hash40("false"), /*DisableHitlag*/ hash40("false"), /*Direct_Hitbox*/ hash40("true"), /*Ground_or_Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ hash40("false"), /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 10.0, /*Angle*/ 45, /*KBG*/ 105, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.3, /*X*/ 0.0, /*Y*/ 3.6, /*Z*/ 7.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ hash40("false"), /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ hash40("false"), /*Absorbable*/ hash40("false"), /*Flinchless*/ hash40("false"), /*DisableHitlag*/ hash40("false"), /*Direct_Hitbox*/ hash40("true"), /*Ground_or_Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ hash40("false"), /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            AttackModule::set_attack_height_all(acmd->module_accessor, ATTACK_HEIGHT_LOW, hash40("false"));
        }
        acmd->wait(2);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
        acmd->frame(14);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 12.0, /*Angle*/ 42, /*KBG*/ 105, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 3.6, /*Z*/ -11.5, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ hash40("false"), /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ hash40("false"), /*Absorbable*/ hash40("false"), /*Flinchless*/ hash40("false"), /*DisableHitlag*/ hash40("false"), /*Direct_Hitbox*/ hash40("true"), /*Ground_or_Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ hash40("false"), /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 12.0, /*Angle*/ 42, /*KBG*/ 105, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.3, /*X*/ 0.0, /*Y*/ 3.6, /*Z*/ -6.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ hash40("false"), /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ hash40("false"), /*Absorbable*/ hash40("false"), /*Flinchless*/ hash40("false"), /*DisableHitlag*/ hash40("false"), /*Direct_Hitbox*/ hash40("true"), /*Ground_or_Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ hash40("false"), /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            AttackModule::set_attack_height_all(acmd->module_accessor, ATTACK_HEIGHT_LOW, hash40("false"));
        }
        acmd->wait(1);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
    }),

    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_MARIO", "attack_dash", "game_attackdash", [] (ACMD* acmd) -> void {
        acmd->frame(6);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 8.0, /*Angle*/ 37, /*KBG*/ 58, /*FKB*/ 0, /*BKB*/ 100, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 1.5, /*Z*/ 5.4, /*Hitlag*/ 1.25, /*SDI*/ 1.0, /*Clang_Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ hash40("false"), /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ hash40("false"), /*Absorbable*/ hash40("false"), /*Flinchless*/ hash40("false"), /*DisableHitlag*/ hash40("false"), /*Direct_Hitbox*/ hash40("true"), /*Ground_or_Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ hash40("false"), /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            acmd->wrap(ATK_SET_SHIELD_SETOFF_MUL, { /*ID*/ L2CValue(0), /*ShieldstunMul*/ L2CValue(1.875) });
            FighterAreaModuleImpl::enable_fix_jostle_area(acmd->module_accessor, 4, 6);
        }
        acmd->wait(4);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 6.0, /*Angle*/ 135, /*KBG*/ 37, /*FKB*/ 0, /*BKB*/ 87, /*Size*/ 2.7, /*X*/ 0.0, /*Y*/ 1.5, /*Z*/ 4.9, /*Hitlag*/ 1.25, /*SDI*/ 1.0, /*Clang_Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ hash40("false"), /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ hash40("false"), /*Absorbable*/ hash40("false"), /*Flinchless*/ hash40("false"), /*DisableHitlag*/ hash40("false"), /*Direct_Hitbox*/ hash40("true"), /*Ground_or_Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ hash40("false"), /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            acmd->wrap(ATK_SET_SHIELD_SETOFF_MUL, { /*ID*/ L2CValue(0), /*ShieldstunMul*/ L2CValue(1.875) });
        }
        acmd->wait(16);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
        acmd->frame(32);
        if (acmd->is_excute()) {
            FighterAreaModuleImpl::enable_fix_jostle_area(acmd->module_accessor, 4, 4);
        }
        acmd->frame(41);
        if (acmd->is_excute()) {
            FighterAreaModuleImpl::enable_fix_jostle_area(acmd->module_accessor, 3, 3);
        }
    }),

    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_MARIO", "attack_air_b", "game_attackairb", [] (ACMD* acmd) -> void {
        acmd->frame(6);
        if (acmd->is_excute()) {
            WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("kneer"), /*Damage*/ 10.5, /*Angle*/ 361, /*KBG*/ 113, /*FKB*/ 0, /*BKB*/ 22, /*Size*/ 4.5, /*X*/ 4.8, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_B, /*SetWeight*/ hash40("false"), /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ hash40("false"), /*Absorbable*/ hash40("false"), /*Flinchless*/ hash40("false"), /*DisableHitlag*/ hash40("false"), /*Direct_Hitbox*/ hash40("true"), /*Ground_or_Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ hash40("false"), /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("legr"), /*Damage*/ 10.5, /*Angle*/ 361, /*KBG*/ 113, /*FKB*/ 0, /*BKB*/ 22, /*Size*/ 5.9, /*X*/ 1.6, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_B, /*SetWeight*/ hash40("false"), /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ hash40("false"), /*Absorbable*/ hash40("false"), /*Flinchless*/ hash40("false"), /*DisableHitlag*/ hash40("false"), /*Direct_Hitbox*/ hash40("true"), /*Ground_or_Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ hash40("false"), /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
        }
        acmd->wait(2);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("kneer"), /*Damage*/ 7.0, /*Angle*/ 361, /*KBG*/ 95, /*FKB*/ 0, /*BKB*/ 13, /*Size*/ 4.5, /*X*/ 4.8, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_B, /*SetWeight*/ hash40("false"), /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ hash40("false"), /*Absorbable*/ hash40("false"), /*Flinchless*/ hash40("false"), /*DisableHitlag*/ hash40("false"), /*Direct_Hitbox*/ hash40("true"), /*Ground_or_Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ hash40("false"), /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("legr"), /*Damage*/ 7.0, /*Angle*/ 361, /*KBG*/ 95, /*FKB*/ 0, /*BKB*/ 13, /*Size*/ 5.3, /*X*/ 1.6, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_B, /*SetWeight*/ hash40("false"), /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ hash40("false"), /*Absorbable*/ hash40("false"), /*Flinchless*/ hash40("false"), /*DisableHitlag*/ hash40("false"), /*Direct_Hitbox*/ hash40("true"), /*Ground_or_Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ hash40("false"), /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
        }
        acmd->wait(3);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
        acmd->frame(19);
        if (acmd->is_excute()) {
            WorkModule::off_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    }),

    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_MARIO", "attack_air_lw", "game_attackairlw", [] (ACMD* acmd) ->void {
        acmd->frame(5);
        if (acmd->is_excute()) {
            WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        acmd->wrap(for, { L2CValue(5 Iterations) });
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 1.4, /*Angle*/ 94, /*KBG*/ 15, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ -0.5, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 0.8, /*Clang_Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ hash40("true"), /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ hash40("false"), /*Absorbable*/ hash40("false"), /*Flinchless*/ hash40("false"), /*DisableHitlag*/ hash40("false"), /*Direct_Hitbox*/ hash40("true"), /*Ground_or_Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ hash40("false"), /*Effect*/ hash40("collision_attr_rush"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_BODY);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 1.4, /*Angle*/ 94, /*KBG*/ 15, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 0.8, /*Clang_Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ hash40("true"), /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ hash40("false"), /*Absorbable*/ hash40("false"), /*Flinchless*/ hash40("false"), /*DisableHitlag*/ hash40("false"), /*Direct_Hitbox*/ hash40("true"), /*Ground_or_Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ hash40("false"), /*Effect*/ hash40("collision_attr_rush"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_BODY);
        }
        acmd->wait(1);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
        acmd->wait(1);
        }
        acmd->frame(23);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 5.5, /*Angle*/ 75, /*KBG*/ 105, /*FKB*/ 0, /*BKB*/ 95, /*Size*/ 11.0, /*X*/ 0.0, /*Y*/ 6.8, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 0.8, /*Clang_Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ hash40("false"), /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ hash40("false"), /*Absorbable*/ hash40("false"), /*Flinchless*/ hash40("false"), /*DisableHitlag*/ hash40("false"), /*Direct_Hitbox*/ hash40("true"), /*Ground_or_Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ hash40("false"), /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_BODY);
        }
        acmd->wait(1);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
        acmd->frame(33);
        if (acmd->is_excute()) {
            WorkModule::off_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    }),

    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_MARIO", "attack_lw_3", "game_attacklw3", [] (ACMD* acmd) -> void {
        acmd->frame(5);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("kneel"), /*Damage*/ 7.0, /*Angle*/ 105, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 3.2, /*X*/ -1.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ hash40("false"), /*ShieldDamage*/ 0, /*Trip*/ 0.4, /*Rehit*/ 0, /*Reflectable*/ hash40("false"), /*Absorbable*/ hash40("false"), /*Flinchless*/ hash40("false"), /*DisableHitlag*/ hash40("false"), /*Direct_Hitbox*/ hash40("true"), /*Ground_or_Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ hash40("false"), /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("toel"), /*Damage*/ 5.0, /*Angle*/ 105, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 4.2, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ hash40("false"), /*ShieldDamage*/ 0, /*Trip*/ 0.4, /*Rehit*/ 0, /*Reflectable*/ hash40("false"), /*Absorbable*/ hash40("false"), /*Flinchless*/ hash40("false"), /*DisableHitlag*/ hash40("false"), /*Direct_Hitbox*/ hash40("true"), /*Ground_or_Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ hash40("false"), /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            AttackModule::set_attack_height_all(acmd->module_accessor, ATTACK_HEIGHT_LOW, hash40("false"));
        }
        acmd->wait(3);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
    }),

    ACMD("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_MARIO", "attack_hi_4", "game_attackhi4", [] (ACMD* acmd)->void {
        acmd->frame(7);
        acmd->wrap(execute, { L2CValue(7) });
        if (acmd->is_excute()) {
            WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
            JostleModule::set_status(acmd->module_accessor, hash40("false"));
        }
        acmd->frame(9);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("ead"), /*Damage*/ 14.0, /*Angle*/ 83, /*KBG*/ 104, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 5.0, /*X*/ 2.5, /*Y*/ 1.1, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ hash40("false"), /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ hash40("false"), /*Absorbable*/ hash40("false"), /*Flinchless*/ hash40("false"), /*DisableHitlag*/ hash40("false"), /*Direct_Hitbox*/ hash40("true"), /*Ground_or_Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ hash40("false"), /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_PUNCH, /*Type*/ ATTACK_REGION_HEAD);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("bust"), /*Damage*/ 14.0, /*Angle*/ 83, /*KBG*/ 104, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 4.0, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ hash40("false"), /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ hash40("false"), /*Absorbable*/ hash40("false"), /*Flinchless*/ hash40("false"), /*DisableHitlag*/ hash40("false"), /*Direct_Hitbox*/ hash40("true"), /*Ground_or_Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ hash40("false"), /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_PUNCH, /*Type*/ ATTACK_REGION_HEAD);
            acmd->wrap(HIT_NODE, { L2CValue(hash40("head")) });
        }
        acmd->wait(4);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
            HitModule::set_status_all(acmd->module_accessor, HIT_STATUS_NORMAL);
        }
        acmd->frame(20);
        if (acmd->is_excute()) {
            JostleModule::set_status(acmd->module_accessor, hash40("true"));
        }
    }),

    ACMD("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_MARIO", "attack_air_f", "game_attackairf", [] (ACMD* acmd) -> void {
        acmd->frame(3);
        if (acmd->is_excute()) {
            WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        acmd->frame(16);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("arml"), /*Damage*/ 12.0, /*Angle*/ 361, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 3.0, /*X*/ 3.2, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ hash40("false"), /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ hash40("false"), /*Absorbable*/ hash40("false"), /*Flinchless*/ hash40("false"), /*DisableHitlag*/ hash40("false"), /*Direct_Hitbox*/ hash40("true"), /*Ground_or_Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ hash40("false"), /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_PUNCH, /*Type*/ ATTACK_REGION_PUNCH);
        }
        acmd->wait(1);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("shoulderl"), /*Damage*/ 14.0, /*Angle*/ 280, /*KBG*/ 78, /*FKB*/ 0, /*BKB*/ 32, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ hash40("false"), /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ hash40("false"), /*Absorbable*/ hash40("false"), /*Flinchless*/ hash40("false"), /*DisableHitlag*/ hash40("false"), /*Direct_Hitbox*/ hash40("true"), /*Ground_or_Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ hash40("false"), /*Effect*/ hash40("collision_attr_fire"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_PUNCH, /*Type*/ ATTACK_REGION_PUNCH);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("arml"), /*Damage*/ 14.0, /*Angle*/ 280, /*KBG*/ 78, /*FKB*/ 0, /*BKB*/ 32, /*Size*/ 4.6, /*X*/ 3.4, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ hash40("false"), /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ hash40("false"), /*Absorbable*/ hash40("false"), /*Flinchless*/ hash40("false"), /*DisableHitlag*/ hash40("false"), /*Direct_Hitbox*/ hash40("true"), /*Ground_or_Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ hash40("false"), /*Effect*/ hash40("collision_attr_fire"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_PUNCH, /*Type*/ ATTACK_REGION_PUNCH);
        }
        acmd->wait(4);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("shoulderl"), /*Damage*/ 10.0, /*Angle*/ 361, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ hash40("false"), /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ hash40("false"), /*Absorbable*/ hash40("false"), /*Flinchless*/ hash40("false"), /*DisableHitlag*/ hash40("false"), /*Direct_Hitbox*/ hash40("true"), /*Ground_or_Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ hash40("false"), /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_PUNCH, /*Type*/ ATTACK_REGION_PUNCH);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("arml"), /*Damage*/ 10.0, /*Angle*/ 361, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 4.6, /*X*/ 3.2, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ hash40("false"), /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ hash40("false"), /*Absorbable*/ hash40("false"), /*Flinchless*/ hash40("false"), /*DisableHitlag*/ hash40("false"), /*Direct_Hitbox*/ hash40("true"), /*Ground_or_Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ hash40("false"), /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_PUNCH, /*Type*/ ATTACK_REGION_PUNCH);
        }
        acmd->wait(1);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
        acmd->frame(43);
        if (acmd->is_excute()) {
            WorkModule::off_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    }),
	// Marth
    ACMD("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_MARTH", "attack_air_lw", "game_attackairlw", [] (ACMD* acmd)->void {
        acmd->frame(3);
        if (acmd->is_excute()) {
            WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        acmd->frame(9);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("armr"), /*Damage*/ 9.0, /*Angle*/ 361, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 1.0, /*Z*/ 0.0, /*Hitlag*/ 0.7, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("sword1"), /*Damage*/ 9.0, /*Angle*/ 80, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 3.5, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 2.0, /*Hitlag*/ 0.7, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 3, /*Part*/ 0, /*Bone*/ hash40("sword1"), /*Damage*/ 10.0, /*Angle*/ 270, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 3.5, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 6.7, /*Hitlag*/ 1.25, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_MARTH_SWORD, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 13.0, /*Angle*/ 270, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ -3.3, /*Z*/ -3.0, /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_MARTH_SWORD, /*Type*/ ATTACK_REGION_SWORD);
        }
        acmd->frame(14);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
        acmd->frame(55);
        if (acmd->is_excute()) {
            WorkModule::off_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    }),
    ACMD("BATTLE_OBJECT_CATEGORY_PLAYER", "PLAYER_KIND_MARTH", "attack_air_f", "game_attackairf", [] (ACMD* acmd)->void {
        if (acmd->is_excute()) {
            WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        acmd->frame(6);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("sword1"), /*Damage*/ 10.0, /*Angle*/ 80, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 3.0, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 3.5, /*Hitlag*/ 0.7, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("armr"), /*Damage*/ 9.0, /*Angle*/ 80, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 3.8, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 0.7, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("sword1"), /*Damage*/ 13.0, /*Angle*/ 80, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 42, /*Size*/ 3.0, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 7.5, /*Hitlag*/ 1.25, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_MARTH_SWORD, /*Type*/ ATTACK_REGION_SWORD);
            AttackModule::set_add_reaction_frame(acmd->module_accessor, 0, 7, false);
            AttackModule::set_add_reaction_frame(acmd->module_accessor, 1, 7, false);
            AttackModule::set_add_reaction_frame(acmd->module_accessor, 2, 7, false);
        }
        acmd->wait(3);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
        acmd->frame(27);
        if (acmd->is_excute()) {
            WorkModule::off_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    }),
    ACMD ("BATTLE_OBJECT_CATEGORY_PLAYER", "PLAYER_KIND_MARTH", "attack_air_hi", "game_attackairhi", [] (ACMD* acmd)->void {
        acmd->frame(3);
        if (acmd->is_excute()) {
            WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        acmd->frame(5);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("sword1"), /*Damage*/ 10, /*Angle*/ 80, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 3.5, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 2.0, /*Hitlag*/ 0.7, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("armr"), /*Damage*/ 9, /*Angle*/ 80, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 1.0, /*Z*/ 0.0, /*Hitlag*/ 0.7, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("colonells"), /*Damage*/ 9, /*Angle*/ 80, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 18, /*Size*/ 2.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 0.7, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 3, /*Part*/ 0, /*Bone*/ hash40("sword1"), /*Damage*/ 13, /*Angle*/ 90, /*KBG*/ 84, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.5, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 6.7, /*Hitlag*/ 1.25, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_MARTH_SWORD, /*Type*/ ATTACK_REGION_SWORD);
        }
        acmd->frame(10);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
        acmd->frame(38);
        if (acmd->is_excute()) {
            WorkModule::off_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    }),
    // Meta Knight
    ACMD("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_METAKNIGHT", "attack_dash", "game_attackdash", [] (ACMD* acmd)->void {
        acmd->frame(7);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("footl"), /*Damage*/ 7.0, /*Angle*/ 70, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ -1.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("footl"), /*Damage*/ 7.0, /*Angle*/ 75, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ -6.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("footl"), /*Damage*/ 6.0, /*Angle*/ 80, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ -9.3, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            AttackModule::set_add_reaction_frame(acmd->module_accessor, 0, 5, false);
            AttackModule::set_add_reaction_frame(acmd->module_accessor, 1, 5, false);
            AttackModule::set_add_reaction_frame(acmd->module_accessor, 2, 5, false);
        }
        acmd->frame(12);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
    }),
    ACMD("BATTLE_OBJECT_CATEGORY_FIGHTER","FIGHTER_KIND_METAKNIGHT","attack_air_lw", "game_attackairlw", [] (ACMD* acmd)->void  {
        acmd->frame(4);
        if (acmd->is_excute()) {
            WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 7.0, /*Angle*/ 30, /*KBG*/ 105, /*FKB*/ 0, /*BKB*/ 15, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ -6.5, /*Z*/ 2.5, /*X2*/ 0.0, /*Y2*/ -6.5, /*Z2*/ -4.5, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 9.0, /*Angle*/ 23, /*KBG*/ 105, /*FKB*/ 0, /*BKB*/ 15, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ -6.5, /*Z*/ 3.5, /*X2*/ 0.0, /*Y2*/ -4.5, /*Z2*/ 6.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 9.0, /*Angle*/ 23, /*KBG*/ 105, /*FKB*/ 0, /*BKB*/ 15, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ -6.5, /*Z*/ -5.5, /*X2*/ 0.0, /*Y2*/ -4.5, /*Z2*/ -8.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
            AttackModule::set_add_reaction_frame(acmd->module_accessor, 0, 5, false);
            AttackModule::set_add_reaction_frame(acmd->module_accessor, 1, 5, false);
            AttackModule::set_add_reaction_frame(acmd->module_accessor, 2, 5, false);
        }
        acmd->wait(2);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
        acmd->frame(25);
        if (acmd->is_excute()) {
            WorkModule::off_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    }),
    ACMD("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_METAKNIGHT", "attack_lw4", "game_attacklw4", [] (ACMD* acmd)->void {
        acmd->frame(2);
        if (acmd->is_excute()) {
            WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        }
        acmd->frame(4);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 11.0, /*Angle*/ 361, /*KBG*/ 93, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 3.8, /*X*/ 0.0, /*Y*/ 4.0, /*Z*/ 11.0, /*X2*/ 0.0, /*Y2*/ 9.8, /*Z2*/ 19.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
        }
        acmd->wait(1);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
        acmd->frame(9);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 13.0, /*Angle*/ 361, /*KBG*/ 93, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 4.6, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ -11.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 13.0, /*Angle*/ 361, /*KBG*/ 93, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 4.6, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ -17.5, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
        }
        acmd->wait(1);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
    }),
    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_METAKNIGHT", "special_n_spin", "game_specialnspin", [] (ACMD* acmd)->void {
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 1.0, /*Angle*/ 366, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 10, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 6, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_BODY);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 1, /*Bone*/ hash40("top"), /*Damage*/ 1.0, /*Angle*/ 366, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 10, /*Size*/ 15.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 0.0, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 6, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ true, /*DisableHitlag*/ true, /*Direct/Indirect*/ false, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_NO_ITEM, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_NONE, /*Type*/ ATTACK_REGION_NONE);
        }
        acmd->frame(104);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
    }),
    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_METAKNIGHT", "attack_air_hi", "game_attackairhi", [](ACMD* acmd)->void {
        if (acmd->is_excute()) {
            MotionModule::set_rate(acmd->module_accessor, 2);
        }
        acmd->frame(2);
        if (acmd->is_excute()) {
            WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 4.0, /*Angle*/ 67, /*KBG*/ 125, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 19.0, /*Z*/ 3.5, /*X2*/ 0.0, /*Y2*/ 19.0, /*Z2*/ -5.5, /*Hitlag*/ 1.0, /*SDI*/ 1.5, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 4.0, /*Angle*/ 67, /*KBG*/ 125, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 19.0, /*Z*/ 4.5, /*X2*/ 0.0, /*Y2*/ 16.0, /*Z2*/ 9.5, /*Hitlag*/ 1.0, /*SDI*/ 1.5, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 4.0, /*Angle*/ 50, /*KBG*/ 125, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 19.0, /*Z*/ -6.5, /*X2*/ 0.0, /*Y2*/ 16.0, /*Z2*/ -11.5, /*Hitlag*/ 1.0, /*SDI*/ 1.5, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
        }
        acmd->wait(2);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
        acmd->frame(20);
        if (acmd->is_excute()) {
            WorkModule::off_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    }),
    ACMD("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_METAKNIGHT", "special_hi_loop", "game_specialhiloop", [] (ACMD* acmd)->void {
        if (acmd->is_excute()) {
            WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 6.0, /*Angle*/ 88, /*KBG*/ 120, /*FKB*/ 116, /*BKB*/ 0, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 2.0, /*Z*/ 12.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 6.0, /*Angle*/ 86, /*KBG*/ 120, /*FKB*/ 110, /*BKB*/ 0, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 12.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
        }
        acmd->frame(2);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 6.0, /*Angle*/ 86, /*KBG*/ 100, /*FKB*/ 80, /*BKB*/ 0, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 2.0, /*Z*/ 12.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 6.0, /*Angle*/ 86, /*KBG*/ 100, /*FKB*/ 80, /*BKB*/ 0, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 12.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
        }
        acmd->frame(5);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
            GroundModule::set_passable_check(acmd->module_accessor, true);
        }
        acmd->frame(14);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("handr"), /*Damage*/ 6.0, /*Angle*/ 80, /*KBG*/ 128, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 5.5, /*X*/ 2.0, /*Y*/ 0.0, /*Z*/ 9.0, /*X2*/ 2.0, /*Y2*/ 0.0, /*Z2*/ 4.0, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("bust"), /*Damage*/ 6.0, /*Angle*/ 80, /*KBG*/ 128, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_BODY);
            acmd->wrap(notify_event_msc_cmd, { L2CValue(hash40("0x2127e37c07")), L2CValue(GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES) });
        }
        acmd->frame(21);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
            GroundModule::set_passable_check(acmd->module_accessor, false);
        }
        acmd->frame(41);
        if (acmd->is_excute()) {
            WorkModule::off_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        }
    }),
    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_METAKNIGHT", "special_n_end", "game_specialnend", [] (ACMD* acmd)->void {
        acmd->frame(1);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 3.0, /*Angle*/ 70, /*KBG*/ 170, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 0.0, /*Hitlag*/ 2.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_BODY);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 1, /*Bone*/ hash40("top"), /*Damage*/ 3.0, /*Angle*/ 70, /*KBG*/ 170, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 15.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 0.0, /*Hitlag*/ 2.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ true, /*DisableHitlag*/ true, /*Direct/Indirect*/ false, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_NO_ITEM, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_NONE, /*Type*/ ATTACK_REGION_NONE);
        }
        acmd->wait(3);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
        acmd->wrap(FT_MOTION_RATE, { /*FSM*/ L2CValue(1.2) });
        acmd->frame(21);
        acmd->wrap(FT_MOTION_RATE, { /*FSM*/ L2CValue(1) });
    }),
    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_METAKNIGHT", "special_air_n_end", "game_specialairnend", [] (ACMD* acmd)->void {
        acmd->frame(1);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 3.0, /*Angle*/ 70, /*KBG*/ 170, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 0.0, /*Hitlag*/ 2.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_BODY);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 1, /*Bone*/ hash40("top"), /*Damage*/ 3.0, /*Angle*/ 70, /*KBG*/ 170, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 15.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 0.0, /*Hitlag*/ 2.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ true, /*DisableHitlag*/ true, /*Direct/Indirect*/ false, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_NO_ITEM, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_NONE, /*Type*/ ATTACK_REGION_NONE);
        }
        acmd->wait(3);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
        acmd->wrap(FT_MOTION_RATE, { /*FSM*/ L2CValue(1.2) });
        acmd->frame(21);
        acmd->wrap(FT_MOTION_RATE, { /*FSM*/ L2CValue(1) });
    })
};