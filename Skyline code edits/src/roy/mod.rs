use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CFighterCommon;
use smashline::*;

//Actual changes

#[acmd_script(agent = "roy", scripts = ["game_attacklw3"], category = ACMD_GAME)]
fn roy_dtilt(fighter: &mut smash::lua2cpp::L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=7)
        if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=11.0, Angle=80, KBG=40, FKB=0, BKB=90, Size=2.7, X=0.0, Y=3.5, Z=13.0, X2=0.0, Y2=4.1, Z2=9.2, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.35, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
        ATTACK(ID=1, Part=0, Bone=hash40("sword1"), Damage=6.5, Angle=80, KBG=40, FKB=0, BKB=70, Size=2.7, X=0.0, Y=0.0, Z=6.7, X2=0.0, Y2=0.0, Z2=4.7, Hitlag=0.9, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.35, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_SWORD)
        //AttackModule::set_attack_height_all(ATTACK_HEIGHT_LOW, false)
        }
        wait(Frames=2)
        if(is_excute){
        AttackModule::clear_all()
        }
         
    });
}

#[acmd_script(agent = "roy", scripts = ["game_attackairf"], category = ACMD_GAME)]
fn roy_fair(fighter: &mut smash::lua2cpp::L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
            FT_MOTION_RATE(FSM=0.4)
            }
            frame(Frame=13)
            if(is_excute){
            FT_MOTION_RATE(FSM=1)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=11.0, Angle=42, KBG=83, FKB=0, BKB=35, Size=4.8, X=0.0, Y=13.0, Z=4.0, X2=0.0, Y2=10.0, Z2=4.0, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=11.0, Angle=42, KBG=83, FKB=0, BKB=35, Size=4.2, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("sword1"), Damage=11.0, Angle=42, KBG=83, FKB=0, BKB=35, Size=3.5, X=0.0, Y=0.0, Z=0.7, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("sword1"), Damage=7.0, Angle=42, KBG=80, FKB=0, BKB=35, Size=3.5, X=0.0, Y=0.0, Z=7.2, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.9, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_SWORD)
            }
            wait(Frames=3)
            if(is_excute){
            AttackModule::clear_all()
            }
            frame(Frame=34)
            if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
            }
    });
}

#[acmd_script(agent = "roy", scripts = ["game_attackairb"], category = ACMD_GAME)]
fn roy_bair(fighter: &mut smash::lua2cpp::L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=3)
        if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        REVERSE_LR()
        }
        frame(Frame=8)
        if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=50, KBG=107, FKB=0, BKB=35, Size=4.2, X=0.0, Y=10.5, Z=-7.0, X2=0.0, Y2=10.5, Z2=-4.5, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
        ATTACK(ID=1, Part=0, Bone=hash40("sword1"), Damage=12.0, Angle=50, KBG=107, FKB=0, BKB=35, Size=4.0, X=2.5, Y=0.0, Z=0.4, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
        ATTACK(ID=2, Part=0, Bone=hash40("sword1"), Damage=9.0, Angle=50, KBG=100, FKB=0, BKB=10, Size=4.0, X=2.5, Y=0.0, Z=7.2, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.9, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_SWORD)
        }
        wait(Frames=3)
        if(is_excute){
        AttackModule::clear_all()
        }
        frame(Frame=32)
        if(is_excute){
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
             
    });
}

#[acmd_script(agent = "roy", scripts = ["game_attackairn"], category = ACMD_GAME)]
fn roy_nair(fighter: &mut smash::lua2cpp::L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=2)
        if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=6)
        if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.0, Angle=80, KBG=30, FKB=0, BKB=47, Size=3.5, X=0.0, Y=9.6, Z=8.0, X2=0.0, Y2=11.7, Z2=3.0, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
        ATTACK(ID=1, Part=0, Bone=hash40("sword1"), Damage=4.0, Angle=80, KBG=30, FKB=0, BKB=47, Size=3.5, X=0.0, Y=0.0, Z=6.8, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.9, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_SWORD)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=6.0, Angle=80, KBG=30, FKB=0, BKB=47, Size=4.8, X=0.0, Y=9.6, Z=6.8, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
        }
        wait(Frames=2)
        if(is_excute){
        AttackModule::clear_all()
        }
        frame(Frame=15)
        if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=8.5, Angle=50, KBG=80, FKB=0, BKB=50, Size=5.6, X=0.0, Y=0.0, Z=-1.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
        ATTACK(ID=1, Part=0, Bone=hash40("sword1"), Damage=8.5, Angle=50, KBG=80, FKB=0, BKB=50, Size=4.2, X=2.5, Y=0.0, Z=0.7, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
        ATTACK(ID=2, Part=0, Bone=hash40("sword1"), Damage=5.0, Angle=50, KBG=80, FKB=0, BKB=50, Size=4.2, X=2.5, Y=0.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.9, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_SWORD)
        }
        wait(Frames=7)
        if(is_excute){
        AttackModule::clear_all()
        }
        frame(Frame=47)
        if(is_excute){
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
    });
}

