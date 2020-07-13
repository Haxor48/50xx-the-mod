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
//Fox
    ACMD("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_FOX", "attack_air_lw", "game_attackairlw", [](ACMD* acmd) ->void {
        acmd->frame(5);
        if (acmd->is_excute()) {

        WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        for (uint32_t i = 0; i < 6; i++) {
               if (acmd->is_excute()) {

        acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 1.4, /*Angle*/ 250, /*KBG*/ 100, /*FKB*/ 5, /*BKB*/ 0, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 8.2, /*Z*/ -0.5, /*X2*/ 0.0, /*Y2*/ 9.0, /*Z2*/ 2.0, /*Hitlag*/ 0.66, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_rush"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);

        acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("toel"), /*Damage*/ 1.4, /*Angle*/ 110, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 38, /*Size*/ 3.2, /*X*/ -0.5, /*Y*/ -0.5, /*Z*/ 0.0, /*X2*/ -0.5, /*Y2*/ -0.5, /*Z2*/ 0.0, /*Hitlag*/ 0.66, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_rush"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
        }
        }    
        acmd->wait(2);
        if (acmd->is_excute()) {

        AttackModule::clear_all(acmd->module_accessor);
        }
        acmd->wait(1);
        if (acmd->is_excute()) {

        acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 3.0, /*Angle*/ 60, /*KBG*/ 140, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ -1.0, /*Hitlag*/ 2.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);

        acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 3.0, /*Angle*/ 60, /*KBG*/ 140, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 2.8, /*Z*/ 3.0, /*Hitlag*/ 2.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
        }
        acmd->wait(1);
        if (acmd->is_excute()) {

        AttackModule::clear_all(acmd->module_accessor);
        }
        acmd->frame(28);
        if (acmd->is_excute()) {

        WorkModule::off_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }   
    }),
    ACMD("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_FOX", "landing_air_lw", "game_landingairlw", [](ACMD* acmd) ->void {

    }),
    ACMD("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_FOX", "attack_hi4", "game_attackhi4", [](ACMD* acmd) ->void {
    acmd->frame(3);
    if (acmd->is_excute()) {

    WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    acmd->frame(8);
    if (acmd->is_excute()) {

    acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("kneer"), /*Damage*/ 16.0, /*Angle*/ 80, /*KBG*/ 112, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 4.1, /*X*/ 0.0, /*Y*/ 1.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);

    acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("kneer"), /*Damage*/ 16.0, /*Angle*/ 80, /*KBG*/ 112, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 5.7, /*X*/ 3.0, /*Y*/ 2.1, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);

    acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 16.0, /*Angle*/ 80, /*KBG*/ 112, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 2.0, /*X*/ 0.0, /*Y*/ 3.0, /*Z*/ 10.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
    }
    acmd->wait(1);
    if (acmd->is_excute()) {

    AttackModule::clear(acmd->module_accessor, /*ID*/ 2, false);
    }
    acmd->wait(1);
    if (acmd->is_excute()) {

    acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("kneer"), /*Damage*/ 11.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 10, /*Size*/ 4.1, /*X*/ 0.0, /*Y*/ 1.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);

    acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("kneer"), /*Damage*/ 11.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 10, /*Size*/ 4.7, /*X*/ 2.9, /*Y*/ 1.4, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
    }
    acmd->wait(2);
    if (acmd->is_excute()) {

    AttackModule::clear_all(acmd->module_accessor);
    }
    }),
    ACMD("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_FOX", "attack_lw3", "game_attacklw3", [](ACMD* acmd) ->void {
    acmd->frame(6);
    if (acmd->is_excute()) {

    acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("tail1"), /*Damage*/ 8.0, /*Angle*/ 70, /*KBG*/ 125, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 3.2, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_FOX_TAIL, /*Type*/ ATTACK_REGION_TAIL);

    acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("tail2"), /*Damage*/ 8.0, /*Angle*/ 80, /*KBG*/ 125, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 2.8, /*X*/ 1.5, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_FOX_TAIL, /*Type*/ ATTACK_REGION_TAIL);

    acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("tail2"), /*Damage*/ 7.0, /*Angle*/ 90, /*KBG*/ 125, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 3.0, /*X*/ 6.0, /*Y*/ 1.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_FOX_TAIL, /*Type*/ ATTACK_REGION_TAIL);

    AttackModule::set_attack_height_all(acmd->module_accessor, ATTACK_HEIGHT_LOW, false);

    AttackModule::set_add_reaction_frame(acmd->module_accessor, /*ID*/ 0, /*Frames*/ 3, /*Unk*/ false);

    AttackModule::set_add_reaction_frame(acmd->module_accessor, /*ID*/ 1, /*Frames*/ 3, /*Unk*/ false);

    AttackModule::set_add_reaction_frame(acmd->module_accessor, /*ID*/ 2, /*Frames*/ 3, /*Unk*/ false);
    }
    acmd->wait(2);
    if (acmd->is_excute()) {

    AttackModule::clear_all(acmd->module_accessor);
    }
    }),
    ACMD("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_FOX", "special_lw_start", "game_speciallwstart", [](ACMD* acmd) ->void {
    acmd->frame(2);
    if (acmd->is_excute()) {

    acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 5.0, /*Angle*/ 360, /*KBG*/ 100, /*FKB*/ 80, /*BKB*/ 0, /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 6.5, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_G, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_elec"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_ELEC, /*Type*/ ATTACK_REGION_ENERGY);

    acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 2.0, /*Angle*/ 24, /*KBG*/ 45, /*FKB*/ 0, /*BKB*/ 66, /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 6.5, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_A, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_elec"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_ELEC, /*Type*/ ATTACK_REGION_ENERGY);
    }
    }),
    ACMD("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_FOX", "special_air_lw_start", "game_specialairlwstart", [](ACMD* acmd) ->void {
    acmd->frame(2);
    if (acmd->is_excute()) {

    acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 5.0, /*Angle*/ 360, /*KBG*/ 100, /*FKB*/ 80, /*BKB*/ 0, /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 6.5, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_G, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_elec"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_ELEC, /*Type*/ ATTACK_REGION_ENERGY);

    acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 2.0, /*Angle*/ 24, /*KBG*/ 45, /*FKB*/ 0, /*BKB*/ 66, /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 6.5, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_A, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_elec"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_ELEC, /*Type*/ ATTACK_REGION_ENERGY);
    }
    }),
    ACMD("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_FOX", "attack_dash", "game_attackdash", [](ACMD* acmd) ->void {
    acmd->frame(4);
    if (acmd->is_excute()) {

    acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("kneel"), /*Damage*/ 6.0, /*Angle*/ 70, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 4.7, /*X*/ 4.4, /*Y*/ -0.7, /*Z*/ 0.0, /*Hitlag*/ 1.15, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);

    acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("kneel"), /*Damage*/ 6.0, /*Angle*/ 70, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 4.7, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.15, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);

    acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("hip"), /*Damage*/ 6.0, /*Angle*/ 70, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.15, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);

    acmd->wrap(ATK_SET_SHIELD_SETOFF_MUL_arg4, { /*ID1*/ L2CValue(0), /*ID2*/ L2CValue(1), /*ID3*/ L2CValue(2), /*ShieldstunMul*/ L2CValue(1.8) });
    }
    acmd->wait(4);
    if (acmd->is_excute()) {

    acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("kneel"), /*Damage*/ 4.0, /*Angle*/ 70, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.8, /*X*/ 3.8, /*Y*/ -0.6, /*Z*/ 0.0, /*Hitlag*/ 1.15, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);

    acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("kneel"), /*Damage*/ 4.0, /*Angle*/ 70, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.8, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.15, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);

    acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("hip"), /*Damage*/ 4.0, /*Angle*/ 70, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.15, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);

    acmd->wrap(ATK_SET_SHIELD_SETOFF_MUL_arg4, { /*ID1*/ L2CValue(0), /*ID2*/ L2CValue(1), /*ID3*/ L2CValue(2), /*ShieldstunMul*/ L2CValue(1.8) });
    }
    acmd->wait(8);
    if (acmd->is_excute()) {

    AttackModule::clear_all(acmd->module_accessor);
    }
    }),
