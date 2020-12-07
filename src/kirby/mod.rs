use smash::*;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CFighterCommon;
use acmd::{acmd, acmd_func};
use smash::app::lua_bind::*;
use smash::lib::L2CValueType;
use smash::phx::Vector3f;
use smash::phx::Hash40;

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_KIRBY,
    animation = "throw_b",
    animcmd = "game_throwb")]
pub fn throwb(fighter: &mut L2CFighterCommon) {
    let kirbycide_boost: smash::phx::Vector3f = smash::phx::Vector3f {x: 0.0, y: -8.0, z: 0.0};
    let mut globals = *fighter.globals_mut();

    if let L2CValueType::Void = globals["prev_pos"].val_type {
        globals["prev_pos"] = 0.0.into();
    }
    if let L2CValueType::Void = globals["pos"].val_type {
        globals["pos"] = 0.0.into();
    }
    acmd!({
        if(is_excute){
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=8.0, Angle=130, KBG=120, FKB=0, BKB=30, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=60, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
        }
        frame(13)
        if(is_excute) {
            WorkModule::on_flag(Flag=FIGHTER_STATUS_THROW_FLAG_START_AIR)
        }
        frame(28)
        if(is_excute) {
            MotionModule::set_rate( 0.05);
            rust {
                KineticModule::add_speed(module_accessor, &kirbycide_boost);
            }
        }
        frame (28.1)
        if(is_excute) {
            rust {
                globals["prev_pos"] = PostureModule::pos_y(module_accessor).into();
                //println!("{}", globals["prev_pos"].get_num());
            }
        }
        frame (28.15)
        if(is_excute) {
            rust {
                globals["pos"] = PostureModule::pos_y(module_accessor).into();
                //println!("{}", globals["prev_pos"].get_num());
                let pos_dif = globals["prev_pos"].get_num() - globals["pos"].get_num();
                if pos_dif.abs() <= 3.0 {
                    MotionModule::set_frame(module_accessor, 30.0, false);
                }
                globals["prev_pos"] = PostureModule::pos_y(module_accessor).into();
            }
        }
        frame (28.2)
        if(is_excute) {
            rust {
                globals["pos"] = PostureModule::pos_y(module_accessor).into();
                let pos_dif = globals["prev_pos"].get_num() - globals["pos"].get_num();
                if pos_dif.abs() <= 3.0 {
                    MotionModule::set_frame(module_accessor, 30.0, false);
                }
                globals["prev_pos"] = PostureModule::pos_y(module_accessor).into();
            }
        }
        frame (28.25)
        if(is_excute) {
            rust {
                globals["pos"] = PostureModule::pos_y(module_accessor).into();
                let pos_dif = globals["prev_pos"].get_num() - globals["pos"].get_num();
                if pos_dif.abs() <= 3.0 {
                    MotionModule::set_frame(module_accessor, 30.0, false);
                }
                globals["prev_pos"] = PostureModule::pos_y(module_accessor).into();
            }
        }
        frame (28.3)
        if(is_excute) {
            rust {
                globals["pos"] = PostureModule::pos_y(module_accessor).into();
                let pos_dif = globals["prev_pos"].get_num() - globals["pos"].get_num();
                if pos_dif.abs() <= 3.0 {
                    MotionModule::set_frame(module_accessor, 30.0, false);
                }
                globals["prev_pos"] = PostureModule::pos_y(module_accessor).into();
            }
        }
        frame (28.35)
        if(is_excute) {
            rust {
                globals["pos"] = PostureModule::pos_y(module_accessor).into();
                let pos_dif = globals["prev_pos"].get_num() - globals["pos"].get_num();
                if pos_dif.abs() <= 3.0 {
                    MotionModule::set_frame(module_accessor, 30.0, false);
                }
                globals["prev_pos"] = PostureModule::pos_y(module_accessor).into();
            }
        }
        frame (28.4)
        if(is_excute) {
            rust {
                globals["pos"] = PostureModule::pos_y(module_accessor).into();
                let pos_dif = globals["prev_pos"].get_num() - globals["pos"].get_num();
                if pos_dif.abs() <= 3.0 {
                    MotionModule::set_frame(module_accessor, 30.0, false);
                }
                globals["prev_pos"] = PostureModule::pos_y(module_accessor).into();
            }
        }
        frame (28.45)
        if(is_excute) {
            rust {
                globals["pos"] = PostureModule::pos_y(module_accessor).into();
                let pos_dif = globals["prev_pos"].get_num() - globals["pos"].get_num();
                if pos_dif.abs() <= 3.0 {
                    MotionModule::set_frame(module_accessor, 30.0, false);
                }
                globals["prev_pos"] = PostureModule::pos_y(module_accessor).into();
            }
        }
        frame (28.5)
        if(is_excute) {
            rust {
                globals["pos"] = PostureModule::pos_y(module_accessor).into();
                let pos_dif = globals["prev_pos"].get_num() - globals["pos"].get_num();
                if pos_dif.abs() <= 3.0 {
                    MotionModule::set_frame(module_accessor, 30.0, false);
                }
                globals["prev_pos"] = PostureModule::pos_y(module_accessor).into();
            }
        }
        frame (28.55)
        if(is_excute) {
            rust {
                globals["pos"] = PostureModule::pos_y(module_accessor).into();
                let pos_dif = globals["prev_pos"].get_num() - globals["pos"].get_num();
                if pos_dif.abs() <= 3.0 {
                    MotionModule::set_frame(module_accessor, 30.0, false);
                }
                globals["prev_pos"] = PostureModule::pos_y(module_accessor).into();
            }
        }
        frame (28.6)
        if(is_excute) {
            rust {
                globals["pos"] = PostureModule::pos_y(module_accessor).into();
                let pos_dif = globals["prev_pos"].get_num() - globals["pos"].get_num();
                if pos_dif.abs() <= 3.0 {
                    MotionModule::set_frame(module_accessor, 30.0, false);
                }
                globals["prev_pos"] = PostureModule::pos_y(module_accessor).into();
            }
        }
        frame (28.65)
        if(is_excute) {
            rust {
                globals["pos"] = PostureModule::pos_y(module_accessor).into();
                let pos_dif = globals["prev_pos"].get_num() - globals["pos"].get_num();
                if pos_dif.abs() <= 3.0 {
                    MotionModule::set_frame(module_accessor, 30.0, false);
                }
                globals["prev_pos"] = PostureModule::pos_y(module_accessor).into();
            }
        }
        frame (28.7)
        if(is_excute) {
            rust {
                globals["pos"] = PostureModule::pos_y(module_accessor).into();
                let pos_dif = globals["prev_pos"].get_num() - globals["pos"].get_num();
                if pos_dif.abs() <= 3.0 {
                    MotionModule::set_frame(module_accessor, 30.0, false);
                }
                globals["prev_pos"] = PostureModule::pos_y(module_accessor).into();
            }
        }
        frame (28.75)
        if(is_excute) {
            rust {
                globals["pos"] = PostureModule::pos_y(module_accessor).into();
                let pos_dif = globals["prev_pos"].get_num() - globals["pos"].get_num();
                if pos_dif.abs() <= 3.0 {
                    MotionModule::set_frame(module_accessor, 30.0, false);
                }
                globals["prev_pos"] = PostureModule::pos_y(module_accessor).into();
            }
        }
        frame (28.8)
        if(is_excute) {
            rust {
                globals["pos"] = PostureModule::pos_y(module_accessor).into();
                let pos_dif = globals["prev_pos"].get_num() - globals["pos"].get_num();
                if pos_dif.abs() <= 3.0 {
                    MotionModule::set_frame(module_accessor, 30.0, false);
                }
                globals["prev_pos"] = PostureModule::pos_y(module_accessor).into();
            }
        }
        frame (28.85)
        if(is_excute) {
            rust {
                globals["pos"] = PostureModule::pos_y(module_accessor).into();
                let pos_dif = globals["prev_pos"].get_num() - globals["pos"].get_num();
                if pos_dif.abs() <= 3.0 {
                    MotionModule::set_frame(module_accessor, 30.0, false);
                }
                globals["prev_pos"] = PostureModule::pos_y(module_accessor).into();
            }
        }
        frame (28.9)
        if(is_excute) {
            rust {
                globals["pos"] = PostureModule::pos_y(module_accessor).into();
                let pos_dif = globals["prev_pos"].get_num() - globals["pos"].get_num();
                if pos_dif.abs() <= 3.0 {
                    MotionModule::set_frame(module_accessor, 30.0, false);
                }
                globals["prev_pos"] = PostureModule::pos_y(module_accessor).into();
            }
        }

        frame(30)
        if(is_excute) {
            MotionModule::set_rate( 1.0);
            rust {
                KineticModule::clear_speed_all(module_accessor);
            }
        }
        frame(41)
        if(is_excute) {
            ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO))
        }
        frame(61)
        if(is_excute){
            StatusModule::change_status_request(FIGHTER_STATUS_KIND_FALL, false)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_KIRBY,
    animation = "throw_f",
    animcmd = "game_throwf")]
