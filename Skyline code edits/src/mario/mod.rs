use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::lua2cpp::L2CFighterBase;
use acmd::{acmd, acmd_func};
use smash::app::lua_bind::*;
use crate::FIGHTER_CUTIN_MANAGER_ADDR;
use skyline::nn::ro::LookupSymbol;
use smash::phx::Vector3f;
use smash::app;

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MARIO, 
    animation = "attack_air_f",
    animcmd = "game_attackairf")]
pub fn mario_fair(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=3)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=16)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=12.0, Angle=361, KBG=95, FKB=0, BKB=45, Size=3.0, X=3.2, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        }
        wait(Frames=1)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("shoulderr"), Damage=14.0, Angle=280, KBG=78, FKB=0, BKB=32, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=14.0, Angle=280, KBG=78, FKB=0, BKB=32, Size=4.6, X=3.4, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        }
        wait(Frames=4)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("shoulderr"), Damage=10.0, Angle=361, KBG=80, FKB=0, BKB=20, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=10.0, Angle=361, KBG=80, FKB=0, BKB=20, Size=4.6, X=3.2, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=43)
        if(is_excute){
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MARIO, 
    animation = "attack_air_lw",
    animcmd = "game_attackairlw")]
pub fn mario_dair(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=5)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        for(5 Iterations){
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.4, Angle=94, KBG=15, FKB=0, BKB=50, Size=4.0, X=0.0, Y=-0.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.8, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_BODY)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.4, Angle=94, KBG=15, FKB=0, BKB=25, Size=7.0, X=0.0, Y=6.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.8, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_BODY)
            }
            wait(Frames=1)
            if(is_excute){
            AttackModule::clear_all()
            }
            wait(Frames=1)
        }
        frame(Frame=23)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.5, Angle=75, KBG=105, FKB=0, BKB=90, Size=10.0, X=0.0, Y=6.8, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.8, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_BODY)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=33)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MARIO, 
    animation = "attack_dash",
    animcmd = "game_attackdash")]
pub fn mario_da(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=6)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=8.0, Angle=43, KBG=58, FKB=0, BKB=100, Size=3.5, X=0.0, Y=1.5, Z=5.4, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            FighterAreaModuleImpl::enable_fix_jostle_area(4.0, 6.0)
        }
        wait(Frames=4)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.0, Angle=120, KBG=37, FKB=0, BKB=87, Size=2.7, X=0.0, Y=1.5, Z=4.9, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        wait(Frames=16)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=32)
        if(is_excute){
            FighterAreaModuleImpl::enable_fix_jostle_area(4.0, 4.0)
        }
        frame(Frame=41)
        if(is_excute){
            FighterAreaModuleImpl::enable_fix_jostle_area(3.0, 3.0)
        }
    });
}
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MARIO, 
    animation = "attack_hi3",
    animcmd = "game_attackhi3")]
pub fn mario_up_tilt(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=5)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("head"), Damage=5.5, Angle=96, KBG=130, FKB=0, BKB=48, Size=3.5, X=-0.5, Y=-0.8, Z=0.2, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=5.5, Angle=96, KBG=130, FKB=0, BKB=48, Size=4.2, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=2, Part=0, Bone=hash40("armr"), Damage=5.5, Angle=96, KBG=130, FKB=0, BKB=48, Size=5.0, X=4.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        }
        wait(Frames=7)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MARIO, 
    animation = "attack_lw3",
    animcmd = "game_attacklw3")]
pub fn mario_down_tilt(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=5)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("kneel"), Damage=7.0, Angle=110, KBG=100, FKB=0, BKB=50, Size=3.2, X=-1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.4, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=1, Part=0, Bone=hash40("toel"), Damage=5.0, Angle=110, KBG=100, FKB=0, BKB=50, Size=4.2, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.4, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            AttackModule::set_attack_height_all(smash::cpp::root::app::AttackHeight(*ATTACK_HEIGHT_LOW), false)
        }
        wait(Frames=3)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MARIO, 
    animation = "attack_air_b",
    animcmd = "game_attackairb")]
