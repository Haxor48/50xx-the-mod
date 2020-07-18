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
    //Cloud
    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_CLOUD", "attack_air_n", "game_attackairn", [] (ACMD* acmd) -> void {
        acmd->frame(2);
        if (acmd->is_excute()) {
            WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        acmd->frame(5);
            MotionModule::set_rate(acmd->module_accessor, 0.75);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("haver"), /*Damage*/ 8.0, /*Angle*/ 40, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 3.8, /*X*/ 3.0, /*Y*/ 14.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_CLOUD_HIT, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("haver"), /*Damage*/ 8.0, /*Angle*/ 40, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 3.8, /*X*/ 3.0, /*Y*/ 9.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_CLOUD_HIT, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("haver"), /*Damage*/ 8.0, /*Angle*/ 40, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 3.8, /*X*/ 3.0, /*Y*/ 4.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_CLOUD_HIT, /*Type*/ ATTACK_REGION_SWORD);
        }
        acmd->frame(19);
        if (acmd->is_excute()) {
        AttackModule::clear_all(acmd->module_accessor);
        }
        MotionModule::set_rate(acmd->module_accessor, 1);
        acmd->frame(34);
        if (acmd->is_excute()) {
            WorkModule::off_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    }),

    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_CLOUD", "attack_air_hi", "game_attackairhi", [] (ACMD* acmd) -> void {
        acmd->frame(2);
        if (acmd->is_excute()) {
            WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        acmd->frame(7);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 12.0, /*Angle*/ 80, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 57, /*Size*/ 4.2, /*X*/ 0.0, /*Y*/ 24.0, /*Z*/ 0.0, /*X2*/ 0.0, /*Y2*/ 17.5, /*Z2*/ 0.0, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CLOUD_HIT, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 12.0, /*Angle*/ 80, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 57, /*Size*/ 4.2, /*X*/ 0.0, /*Y*/ 25.0, /*Z*/ 4.5, /*X2*/ 0.0, /*Y2*/ 17.5, /*Z2*/ 4.5, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CLOUD_HIT, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 12.0, /*Angle*/ 80, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 57, /*Size*/ 4.2, /*X*/ 0.0, /*Y*/ 26.0, /*Z*/ 9.0, /*X2*/ 0.0, /*Y2*/ 17.5, /*Z2*/ 9.0, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_CLOUD_HIT, /*Type*/ ATTACK_REGION_SWORD);
        }
        acmd->wait(3);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("haver"), /*Damage*/ 8.5, /*Angle*/ 80, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 57, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 4.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_CLOUD_HIT, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("haver"), /*Damage*/ 8.5, /*Angle*/ 80, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 57, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_CLOUD_HIT, /*Type*/ ATTACK_REGION_SWORD);
            acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("haver"), /*Damage*/ 8.5, /*Angle*/ 80, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 57, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 0.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_cutup"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_CLOUD_HIT, /*Type*/ ATTACK_REGION_SWORD);
        }
        acmd->wait(17);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
        acmd->frame(32);
        if (acmd->is_excute()) {
            WorkModule::off_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    }),
    // Squirtle
    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_PZENIGAME", "attack_air_f", "game_attackairf", [] (ACMD* acmd) -> void {
        acmd->frame(5);
        if (acmd->is_excute()) {
            WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 7.0, /*Angle*/ 48, /*KBG*/ 120, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 7.5, /*X2*/ 0.0, /*Y2*/ 5.0, /*Z2*/ 7.5, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_water"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 7.0, /*Angle*/ 48, /*KBG*/ 120, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 4.0, /*X2*/ 0.0, /*Y2*/ 5.0, /*Z2*/ 4.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_water"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
        }
        acmd->wait(2);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 6.0, /*Angle*/ 48, /*KBG*/ 116, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 7.5, /*X2*/ 0.0, /*Y2*/ 5.0, /*Z2*/ 7.5, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_water"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 6.0, /*Angle*/ 48, /*KBG*/ 116, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 4.0, /*X2*/ 0.0, /*Y2*/ 5.0, /*Z2*/ 4.0, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_water"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_M, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_KICK);
        }
        acmd->wait(8);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
        acmd->frame(35);
        if (acmd->is_excute()) {
            WorkModule::off_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    }),

    ACMD ("BATTLE_OBJECT_CATEGORY_FIGHTER", "FIGHTER_KIND_PZENIGAME", "attack_air_lw", "game_attackairlw", [] (ACMD* acmd) -> void {
        acmd->frame(6);
        if (acmd->is_excute()) {
            WorkModule::on_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 1.5, /*Angle*/ 98, /*KBG*/ 100, /*FKB*/ 10, /*BKB*/ 0, /*Size*/ 4.2, /*X*/ 0.0, /*Y*/ 1.0, /*Z*/ 0.0, /*Hitlag*/ 0.8, /*SDI*/ 0.3, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 3, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_water"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_TAIL);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 1.5, /*Angle*/ 98, /*KBG*/ 100, /*FKB*/ 10, /*BKB*/ 0, /*Size*/ 4.4, /*X*/ 0.0, /*Y*/ -1.5, /*Z*/ 0.0, /*Hitlag*/ 0.8, /*SDI*/ 0.3, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 3, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_water"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_TAIL);
            acmd->ATTACK(/*ID*/ 2, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 1.5, /*Angle*/ 98, /*KBG*/ 100, /*FKB*/ 10, /*BKB*/ 0, /*Size*/ 4.6, /*X*/ 0.0, /*Y*/ -4.0, /*Z*/ 0.0, /*Hitlag*/ 0.8, /*SDI*/ 0.3, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 3, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_water"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_S, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_TAIL);
        }
        acmd->wait(15);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
        acmd->frame(22);
        if (acmd->is_excute()) {
            acmd->ATTACK(/*ID*/ 0, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 4.0, /*Angle*/ 35, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*Hitlag*/ 2.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_water"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_TAIL);
            acmd->ATTACK(/*ID*/ 1, /*Part*/ 0, /*Bone*/ hash40("top"), /*Damage*/ 4.0, /*Angle*/ 35, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ -4.5, /*Z*/ 0.0, /*Hitlag*/ 2.0, /*SDI*/ 1.0, /*Clang/Rebound*/ ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ COLLISION_SITUATION_MASK_GA, /*Hitbits*/ COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ hash40("collision_attr_water"), /*SFXLevel*/ ATTACK_SOUND_LEVEL_L, /*SFXType*/ COLLISION_SOUND_ATTR_KICK, /*Type*/ ATTACK_REGION_TAIL);
        }
        acmd->wait(1);
        if (acmd->is_excute()) {
            AttackModule::clear_all(acmd->module_accessor);
        }
        acmd->frame(33);
        if (acmd->is_excute()) {
            WorkModule::off_flag(acmd->module_accessor, /*Flag*/ FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
    })
};