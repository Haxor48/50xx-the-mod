use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::lua2cpp::L2CFighterBase;
use smashline::*;
use smash::phx::*;

#[acmd_script(agent = "mewtwo", scripts = ["game_attackairf"], category = ACMD_GAME)]
fn mewtwo_fair(fighter: &mut smash::lua2cpp::L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=3)
        FT_MOTION_RATE(FSM=1.5)
        frame(Frame=5)
        FT_MOTION_RATE(FSM=1)
        frame(Frame=6)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=14.0, Angle=84, KBG=100, FKB=0, BKB=40, Size=5.5, X=0.0, Y=8.2, Z=13.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=14.0, Angle=84, KBG=100, FKB=0, BKB=40, Size=4.5, X=0.0, Y=8.2, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=14.0, Angle=84, KBG=100, FKB=0, BKB=40, Size=4.5, X=0.0, Y=8.2, Z=4.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PUNCH)
        }
        frame(Frame=9)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=36)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
    });
}

#[acmd_script(agent = "mewtwo", scripts = ["game_attackairn"], category = ACMD_GAME)]
fn mewtwo_nair(fighter: &mut smash::lua2cpp::L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=7)
        for(5 Iterations){
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.6, Angle=367, KBG=100, FKB=60, BKB=0, Size=10.5, X=0.0, Y=7.0, Z=-2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_BODY)
            }
            wait(Frames=2)
            if(is_excute){
                AttackModule::clear_all()
            }
            wait(Frames=2)
        }
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=70, KBG=110, FKB=0, BKB=40, Size=13.5, X=0.0, Y=7.0, Z=-2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_BODY)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=46)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
    });
}

#[acmd_script(agent = "mewtwo", scripts = ["game_attackairhi"], category = ACMD_GAME)]
fn mewtwo_uair(fighter: &mut smash::lua2cpp::L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=3)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=10)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("s_tail3"), Damage=12.0, Angle=72, KBG=98, FKB=0, BKB=40, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_TAIL)
            ATTACK(ID=1, Part=0, Bone=hash40("s_tail5"), Damage=12.0, Angle=65, KBG=92, FKB=0, BKB=40, Size=4.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_TAIL)
            ATTACK(ID=2, Part=0, Bone=hash40("s_tail7"), Damage=12.0, Angle=361, KBG=92, FKB=0, BKB=40, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_TAIL)
        }
        wait(Frames=5)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=36)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
    });
}

#[acmd_script(agent = "mewtwo", scripts = ["game_attackairlw"], category = ACMD_GAME)]
fn mewtwo_dair(fighter: &mut smash::lua2cpp::L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        FT_MOTION_RATE(FSM=0.75)
        frame(Frame=4)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=19)
        FT_MOTION_RATE(FSM=1)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=15.0, Angle=270, KBG=100, FKB=0, BKB=10, Size=4.5, X=0.0, Y=-7.7, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=14.0, Angle=270, KBG=98, FKB=0, BKB=20, Size=7.5, X=0.0, Y=-5.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        wait(Frames=4)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=46)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
    });
}

#[acmd_script(agent = "mewtwo", scripts = ["game_attacklw3"], category = ACMD_GAME)]
fn mewtwo_dtilt(fighter: &mut smash::lua2cpp::L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=5)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=9.0, Angle=80, KBG=80, FKB=0, BKB=50, Size=4.3, X=0.0, Y=3.0, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_TAIL)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=8.0, Angle=80, KBG=80, FKB=0, BKB=50, Size=3.8, X=0.0, Y=1.5, Z=13.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_TAIL)
            ATTACK(ID=2, Part=0, Bone=hash40("s_tail7"), Damage=5.0, Angle=90, KBG=80, FKB=0, BKB=50, Size=3.2, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_TAIL)
            AttackModule::set_attack_height_all(smash::cpp::root::app::AttackHeight(*ATTACK_HEIGHT_LOW), false)
        }
        frame(Frame=7)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("s_tail3"), Damage=9.0, Angle=80, KBG=80, FKB=0, BKB=50, Size=4.3, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_TAIL)
            ATTACK(ID=1, Part=0, Bone=hash40("s_tail5"), Damage=8.0, Angle=80, KBG=80, FKB=0, BKB=50, Size=3.8, X=1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_TAIL)
            ATTACK(ID=2, Part=0, Bone=hash40("s_tail7"), Damage=5.0, Angle=90, KBG=80, FKB=0, BKB=50, Size=3.2, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_TAIL)
            AttackModule::set_attack_height_all(smash::cpp::root::app::AttackHeight(*ATTACK_HEIGHT_LOW), false)
        }
        frame(Frame=8)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(agent = "mewtwo", scripts = ["game_attackhi4"], category = ACMD_GAME)]