//Fox
//Falco
	ACMD("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_FALCO", "attack_air_lw", "game_attackairlw", [](ACMD* acmd) ->void {
	if (acmd->is_excute()) {
		MotionModule::set_rate(acmd->module_accessor, 2);
	}
	acmd->frame(4);
	if (acmd->is_excute()) {

		WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
    	acmd->frame(9);
	if (acmd->is_excute()) {
        MotionModule::set_rate(acmd->module_accessor, 1);
	}
	acmd->frame(10);
	if (acmd->is_excute()) {

		acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("kneel"), /*Damage*/ 13.0, /*Angle*/ 290, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 10, /*Size*/ 4.2, /*X*/ 4.2, /*Y*/ 0.0, /*Z*/ -1.0, /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
	}
	acmd->frame(15);
	if (acmd->is_excute()) {

		acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("kneel"), /*Damage*/ 8.0, /*Angle*/ 285, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 5.3, /*X*/ 3.5, /*Y*/ 0.0, /*Z*/ -1.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);

		AttackModule::clear(acmd->module_accessor, /*ID*/ 1, false);
	}
	acmd->frame(27);
	if (acmd->is_excute()) {

		AttackModule::clear_all(acmd->module_accessor);
	}
	acmd->frame(30);
	if (acmd->is_excute()) {

		WorkModule::off_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	}),
	ACMD("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_FALCO", "attack_air_hi", "game_attackairhi", [](ACMD* acmd) ->void {
	acmd->frame(1);
	if (acmd->is_excute()) {
        MotionModule::set_rate(acmd->module_accessor, 1.4925);
	}
	acmd->frame(4);
	if (acmd->is_excute()) {

		WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	acmd->frame(9);
	if (acmd->is_excute()) {
         MotionModule::set_rate(acmd->module_accessor, 1);
		acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("kneer"), /*Damage*/ 9.0, /*Angle*/ 50, /*KBG*/ 95, /*FKB*/ 0, /*BKB*/ 38, /*Size*/ 5.0, /*X*/ 6.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);

		acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("kneer"), /*Damage*/ 9.0, /*Angle*/ 60, /*KBG*/ 95, /*FKB*/ 0, /*BKB*/ 38, /*Size*/ 4.2, /*X*/ 0.5, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);

		acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("hip"), /*Damage*/ 9.0, /*Angle*/ 70, /*KBG*/ 95, /*FKB*/ 0, /*BKB*/ 38, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
	}
	acmd->wait(5);
	if (acmd->is_excute()) {

		AttackModule::clear_all(acmd->module_accessor);
	}
	acmd->wait(11);
	if (acmd->is_excute()) {

		WorkModule::off_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	}),
	ACMD("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_FALCO", "attack_air_b", "game_attackairb", [](ACMD* acmd) ->void {
	if (acmd->is_excute()) {
		MotionModule::set_rate(acmd->module_accessor, 2.5);
	}
	acmd->frame(4);
	if (acmd->is_excute()) {

		WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
    acmd->frame(8);
	if (acmd->is_excute()) {

		MotionModule::set_rate(acmd->module_accessor, 1);
	}
	acmd->frame(9);
	if (acmd->is_excute()) {
		MotionModule::set_rate(acmd->module_accessor, 1);
		acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("kneer"), /*Damage*/ 13.0, /*Angle*/ 361, /*KBG*/ 130, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 4.8, /*X*/ 4.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);

		acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("kneer"), /*Damage*/ 13.0, /*Angle*/ 361, /*KBG*/ 130, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 3.2, /*X*/ -3.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
	}
	acmd->wait(2);
	if (acmd->is_excute()) {

		acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("kneer"), /*Damage*/ 7.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 3.8, /*X*/ 4.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);

		acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("kneer"), /*Damage*/ 7.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 2.8, /*X*/ -2.6, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
	}
	acmd->frame(17);
	if (acmd->is_excute()) {

		AttackModule::clear_all(acmd->module_accessor);
	}
	acmd->wait(3);
	if (acmd->is_excute()) {

		WorkModule::off_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	}),
    ACMD("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_FALCO", "landing_air_f", "game_landing_air_f", [](ACMD* acmd) ->void {
    acmd->frame(1);
    if (acmd->is_excute()) {

    acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 3.0, /*Angle*/ 65, /*KBG*/ 160, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 5.5, /*Z*/ 10.0, /*X2*/ 0.0, /*Y2*/ 5.5, /*Z2*/ 5.0, /*Hitlag*/ 2.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_NONE);
    }
    acmd->frame(2);
    if (acmd->is_excute()) {

    AttackModule::clear_all(acmd->module_accessor);
    }
	}),
	// Mario
    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_MARIO", "attack_hi3", "game_attackhi3", [] (ACMD* acmd) ->void {
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

    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_MARIO", "attack_hi4", "game_attackhi4", [] (ACMD* acmd) -> void {
        acmd->frame(7);
        if (acmd->is_excute()) {
            WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
            JostleModule::set_status(acmd->module_accessor, false);
        }
        acmd->frame(9);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("head"), /*Damage*/ 14.0, /*Angle*/ 83, /*KBG*/ 94, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 5.0, /*X*/ 2.5, /*Y*/ 1.1, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_PUNCH, /*Type*/ ATTACK_REGION_HEAD);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("bust"), /*Damage*/ 14.0, /*Angle*/ 83, /*KBG*/ 94, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 4.0, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_PUNCH, /*Type*/ ATTACK_REGION_HEAD);
            HitModule::set_status_joint(acmd->module_accessor, hash40("head"), HIT_STATUS_XLU, 0);
        }
        acmd->wait(4);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
            HitModule::set_status_all(acmd->module_accessor, HIT_STATUS_NORMAL, 0);
        }
        acmd->frame(20);
        if (acmd->is_excute()) {
            JostleModule::set_status(acmd->module_accessor, true);
        }
    }), 

    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_MARIO", "attack_s4hi", "game_attacks4hi", [] (ACMD* acmd) -> void {
        acmd->frame(15);
        if (acmd->is_excute()) {
        acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("arml"), /*Damage*/ 14.7, /*Angle*/ 361, /*KBG*/ 110, /*FKB*/ 0, /*BKB*/ 33, /*Size*/ 2.0, /*X*/ -1.0, /*Y*/ 0.7, /*Z*/ 0.0, /*X2*/ -3.0, /*Y2*/ 1.0, /*Z2*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_PUNCH, /*Type*/ ATTACK_REGION_PUNCH);
        acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("arml"), /*Damage*/ 17.799999, /*Angle*/ 361, /*KBG*/ 107, /*FKB*/ 0, /*BKB*/ 33, /*Size*/ 5.0, /*X*/ 5.4, /*Y*/ 0.0, /*Z*/ -1.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_fire"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_FIRE, /*Type*/ ATTACK_REGION_PUNCH);
        }
        acmd->wait(3);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
    }), 

    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_MARIO", "attack_s4", "game_attacks4", [] (ACMD* acmd) -> void {
        acmd->frame(6);
        if (acmd->is_excute()) {
            WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        }
        acmd->frame(15);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("arml"), /*Damage*/ 14.7, /*Angle*/ 361, /*KBG*/ 110, /*FKB*/ 0, /*BKB*/ 33, /*Size*/ 2.0, /*X*/ -1.0, /*Y*/ 0.7, /*Z*/ 0.0, /*X2*/ -3.0, /*Y2*/ 1.0, /*Z2*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_PUNCH, /*Type*/ ATTACK_REGION_PUNCH);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("arml"), /*Damage*/ 17.799999, /*Angle*/ 361, /*KBG*/ 105, /*FKB*/ 0, /*BKB*/ 33, /*Size*/ 5.0, /*X*/ 5.4, /*Y*/ 0.0, /*Z*/ -1.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_fire"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_FIRE, /*Type*/ ATTACK_REGION_PUNCH);
        }
        acmd->wait(3);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
    }),

    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_MARIO", "attack_s4lw", "game_attacks4lw", [] (ACMD* acmd) -> void {
        acmd->frame(6);
        if (acmd->is_excute()) {
            WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        }
        acmd->frame(15);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("arml"), /*Damage*/ 14.7, /*Angle*/ 361, /*KBG*/ 110, /*FKB*/ 0, /*BKB*/ 33, /*Size*/ 2.0, /*X*/ -1.0, /*Y*/ 0.7, /*Z*/ 0.0, /*X2*/ -3.0, /*Y2*/ 1.0, /*Z2*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_PUNCH, /*Type*/ ATTACK_REGION_PUNCH);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("arml"), /*Damage*/ 17.799999, /*Angle*/ 361, /*KBG*/ 103, /*FKB*/ 0, /*BKB*/ 33, /*Size*/ 5.0, /*X*/ 5.4, /*Y*/ 0.0, /*Z*/ -1.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_fire"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_FIRE, /*Type*/ ATTACK_REGION_PUNCH);
        }
        acmd->wait(3);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
    }),

    ACMD("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_MARIO", "attack_lw4", "game_attacklw4", [] (ACMD* acmd) -> void {
        acmd->frame(3);
        if (acmd->is_excute()) {
            WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        }
        acmd->frame(5);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 10.0, /*Angle*/ 45, /*KBG*/ 105, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 3.6, /*Z*/ 12.5, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 10.0, /*Angle*/ 45, /*KBG*/ 105, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.3, /*X*/ 0.0, /*Y*/ 3.6, /*Z*/ 7.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            AttackModule::set_attack_height_all(acmd->module_accessor, ATTACK_HEIGHT_LOW, false);
        }
        acmd->wait(2);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
        acmd->frame(14);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 12.0, /*Angle*/ 42, /*KBG*/ 105, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 3.6, /*Z*/ -11.5, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 12.0, /*Angle*/ 42, /*KBG*/ 105, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.3, /*X*/ 0.0, /*Y*/ 3.6, /*Z*/ -6.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            AttackModule::set_attack_height_all(acmd->module_accessor, ATTACK_HEIGHT_LOW, false);
        }
        acmd->wait(1);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
    }),

    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_MARIO", "attack_dash", "game_attackdash", [] (ACMD* acmd) -> void {
        acmd->frame(6);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 8.0, /*Angle*/ 43, /*KBG*/ 58, /*FKB*/ 0, /*BKB*/ 100, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 1.5, /*Z*/ 5.4, /*Hitlag*/ 1.25, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            FighterAreaModuleImpl::enable_fix_jostle_area(acmd->module_accessor, 4, 6);
        }
        acmd->wait(4);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 6.0, /*Angle*/ 135, /*KBG*/ 37, /*FKB*/ 0, /*BKB*/ 87, /*Size*/ 2.7, /*X*/ 0.0, /*Y*/ 1.5, /*Z*/ 4.9, /*Hitlag*/ 1.25, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
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
        for (uint32_t i = 0; i < 5; i++) {
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

    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_MARIO", "attack_lw3", "game_attacklw3", [] (ACMD* acmd) -> void {
        acmd->frame(5);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("kneel"), /*Damage*/ 7.0, /*Angle*/ 105, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 3.2, /*X*/ -1.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ hash40("false"), /*ShieldDamage*/ 0, /*Trip*/ 0.4, /*Rehit*/ 0, /*Reflectable*/ hash40("false"), /*Absorbable*/ hash40("false"), /*Flinchless*/ hash40("false"), /*DisableHitlag*/ hash40("false"), /*Direct_Hitbox*/ hash40("true"), /*Ground_or_Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ hash40("false"), /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("toel"), /*Damage*/ 5.0, /*Angle*/ 105, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 4.2, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ hash40("false"), /*ShieldDamage*/ 0, /*Trip*/ 0.4, /*Rehit*/ 0, /*Reflectable*/ hash40("false"), /*Absorbable*/ hash40("false"), /*Flinchless*/ hash40("false"), /*DisableHitlag*/ hash40("false"), /*Direct_Hitbox*/ hash40("true"), /*Ground_or_Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ hash40("false"), /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            AttackModule::set_attack_height_all(acmd->module_accessor, ATTACK_HEIGHT_LOW, false);
        }
        acmd->wait(3);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
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
    //Falcon
    ACMD("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_CAPTAIN", "special_s_end", "game_specialsend", [] (ACMD* acmd) -> void {
        acmd->frame(1);
        if (acmd->is_excute()) {
            JostleModule::set_status(acmd->module_accessor, false);
            MotionModule::set_rate(acmd->module_accessor, 3);
            acmd->wrap(damage, { L2CValue(MA_MSC_DAMAGE_DAMAGE_NO_REACTION), L2CValue(DAMAGE_NO_REACTION_MODE_DAMAGE_POWER), L2CValue(10) });
        }
        acmd->frame(2);
        if (acmd->is_excute()) {
            MotionModule::set_rate(acmd->module_accessor, 1);
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("handl"), /*Damage*/ 10.0, /*Angle*/ 85, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 87, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ -2.0, /*Z*/ -1.0, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_fire"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_FIRE, /*Type*/ ATTACK_REGION_PUNCH);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 10.0, /*Angle*/ 85, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 87, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 5.5, /*X2*/ 0.0, /*Y2*/ 9.0, /*Z2*/ 12.5, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_fire"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_FIRE, /*Type*/ ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame(acmd->module_accessor, /*ID*/ 0, /*Frames*/ 3, /*Unk*/ false);
            AttackModule::set_add_reaction_frame(acmd->module_accessor, /*ID*/ 1, /*Frames*/ 3, /*Unk*/ false);
        }
        acmd->wait(1);
        if (acmd->is_excute()) {
            AttackModule::clear(acmd->module_accessor, /*ID*/ 1, false);
            acmd->wrap(damage, { L2CValue(MA_MSC_DAMAGE_DAMAGE_NO_REACTION), L2CValue(DAMAGE_NO_REACTION_MODE_NORMAL), L2CValue(0) });
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("handl"), /*Damage*/ 10.0, /*Angle*/ 85, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 87, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 4, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_fire"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_FIRE, /*Type*/ ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame(acmd->module_accessor, /*ID*/ 0, /*Frames*/ 3, /*Unk*/ false);
        }
        acmd->wait(2);
        MotionModule::set_rate(acmd->module_accessor, 0.75);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
            JostleModule::set_status(acmd->module_accessor, true);
        }
    }),

    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_CAPTAIN", "attack_dash", "game_attackdash", [] (ACMD* acmd) -> void {
        acmd->frame(7);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 10.0, /*Angle*/ 72, /*KBG*/ 77, /*FKB*/ 0, /*BKB*/ 90, /*Size*/ 6.2, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 12.0, /*X2*/ 0.0, /*Y2*/ 8.0, /*Z2*/ 12.0, /*Hitlag*/ 1.25, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_PUNCH, /*Type*/ ATTACK_REGION_BODY);
        }
        acmd->wait(3);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 6.0, /*Angle*/ 72, /*KBG*/ 77, /*FKB*/ 0, /*BKB*/ 90, /*Size*/ 4.8, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 12.0, /*X2*/ 0.0, /*Y2*/ 6.0, /*Z2*/ 12.0, /*Hitlag*/ 1.25, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 1, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_PUNCH, /*Type*/ ATTACK_REGION_BODY);
        }
        acmd->wait(7);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
    }),

    ACMD("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_CAPTAIN", "attack_air_hi", "game_attackairhi", [] (ACMD* acmd) -> void {
        if (acmd->is_excute()) {
            WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        acmd->frame(6);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("legl"), /*Damage*/ 13.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 10, /*Size*/ 4.5, /*X*/ 3.2, /*Y*/ 2.1, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ hash40("false"), /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ hash40("false"), /*Absorbable*/ hash40("false"), /*Flinchless*/ hash40("false"), /*DisableHitlag*/ hash40("false"), /*Direct_Hitbox*/ hash40("true"), /*Ground_or_Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ hash40("false"), /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("kneel"), /*Damage*/ 12.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 10, /*Size*/ 5.0, /*X*/ 6.2, /*Y*/ 0.9, /*Z*/ -0.4, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ hash40("false"), /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ hash40("false"), /*Absorbable*/ hash40("false"), /*Flinchless*/ hash40("false"), /*DisableHitlag*/ hash40("false"), /*Direct_Hitbox*/ hash40("true"), /*Ground_or_Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ hash40("false"), /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
        }
        acmd->frame(11);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("legl"), /*Damage*/ 12.0, /*Angle*/ 30, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 8, /*Size*/ 4.5, /*X*/ 3.2, /*Y*/ 2.1, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ hash40("false"), /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ hash40("false"), /*Absorbable*/ hash40("false"), /*Flinchless*/ hash40("false"), /*DisableHitlag*/ hash40("false"), /*Direct_Hitbox*/ hash40("true"), /*Ground_or_Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ hash40("false"), /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("kneel"), /*Damage*/ 10.0, /*Angle*/ 30, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 8, /*Size*/ 5.0, /*X*/ 6.2, /*Y*/ 0.9, /*Z*/ -0.4, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ hash40("false"), /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ hash40("false"), /*Absorbable*/ hash40("false"), /*Flinchless*/ hash40("false"), /*DisableHitlag*/ hash40("false"), /*Direct_Hitbox*/ hash40("true"), /*Ground_or_Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ hash40("false"), /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
        }
        acmd->frame(14);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("legl"), /*Damage*/ 8.0, /*Angle*/ 0, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 6, /*Size*/ 4.5, /*X*/ 3.2, /*Y*/ 2.1, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ hash40("false"), /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ hash40("false"), /*Absorbable*/ hash40("false"), /*Flinchless*/ hash40("false"), /*DisableHitlag*/ hash40("false"), /*Direct_Hitbox*/ hash40("true"), /*Ground_or_Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ hash40("false"), /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("kneel"), /*Damage*/ 6.0, /*Angle*/ 0, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 6, /*Size*/ 5.0, /*X*/ 6.2, /*Y*/ 0.9, /*Z*/ -0.4, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ hash40("false"), /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ hash40("false"), /*Absorbable*/ hash40("false"), /*Flinchless*/ hash40("false"), /*DisableHitlag*/ hash40("false"), /*Direct_Hitbox*/ hash40("true"), /*Ground_or_Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ hash40("false"), /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
        }
        acmd->frame(17);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
        acmd->frame(24);
        if (acmd->is_excute()) {
            WorkModule::off_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    }),

    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_CAPTAIN", "attack_air_lw", "game_attackairlw", [] (ACMD* acmd) -> void {
        acmd->frame(4);
        if (acmd->is_excute()) {
            WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            JostleModule::set_status(acmd->module_accessor, false);
        }
        acmd->frame(16);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 14.0, /*Angle*/ 270, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 10, /*Size*/ 5.9, /*X*/ 0.0, /*Y*/ -5.2, /*Z*/ 0.8, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ hash40("false"), /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ hash40("false"), /*Absorbable*/ hash40("false"), /*Flinchless*/ hash40("false"), /*DisableHitlag*/ hash40("false"), /*Direct_Hitbox*/ hash40("true"), /*Ground_or_Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ hash40("false"), /*Effect*/ hash40("collision_attr_fire"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("bust"), /*Damage*/ 14.0, /*Angle*/ 290, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 10, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.8, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ hash40("false"), /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ hash40("false"), /*Absorbable*/ hash40("false"), /*Flinchless*/ hash40("false"), /*DisableHitlag*/ hash40("false"), /*Direct_Hitbox*/ hash40("true"), /*Ground_or_Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ hash40("false"), /*Effect*/ hash40("collision_attr_fire"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_BODY);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 14.0, /*Angle*/ 270, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 10, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 1.0, /*Z*/ 0.8, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ hash40("false"), /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ hash40("false"), /*Absorbable*/ hash40("false"), /*Flinchless*/ hash40("false"), /*DisableHitlag*/ hash40("false"), /*Direct_Hitbox*/ hash40("true"), /*Ground_or_Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ hash40("false"), /*Effect*/ hash40("collision_attr_fire"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
        }
        acmd->wait(3);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
            JostleModule::set_status(acmd->module_accessor, true);
        }
        acmd->frame(39);
        if (acmd->is_excute()) {
            WorkModule::off_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    }),

    ACMD("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_CAPTAIN", "attack_hi4", "game_attackhi4", [](ACMD* acmd) -> void {
        acmd->frame(9);
        if (acmd->is_excute()) {
            WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
      }
        acmd->frame(22);
        if (acmd->is_excute()) {
            HitModule::set_status_joint(acmd->module_accessor, hash40("kneel"), HIT_STATUS_XLU, 0);
            HitModule::set_status_joint(acmd->module_accessor, hash40("arml"), HIT_STATUS_XLU, 0);
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 6.0, /*Angle*/ 110, /*KBG*/ 90, /*FKB*/ 140, /*BKB*/ 0, /*Size*/ 5.7, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ 10.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_NONE, /*Type*/ ATTACK_REGION_NONE);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 6.0, /*Angle*/ 95, /*KBG*/ 90, /*FKB*/ 110, /*BKB*/ 0, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 17.0, /*Z*/ 5.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 8.0, /*Angle*/ 80, /*KBG*/ 90, /*FKB*/ 20, /*BKB*/ 30, /*Size*/ 4.8, /*X*/ 0.0, /*Y*/ 21.0, /*Z*/ 1.5, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            acmd->ATTACK(/*ID*/ 3, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 8.0, /*Angle*/ 90, /*KBG*/ 90, /*FKB*/ 20, /*BKB*/ 30, /*Size*/ 4.8, /*X*/ 0.0, /*Y*/ 28.0, /*Z*/ 5.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
        }
        acmd->frame(23);
        if (acmd->is_excute()) {
            AttackModule::clear(acmd->module_accessor, /*ID*/ 1, false);
        }
        acmd->frame(24);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
            HitModule::set_status_all(acmd->module_accessor, HIT_STATUS_NORMAL, 0);
        }
        acmd->frame(28);
        if (acmd->is_excute()) {
            HitModule::set_status_joint(acmd->module_accessor, hash40("kneel"), HIT_STATUS_XLU, 0);
            HitModule::set_status_joint(acmd->module_accessor, hash40("arml"), HIT_STATUS_XLU, 0);
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 12.0, /*Angle*/ 83, /*KBG*/ 93, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 29.0, /*Z*/ 5.5, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 12.0, /*Angle*/ 85, /*KBG*/ 94, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 21.0, /*Z*/ 2.5, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 11.0, /*Angle*/ 70, /*KBG*/ 101, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 16.0, /*Z*/ 0.5, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
        }
        acmd->frame(30);
        if (acmd->is_excute()) {
            HitModule::set_status_all(acmd->module_accessor, HIT_STATUS_NORMAL, 0);
            AttackModule::clear_all(acmd->module_accessor);
        }
    }),

    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_CAPTAIN", "attack_lw3", "game_attacklw3", [] (ACMD* acmd) -> void {
        acmd->frame(11);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("kneel"), /*Damage*/ 10.0, /*Angle*/ 80, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 4.8, /*X*/ 6.5, /*Y*/ -1.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.5, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("legl"), /*Damage*/ 10.0, /*Angle*/ 70, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 4.8, /*X*/ 3.5, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.5, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 10.0, /*Angle*/ 60, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 3.5, /*Z*/ -1.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.5, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            AttackModule::set_attack_height_all(acmd->module_accessor, ATTACK_HEIGHT_LOW, false);
        }
        acmd->wait(2);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
    }), 

    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_CAPTAIN", "attack_lw4", "game_attacklw4", [] (ACMD* acmd) -> void {
        acmd->frame(12);
        if (acmd->is_excute()) {
            WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        }
        acmd->frame(19);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("kneer"), /*Damage*/ 8.0, /*Angle*/ 160, /*KBG*/ 100, /*FKB*/ 138, /*BKB*/ 0, /*Size*/ 4.5, /*X*/ 4.9, /*Y*/ -0.9, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("kneer"), /*Damage*/ 8.0, /*Angle*/ 160, /*KBG*/ 100, /*FKB*/ 98, /*BKB*/ 0, /*Size*/ 4.2, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("legr"), /*Damage*/ 8.0, /*Angle*/ 160, /*KBG*/ 100, /*FKB*/ 98, /*BKB*/ 0, /*Size*/ 3.7, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
        }
        acmd->wait(2);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
        acmd->frame(29);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 1, /*Bone*/ hash40("kneer"), /*Damage*/ 12.0, /*Angle*/ 120, /*KBG*/ 110, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 4.5, /*X*/ 4.9, /*Y*/ -0.9, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 1, /*Bone*/ hash40("kneer"), /*Damage*/ 12.0, /*Angle*/ 120, /*KBG*/ 110, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 4.2, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            acmd->ATTACK(/*ID*/ 2, /*Part*/ 1, /*Bone*/ hash40("legr"), /*Damage*/ 12.0, /*Angle*/ 120, /*KBG*/ 110, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 3.7, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
        }
        acmd->wait(2);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
    }),

    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_CAPTAIN", "special_n", "game_specialn", [] (ACMD* acmd) -> void {
        acmd->frame(1);
        MotionModule::set_rate(acmd->module_accessor, 2);
        acmd->frame(7);
        if (acmd->is_excute()) {
            WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_TURN);
        }
        acmd->frame(26);
        if (acmd->is_excute()) {
            MotionModule::set_rate(acmd->module_accessor, 1);
            WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_GENERATE_BIRD);
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("armr"), /*Damage*/ 19.0, /*Angle*/ 361, /*KBG*/ 109, /*FKB*/ 0, /*BKB*/ 53, /*Size*/ 4.5, /*X*/ -1.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.5, /*SDI*/ 0.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_fire"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_PUNCH);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("shoulderr"), /*Damage*/ 19.0, /*Angle*/ 361, /*KBG*/ 109, /*FKB*/ 0, /*BKB*/ 53, /*Size*/ 2.5, /*X*/ -2.5, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.5, /*SDI*/ 0.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_fire"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_PUNCH);
            acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("armr"), /*Damage*/ 19.0, /*Angle*/ 361, /*KBG*/ 109, /*FKB*/ 0, /*BKB*/ 53, /*Size*/ 4.0, /*X*/ 4.2, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.5, /*SDI*/ 0.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_fire"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_PUNCH);
        }
        acmd->wait(5);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
    }),

    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_CAPTAIN", "special_n_turn", "game_specialnturn", [] (ACMD* acmd) -> void {
        acmd->frame(1);
        MotionModule::set_rate(acmd->module_accessor, 2);
        acmd->frame(10);
        if (acmd->is_excute()) {
            PostureModule::reverse_lr(acmd->module_accessor);
        }
        acmd->frame(24);
        if (acmd->is_excute()) {
            MotionModule::set_rate(acmd->module_accessor, 1);
            WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_GENERATE_BIRD);
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("armr"), /*Damage*/ 22.0, /*Angle*/ 361, /*KBG*/ 109, /*FKB*/ 0, /*BKB*/ 53, /*Size*/ 4.5, /*X*/ -1.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.5, /*SDI*/ 0.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_fire"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_PUNCH);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("shoulderr"), /*Damage*/ 22.0, /*Angle*/ 361, /*KBG*/ 109, /*FKB*/ 0, /*BKB*/ 53, /*Size*/ 2.5, /*X*/ -2.5, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.5, /*SDI*/ 0.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_fire"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_PUNCH);
            acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("armr"), /*Damage*/ 22.0, /*Angle*/ 361, /*KBG*/ 109, /*FKB*/ 0, /*BKB*/ 53, /*Size*/ 4.0, /*X*/ 4.2, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.5, /*SDI*/ 0.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_fire"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_PUNCH);
        }
        acmd->wait(5);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
    }),

    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_CAPTAIN", "special_air_n", "game_specialairn", [] (ACMD* acmd) -> void {
        acmd->frame(1);
        MotionModule::set_rate(acmd->module_accessor, 2);
        if (acmd->is_excute()) {
            Vector3f new_speed = {.x = 0.0, .y = 0.2, .z = 0.0};
            KineticModule::add_speed(acmd->module_accessor, &new_speed);
        }
        acmd->frame(7);
        if (acmd->is_excute()) {
            WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_TURN);
        }
        acmd->frame(25);
        if (acmd->is_excute()) {
            WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_DIR_DECIDE);
            WorkModule::set_int(acmd->module_accessor, 1, FIGHTER_CAPTAIN_STATUS_WORK_ID_INT_FALCON_PUNCH_AIR_PHASE);
        }
        acmd->frame(26);
        if (acmd->is_excute()) {
            MotionModule::set_rate(acmd->module_accessor, 1);
            WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_GENERATE_BIRD);
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("armr"), /*Damage*/ 17.0, /*Angle*/ 361, /*KBG*/ 109, /*FKB*/ 0, /*BKB*/ 53, /*Size*/ 5.175, /*X*/ -1.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.5, /*SDI*/ 0.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_fire"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_PUNCH);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("shoulderr"), /*Damage*/ 17.0, /*Angle*/ 361, /*KBG*/ 109, /*FKB*/ 0, /*BKB*/ 53, /*Size*/ 2.875, /*X*/ -2.5, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.5, /*SDI*/ 0.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_fire"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_PUNCH);
            acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("armr"), /*Damage*/ 17.0, /*Angle*/ 361, /*KBG*/ 109, /*FKB*/ 0, /*BKB*/ 53, /*Size*/ 4.6, /*X*/ 4.2, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.5, /*SDI*/ 0.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_fire"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_PUNCH);
        }
        acmd->wait(5);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
        acmd->wait(12);
        if (acmd->is_excute()) {
            WorkModule::set_int(acmd->module_accessor, 2, FIGHTER_CAPTAIN_STATUS_WORK_ID_INT_FALCON_PUNCH_AIR_PHASE);
        }
    }),

    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_CAPTAIN", "special_air_n_turn", "game_specialairnturn", [] (ACMD* acmd) -> void {
        acmd->frame(1);
        MotionModule::set_rate(acmd->module_accessor, 2);
        acmd->frame(10);
        if (acmd->is_excute()) {
            PostureModule::reverse_lr(acmd->module_accessor);
        }
        acmd->frame(23);
        if (acmd->is_excute()) {
            WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_DIR_DECIDE);
            WorkModule::set_int(acmd->module_accessor, 1, FIGHTER_CAPTAIN_STATUS_WORK_ID_INT_FALCON_PUNCH_AIR_PHASE);
        }
        acmd->frame(24);
        if (acmd->is_excute()) {
            MotionModule::set_rate(acmd->module_accessor, 1);
            WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_GENERATE_BIRD);
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("armr"), /*Damage*/ 19.0, /*Angle*/ 361, /*KBG*/ 109, /*FKB*/ 0, /*BKB*/ 53, /*Size*/ 5.175, /*X*/ -1.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.5, /*SDI*/ 0.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_fire"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_PUNCH);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("shoulderr"), /*Damage*/ 19.0, /*Angle*/ 361, /*KBG*/ 109, /*FKB*/ 0, /*BKB*/ 53, /*Size*/ 2.875, /*X*/ -2.5, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.5, /*SDI*/ 0.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_fire"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_PUNCH);
            acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("armr"), /*Damage*/ 19.0, /*Angle*/ 361, /*KBG*/ 109, /*FKB*/ 0, /*BKB*/ 53, /*Size*/ 4.6, /*X*/ 4.2, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.5, /*SDI*/ 0.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_fire"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_PUNCH);
        }
        acmd->wait(5);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
        acmd->wait(12);
        if (acmd->is_excute()) {
            WorkModule::set_int(acmd->module_accessor, 2, FIGHTER_CAPTAIN_STATUS_WORK_ID_INT_FALCON_PUNCH_AIR_PHASE);
        }
    }),
	// Marth
    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_MARTH", "attack_air_lw", "game_attackairlw", [] (ACMD* acmd) -> void {
    if (acmd->is_excute()) {
        MotionModule::set_rate(acmd->module_accessor, 1.4);
    }
    acmd->frame(3);
    if (acmd->is_excute()) {

    WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    acmd->frame(9);
    if (acmd->is_excute()) {
     MotionModule::set_rate(acmd->module_accessor, 1);
    acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("armr"), /*Damage*/ 12.0, /*Angle*/ 361, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 1.0, /*Z*/ 0.0, /*Hitlag*/ 0.9, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);

    acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("sword1"), /*Damage*/ 12.0, /*Angle*/ 80, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.5, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 2.0, /*Hitlag*/ 0.9, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);

    acmd->ATTACK(/*ID*/ 3, /*Part*/ 0, /*Bone*/ hash40("sword1"), /*Damage*/ 14.0, /*Angle*/ 290, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.5, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 6.7, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_MARTH_SWORD, /*Type*/ ATTACK_REGION_SWORD);
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
    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_MARTH", "attack_air_f", "game_attackairf", [] (ACMD* acmd) -> void {
    if (acmd->is_excute()) {
    MotionModule::set_rate(acmd->module_accessor, 1.4286);
    WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    acmd->frame(6);
    if (acmd->is_excute()) {
    MotionModule::set_rate(acmd->module_accessor, 1);
    acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("sword1"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 3.0, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 3.5, /*Hitlag*/ 0.9, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);

    acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("armr"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 3.8, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 0.9, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);

    acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("0x0954eb78b2"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 3.9, /*X*/ 1.0, /*Y*/ 8.0, /*Z*/ 0.0, /*Hitlag*/ 0.9, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);

    acmd->ATTACK(/*ID*/ 3, /*Part*/ 0, /*Bone*/ hash40("sword1"), /*Damage*/ 11.5, /*Angle*/ 67, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 42, /*Size*/ 3.0, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 7.5, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_MARTH_SWORD, /*Type*/ ATTACK_REGION_SWORD);
    }
    acmd->wait(3);
    if (acmd->is_excute()) {

    AttackModule::clear_all(acmd->module_accessor);
    }
    acmd->frame(36);
    if (acmd->is_excute()) {

    WorkModule::off_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }   
    }),
    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_MARTH", "attack_air_hi", "game_attackairhi", [] (ACMD* acmd) -> void {
    acmd->frame(3);
    if (acmd->is_excute()) {

    WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    acmd->frame(5);
    if (acmd->is_excute()) {

    acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("sword1"), /*Damage*/ 9.5, /*Angle*/ 80, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.5, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 2.0, /*Hitlag*/ 0.9, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);

    acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("armr"), /*Damage*/ 9.5, /*Angle*/ 80, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 1.0, /*Z*/ 0.0, /*Hitlag*/ 0.9, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);

    acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("0x0954eb78b2"), /*Damage*/ 9.5, /*Angle*/ 80, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 2.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 0.0, /*Hitlag*/ 0.9, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);

    acmd->ATTACK(/*ID*/ 3, /*Part*/ 0, /*Bone*/ hash40("sword1"), /*Damage*/ 13.0, /*Angle*/ 90, /*KBG*/ 84, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.5, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 6.7, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_MARTH_SWORD, /*Type*/ ATTACK_REGION_SWORD);
    }
    acmd->frame(13);
    if (acmd->is_excute()) {

    AttackModule::clear_all(acmd->module_accessor);
    }
    acmd->frame(38);
    if (acmd->is_excute()) {

    WorkModule::off_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }   
    }),
    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_MARTH", "attack_air_b", "game_attackairb", [] (ACMD* acmd) -> void {
     acmd->frame(3);
    if (acmd->is_excute()) {

    WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    acmd->frame(7);
    if (acmd->is_excute()) {

    acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("sword1"), /*Damage*/ 9.0, /*Angle*/ 361, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 3.7, /*Hitlag*/ 0.9, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);

    acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("armr"), /*Damage*/ 9.0, /*Angle*/ 361, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.5, /*X*/ 2.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 0.9, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
    
    acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("0x0954eb78b2"), /*Damage*/ 9.0, /*Angle*/ 361, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.9, /*X*/ 2.0, /*Y*/ 8.0, /*Z*/ 0.0, /*Hitlag*/ 0.9, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);

    acmd->ATTACK(/*ID*/ 3, /*Part*/ 0, /*Bone*/ hash40("sword1"), /*Damage*/ 12.5, /*Angle*/ 361, /*KBG*/ 94, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 7.0, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_MARTH_SWORD, /*Type*/ ATTACK_REGION_SWORD);
    }
    acmd->wait(5);
    if (acmd->is_excute()) {

    AttackModule::clear_all(acmd->module_accessor);
    }
    acmd->frame(32);
    if (acmd->is_excute()) {

    WorkModule::off_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }  
    }),
    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_MARTH", "attack_11", "game_attackair11", [] (ACMD* acmd) -> void {
    acmd->frame(5);
    if (acmd->is_excute()) {

    acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("armr"), /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 15, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);

    acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("sword1"), /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 12, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 3.0, /*X*/ 0.5, /*Y*/ 0.0, /*Z*/ 1.5, /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);

    acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("sword1"), /*Damage*/ 5.0, /*Angle*/ 78, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 3.0, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 7.0, /*Hitlag*/ 1.7, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_FIGHTER, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_MARTH_SWORD, /*Type*/ ATTACK_REGION_SWORD);

    acmd->ATTACK(/*ID*/ 3, /*Part*/ 0, /*Bone*/ hash40("sword1"), /*Damage*/ 5.0, /*Angle*/ 78, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 3.0, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 7.0, /*Hitlag*/ 1.7, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_MARTH_SWORD, /*Type*/ ATTACK_REGION_SWORD);
    }
    acmd->wait(2);
    if (acmd->is_excute()) {

    AttackModule::clear_all(acmd->module_accessor);
    }
    acmd->frame(10);
    if (acmd->is_excute()) {

    WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    }),
    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_MARTH", "attack_dash", "game_attackdash", [] (ACMD* acmd) -> void {
    acmd->frame(13);
    if (acmd->is_excute()) {

    acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("sword1"), /*Damage*/ 10.0, /*Angle*/ 110, /*KBG*/ 55, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 3.5, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 1.0, /*Hitlag*/ 0.9, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);

    acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("armr"), /*Damage*/ 9.0, /*Angle*/ 361, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 1.0, /*Z*/ 0.0, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);

    acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("0x0954eb78b2"), /*Damage*/ 9.0, /*Angle*/ 361, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 2.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);

    acmd->ATTACK(/*ID*/ 3, /*Part*/ 0, /*Bone*/ hash40("sword1"), /*Damage*/ 12.0, /*Angle*/ 110, /*KBG*/ 55, /*FKB*/ 0, /*BKB*/ 95, /*Size*/ 3.5, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 6.5, /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_MARTH_SWORD, /*Type*/ ATTACK_REGION_SWORD);
    }
    acmd->frame(16);
    if (acmd->is_excute()) {

    acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("sword1"), /*Damage*/ 10.0, /*Angle*/ 45, /*KBG*/ 55, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 3.5, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 2.0, /*X2*/ 8.0, /*Y2*/ 1.5, /*Z2*/ -2.5, /*Hitlag*/ 0.9, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);

    acmd->ATTACK(/*ID*/ 3, /*Part*/ 0, /*Bone*/ hash40("sword1"), /*Damage*/ 12.0, /*Angle*/ 45, /*KBG*/ 55, /*FKB*/ 0, /*BKB*/ 95, /*Size*/ 3.5, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 6.5, /*X2*/ 12.0, /*Y2*/ 1.5, /*Z2*/ -2.5, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_MARTH_SWORD, /*Type*/ ATTACK_REGION_SWORD);
    }
    acmd->frame(17);
    if (acmd->is_excute()) {
    MotionModule::set_rate(acmd->module_accessor, 1.389);
    AttackModule::clear_all(acmd->module_accessor);
    }   
    }),
    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_MARTH", "attack_lw3", "game_attacklw3", [] (ACMD* acmd) -> void {
    acmd->frame(7);
    if (acmd->is_excute()) {

    acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 7.0, /*Angle*/ 30, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 2.7, /*X*/ 0.0, /*Y*/ 2.7, /*Z*/ 16.700001, /*X2*/ 0.0, /*Y2*/ 4.4, /*Z2*/ 9.2, /*Hitlag*/ 0.9, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.35, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_sting"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);

    acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("sword1"), /*Damage*/ 10.0, /*Angle*/ 30, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 2.7, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 8.2, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.35, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_sting"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_MARTH_SWORD, /*Type*/ ATTACK_REGION_SWORD);

    AttackModule::set_attack_height_all(acmd->module_accessor, ATTACK_HEIGHT_LOW, false);
    }
    acmd->wait(2);
    if (acmd->is_excute()) {

    AttackModule::clear_all(acmd->module_accessor);
    }
    }),
    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_MARTH", "attack_hi3", "game_attackhi3", [] (ACMD* acmd) -> void {
    acmd->frame(6);
    if (acmd->is_excute()) {

    acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("sword1"), /*Damage*/ 6.0, /*Angle*/ 110, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 2.0, /*Hitlag*/ 0.9, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);

    acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("armr"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 0.9, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);

    acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("0x0954eb78b2"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 2.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 0.9, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);

    acmd->ATTACK(/*ID*/ 3, /*Part*/ 0, /*Bone*/ hash40("sword1"), /*Damage*/ 9.0, /*Angle*/ 110, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 6.7, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_MARTH_SWORD, /*Type*/ ATTACK_REGION_SWORD);

    acmd->ATTACK(/*ID*/ 4, /*Part*/ 0, /*Bone*/ hash40("sword1"), /*Damage*/ 6.0, /*Angle*/ 110, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 2.0, /*X2*/ 4.5, /*Y2*/ 0.0, /*Z2*/ 2.0, /*Hitlag*/ 0.9, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);

    acmd->ATTACK(/*ID*/ 5, /*Part*/ 0, /*Bone*/ hash40("sword1"), /*Damage*/ 9.0, /*Angle*/ 110, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 6.7, /*X2*/ 6.0, /*Y2*/ 0.0, /*Z2*/ 6.7, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_MARTH_SWORD, /*Type*/ ATTACK_REGION_SWORD);
    }
    acmd->wait(1);
    if (acmd->is_excute()) {

    AttackModule::clear(acmd->module_accessor, /*ID*/ 4, false);

    AttackModule::clear(acmd->module_accessor, /*ID*/ 5, false);
    }
    acmd->wait(2);
    if (acmd->is_excute()) {

    acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("sword1"), /*Damage*/ 6.0, /*Angle*/ 85, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 52, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 2.0, /*Hitlag*/ 0.9, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_MARTH_SWORD, /*Type*/ ATTACK_REGION_SWORD);

    acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("armr"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 52, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 0.9, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_MARTH_SWORD, /*Type*/ ATTACK_REGION_SWORD);

    acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("0x0954eb78b2"), /*Damage*/ 5.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 52, /*Size*/ 2.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 0.9, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_MARTH_SWORD, /*Type*/ ATTACK_REGION_SWORD);

    acmd->ATTACK(/*ID*/ 3, /*Part*/ 0, /*Bone*/ hash40("sword1"), /*Damage*/ 9.0, /*Angle*/ 85, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 52, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 6.7, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_MARTH_SWORD, /*Type*/ ATTACK_REGION_SWORD);
    }
    acmd->wait(4);
    if (acmd->is_excute()) {

    AttackModule::clear_all(acmd->module_accessor);
    }
    }),
    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_MARTH", "attack_s3s", "game_attacks3s", [] (ACMD* acmd) -> void {
    acmd->frame(8);
    if (acmd->is_excute()) {

    acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("sword1"), /*Damage*/ 9.0, /*Angle*/ 361, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 3.5, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 2.5, /*Hitlag*/ 0.9, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);

    acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("armr"), /*Damage*/ 9.0, /*Angle*/ 361, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 1.0, /*Z*/ 0.0, /*Hitlag*/ 0.9, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);

    acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("sword1"), /*Damage*/ 12.0, /*Angle*/ 361, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 3.5, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 7.7, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_MARTH_SWORD, /*Type*/ ATTACK_REGION_SWORD);
    }
    acmd->wait(4);
    if (acmd->is_excute()) {

    AttackModule::clear_all(acmd->module_accessor);
    }   
    }),
    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_MARTH", "attack_s4s", "game_attacks4s", [] (ACMD* acmd) -> void {
    if (acmd->is_excute()) {
        MotionModule::set_rate(acmd->module_accessor, 1.2);
    }
    acmd->frame(3);
    if (acmd->is_excute()) {

    WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    acmd->frame(10);
    if (acmd->is_excute()) {
    MotionModule::set_rate(acmd->module_accessor, 1);
    acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("sword1"), /*Damage*/ 13.0, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 48, /*Size*/ 3.5, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 1.0, /*Hitlag*/ 0.9, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);

    acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("armr"), /*Damage*/ 13.0, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 48, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 1.0, /*Z*/ 0.0, /*Hitlag*/ 0.9, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);

    acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("bust"), /*Damage*/ 13.0, /*Angle*/ 361, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 48, /*Size*/ 2.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 0.9, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);

    acmd->ATTACK(/*ID*/ 3, /*Part*/ 0, /*Bone*/ hash40("sword1"), /*Damage*/ 18.0, /*Angle*/ 361, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 3.5, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 7.3, /*Hitlag*/ 1.3, /*SDI*/ 0.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_MARTH_SWORD, /*Type*/ ATTACK_REGION_SWORD);

    AttackModule::set_attack_height_all(acmd->module_accessor, ATTACK_HEIGHT_HIGH, false);
    }
    acmd->frame(14);
    if (acmd->is_excute()) {

    AttackModule::clear_all(acmd->module_accessor);
    }
    }),
    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_MARTH", "attack_hi4", "game_attackhi4", [] (ACMD* acmd) -> void {
    acmd->frame(5);
    if (acmd->is_excute()) {

    WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    acmd->frame(13);
    if (acmd->is_excute()) {

    acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("sword1"), /*Damage*/ 13.0, /*Angle*/ 89, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 2.0, /*Hitlag*/ 0.9, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_sting"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);

    acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("sword1"), /*Damage*/ 17.0, /*Angle*/ 89, /*KBG*/ 95, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 4.6, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 7.3, /*Hitlag*/ 1.3, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_sting"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_MARTH_SWORD, /*Type*/ ATTACK_REGION_SWORD);

    acmd->ATTACK(/*ID*/ 3, /*Part*/ 0, /*Bone*/ hash40("sword1"), /*Damage*/ 13.0, /*Angle*/ 90, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ -3.0, /*Hitlag*/ 0.9, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_sting"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);

    acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 3.0, /*Angle*/ 125, /*KBG*/ 100, /*FKB*/ 155, /*BKB*/ 0, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 9.0, /*X2*/ 0.0, /*Y2*/ 5.0, /*Z2*/ -9.0, /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 2, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ false, /*Ground/Air*/ COLLISION_SITUATION_MASK_G, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_PUNCH, /*Type*/ ATTACK_REGION_SWORD);
    }
    acmd->wait(2);
    if (acmd->is_excute()) {

    AttackModule::clear(acmd->module_accessor, /*ID*/ 0, false);
    }
    acmd->wait(3);
    if (acmd->is_excute()) {

    AttackModule::clear_all(acmd->module_accessor);
    }
    }),
    // Meta Knight
    ACMD("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_METAKNIGHT", "attack_dash", "game_attackdash", [] (ACMD* acmd)->void {
        acmd->frame(7);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("footl"), /*Damage*/ 7.0, /*Angle*/ 70, /*KBG*/ 97, /*FKB*/ 0, /*BKB*/ 77, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ -1.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("footl"), /*Damage*/ 7.0, /*Angle*/ 75, /*KBG*/ 97, /*FKB*/ 0, /*BKB*/ 77, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ -6.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("footl"), /*Damage*/ 6.0, /*Angle*/ 80, /*KBG*/ 97, /*FKB*/ 0, /*BKB*/ 77, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ -9.3, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
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
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 11.0, /*Angle*/ 361, /*KBG*/ 93, /*FKB*/ 0, /*BKB*/ 57, /*Size*/ 3.8, /*X*/ 0.0, /*Y*/ 4.0, /*Z*/ 11.0, /*X2*/ 0.0, /*Y2*/ 9.8, /*Z2*/ 19.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
        }
        acmd->wait(1);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
        acmd->frame(9);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 13.0, /*Angle*/ 361, /*KBG*/ 93, /*FKB*/ 0, /*BKB*/ 57, /*Size*/ 4.6, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ -11.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 13.0, /*Angle*/ 361, /*KBG*/ 93, /*FKB*/ 0, /*BKB*/ 57, /*Size*/ 4.6, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ -17.5, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
        }
        acmd->wait(1);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
    }),
    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_METAKNIGHT", "special_n_spin", "game_specialnspin", [] (ACMD* acmd)->void {
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 0.5, /*Angle*/ 366, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 10, /*Size*/ 9.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 6, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_BODY);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 1, /*Bone*/ hash40("top"), /*Damage*/ 0.5, /*Angle*/ 366, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 10, /*Size*/ 15.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 0.0, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 6, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ true, /*DisableHitlag*/ true, /*Direct/Indirect*/ false, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_NO_ITEM, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_NONE, /*Type*/ ATTACK_REGION_NONE);
        }
        acmd->frame(104);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
    }),
    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_METAKNIGHT", "attack_air_hi", "game_attackairhi", [](ACMD* acmd)->void {
        if (acmd->is_excute()) {
            MotionModule::set_rate(acmd->module_accessor, 1.5);
        }
        acmd->frame(4);
        if (acmd->is_excute()) {
            MotionModule::set_rate(acmd->module_accessor, 1);
            WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 4.0, /*Angle*/ 73, /*KBG*/ 125, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 19.0, /*Z*/ 3.5, /*X2*/ 0.0, /*Y2*/ 19.0, /*Z2*/ -5.5, /*Hitlag*/ 1.0, /*SDI*/ 1.5, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 4.0, /*Angle*/ 73, /*KBG*/ 125, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 19.0, /*Z*/ 4.5, /*X2*/ 0.0, /*Y2*/ 16.0, /*Z2*/ 9.5, /*Hitlag*/ 1.0, /*SDI*/ 1.5, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 4.0, /*Angle*/ 62, /*KBG*/ 125, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 19.0, /*Z*/ -6.5, /*X2*/ 0.0, /*Y2*/ 16.0, /*Z2*/ -11.5, /*Hitlag*/ 1.0, /*SDI*/ 1.5, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
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
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("haver"), /*Damage*/ 6.0, /*Angle*/ 80, /*KBG*/ 118, /*FKB*/ 0, /*BKB*/ 75, /*Size*/ 7.7, /*X*/ 0.0, /*Y*/ 3.5, /*Z*/ -2.0, /*X2*/ 0.0, /*Y2*/ 10.0, /*Z2*/ -2.0, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("bust"), /*Damage*/ 6.0, /*Angle*/ 80, /*KBG*/ 118, /*FKB*/ 0, /*BKB*/ 75, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_BODY);
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

    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_METAKNIGHT", "special_hi", "game_specialhi", [] (ACMD* acmd) -> void {
        acmd->frame(6);
        if (acmd->is_excute()) {
            WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        }
        acmd->frame(8);
        if (acmd->is_excute()) {
            acmd->wrap(notify_event_msc_cmd, { L2CValue(hash40("0x2127e37c07")), L2CValue(GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES) });
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 9.0, /*Angle*/ 86, /*KBG*/ 120, /*FKB*/ 125, /*BKB*/ 0, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 8.0, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 9.0, /*Angle*/ 88, /*KBG*/ 120, /*FKB*/ 125, /*BKB*/ 0, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 14.0, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 9.0, /*Angle*/ 86, /*KBG*/ 120, /*FKB*/ 120, /*BKB*/ 0, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 12.0, /*Z*/ 8.0, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 3, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 9.0, /*Angle*/ 88, /*KBG*/ 120, /*FKB*/ 120, /*BKB*/ 0, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 12.0, /*Z*/ 14.0, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
        }
        acmd->frame(9);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 6.0, /*Angle*/ 86, /*KBG*/ 120, /*FKB*/ 100, /*BKB*/ 0, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 12.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 6.0, /*Angle*/ 86, /*KBG*/ 120, /*FKB*/ 100, /*BKB*/ 0, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 2.0, /*Z*/ 12.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
            AttackModule::clear(acmd->module_accessor, /*ID*/ 2, false);
            AttackModule::clear(acmd->module_accessor, /*ID*/ 3, false);
        }
        acmd->frame(10);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 6.0, /*Angle*/ 84, /*KBG*/ 100, /*FKB*/ 68, /*BKB*/ 0, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 12.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 6.0, /*Angle*/ 84, /*KBG*/ 100, /*FKB*/ 68, /*BKB*/ 0, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 2.0, /*Z*/ 12.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
        }
        acmd->frame(13);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
            GroundModule::set_passable_check(acmd->module_accessor, true);
        }
        acmd->frame(22);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("haver"), /*Damage*/ 6.0, /*Angle*/ 80, /*KBG*/ 150, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 7.7, /*X*/ 0.0, /*Y*/ 3.5, /*Z*/ -2.0, /*X2*/ 0.0, /*Y2*/ 10.0, /*Z2*/ -2.0, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("bust"), /*Damage*/ 6.0, /*Angle*/ 80, /*KBG*/ 150, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_BODY);
        }
        acmd->frame(28);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
            GroundModule::set_passable_check(acmd->module_accessor, false);
        }
        acmd->frame(48);
        if (acmd->is_excute()) {
            WorkModule::off_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        }
    }),

    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_METAKNIGHT", "attack_hi3", "game_attackhi3", [] (ACMD* acmd) -> void {
        acmd->frame(8);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 7.0, /*Angle*/ 90, /*KBG*/ 95, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 25.200001, /*Z*/ 0.9, /*X2*/ 0.0, /*Y2*/ 20.5, /*Z2*/ 1.4, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_sting"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 6.0, /*Angle*/ 90, /*KBG*/ 95, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 15.7, /*Z*/ 2.2, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_sting"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 6.0, /*Angle*/ 95, /*KBG*/ 95, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 4.0, /*Z*/ 8.6, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_sting"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 3, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 6.0, /*Angle*/ 95, /*KBG*/ 95, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 4.0, /*Z*/ -10.2, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_sting"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
        }
        acmd->wait(1);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("haver"), /*Damage*/ 7.0, /*Angle*/ 90, /*KBG*/ 95, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 0.0, /*X2*/ 0.0, /*Y2*/ 4.0, /*Z2*/ 0.0, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_sting"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("haver"), /*Damage*/ 6.0, /*Angle*/ 90, /*KBG*/ 95, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ -1.0, /*Z*/ 0.0, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_sting"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
        }
        acmd->wait(2);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("haver"), /*Damage*/ 5.0, /*Angle*/ 90, /*KBG*/ 110, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ -1.0, /*Z*/ 0.0, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_sting"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
        }
        acmd->wait(4);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
    }),

    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_METAKNIGHT", "attack_air_f", "game_attackairf", [] (ACMD* acmd) -> void {
        acmd->frame(2);
        if (acmd->is_excute()) {
            WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        acmd->frame(9);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 1.5, /*Angle*/ 367, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 13.5, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_A, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 1.5, /*Angle*/ 367, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 3.0, /*Z*/ 7.0, /*X2*/ 0.0, /*Y2*/ 9.0, /*Z2*/ 7.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_A, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 1.5, /*Angle*/ 70, /*KBG*/ 100, /*FKB*/ 65, /*BKB*/ 0, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 13.5, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_G, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 3, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 1.5, /*Angle*/ 60, /*KBG*/ 100, /*FKB*/ 65, /*BKB*/ 0, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 3.0, /*Z*/ 7.0, /*X2*/ 0.0, /*Y2*/ 9.0, /*Z2*/ 7.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_G, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
        }
        acmd->wait(1);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
        acmd->frame(12);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 1.5, /*Angle*/ 367, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 13.5, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_A, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 1.5, /*Angle*/ 367, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 3.0, /*Z*/ 7.0, /*X2*/ 0.0, /*Y2*/ 9.0, /*Z2*/ 7.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_A, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 1.5, /*Angle*/ 70, /*KBG*/ 100, /*FKB*/ 65, /*BKB*/ 0, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 13.5, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_G, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 3, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 1.5, /*Angle*/ 60, /*KBG*/ 100, /*FKB*/ 65, /*BKB*/ 0, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 3.0, /*Z*/ 7.0, /*X2*/ 0.0, /*Y2*/ 9.0, /*Z2*/ 7.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_G, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
        }
        acmd->wait(1);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
        acmd->frame(15);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 3.0, /*Angle*/ 72, /*KBG*/ 150, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 13.5, /*Hitlag*/ 2.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 3.0, /*Angle*/ 72, /*KBG*/ 150, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 6.5, /*X*/ 0.0, /*Y*/ 2.5, /*Z*/ 7.0, /*X2*/ 0.0, /*Y2*/ 8.5, /*Z2*/ 7.0, /*Hitlag*/ 2.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
        }
        acmd->wait(1);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
        acmd->frame(44);
        if (acmd->is_excute()) {
            WorkModule::off_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    }),

    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_METAKNIGHT", "attack_hi4", "game_attackhi4", [] (ACMD* acmd) -> void {
        acmd->frame(4);
        if (acmd->is_excute()) {
            WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        }
        acmd->frame(8);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 4.0, /*Angle*/ 366, /*KBG*/ 100, /*FKB*/ 30, /*BKB*/ 30, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 20.0, /*Z*/ -3.8, /*X2*/ 0.0, /*Y2*/ 20.0, /*Z2*/ 3.2, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 4.0, /*Angle*/ 130, /*KBG*/ 100, /*FKB*/ 30, /*BKB*/ 30, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 16.0, /*Z*/ -10.2, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 4.0, /*Angle*/ 130, /*KBG*/ 100, /*FKB*/ 30, /*BKB*/ 30, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 16.0, /*Z*/ 8.6, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 3, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 4.0, /*Angle*/ 115, /*KBG*/ 100, /*FKB*/ 30, /*BKB*/ 30, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 8.6, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 3, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 4.0, /*Angle*/ 115, /*KBG*/ 100, /*FKB*/ 30, /*BKB*/ 30, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ -10.2, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
        }
        acmd->wait(1);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
        acmd->frame(12);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 3.0, /*Angle*/ 366, /*KBG*/ 100, /*FKB*/ 30, /*BKB*/ 30, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 20.0, /*Z*/ -3.8, /*X2*/ 0.0, /*Y2*/ 20.0, /*Z2*/ 3.2, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 3.0, /*Angle*/ 130, /*KBG*/ 100, /*FKB*/ 30, /*BKB*/ 30, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 16.0, /*Z*/ -10.2, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 3.0, /*Angle*/ 130, /*KBG*/ 100, /*FKB*/ 30, /*BKB*/ 30, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 16.0, /*Z*/ 8.6, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
        }
        acmd->wait(1);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
        acmd->frame(17);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 5.0, /*Angle*/ 85, /*KBG*/ 158, /*FKB*/ 0, /*BKB*/ 85, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 17.0, /*Z*/ -10.2, /*Hitlag*/ 2.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 5.0, /*Angle*/ 85, /*KBG*/ 158, /*FKB*/ 0, /*BKB*/ 85, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 17.0, /*Z*/ 8.6, /*Hitlag*/ 2.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 5.0, /*Angle*/ 90, /*KBG*/ 158, /*FKB*/ 0, /*BKB*/ 85, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 20.0, /*Z*/ -3.8, /*X2*/ 0.0, /*Y2*/ 20.0, /*Z2*/ 3.2, /*Hitlag*/ 2.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_SWORD);
        }
        acmd->wait(1);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
    }),

    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_METAKNIGHT", "attack_s4", "game_attacks4", [] (ACMD* acmd) -> void {
        acmd->frame(21);
        if (acmd->is_excute()) {
            WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        }
        acmd->frame(24);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 16.0, /*Angle*/ 361, /*KBG*/ 113, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 6.2, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 9.0, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 16.0, /*Angle*/ 361, /*KBG*/ 103, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 5.2, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 13.0, /*X2*/ 0.0, /*Y2*/ 5.0, /*Z2*/ 17.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_SWORD);
        }
        acmd->wait(1);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
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
        MotionModule::set_rate(acmd->module_accessor, 1.2);
        acmd->frame(21);
        MotionModule::set_rate(acmd->module_accessor, 1);
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
        MotionModule::set_rate(acmd->module_accessor, 1.2);
        acmd->frame(21);
        MotionModule::set_rate(acmd->module_accessor, 1);
    }),
    //Wolf
    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_WOLF", "attack_dash", "game_attackdash", [] (ACMD* acmd) -> void {
        if (acmd->is_excute()) {
            MotionModule::set_rate(acmd->module_accessor, 1.3);
        }
        acmd->frame(8);
        if (acmd->is_excute()) {
            MotionModule::set_rate(acmd->module_accessor, 1);
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("hip"), /*Damage*/ 11.0, /*Angle*/ 80, /*KBG*/ 92, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 2.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ -3.3, /*Y2*/ 0.0, /*Z2*/ -3.3, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("kneel"), /*Damage*/ 11.0, /*Angle*/ 50, /*KBG*/ 91, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("kneel"), /*Damage*/ 11.0, /*Angle*/ 361, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 4.0, /*X*/ 4.3, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
        }
        acmd->wait(4);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("hip"), /*Damage*/ 8.0, /*Angle*/ 80, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 2.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ -3.5, /*Y2*/ 0.0, /*Z2*/ -3.5, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("kneel"), /*Damage*/ 8.0, /*Angle*/ 50, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("kneel"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 4.0, /*X*/ 4.3, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
        }
        acmd->wait(4);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
    }), 

    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_WOLF", "attack_air_f", "game_attackairf", [] (ACMD* acmd) -> void {
        if (acmd->is_excute()) {
            WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        acmd->frame(7);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("handl"), /*Damage*/ 9.0, /*Angle*/ 60, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 4.5, /*X*/ 4.0, /*Y*/ -0.5, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_PUNCH);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("arml"), /*Damage*/ 9.0, /*Angle*/ 60, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 4.0, /*X*/ 1.5, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_PUNCH);
            acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("shoulderl"), /*Damage*/ 9.0, /*Angle*/ 60, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 3.5, /*X*/ -1.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_PUNCH);
        }
        acmd->wait(3);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
        acmd->frame(30);
        if (acmd->is_excute()) {
            WorkModule::off_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    }),

    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_WOLF", "attack_air_n", "game_attackairn", [] (ACMD* acmd) -> void {
        acmd->frame(1);
        MotionModule::set_rate(acmd->module_accessor, 1.55);
        acmd->frame(4);
        MotionModule::set_rate(acmd->module_accessor, 1);
        acmd->frame(5);
        if (acmd->is_excute()) {
            WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("hip"), /*Damage*/ 12.0, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 4.2, /*X*/ -0.6, /*Y*/ 0.0, /*Z*/ 0.5, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("kneel"), /*Damage*/ 12.0, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 4.2, /*X*/ 3.4, /*Y*/ 0.0, /*Z*/ -0.4, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("kneer"), /*Damage*/ 12.0, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 3.5, /*X*/ 2.6, /*Y*/ -0.4, /*Z*/ 0.5, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
        }
        acmd->frame(8);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("hip"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 4.0, /*X*/ -0.6, /*Y*/ 0.0, /*Z*/ 0.5, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("kneel"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 4.0, /*X*/ 3.4, /*Y*/ 0.0, /*Z*/ -0.4, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("kneer"), /*Damage*/ 8.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 3.0, /*X*/ 2.6, /*Y*/ -0.4, /*Z*/ 0.5, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
        }
        acmd->frame(25);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
        acmd->frame(30);
        MotionModule::set_rate(acmd->module_accessor, 0.8);
        acmd->frame(38);
        if (acmd->is_excute()) {
            WorkModule::off_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    }),

    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_WOLF", "attack_air_b", "game_attackairb", [] (ACMD* acmd) -> void {
        acmd->frame(8);
        if (acmd->is_excute()) {
            WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        acmd->frame(13);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("hip"), /*Damage*/ 13.0, /*Angle*/ 36, /*KBG*/ 101, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("kneer"), /*Damage*/ 14.0, /*Angle*/ 36, /*KBG*/ 101, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("kneer"), /*Damage*/ 15.0, /*Angle*/ 36, /*KBG*/ 101, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 5.3, /*X*/ 5.5, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
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

    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_WOLF", "attack_air_hi", "game_attackairhi", [] (ACMD* acmd) -> void {
       acmd->frame(4);
        if (acmd->is_excute()) {
            WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        acmd->frame(7);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("handr"), /*Damage*/ 12.0, /*Angle*/ 80, /*KBG*/ 95, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 5.0, /*X*/ 2.0, /*Y*/ -0.5, /*Z*/ 0.2, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_PUNCH);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("armr"), /*Damage*/ 12.0, /*Angle*/ 80, /*KBG*/ 95, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 4.0, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_PUNCH);
            acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("shoulderr"), /*Damage*/ 12.0, /*Angle*/ 80, /*KBG*/ 95, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 4.0, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_PUNCH);
        }
        acmd->wait(3);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
        acmd->frame(31);
        if (acmd->is_excute()) {
            WorkModule::off_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        } 
    }),

    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_WOLF", "attack_s3hi", "game_attacks3hi", [] (ACMD* acmd) -> void {
        acmd->frame(8);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 5.0, /*Angle*/ 60, /*KBG*/ 60, /*FKB*/ 30, /*BKB*/ 0, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 11.0, /*Z*/ 14.0, /*Hitlag*/ 1.8, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_PUNCH);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 5.0, /*Angle*/ 60, /*KBG*/ 60, /*FKB*/ 30, /*BKB*/ 0, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 11.0, /*Hitlag*/ 1.8, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_PUNCH);
            acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 5.0, /*Angle*/ 60, /*KBG*/ 60, /*FKB*/ 30, /*BKB*/ 0, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 6.0, /*Hitlag*/ 1.8, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_PUNCH);
            AttackModule::set_attack_height_all(acmd->module_accessor, ATTACK_HEIGHT_HIGH, false);
        }
        acmd->frame(9);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 106, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 13.0, /*Z*/ 14.0, /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_PUNCH);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 106, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 11.0, /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_PUNCH);
            acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 106, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 7.0, /*Z*/ 6.0, /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_PUNCH);
            AttackModule::set_attack_height_all(acmd->module_accessor, ATTACK_HEIGHT_HIGH, false);
        }
        acmd->wait(2);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
    }),

    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_WOLF", "attack_s3", "game_attacks3", [] (ACMD* acmd) -> void {
        acmd->frame(8);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 5.0, /*Angle*/ 60, /*KBG*/ 70, /*FKB*/ 30, /*BKB*/ 0, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 6.5, /*Z*/ 16.0, /*Hitlag*/ 1.8, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_PUNCH);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 5.0, /*Angle*/ 60, /*KBG*/ 70, /*FKB*/ 30, /*BKB*/ 0, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 11.0, /*Hitlag*/ 1.8, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_PUNCH);
            acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 5.0, /*Angle*/ 60, /*KBG*/ 70, /*FKB*/ 30, /*BKB*/ 0, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 6.0, /*Hitlag*/ 1.8, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_PUNCH);
        }
        acmd->frame(9);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 106, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 6.5, /*Z*/ 16.0, /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_PUNCH);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 106, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 11.0, /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_PUNCH);
            acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 106, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 6.0, /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_PUNCH);
        }
        acmd->wait(2);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
    }),

    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_WOLF", "attack_s3lw", "game_attacks3lw", [] (ACMD* acmd) -> void {
        acmd->frame(8);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 5.0, /*Angle*/ 10, /*KBG*/ 70, /*FKB*/ 30, /*BKB*/ 0, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 3.0, /*Z*/ 16.0, /*Hitlag*/ 1.8, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_PUNCH);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 5.0, /*Angle*/ 10, /*KBG*/ 70, /*FKB*/ 30, /*BKB*/ 0, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ 10.5, /*Hitlag*/ 1.8, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_PUNCH);
            acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 5.0, /*Angle*/ 10, /*KBG*/ 70, /*FKB*/ 30, /*BKB*/ 0, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 6.0, /*Hitlag*/ 1.8, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_PUNCH);
            AttackModule::set_attack_height_all(acmd->module_accessor, ATTACK_HEIGHT_LOW, false);
        }
        acmd->frame(9);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 106, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 3.0, /*Z*/ 16.0, /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_PUNCH);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 106, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ 10.5, /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_PUNCH);
            acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 106, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 6.0, /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_PUNCH);
            AttackModule::set_attack_height_all(acmd->module_accessor, ATTACK_HEIGHT_LOW, false);
        }
        acmd->wait(2);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
    }),

    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_WOLF", "attack_13", "game_attack13", [] (ACMD* acmd) -> void {
        acmd->frame(4);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 4.0, /*Angle*/ 70, /*KBG*/ 140, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 2.5, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 2.5, /*Hitlag*/ 2.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_PUNCH);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 4.0, /*Angle*/ 70, /*KBG*/ 140, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 6.0, /*Hitlag*/ 2.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_PUNCH);
            acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 4.0, /*Angle*/ 70, /*KBG*/ 140, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 11.0, /*Hitlag*/ 2.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_PUNCH);
        }
        acmd->wait(1);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
    }),

    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_WOLF", "attack_hi3", "game_attackhi3", [] (ACMD* acmd) -> void {
        acmd->frame(7);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("legr"), /*Damage*/ 10.0, /*Angle*/ 85, /*KBG*/ 115, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 4.5, /*X*/ -5.0, /*Y*/ 5.4, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_G, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("legr"), /*Damage*/ 8.0, /*Angle*/ 75, /*KBG*/ 115, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("kneer"), /*Damage*/ 9.0, /*Angle*/ 80, /*KBG*/ 110, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 1.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            acmd->ATTACK(/*ID*/ 3, /*Part*/ 0, /*Bone*/ hash40("kneer"), /*Damage*/ 10.0, /*Angle*/ 85, /*KBG*/ 115, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 6.5, /*X*/ 6.0, /*Y*/ 1.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
        }
        acmd->wait(2);
        if (acmd->is_excute()) {
            AttackModule::clear(acmd->module_accessor, /*ID*/ 0, false);
        }
        acmd->wait(3);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
    }), 

    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_WOLF", "attack_lw4", "game_attacklw4", [] (ACMD* acmd) -> void {
        acmd->frame(1);
        MotionModule::set_rate(acmd->module_accessor, 1.6);
        acmd->frame(2);
        if (acmd->is_excute()) {
            WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        }
        acmd->frame(9);
        MotionModule::set_rate(acmd->module_accessor, 1);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("shoulderr"), /*Damage*/ 14.0, /*Angle*/ 35, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 3.5, /*X*/ -1.0, /*Y*/ -0.6, /*Z*/ -0.6, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.35, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_PUNCH);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("armr"), /*Damage*/ 16.0, /*Angle*/ 30, /*KBG*/ 103, /*FKB*/ 0, /*BKB*/ 37, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 0.5, /*Z*/ 0.5, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.35, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_PUNCH);
            acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("handr"), /*Damage*/ 16.0, /*Angle*/ 30, /*KBG*/ 103, /*FKB*/ 0, /*BKB*/ 37, /*Size*/ 5.5, /*X*/ 3.0, /*Y*/ -0.5, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.35, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_PUNCH);
            AttackModule::set_attack_height_all(acmd->module_accessor, ATTACK_HEIGHT_LOW, false);
        }
        acmd->wait(2);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
        acmd->frame(16);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("shoulderr"), /*Damage*/ 13.0, /*Angle*/ 35, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 2.5, /*X*/ -0.3, /*Y*/ 0.0, /*Z*/ -2.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.35, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_PUNCH);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("armr"), /*Damage*/ 15.0, /*Angle*/ 30, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 3.0, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.35, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_PUNCH);
            acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("handr"), /*Damage*/ 15.0, /*Angle*/ 30, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 3.5, /*X*/ 4.0, /*Y*/ -0.5, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.35, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_CUTUP, /*Type*/ ATTACK_REGION_PUNCH);
            AttackModule::set_attack_height_all(acmd->module_accessor, ATTACK_HEIGHT_LOW, false);
        }
        acmd->wait(2);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
        MotionModule::set_rate(acmd->module_accessor, 1.2);
    }),

    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_WOLF", "attack_s4", "game_attacks4", [] (ACMD* acmd) -> void {
        acmd->frame(1);
        MotionModule::set_rate(acmd->module_accessor, 1.7);
        acmd->frame(4);
        if (acmd->is_excute()) {
            WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        }
        acmd->frame(11);
        MotionModule::set_rate(acmd->module_accessor, 1);
        acmd->frame(13);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("arml"), /*Damage*/ 15.0, /*Angle*/ 361, /*KBG*/ 116, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 3.5, /*X*/ 3.0, /*Y*/ 0.0, /*Z*/ -1.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_PUNCH, /*Type*/ ATTACK_REGION_PUNCH);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("arml"), /*Damage*/ 15.0, /*Angle*/ 361, /*KBG*/ 116, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 3.0, /*X*/ -1.5, /*Y*/ 0.0, /*Z*/ -0.3, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_PUNCH, /*Type*/ ATTACK_REGION_PUNCH);
        }
        acmd->wait(4);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
        MotionModule::set_rate(acmd->module_accessor, 0.6);
    }), 

    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_WOLF", "attack_hi4", "game_attackhi4", [] (ACMD* acmd) -> void {
        acmd->frame(4);
        if (acmd->is_excute()) {
            WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        }
        acmd->frame(13);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 6.0, /*Angle*/ 110, /*KBG*/ 15, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 7.0, /*Z*/ 4.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 6.0, /*Angle*/ 110, /*KBG*/ 15, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 7.0, /*Z*/ -4.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 6.0, /*Angle*/ 125, /*KBG*/ 15, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 7.0, /*Z*/ 10.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            acmd->ATTACK(/*ID*/ 3, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 6.0, /*Angle*/ 125, /*KBG*/ 15, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 7.0, /*Z*/ -10.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
        }
        acmd->wait(3);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
        acmd->frame(20);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 12.0, /*Angle*/ 95, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 85, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 23.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 12.0, /*Angle*/ 95, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 85, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 18.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 12.0, /*Angle*/ 95, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 85, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 13.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
        }
        acmd->wait(2);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 12.0, /*Angle*/ 95, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 85, /*Size*/ 8.0, /*X*/ 0.0, /*Y*/ 26.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 12.0, /*Angle*/ 95, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 85, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 21.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 12.0, /*Angle*/ 95, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 85, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 16.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_THRU, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_normal"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
        }
        acmd->wait(2);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
    }),

    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_WOLF", "special_air_s_end", "game_specialairsend", [] (ACMD* acmd) -> void {
        acmd->frame(2);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 20.0, /*Angle*/ 270, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 5.5, /*Z*/ 5.5, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_A, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_elec"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_MAGIC, /*Type*/ ATTACK_REGION_PUNCH);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 17.0, /*Angle*/ 48, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 5.5, /*Z*/ 5.5, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_elec"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_MAGIC, /*Type*/ ATTACK_REGION_PUNCH);
        }
        acmd->frame(4);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
        acmd->frame(7);
        if (acmd->is_excute()) {
            acmd->wrap(notify_event_msc_cmd, { L2CValue(hash40("0x2127e37c07")), L2CValue(GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES) });
        }
        acmd->frame(10);
        if (acmd->is_excute()) {
            JostleModule::set_status(acmd->module_accessor, true);
        }
    }),

    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_WOLF", "special_s_end", "game_specialsend", [] (ACMD* acmd) -> void {
        acmd->frame(2);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 20.0, /*Angle*/ 270, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 5.5, /*Z*/ 5.5, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_A, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_elec"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_MAGIC, /*Type*/ ATTACK_REGION_PUNCH);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 17.0, /*Angle*/ 48, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 5.5, /*Z*/ 5.5, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_elec"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_MAGIC, /*Type*/ ATTACK_REGION_PUNCH);
        }
        acmd->frame(4);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
        acmd->frame(7);
        if (acmd->is_excute()) {
            acmd->wrap(notify_event_msc_cmd, { L2CValue(hash40("0x2127e37c07")), L2CValue(GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES) });
        }
        acmd->frame(10);
        if (acmd->is_excute()) {
            JostleModule::set_status(acmd->module_accessor, true);
        }
    })
   /* ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_SHEIK", "attack_dash", "game_attackdash", [] (ACMD* acmd) -> void {
        
    }),
    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_SHEIK", "attack_s3s", "game_attacks3s", [] (ACMD* acmd) -> void {
        
    }),
    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_SHEIK", "attack_lw3", "game_attacklw3", [] (ACMD* acmd) -> void {
        
    }),
    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_SHEIK", "attack_hi3", "game_attackhi3", [] (ACMD* acmd) -> void {
        
    }),
    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_SHEIK", "attack_air_hi", "game_attackairhi", [] (ACMD* acmd) -> void {
        
    }),
    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_SHEIK", "attack_air_f", "game_attackairf", [] (ACMD* acmd) -> void {
        
    }),
    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_SHEIK", "attack_air_b", "game_attackairb", [] (ACMD* acmd) -> void {
        
    }),
    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_SHEIK", "attack_air_lw", "game_attackairlw", [] (ACMD* acmd) -> void {
        
    }),
    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_SHEIK", "attack_air_n", "game_attackairn", [] (ACMD* acmd) -> void {
        
    }),
    */
};