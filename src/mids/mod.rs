use std::{
    fmt::Debug,
    io::{Read, Write},
};

use serde::{Deserialize, Serialize};

use crate::netbinary;

pub mod enums;
use enums::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Database {
    pub version: String,
    pub legacy_year: i32,
    pub datetime: i64,
    pub issue: u32,
    pub page_volume: u32,
    pub page_volume_name: String,
    pub archetypes_magic: String,
    pub archetypes: Vec<Archetype>,
    pub powersets_magic: String,
    pub powersets: Vec<Powerset>,
    pub powers_magic: String,
    pub powers: Vec<Power>,
    pub summons_magic: String,
    pub summons: Vec<Summon>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Archetype {
    pub display_name: String,    // = display_name
    pub hitpoints: i32,          // = round(attrib_max.hit_points[-1])
    pub hp_cap: f32,             // = attrib.attrib_max_max.hit_points[-1]
    pub desc_long: String,       // = display_help
    pub res_cap: f32,            // = attrib_resistance_max.damage_type[*][-1]
    pub origins: Vec<String>,    // no match
    pub class_name: String,      // ~= "class_{name}" (or class_key without @)
    pub class_type: ClassType,   // no match
    pub column: i32,             // no match
    pub desc_short: String,      // = display_short_help
    pub primary_group: String,   // ~= primary_category
    pub secondary_group: String, // ~= secondary_category
    pub playable: bool,          // derivable from at_index: player_archetypes
    pub recharge_cap: f32,       // = attrib_max_max.recharge_time[-1]
    pub damage_cap: f32,         // = attrib_strength_max.damage_type[*][-1]
    pub recovery_cap: f32,       // = attrib_max_max.recovery[-1]
    pub regen_cap: f32,          // = attrib_max_max.regeneration[-1]
    pub base_recovery: f32,      // = attrib_base.recovery
    pub base_regen: f32,         // = attrib_base.regeneration
    pub base_threat: f32,        // = attrib_base.threat_level
    pub perception_cap: f32,     // = attrib_max_max.perception_radius[-1]
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Powerset {
    pub display_name: String,   // = display_name
    pub nid_archetype: i32,     // = uid_to_nid(at_class)
    pub set_type: PowerSetType, // no match
    pub image_name: String,     // no match
    pub full_name: String,      // = (parent).name + "." + name
    pub set_name: String,       // = name
    pub description: String,    // = display_help
    pub sub_name: String,       // messy, unused in mids code
    pub at_class: String,       // references Archetype.class_name; reliable only for primary & secondary
    pub uid_trunk_set: String,
    pub uid_link_secondary: String,
    pub mutex_sets: Vec<MutexSet>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MutexSet {
    pub uid: String, // references Powerset.full_name
    pub nid: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Power {
    pub static_index: i32,
    pub full_name: String,             // = full_name
    pub group_name: String,            // = split(full_name, ".")[0]
    pub set_name: String,              // = split(full_name, ".")[1]
    pub power_name: String,            // = short_name/name or split(full_name, ".")[2]
    pub display_name: String,          // = display_name
    pub available: i32,                // unused
    pub requires: Requirement,         // = parse(requires)
    pub modes_required: ModeFlags,     // = bv(modes_required)
    pub modes_disallowed: ModeFlags,   // = bv(modes_disallowed)
    pub power_type: PowerType,         // = enum(type)
    pub accuracy: f32,                 // = accuracy
    pub attack_types: Vector,          // = bv(attack_types)
    pub group_membership: Vec<String>, // = exclusion_groups
    pub entities_affected: Entity,     // = bv(targets_affected)
    pub entities_auto_hit: Entity,     // = bv(targets_autohit)
    pub target: Entity,                // = enum(target_type)
    pub target_lo_s: bool,             // = target_visibility == "LineOfSight"
    pub range: f32,                    // = range
    pub target_secondary: Entity,      // = enum(target_type_secondary)
    pub range_secondary: f32,          // = range_secondary
    pub end_cost: f32,                 // = endurance_cost
    pub interrupt_time: f32,           // = interrupt_time
    pub cast_time: f32,                // = activation_time
    pub recharge_time: f32,            // = recharge_time
    pub base_recharge_time: f32,       // = recharge_time
    pub activate_period: f32,          // = activate_period
    pub effect_area: EffectArea,       // = enum(effect_area)
    pub radius: f32,                   // = radius
    pub arc: i32,                      // = arc
    pub max_targets: i32,              // = max_targets_hit
    pub max_boosts: String,            // = str(max_boosts) // unused
    pub cast_flags: CastFlags,         // = bv(caster_near_ground, target_near_ground, cast_when_dead)
    pub ai_report: Notify,             // = enum(notify_ai_when)
    pub num_charges: i32,              // = number_of_charges
    pub usage_time: i32,               // = toggle_detoggle_time
    pub life_time: i32,                // = power_lifetime
    pub life_time_in_game: i32,        // = power_lifetime_ingame
    pub num_allowed: i32,              // = number_allowed
    pub do_not_save: bool,             // = do_not_save
    pub boosts_allowed: Vec<String>,   // = translate(boosts_allowed)
    pub cast_through_hold: bool,       // = contains(cast_through, "hold")
    pub ignore_strength: bool,         // = ignore_strength
    pub desc_short: String,            // = display_short_help
    pub desc_long: String,             // = display_help
    pub enhancements: Vec<u32>,        // = translate_via(boosts_allowed, "EClasses.mhd")
    pub set_types: Vec<SetType>,       // = enum(allowed_boostset_cats)
    pub click_buff: bool,
    pub always_toggle: bool,
    pub level: i32, // available_level + 1
    pub allow_front_loading: bool,
    pub variable_enabled: bool,
    pub variable_override: bool,
    pub variable_name: String,
    pub variable_min: i32,
    pub variable_max: i32,
    pub uid_sub_power: Vec<String>,
    pub ignore_enh: Vec<Enhance>,
    pub ignore_buff: Vec<Enhance>,
    pub skip_max: bool,
    pub inherent_type: GridType,
    pub display_location: i32,
    pub mutex_auto: bool,
    pub mutex_ignore: bool,
    pub absorb_summon_effects: bool,
    pub absorb_summon_attributes: bool,
    pub show_summon_anyway: bool,
    pub never_auto_update: bool,
    pub never_auto_update_requirements: bool,
    pub include_flag: bool,
    pub forced_class: String,
    pub sort_override: bool,
    pub boost_boostable: bool,
    pub boost_use_player_level: bool,
    pub effects: Vec<Effect>,
    pub hidden_power: bool,
    pub active: bool,
    pub taken: bool,
    pub stacks: i32,
    pub variable_start: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Requirement {
    pub class_name: Vec<String>,
    pub class_name_not: Vec<String>,
    pub power_id: Vec<(String, String)>,
    pub power_id_not: Vec<(String, String)>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Effect {
    pub power_full_name: String,
    pub unique_id: i32,
    pub effect_class: EffectClass,
    pub effect_type: EffectType,
    pub damage_type: Damage,
    pub mez_type: Mez,
    pub et_modifies: EffectType,
    pub summon: String,
    pub delayed_time: f32,
    pub ticks: i32,
    pub stacking: Stacking,
    pub base_probability: f32,
    pub suppression: Suppress,
    pub buffable: bool,
    pub resistible: bool,
    pub special_case: SpecialCase,
    pub variable_modified_override: bool,
    pub ignore_scaling: bool,
    pub pv_mode: PvX,
    pub to_who: ToWho,
    pub display_percentage_override: OverrideBoolean,
    pub scale: f32,
    pub n_magnitude: f32,
    pub n_duration: f32,
    pub attrib_type: AttribType,
    pub aspect: Aspect,
    pub modifier_table: String,
    pub near_ground: bool,
    pub cancel_on_miss: bool,
    pub requires_to_hit_check: bool,
    pub uid_class_name: String,
    pub n_id_class_name: i32,
    pub expression_duration: String,
    pub expression_magnitude: String,
    pub expression_probability: String,
    pub reward: String,
    pub effect_id: String,
    pub ignore_ed: bool,
    pub override_: String,
    pub procs_per_minute: f32,
    pub power_attribs: PowerAttribs,
    pub atr_orig_accuracy: f32,
    pub atr_orig_activate_period: f32,
    pub atr_orig_arc: i32,
    pub atr_orig_cast_time: f32,
    pub atr_orig_effect_area: EffectArea,
    pub atr_orig_endurance_cost: f32,
    pub atr_orig_interrupt_time: f32,
    pub atr_orig_max_targets: i32,
    pub atr_orig_radius: f32,
    pub atr_orig_range: f32,
    pub atr_orig_recharge_time: f32,
    pub atr_orig_secondary_range: f32,
    pub atr_mod_accuracy: f32,
    pub atr_mod_activate_period: f32,
    pub atr_mod_arc: i32,
    pub atr_mod_cast_time: f32,
    pub atr_mod_effect_area: EffectArea,
    pub atr_mod_endurance_cost: f32,
    pub atr_mod_interrupt_time: f32,
    pub atr_mod_max_targets: i32,
    pub atr_mod_radius: f32,
    pub atr_mod_range: f32,
    pub atr_mod_recharge_time: f32,
    pub atr_mod_secondary_range: f32,
    #[serde(with = "netbinary::array_hack")]
    pub active_conditionals_kv: Vec<(String, String)>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Summon {
    pub uid: String,
    pub display_name: String,
    pub entity_type: SummonEntity,
    pub class_name: String,
    #[serde(with = "netbinary::array_hack")]
    pub powerset_full_name: Vec<String>,
    #[serde(with = "netbinary::array_hack")]
    pub upgrade_power_full_name: Vec<String>,
}

pub fn from_reader<R: Read>(mut reader: R) -> netbinary::Result<Database> {
    if netbinary::read_bytes(&mut reader)? != b"Mids Reborn Powers Database" {
        return Err(netbinary::Error::Custom("wrong database type (must choose I12.mhd)".to_string()));
    }
    netbinary::from_reader(reader)
}

pub fn to_writer<W: Write>(writer: W, db: &Database) -> netbinary::Result<()> {
    let mut s = netbinary::Serializer { writer };
    "Mids Reborn Powers Database".serialize(&mut s)?;
    db.serialize(&mut s)
}