#[acmd_script(agent = "roy", scripts = ["game_attackairhi"], category = ACMD_GAME)]
fn roy_uair(fighter: &mut smash::lua2cpp::L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=3)
        if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=5)
        if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("colonells"), Damage=9.0, Angle=80, KBG=105, FKB=0, BKB=40, Size=3.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
        ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=9.0, Angle=80, KBG=105, FKB=0, BKB=40, Size=4.2, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
        ATTACK(ID=2, Part=0, Bone=hash40("sword1"), Damage=9.0, Angle=80, KBG=105, FKB=0, BKB=40, Size=3.0, X=0.0, Y=0.0, Z=1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
        ATTACK(ID=3, Part=0, Bone=hash40("sword1"), Damage=6.0, Angle=70, KBG=70, FKB=0, BKB=10, Size=3.9, X=0.0, Y=0.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.9, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_SWORD)
        }
        frame(Frame=13)
        if(is_excute){
        AttackModule::clear_all()
        }
        frame(Frame=38)
        if(is_excute){
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }    
    });
}

#[acmd_script(agent = "roy", scripts = ["game_specials4s", "specialairs4s"], category = ACMD_GAME)]
fn roy_warioware(fighter: &mut smash::lua2cpp::L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=7)
        if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.0, Angle=361, KBG=110, FKB=0, BKB=75, Size=7.0, X=0.0, Y=9.0, Z=7.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=4.0, Angle=361, KBG=105, FKB=0, BKB=70, Size=8.0, X=0.0, Y=9.0, Z=11.7, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=4.0, Angle=361, KBG=105, FKB=0, BKB=70, Size=7.0, X=0.0, Y=9.0, Z=15.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=5, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
        }
        frame(Frame=10)
        if(is_excute){
        AttackModule::clear_all()
        }
    });
}

//Hitlag multipliers amarite game_attacks3s

#[acmd_script(agent = "roy", scripts = ["game_attacks3s"], category = ACMD_GAME)]
fn roy_ftilt(fighter: &mut smash::lua2cpp::L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=6)
        if(is_excute){
        FighterAreaModuleImpl::enable_fix_jostle_area(4.0, 4.0)
        }
        frame(Frame=8)
        if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.5, Angle=361, KBG=105, FKB=0, BKB=30, Size=4.3, X=0.0, Y=9.0, Z=6.2, X2=0.0, Y2=9.0, Z2=3.0, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
        ATTACK(ID=1, Part=0, Bone=hash40("sword1"), Damage=9.0, Angle=361, KBG=80, FKB=0, BKB=40, Size=4.1, X=0.0, Y=0.0, Z=1.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
        ATTACK(ID=2, Part=0, Bone=hash40("sword1"), Damage=8.0, Angle=361, KBG=80, FKB=0, BKB=40, Size=3.5, X=0.0, Y=0.0, Z=7.2, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.9, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_SWORD)
        //AttackModule::set_attack_height_all(ATTACK_HEIGHT_HIGH, false)
        }
        wait(Frames=3)
        if(is_excute){
        AttackModule::clear_all()
        }
        
    });
}

#[acmd_script(agent = "roy", scripts = ["game_attack11"], category = ACMD_GAME)]
fn roy_jab(fighter: &mut smash::lua2cpp::L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=5)
        if(is_excute){
        FighterAreaModuleImpl::enable_fix_jostle_area(5.0, 5.0)
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.5, Angle=69, KBG=40, FKB=0, BKB=70, Size=3.7, X=0.0, Y=10.0, Z=7.0, X2=0.0, Y2=10.0, Z2=5.5, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
        ATTACK(ID=1, Part=0, Bone=hash40("sword1"), Damage=7.5, Angle=69, KBG=40, FKB=0, BKB=70, Size=3.8, X=0.0, Y=0.0, Z=1.7, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
        ATTACK(ID=2, Part=0, Bone=hash40("sword1"), Damage=4.8, Angle=38, KBG=60, FKB=0, BKB=47, Size=3.5, X=0.0, Y=0.0, Z=8.4, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.9, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_SWORD)
        }
        frame(Frame=8)
        if(is_excute){
        AttackModule::clear_all()
        }
    });
}

#[acmd_script(agent = "roy", scripts = ["game_attackhi3"], category = ACMD_GAME)]
fn roy_utilt(fighter: &mut smash::lua2cpp::L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=6)
        if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=98, KBG=103, FKB=0, BKB=35, Size=4.2, X=0.0, Y=16.0, Z=0.0, X2=0.0, Y2=16.0, Z2=0.0, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
        ATTACK(ID=1, Part=0, Bone=hash40("sword1"), Damage=12.0, Angle=98, KBG=103, FKB=0, BKB=35, Size=2.8, X=0.0, Y=0.0, Z=1.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=12.0, Angle=98, KBG=103, FKB=0, BKB=35, Size=1.5, X=0.0, Y=18.0, Z=6.0, X2=0.0, Y2=10.0, Z2=6.0, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
        ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=7.0, Angle=65, KBG=100, FKB=0, BKB=30, Size=2.0, X=0.0, Y=16.0, Z=10.0, X2=0.0, Y2=10.0, Z2=10.0, Hitlag=0.9, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_SWORD)
        }
        wait(Frames=1)
        if(is_excute){
        ATTACK(ID=2, Part=0, Bone=hash40("sword1"), Damage=7.0, Angle=65, KBG=100, FKB=0, BKB=30, Size=3.5, X=0.0, Y=0.0, Z=7.2, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.9, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_SWORD)
        AttackModule::clear(ID=3, false)
        }
        frame(Frame=9)
        if(is_excute){
        AttackModule::clear(ID=0, false)
        }
        frame(Frame=12)
        if(is_excute){
        AttackModule::clear_all()
        }
    });
}