pub fn mario_bair(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=6)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
            ATTACK(ID=0, Part=0, Bone=hash40("kneer"), Damage=10.5, Angle=361, KBG=113, FKB=0, BKB=22, Size=4.5, X=4.8, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=1, Part=0, Bone=hash40("legr"), Damage=10.5, Angle=361, KBG=113, FKB=0, BKB=22, Size=5.9, X=1.6, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        wait(Frames=2)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("kneer"), Damage=7.0, Angle=361, KBG=95, FKB=0, BKB=13, Size=4.5, X=4.8, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=1, Part=0, Bone=hash40("legr"), Damage=7.0, Angle=361, KBG=95, FKB=0, BKB=13, Size=5.3, X=1.6, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        wait(Frames=3)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=19)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MARIO, 
    animation = "attack_hi4",
    animcmd = "game_attackhi4")]
pub fn mario_usmash(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=7)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
            JostleModule::set_status(false)
        }
        frame(Frame=9)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("head"), Damage=14.0, Angle=83, KBG=94, FKB=0, BKB=50, Size=5.0, X=2.5, Y=1.1, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_HEAD)
            ATTACK(ID=1, Part=0, Bone=hash40("bust"), Damage=14.0, Angle=83, KBG=94, FKB=0, BKB=50, Size=4.0, X=1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_HEAD)
            HIT_NODE(hash40("head"), HIT_STATUS_XLU, 0)
        }
        wait(Frames=4)
        if(is_excute){
            AttackModule::clear_all()
            HitModule::set_status_all(smash::cpp::root::app::HitStatus(*HIT_STATUS_NORMAL), 0)
        }
        frame(Frame=20)
        if(is_excute){
            JostleModule::set_status(true)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MARIO, 
    animation = "attack_lw4",
    animcmd = "game_attacklw4")]
pub fn mario_dsmash(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=3)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        }
        frame(Frame=5)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=10.0, Angle=45, KBG=105, FKB=0, BKB=40, Size=4.0, X=0.0, Y=3.6, Z=12.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=10.0, Angle=45, KBG=105, FKB=0, BKB=40, Size=3.3, X=0.0, Y=3.6, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            AttackModule::set_attack_height_all(smash::cpp::root::app::AttackHeight(*ATTACK_HEIGHT_LOW), false)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=14)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=42, KBG=105, FKB=0, BKB=40, Size=4.0, X=0.0, Y=3.6, Z=-11.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=12.0, Angle=42, KBG=105, FKB=0, BKB=40, Size=3.3, X=0.0, Y=3.6, Z=-6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            AttackModule::set_attack_height_all(smash::cpp::root::app::AttackHeight( *ATTACK_HEIGHT_LOW), false)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MARIO, 
    animation = "attack_s4_hi",
    animcmd = "game_attacks4hi")]
pub fn mario_fsmash_hi(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=6)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        }
        frame(Frame=15)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("arml"), Damage=14.7, Angle=361, KBG=110, FKB=0, BKB=33, Size=2.0, X=-1.0, Y=0.7, Z=0.0, X2=-3.0, Y2=1.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("arml"), Damage=17.799999, Angle=361, KBG=107, FKB=0, BKB=33, Size=5.0, X=5.4, Y=0.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_PUNCH)
        }
        wait(Frames=3)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MARIO, 
    animation = "attack_s4_s",
    animcmd = "game_attacks4s")]
pub fn mario_fsmash_mid(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=6)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        }
        frame(Frame=15)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("arml"), Damage=14.7, Angle=361, KBG=110, FKB=0, BKB=33, Size=2.0, X=-1.0, Y=0.7, Z=0.0, X2=-3.0, Y2=1.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("arml"), Damage=17.799999, Angle=361, KBG=105, FKB=0, BKB=33, Size=5.0, X=5.4, Y=0.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_PUNCH)
        }
        wait(Frames=3)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MARIO, 
    animation = "attack_s4_lw",
    animcmd = "game_attacks4lw")]
