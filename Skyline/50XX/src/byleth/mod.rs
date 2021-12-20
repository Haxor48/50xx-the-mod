use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CFighterCommon;
use acmd::{acmd, acmd_func};
use smash::phx::*;
use crate::custom::get_player_number;
use smashline::*;

pub static mut WEAPONMODE: [i32; 9] = [0; 9];

#[acmd_script(agent = "master", scripts = ["game_attackairf"], category = ACMD_GAME)]
fn byleth_fair(fighter: &mut smash::lua2cpp::L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        rust {
            if WEAPONMODE[get_player_number(module_accessor)] == 2 {
                acmd! ({
                    FT_MOTION_RATE(1.5)
                });
            }
            else if WEAPONMODE[get_player_number(module_accessor)] == 3 {
                acmd! ({
                    FT_MOTION_RATE(0.6)
                });
            }
            else if WEAPONMODE[get_player_number(module_accessor)] == 4 {
                acmd! ({
                    FT_MOTION_RATE(0.9)
                });
            }
        }
        if(is_excute){
            rust {
                if WEAPONMODE[get_player_number(module_accessor)] == 0 || WEAPONMODE[get_player_number(module_accessor)] == 1 {
                    ArticleModule::generate_article(module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, false, 0);
                }
                else if WEAPONMODE[get_player_number(module_accessor)] == 2 {
                    ArticleModule::generate_article(module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_AXE, false, 0);
                }
                else if WEAPONMODE[get_player_number(module_accessor)] == 3 {
                    ArticleModule::generate_article(module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_BOW, false, 0);
                }
                else if WEAPONMODE[get_player_number(module_accessor)] == 4 {
                    ArticleModule::generate_article(module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, false, 0);
                }
            }
        }
        frame(Frame=3)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=12)
        //FT_MOTION_RATE(1.0)
        rust {
            if WEAPONMODE[get_player_number(module_accessor)] == 0 || WEAPONMODE[get_player_number(module_accessor)] == 1 {
                acmd! ({
                    if (is_excute) {
                        ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=8.5, Angle=48, KBG=100, FKB=0, BKB=52, Size=3.0, X=-0.5, Y=4.0, Z=-0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
                        ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=8.5, Angle=48, KBG=100, FKB=0, BKB=52, Size=2.4, X=-0.5, Y=9.2, Z=-0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
                        ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=8.5, Angle=48, KBG=100, FKB=0, BKB=52, Size=2.4, X=-0.5, Y=13.5, Z=-0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
                        ATTACK(ID=3, Part=0, Bone=hash40("haver"), Damage=12.75, Angle=361, KBG=84, FKB=0, BKB=52, Size=3.4, X=-0.5, Y=19.0, Z=-0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
                        ATTACK(ID=4, Part=0, Bone=hash40("haver"), Damage=12.75, Angle=361, KBG=84, FKB=0, BKB=52, Size=3.0, X=-0.5, Y=25.0, Z=-0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
                    }
                });
            }
            else if WEAPONMODE[get_player_number(module_accessor)] == 2 {
                acmd! ({
                    if (is_excute) {
                        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=15.0, Angle=361, KBG=57, FKB=0, BKB=87, Size=4.3, X=0.0, Y=8.0, Z=4.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=6, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MASTER_AXE, Type=ATTACK_REGION_OBJECT)
                        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=15.0, Angle=361, KBG=57, FKB=0, BKB=87, Size=4.0, X=0.0, Y=8.0, Z=-2.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=6, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MASTER_AXE, Type=ATTACK_REGION_OBJECT)
                        ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=15.0, Angle=361, KBG=82, FKB=0, BKB=26, Size=4.5, X=0.0, Y=13.5, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=6, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MASTER_AXE, Type=ATTACK_REGION_OBJECT)
                    }
                });
            }
            else if WEAPONMODE[get_player_number(module_accessor)] == 3 {
                acmd! ({
                    if (is_excute) {
                        ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=6.5, Angle=361, KBG=100, FKB=0, BKB=50, Size=2.7, X=-4.0, Y=1.3, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
                        ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=6.5, Angle=361, KBG=100, FKB=0, BKB=50, Size=3.5, X=-4.0, Y=-2.8, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
                    }
                });
            }
            else if WEAPONMODE[get_player_number(module_accessor)] == 4 {
                acmd! ({
                    if (is_excute) {
                        ATTACK(ID=0, Part=0, Bone=hash40("sword1"), Damage=10.0, Angle=361, KBG=76, FKB=0, BKB=48, Size=3.5, X=3.0, Y=1.0, Z=-2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                        ATTACK(ID=1, Part=0, Bone=hash40("sword1"), Damage=10.0, Angle=361, KBG=76, FKB=0, BKB=48, Size=3.5, X=7.0, Y=1.0, Z=-2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                        ATTACK(ID=2, Part=0, Bone=hash40("sword1"), Damage=10.0, Angle=361, KBG=76, FKB=0, BKB=48, Size=3.5, X=11.5, Y=1.0, Z=-2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                    }
                });
            }
        }
        frame(Frame=13)
        rust {
            if WEAPONMODE[get_player_number(module_accessor)] == 0 || WEAPONMODE[get_player_number(module_accessor)] == 1 {
                acmd! ({
                    if (is_excute) {
                        ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=8.5, Angle=48, KBG=100, FKB=0, BKB=52, Size=2.0, X=0.0, Y=5.0, Z=5.0, X2=0.0, Y2=5.0, Z2=13.0, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
                        ATTACK(ID=6, Part=0, Bone=hash40("top"), Damage=12.75, Angle=361, KBG=84, FKB=0, BKB=52, Size=2.5, X=0.0, Y=5.0, Z=16.5, X2=0.0, Y2=5.0, Z2=22.0, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_OBJECT)
                    }
                });
            }
        }
        frame(Frame=14)
        FT_MOTION_RATE(FSM=1.0)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=36)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
            rust {
                if WEAPONMODE[get_player_number(module_accessor)] == 2 {
                    ArticleModule::remove_exist(module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_AXE, smash::cpp::root::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
                }
            }
        }
        frame(Frame=53)
        if(is_excute){
            ArticleModule::remove_exist(FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, smash::cpp::root::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
            ArticleModule::remove_exist(FIGHTER_MASTER_GENERATE_ARTICLE_AXE, smash::cpp::root::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
            ArticleModule::remove_exist(FIGHTER_MASTER_GENERATE_ARTICLE_BOW, smash::cpp::root::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
            ArticleModule::remove_exist(FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, smash::cpp::root::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
        }
    });
}

#[acmd_script(agent = "master", scripts = ["effect_attackairf"], category = ACMD_EFFECT)]
fn byleth_effect_fair(fighter: &mut smash::lua2cpp::L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {

    });
}

/*#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MASTER, 
    animation = "landing_air_f",
    animcmd = "game_landingairf")]
pub fn byleth_landing_fair(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=41)
        if(is_excute){
            //ArticleModule::remove_exist(FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR)
            ArticleModule::remove_exist(FIGHTER_MASTER_GENERATE_ARTICLE_AXE, smash::cpp::root::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
        }
    });
}*/

pub fn installByleth() {
    install_acmd_scripts!(
        byleth_fair,
        byleth_effect_fair
    );
}