#[acmd_script(agent = "roy", scripts = ["game_attackairlw"], category = ACMD_GAME)]
fn roy_dair(fighter: &mut smash::lua2cpp::L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=3)
        if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=6)
        if(is_excute){
            ATTACK(ID=3, Part=0, Bone=hash40("colonells"), Damage=18.0, Angle=280, KBG=70, FKB=0, BKB=30, Size=3.0, X=0.0, Y=1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.7, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("armr"), Damage=18.0, Angle=280, KBG=70, FKB=0, BKB=30, Size=3.0, X=0.0, Y=1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.7, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=0, Part=0, Bone=hash40("sword1"), Damage=12.0, Angle=90, KBG=70, FKB=0, BKB=40, Size=3.5, X=1.0, Y=0.0, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.7, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("sword1"), Damage=9.0, Angle=37, KBG=80, FKB=0, BKB=20, Size=3.5, X=1.0, Y=0.0, Z=6.7, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)        }
        frame(Frame=10)
        if(is_excute){
        AttackModule::clear_all()
        }
        frame(Frame=52)
        if(is_excute){
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        
    });
}

#[acmd_script(agent = "roy", scripts = ["game_attackdash"], category = ACMD_GAME)]
fn roy_dashattack(fighter: &mut smash::lua2cpp::L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=13)
        if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=13.0, Angle=52, KBG=85, FKB=0, BKB=65, Size=4.4, X=0.0, Y=6.5, Z=12.5, X2=0.0, Y2=6.5, Z2=6.0, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=9.0, Angle=361, KBG=60, FKB=0, BKB=60, Size=4.4, X=0.0, Y=6.5, Z=18.5, X2=0.0, Y2=6.5, Z2=6.0, Hitlag=0.7, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_SWORD)
        }
        wait(Frames=4)
        if(is_excute){
        AttackModule::clear_all()
        }
    });
}

#[acmd_script(agent = "roy", scripts = ["game_attacks4s"], category = ACMD_GAME)]
fn roy_fsmash(fighter: &mut smash::lua2cpp::L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=7)
        if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        }
        frame(Frame=12)
        FT_MOTION_RATE(FSM=0.5)
        frame(Frame=14)
        FT_MOTION_RATE(FSM=1)
        if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=20.0, Angle=361, KBG=75, FKB=0, BKB=60, Size=3.3, X=0.0, Y=7.7, Z=9.1, X2=0.0, Y2=7.7, Z2=7.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
        ATTACK(ID=1, Part=0, Bone=hash40("sword1"), Damage=17.0, Angle=361, KBG=75, FKB=0, BKB=60, Size=4.0, X=2.0, Y=1.0, Z=1.5, X2=14.0, Y2=1.0, Z2=1.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
        ATTACK(ID=2, Part=0, Bone=hash40("sword1"), Damage=12.0, Angle=361, KBG=70, FKB=0, BKB=30, Size=3.5, X=2.0, Y=1.0, Z=7.5, X2=9.5, Y2=1.0, Z2=7.5, Hitlag=0.7, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_SWORD)
        //AttackModule::set_attack_height_all(ATTACK_HEIGHT_HIGH, false)
        }
        frame(Frame=16)
        if(is_excute){
        AttackModule::clear_all()
        }
    });
}

#[acmd_script(agent = "roy", scripts = ["effect_attackairlw"], category = ACMD_EFFECT)]
fn roy_effect_dair(fighter: &mut smash::lua2cpp::L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(4)
        if(is_excute){
            AFTER_IMAGE4_ON_arg29(0x0e4d7b4c4b as u64, 0x0ed4721df1 as u64, 20, hash40("sword1"), 0, 0, -0.8, hash40("sword1"), 0.0, 0.0, 14.5, true, 0x09e9785222 as u64, hash40("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.5, 0.2)
        }
        frame(17)
        if(is_excute){
            AFTER_IMAGE_OFF(2.0)
        }
    });
}

pub fn installRoy() {
    install_acmd_scripts!(
        roy_bair, roy_dair, roy_dashattack, roy_dtilt, roy_effect_dair, roy_fair, roy_fsmash, roy_ftilt, roy_jab, roy_nair, roy_uair, roy_utilt, roy_warioware
    );
}