pub fn mario_fsmash_lw(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=6)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        }
        frame(Frame=15)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("arml"), Damage=14.7, Angle=361, KBG=110, FKB=0, BKB=33, Size=2.0, X=-1.0, Y=0.7, Z=0.0, X2=-3.0, Y2=1.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("arml"), Damage=17.799999, Angle=361, KBG=103, FKB=0, BKB=33, Size=5.0, X=5.4, Y=0.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_PUNCH)
        }
        wait(Frames=3)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_WEAPON, 
    battle_object_kind = WEAPON_KIND_MARIO_PUMP_WATER, 
    animation = "regular",
    animcmd = "game_regular")]
pub fn flood(fighter: &mut L2CFighterBase) {
    acmd!({
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage= 9.0, Angle=55, KBG=100, FKB=70, BKB=0, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=true, DisableHitlag=true, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_water"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_MARIO_WATER_PUMP, Type=ATTACK_REGION_OBJECT)
            AttackModule::enable_safe_pos()
            }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MARIO, 
    animation = "attack_air_hi",
    animcmd = "game_attackairhi")]
pub fn mario_uair(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=1)
        FT_MOTION_RATE(FSM=0.75)
        frame(Frame=2)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=5)
        FT_MOTION_RATE(FSM=1)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("legr"), Damage=7.0, Angle=75, KBG=135, FKB=0, BKB=38, Size=4.4, X=1.2, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=7.0, Angle=75, KBG=135, FKB=0, BKB=38, Size=5.5, X=3.8, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        frame(Frame=10)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=18)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MARIO, 
    animation = "special_hi",
    animcmd = "game_specialhi")]