fn mewtwo_usmash(fighter: &mut smash::lua2cpp::L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=3)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        }
        frame(Frame=9)
        if(is_excute){
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=2.0, Angle=367, KBG=100, FKB=90, BKB=0, Size=4.5, X=0.0, Y=8.7, Z=-4.5, X2=0.0, Y2=8.7, Z2=6.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_NONE)
            rust {
                let temp = smash::phx::Vector2f{x: 0.0, y: 24.0};
                AttackModule::set_vec_target_pos(module_accessor, 2, Hash40{hash: hash40("top")}, &temp, 7, false);
            }
        }
        frame(Frame=10)
        for(3 Iterations){
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=366, KBG=100, FKB=18, BKB=0, Size=4.6, X=0.0, Y=22.0, Z=-5.8, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_NONE)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=2.0, Angle=366, KBG=100, FKB=18, BKB=0, Size=4.6, X=0.0, Y=22.0, Z=5.8, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_NONE)
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=2.0, Angle=366, KBG=100, FKB=40, BKB=0, Size=4.0, X=0.0, Y=14.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_NONE)
                ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=2.0, Angle=366, KBG=100, FKB=5, BKB=0, Size=4.0, X=0.0, Y=23.6, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_NONE)
            }
            wait(Frames=2)
            if(is_excute){
                AttackModule::clear_all()
            }
            wait(Frames=2)
        }
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=10.0, Angle=80, KBG=111, FKB=0, BKB=60, Size=6.4, X=0.0, Y=22.0, Z=-7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_NONE)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=10.0, Angle=80, KBG=111, FKB=0, BKB=60, Size=6.4, X=0.0, Y=22.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_NONE)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=10.0, Angle=90, KBG=111, FKB=0, BKB=60, Size=5.2, X=0.0, Y=23.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_NONE)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=10.0, Angle=90, KBG=111, FKB=0, BKB=60, Size=4.0, X=0.0, Y=14.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_NONE)
        }
        wait(Frames=4)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(agent = "mewtwo", scripts = ["game_attackdash"], category = ACMD_GAME)]
fn mewtwo_da(fighter: &mut smash::lua2cpp::L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=10)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=9.0, Angle=80, KBG=60, FKB=0, BKB=80, Size=5.5, X=0.0, Y=10.0, Z=16.299999, X2=0.0, Y2=8.5, Z2=16.299999, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=9.0, Angle=80, KBG=60, FKB=0, BKB=80, Size=2.0, X=0.0, Y=10.0, Z=13.0, X2=0.0, Y2=10.0, Z2=6.0, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_PUNCH)
        }
        wait(Frames=2)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.0, Angle=80, KBG=60, FKB=0, BKB=40, Size=4.0, X=0.0, Y=10.0, Z=16.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=6.0, Angle=80, KBG=60, FKB=0, BKB=40, Size=2.0, X=0.0, Y=10.0, Z=13.0, X2=0.0, Y2=10.0, Z2=8.0, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_PUNCH)
        }
        frame(Frame=21)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(agent = "mewtwo", scripts = ["game_attacklw4"], category = ACMD_GAME)]
fn mewtwo_dsmash(fighter: &mut smash::lua2cpp::L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=16)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        }
        frame(Frame=21)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=17.0, Angle=35, KBG=118, FKB=0, BKB=20, Size=7.5, X=0.0, Y=4.0, Z=13.7, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=8, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_NONE)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=17.0, Angle=35, KBG=118, FKB=0, BKB=20, Size=11.0, X=0.0, Y=4.0, Z=13.7, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=4, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_NONE)
        }
        wait(Frames=3)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(agent = "mewtwo", scripts = ["game_attacks4s", "game_attacks4hi", "game_attacks4lw"], category = ACMD_GAME)]
fn mewtwo_fsmash(fighter: &mut smash::lua2cpp::L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        FT_MOTION_RATE(FSM=1.5)
        frame(Frame=3)
        FT_MOTION_RATE(FSM=1.0)
        frame(Frame=12)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        }
        frame(Frame=18)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=18.0, Angle=361, KBG=100, FKB=0, BKB=30, Size=3.0, X=0.0, Y=9.5, Z=10.2, X2=0.0, Y2=9.5, Z2=11.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=20.0, Angle=361, KBG=100, FKB=0, BKB=30, Size=6.3, X=0.0, Y=9.5, Z=19.200001, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_PUNCH)
        }
        frame(Frame=22)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(agent = "mewtwo_bindball", scripts = ["game_shoot"], category = ACMD_GAME)]
fn mewtwo_disable(fighter: &mut smash::lua2cpp::L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=361, KBG=140, FKB=0, BKB=0, Size=6.0, X=0.0, Y=-4.7, Z=2.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_G_d, Hitbits=COLLISION_CATEGORY_MASK_FEB, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_bind_extra"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_NONE)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.0, Angle=361, KBG=180, FKB=0, BKB=20, Size=6.0, X=0.0, Y=-4.7, Z=2.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_FEB, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_bind_extra"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_NONE)
            AttackModule::set_lr_check_front_lr(0)
            AttackModule::set_no_finish_camera(0, true, false)
        }
    });
}

pub fn installMewtwo() {
    install_acmd_scripts!(
        mewtwo_fair,
        mewtwo_nair,
        mewtwo_uair,
        mewtwo_dair,
        mewtwo_dtilt,
        mewtwo_usmash,
        mewtwo_da,
        mewtwo_dsmash,
        mewtwo_fsmash,
        mewtwo_disable
    );
}
