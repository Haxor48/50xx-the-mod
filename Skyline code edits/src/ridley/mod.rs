use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::lua2cpp::L2CFighterBase;
use acmd::{acmd, acmd_func};

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_RIDLEY, 
    animation = "attack_air_lw",
    animcmd = "game_attackairlw")]
pub fn ridley_dair(fighter: &mut L2CFighterCommon) {
    acmd!({

    });
}

pub fn install() {
    acmd::add_hooks!(

    );
}