pub fn mario_upB_grnd(fighter: &mut L2CFighterCommon) {
    acmd!({
        rust {
            if WorkModule::is_flag(module_accessor, *FIGHTER_MARIO_STATUS_SPECIAL_HI_FLAG_CAPPY) {
                acmd! ({
                    frame(Frame=3)
                    if(is_excute){
                        SA_SET(State=SITUATION_KIND_AIR)
                        WorkModule::on_flag(Flag=FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR)
                        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=60, KBG=100, FKB=160, BKB=0, Size=2.5, X=0.0, Y=6.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_mario_local_coin"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, Type=ATTACK_REGION_PUNCH)
                        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=5.0, Angle=86, KBG=100, FKB=150, BKB=0, Size=4.0, X=0.0, Y=6.5, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_mario_local_coin"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, Type=ATTACK_REGION_PUNCH)
                        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=5.0, Angle=100, KBG=100, FKB=150, BKB=0, Size=4.0, X=0.0, Y=6.3, Z=9.2, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_mario_local_coin"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, Type=ATTACK_REGION_PUNCH)
                        AttackModule::set_no_damage_fly_smoke_all(true, false)
                    }
                    wait(Frames=3)
                    if(is_excute){
                        WorkModule::on_flag(Flag=FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS)
                    }
                    wait(Frames=1)
                    if(is_excute){
                        AttackModule::clear_all()
                        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.6, Angle=60, KBG=100, FKB=180, BKB=0, Size=3.0, X=0.0, Y=6.5, Z=2.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_mario_local_coin"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, Type=ATTACK_REGION_PUNCH)
                        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=0.6, Angle=92, KBG=100, FKB=170, BKB=0, Size=3.8, X=0.0, Y=6.5, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_mario_local_coin"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, Type=ATTACK_REGION_PUNCH)
                        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=0.6, Angle=60, KBG=100, FKB=110, BKB=0, Size=3.0, X=0.0, Y=11.5, Z=2.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_mario_local_coin"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, Type=ATTACK_REGION_PUNCH)
                        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=0.6, Angle=92, KBG=100, FKB=110, BKB=0, Size=3.8, X=0.0, Y=11.5, Z=8.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_mario_local_coin"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, Type=ATTACK_REGION_PUNCH)
                        AttackModule::set_no_finish_camera(0, true, false)
                        AttackModule::set_no_finish_camera(1, true, false)
                        AttackModule::set_no_finish_camera(2, true, false)
                        AttackModule::set_no_finish_camera(3, true, false)
                        AttackModule::set_no_damage_fly_smoke_all(true, false)
                    }
                    frame(Frame=13)
                    if(is_excute){
                        sv_battle_object::notify_event_msc_cmd(0x2127e37c07, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
                    }
                    frame(Frame=17)
                    if(is_excute){
                        AttackModule::clear_all()
                        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=60, KBG=145, FKB=0, BKB=50, Size=9.0, X=0.0, Y=11.5, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_mario_local_coin"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN_LAST, Type=ATTACK_REGION_PUNCH)
                        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=4.0, Angle=60, KBG=145, FKB=0, BKB=50, Size=9.0, X=0.0, Y=11.5, Z=2.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_mario_local_coin"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN_LAST, Type=ATTACK_REGION_PUNCH)
                    }
                });
            }
            else {
                acmd! ({
                    frame(Frame=3)
                    if(is_excute){
                        SA_SET(State=SITUATION_KIND_AIR)
                        WorkModule::on_flag(Flag=FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR)
                        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=60, KBG=100, FKB=160, BKB=0, Size=2.5, X=0.0, Y=6.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_coin"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_COIN, Type=ATTACK_REGION_PUNCH)
                        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=5.0, Angle=86, KBG=100, FKB=150, BKB=0, Size=4.0, X=0.0, Y=6.5, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_coin"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_COIN, Type=ATTACK_REGION_PUNCH)
                        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=5.0, Angle=100, KBG=100, FKB=150, BKB=0, Size=4.0, X=0.0, Y=6.3, Z=9.2, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_coin"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_COIN, Type=ATTACK_REGION_PUNCH)
                        AttackModule::set_no_damage_fly_smoke_all(true, false)
                    }
                    wait(Frames=3)
                    if(is_excute){
                        WorkModule::on_flag(Flag=FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS)
                    }
                    wait(Frames=1)
                    if(is_excute){
                        AttackModule::clear_all()
                        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.6, Angle=60, KBG=100, FKB=180, BKB=0, Size=3.0, X=0.0, Y=6.5, Z=2.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_coin"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_COIN, Type=ATTACK_REGION_PUNCH)
                        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=0.6, Angle=92, KBG=100, FKB=170, BKB=0, Size=3.8, X=0.0, Y=6.5, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_coin"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_COIN, Type=ATTACK_REGION_PUNCH)
                        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=0.6, Angle=60, KBG=100, FKB=110, BKB=0, Size=3.0, X=0.0, Y=11.5, Z=2.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_coin"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_COIN, Type=ATTACK_REGION_PUNCH)
                        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=0.6, Angle=92, KBG=100, FKB=110, BKB=0, Size=3.8, X=0.0, Y=11.5, Z=8.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_coin"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_COIN, Type=ATTACK_REGION_PUNCH)
                        AttackModule::set_no_finish_camera(0, true, false)
                        AttackModule::set_no_finish_camera(1, true, false)
                        AttackModule::set_no_finish_camera(2, true, false)
                        AttackModule::set_no_finish_camera(3, true, false)
                        AttackModule::set_no_damage_fly_smoke_all(true, false)
                    }
                    frame(Frame=13)
                    if(is_excute){
                        sv_battle_object::notify_event_msc_cmd(0x2127e37c07, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
                    }
                    frame(Frame=17)
                    if(is_excute){
                        AttackModule::clear_all()
                        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=60, KBG=145, FKB=0, BKB=50, Size=9.0, X=0.0, Y=11.5, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_coin"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARIO_COIN_LAST, Type=ATTACK_REGION_PUNCH)
                        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=4.0, Angle=60, KBG=145, FKB=0, BKB=50, Size=9.0, X=0.0, Y=11.5, Z=2.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_coin"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARIO_COIN_LAST, Type=ATTACK_REGION_PUNCH)
                    }
                    wait(Frames=2)
                    if(is_excute){
                        AttackModule::clear_all()
                    }
                });
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MARIO, 
    animation = "special_air_hi",
    animcmd = "game_specialairhi")]
pub fn mario_upB_air(fighter: &mut L2CFighterCommon) {
    acmd!({
        rust {
            if WorkModule::is_flag(module_accessor, *FIGHTER_MARIO_STATUS_SPECIAL_HI_FLAG_CAPPY) {
                acmd! ({
                    frame(Frame=3)
                    if(is_excute){
                        SA_SET(State=SITUATION_KIND_AIR)
                        WorkModule::on_flag(Flag=FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR)
                        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=60, KBG=100, FKB=160, BKB=0, Size=2.5, X=0.0, Y=6.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_mario_local_coin"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, Type=ATTACK_REGION_PUNCH)
                        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=5.0, Angle=86, KBG=100, FKB=150, BKB=0, Size=4.0, X=0.0, Y=6.5, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_mario_local_coin"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, Type=ATTACK_REGION_PUNCH)
                        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=5.0, Angle=100, KBG=100, FKB=150, BKB=0, Size=4.0, X=0.0, Y=6.3, Z=9.2, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_mario_local_coin"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, Type=ATTACK_REGION_PUNCH)
                        AttackModule::set_no_damage_fly_smoke_all(true, false)
                    }
                    wait(Frames=3)
                    if(is_excute){
                        WorkModule::on_flag(Flag=FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS)
                    }
                    wait(Frames=1)
                    if(is_excute){
                        AttackModule::clear_all()
                        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.6, Angle=60, KBG=100, FKB=180, BKB=0, Size=3.0, X=0.0, Y=6.5, Z=2.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_mario_local_coin"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, Type=ATTACK_REGION_PUNCH)
                        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=0.6, Angle=92, KBG=100, FKB=170, BKB=0, Size=3.8, X=0.0, Y=6.5, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_mario_local_coin"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, Type=ATTACK_REGION_PUNCH)
                        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=0.6, Angle=60, KBG=100, FKB=110, BKB=0, Size=3.0, X=0.0, Y=11.5, Z=2.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_mario_local_coin"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, Type=ATTACK_REGION_PUNCH)
                        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=0.6, Angle=92, KBG=100, FKB=110, BKB=0, Size=3.8, X=0.0, Y=11.5, Z=8.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_mario_local_coin"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, Type=ATTACK_REGION_PUNCH)
                        AttackModule::set_no_finish_camera(0, true, false)
                        AttackModule::set_no_finish_camera(1, true, false)
                        AttackModule::set_no_finish_camera(2, true, false)
                        AttackModule::set_no_finish_camera(3, true, false)
                        AttackModule::set_no_damage_fly_smoke_all(true, false)
                    }
                    frame(Frame=13)
                    if(is_excute){
                        sv_battle_object::notify_event_msc_cmd(0x2127e37c07, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
                    }
                    frame(Frame=17)
                    if(is_excute){
                        AttackModule::clear_all()
                        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=60, KBG=145, FKB=0, BKB=50, Size=9.0, X=0.0, Y=11.5, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_mario_local_coin"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN_LAST, Type=ATTACK_REGION_PUNCH)
                        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=4.0, Angle=60, KBG=145, FKB=0, BKB=50, Size=9.0, X=0.0, Y=11.5, Z=2.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_mario_local_coin"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN_LAST, Type=ATTACK_REGION_PUNCH)
                    }
                });
            }
            else {
                acmd! ({
                    frame(Frame=3)
                    if(is_excute){
                        SA_SET(State=SITUATION_KIND_AIR)
                        WorkModule::on_flag(Flag=FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR)
                        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=60, KBG=100, FKB=160, BKB=0, Size=2.5, X=0.0, Y=6.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_coin"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_COIN, Type=ATTACK_REGION_PUNCH)
                        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=5.0, Angle=86, KBG=100, FKB=150, BKB=0, Size=4.0, X=0.0, Y=6.5, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_coin"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_COIN, Type=ATTACK_REGION_PUNCH)
                        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=5.0, Angle=100, KBG=100, FKB=150, BKB=0, Size=4.0, X=0.0, Y=6.3, Z=9.2, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_coin"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_COIN, Type=ATTACK_REGION_PUNCH)
                        AttackModule::set_no_damage_fly_smoke_all(true, false)
                    }
                    wait(Frames=3)
                    if(is_excute){
                        WorkModule::on_flag(Flag=FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS)
                    }
                    wait(Frames=1)
                    if(is_excute){
                        AttackModule::clear_all()
                        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.6, Angle=60, KBG=100, FKB=180, BKB=0, Size=3.0, X=0.0, Y=6.5, Z=2.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_coin"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_COIN, Type=ATTACK_REGION_PUNCH)
                        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=0.6, Angle=92, KBG=100, FKB=170, BKB=0, Size=3.8, X=0.0, Y=6.5, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_coin"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_COIN, Type=ATTACK_REGION_PUNCH)
                        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=0.6, Angle=60, KBG=100, FKB=110, BKB=0, Size=3.0, X=0.0, Y=11.5, Z=2.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_coin"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_COIN, Type=ATTACK_REGION_PUNCH)
                        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=0.6, Angle=92, KBG=100, FKB=110, BKB=0, Size=3.8, X=0.0, Y=11.5, Z=8.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_coin"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_COIN, Type=ATTACK_REGION_PUNCH)
                        AttackModule::set_no_finish_camera(0, true, false)
                        AttackModule::set_no_finish_camera(1, true, false)
                        AttackModule::set_no_finish_camera(2, true, false)
                        AttackModule::set_no_finish_camera(3, true, false)
                        AttackModule::set_no_damage_fly_smoke_all(true, false)
                    }
                    frame(Frame=13)
                    if(is_excute){
                        sv_battle_object::notify_event_msc_cmd(0x2127e37c07, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
                    }
                    frame(Frame=17)
                    if(is_excute){
                        AttackModule::clear_all()
                        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=60, KBG=145, FKB=0, BKB=50, Size=9.0, X=0.0, Y=11.5, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_coin"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARIO_COIN_LAST, Type=ATTACK_REGION_PUNCH)
                        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=4.0, Angle=60, KBG=145, FKB=0, BKB=50, Size=9.0, X=0.0, Y=11.5, Z=2.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_coin"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARIO_COIN_LAST, Type=ATTACK_REGION_PUNCH)
                    }
                    wait(Frames=2)
                    if(is_excute){
                        AttackModule::clear_all()
                    }
                });
            }
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MARIO, 
    animation = "throw_lw",
    animcmd = "game_throwlw")]
pub fn mario_dthrow(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=6.0, Angle=68, KBG=90, FKB=0, BKB=55, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=40, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
        }
        frame(Frame=17)
        if(is_excute){
            CHECK_FINISH_CAMERA(4, 0)
            rust {
                let fighter_cutin_manager = *(FIGHTER_CUTIN_MANAGER_ADDR as *mut *mut app::FighterCutInManager);
                FighterCutInManager::set_throw_finish_zoom_rate(fighter_cutin_manager, 1.7);
                FighterCutInManager::set_throw_finish_offset(fighter_cutin_manager, Vector3f{x: 0.0, y: 0.0, z: 0.0});
            }
        }
        frame(Frame=18)
        if(is_excute){
            ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO))
        }
    });
}

pub fn install() {
    acmd::add_hooks!(
        mario_fair,
        mario_dair,
        mario_da,
        mario_up_tilt,
        mario_down_tilt,
        mario_bair,
        mario_usmash,
        mario_dsmash,
        mario_fsmash_hi,
        mario_fsmash_mid,
        mario_fsmash_lw,
        flood,
        mario_uair,
        mario_upB_grnd,
        mario_upB_air,
        mario_dthrow
    );
}