pub fn throwf(fighter: &mut L2CFighterCommon) {
    let kirbycide_boost: smash::phx::Vector3f = smash::phx::Vector3f {x: 0.0, y: -8.0, z: 0.0};
    let mut globals = *fighter.globals_mut();

    if let L2CValueType::Void = globals["prev_pos"].val_type {
        globals["prev_pos"] = 0.0.into();
    }
    if let L2CValueType::Void = globals["pos"].val_type {
        globals["pos"] = 0.0.into();
    }
    acmd!({
        if(is_excute){
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=5.0, Angle=75, KBG=125, FKB=0, BKB=40, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=60, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
        }
        frame(9)
        if(is_excute) {
            WorkModule::on_flag(Flag=FIGHTER_STATUS_THROW_FLAG_START_AIR)
        }
        frame(32)
        if(is_excute) {
            MotionModule::set_rate( 0.05);
            rust {
                KineticModule::add_speed(module_accessor, &kirbycide_boost);
            }
        }
        frame (32.1)
        if(is_excute) {
            for (19 Iterations) {
                rust {globals["prev_pos"] = PostureModule::pos_y(module_accessor).into();}
                wait(0.05)
                rust {
                    globals["pos"] = PostureModule::pos_y(module_accessor).into();
                    let pos_dif = globals["prev_pos"].get_num() - globals["pos"].get_num();
                    if pos_dif.abs() <= 3.0 {
                        MotionModule::set_frame(module_accessor, 34.0, false);
                    }
                    globals["prev_pos"] = PostureModule::pos_y(module_accessor).into();
                }
                wait(0.05)
            }
        }
        frame(34)
        if(is_excute) {
            MotionModule::set_rate( 1.0);
            rust {
                KineticModule::clear_speed_all(module_accessor);
            }
        }
        frame(45)
        if(is_excute) {
            ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO))
        }
    });
}

pub fn install() {
    acmd::add_hooks!(
        throwb,
        throwf
    );
}
