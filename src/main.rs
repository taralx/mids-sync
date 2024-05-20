//#![windows_subsystem = "windows"]

use std::{
    collections::{BTreeMap, BTreeSet},
    fs::File,
    io::{BufRead, BufReader, BufWriter},
    path::PathBuf,
};

use anyhow::{bail, Context, Result};
use native_windows_gui as nwg;
use zip::{read::ZipArchive, result::ZipError};

mod cod;
mod mids;
mod netbinary;

fn prompt_path(title: &str, filter: &str) -> Result<PathBuf, anyhow::Error> {
    let mut fd = nwg::FileDialog::default();
    nwg::FileDialog::builder()
        .title(title)
        .action(nwg::FileDialogAction::Open)
        .filters(filter)
        .build(&mut fd)?;
    if !fd.run::<nwg::ControlHandle>(None) {
        bail!("user canceled prompt");
    }
    Ok(PathBuf::from(fd.get_selected_item()?))
}

fn prompt_save(title: &str, filter: &str) -> Result<BufWriter<File>> {
    let mut fd = nwg::FileDialog::default();
    nwg::FileDialog::builder()
        .title(title)
        .action(nwg::FileDialogAction::Save)
        .filters(filter)
        .build(&mut fd)?;
    if !fd.run::<nwg::ControlHandle>(None) {
        bail!("user canceled prompt");
    }
    Ok(BufWriter::new(File::create(fd.get_selected_item()?)?))
}

fn main() -> Result<()> {
    nwg::init().unwrap();

    let mids_path = prompt_path("Locate mids data", "Mids Reborn Database (I12.mhd)")?;
    let zipf = File::open(prompt_path("Locate CoD raw data", "Zip File (*.zip)")?)?;

    let mut eclasses = Vec::new();
    let mut eclasses_map = BTreeMap::new();
    {
        let f = File::open(mids_path.join("../EClasses.mhd"))?;
        let mut lines = BufReader::new(f).lines();
        for line in &mut lines {
            if line?.starts_with("Index\t") {
                break;
            }
        }
        for line in lines {
            let line = line?;
            if line.starts_with("End") {
                break;
            }
            let fields: Vec<&str> = line.split("\t").collect();
            let index: usize = fields[0].parse().with_context(|| format!("line = {:?}", line))?;
            if index >= eclasses.len() {
                eclasses.resize(index + 1, None);
            }
            eclasses[index] = Some(fields[3].to_string());
        }
        for (i, v) in eclasses.iter().enumerate() {
            if let Some(v) = v {
                eclasses_map.insert(v.as_str(), u32::try_from(i)?);
            }
        }
    }

    let mut mdb = mids::from_reader(BufReader::new(File::open(mids_path)?))?;
    println!("Using Mids DB version {}", mdb.version);
    let mut cdb = cod::Database { zip: ZipArchive::new(zipf)? };
    println!("Using CoD revision {}", cdb.revision()?);
    let cidx = cdb.index()?;

    // This mapping should be applied to Powerset::full_name and the first two parts of Power::full_name.
    let powerset_map = BTreeMap::from([
        ("blaster_support.temporal_manipulation", "blaster_support.time_manipulation"),
        ("controller_buff.electrical_affinity", "controller_buff.shock_therapy"),
        ("corruptor_buff.electrical_affinity", "corruptor_buff.shock_therapy"),
        ("epic.corr_flame_mastery", "epic.corruptor_fire_mastery"),
        ("epic.dark_mastery_blaster", "epic.blaster_dark_mastery"),
        ("epic.dark_mastery_controller", "epic.controller_dark_mastery"),
        ("epic.dark_mastery_dominator", "epic.dominator_dark_mastery"),
        ("epic.dark_mastery_mastermind", "epic.mastermind_dark_mastery"),
        ("epic.dark_mastery_tankbrute", "epic.tank_dark_mastery"),
        ("epic.def_flame_mastery", "epic.defender_fire_mastery"),
        ("epic.ice_mastery_defcorr", "epic.defender_ice_mastery"),
        ("epic.ice_mastery_scrapstalk", "epic.scrapper_ice_mastery"),
        ("epic.psionic_mastery_scrapstalk", "epic.melee_psionic_mastery"),
        ("epic.psionic_mastery_tankbrute", "epic.tank_psionic_mastery"),
        ("epic.scrapper_mace_mastery", "epic.stalker_mace_mastery"),
        ("epic.sentinel_elec_mastery", "epic.sentinel_electricity_mastery"),
        ("epic.sentinel_lev_mastery", "epic.sentinel_leviathan_mastery"),
        ("epic.sentinel_psi_mastery", "epic.sentinel_psionic_mastery"),
        ("mastermind_buff.electrical_affinity", "mastermind_buff.shock_therapy"),
    ]);

    let known_bad_display_name = BTreeSet::from([
        "pets.titan_weapons.defensive_sweep_fast",
        "pets.titan_weapons.crushing_blow_fast",
        "pets.titan_weapons.sweeping_strike_fast",
        "pets.titan_weapons.shatter_armor_fast",
        "pets.titan_weapons.arc_of_destruction_fast",
        "pets.titan_weapons_tanker.defensive_sweep_fast",
        "pets.titan_weapons_tanker.crushing_blow_fast",
        "pets.titan_weapons_tanker.sweeping_strike_fast",
        "pets.titan_weapons_tanker.shatter_armor_fast",
        "pets.titan_weapons_tanker.arc_of_destruction_fast",
        "villain_pets.titan_weapons_brute.arc_of_destruction_fast",
        "villain_pets.titan_weapons_brute.crushing_blow_fast",
        "villain_pets.titan_weapons_brute.defensive_sweep_fast",
        "villain_pets.titan_weapons_brute.shatter_armor_fast",
        "villain_pets.titan_weapons_brute.sweeping_strike_fast",
    ]);

    let boost_map = BTreeMap::from_iter(
        [
            ("Enhance Accuracy", "Accuracy_Boost"),
            ("Enhance Confuse", "Confuse_Boost"),
            ("Enhance Damage", "Damage_Boost"),
            ("Enhance Damage Resistance", "Res_Damage_Boost"),
            ("Enhance Defense", "Buff_Defense_Boost"),
            ("Enhance Defense DeBuff", "Debuff_Defense_Boost"),
            ("Enhance Disorient", "Stunned_Boost"),
            ("Enhance Endurance Modification", "Recovery_Boost"),
            ("Enhance Fear", "Fear_Boost"),
            ("Enhance Flying Speed", "SpeedFlying_Boost"),
            ("Enhance Heal", "Heal_Boost"),
            ("Enhance Hold", "Hold_Boost"),
            ("Enhance Immobilization", "Immobilized_Boost"),
            ("Enhance Intangibility", "Intangible_Boost"),
            ("Enhance Jump", "Jump_Boost"),
            ("Enhance KnockBack", "Knockback_Boost"),
            ("Enhance Range", "Range_Boost"),
            ("Enhance Recharge Speed", "Recharge_Boost"),
            ("Enhance Running Speed", "SpeedRunning_Boost"),
            ("Enhance Sleep", "Sleep_Boost"),
            ("Enhance Slow Movement", "Slow_Boost"),
            ("Enhance Threat Duration", "Taunt_Boost"),
            ("Enhance ToHit Buffs", "Buff_ToHit_Boost"),
            ("Enhance ToHit DeBuffs", "Debuff_ToHit_Boost"),
            ("Incarnate: Destiny Capable", "Incarnate_Destiny_Boost"),
            ("Incarnate: Interface Capable", "Incarnate_Interface_Boost"),
            ("Incarnate: Judgement Capable", "Incarnate_Judgement_Boost"),
            ("Incarnate: Lore Capable", "Incarnate_Lore_Boost"),
            ("Reduce Endurance Cost", "EnduranceDiscount_Boost"),
            ("Reduce Interrupt Time", "Interrupt_Boost"),
        ]
        .map(|(k, v)| (k, *eclasses_map.get(v).unwrap())),
    );

    let boostset_map = BTreeMap::from([
        ("Accurate Defense Debuff", mids::enums::SetType::AccDefDeb),
        ("Accurate Healing", mids::enums::SetType::AccHeal),
        ("Accurate To-Hit Debuff", mids::enums::SetType::AccToHitDeb),
        ("Blaster Archetype Sets", mids::enums::SetType::Blaster),
        ("Brute Archetype Sets", mids::enums::SetType::Brute),
        ("Confuse", mids::enums::SetType::Confuse),
        ("Controller Archetype Sets", mids::enums::SetType::Controller),
        ("Corruptor Archetype Sets", mids::enums::SetType::Corruptor),
        ("Defender Archetype Sets", mids::enums::SetType::Defender),
        ("Defense Debuff", mids::enums::SetType::DefDebuff),
        ("Defense Sets", mids::enums::SetType::Defense),
        ("Dominator Archetype Sets", mids::enums::SetType::Dominator),
        ("Endurance Modification", mids::enums::SetType::EndMod),
        ("Fear", mids::enums::SetType::Fear),
        ("Flight", mids::enums::SetType::Flight),
        ("Healing", mids::enums::SetType::Heal),
        ("Holds", mids::enums::SetType::Hold),
        ("Immobilize", mids::enums::SetType::Immob),
        ("Kheldian Archetype Sets", mids::enums::SetType::Kheldian),
        ("Knockback", mids::enums::SetType::Knockback),
        ("Leaping", mids::enums::SetType::JumpNoSprint),
        ("Leaping & Sprints", mids::enums::SetType::Jump),
        ("Mastermind Archetype Sets", mids::enums::SetType::Mastermind),
        ("Melee AoE Damage", mids::enums::SetType::MeleeAoE),
        ("Melee Damage", mids::enums::SetType::MeleeST),
        ("Pet Damage", mids::enums::SetType::Pets),
        ("Ranged AoE Damage", mids::enums::SetType::RangedAoE),
        ("Ranged Damage", mids::enums::SetType::RangedST),
        ("Recharge Intensive Pets", mids::enums::SetType::PetRech),
        ("Resist Damage", mids::enums::SetType::Resistance),
        ("Running", mids::enums::SetType::RunNoSprint),
        ("Running & Sprints", mids::enums::SetType::Run),
        ("Scrapper Archetype Sets", mids::enums::SetType::Scrapper),
        ("Sentinel Archetype Sets", mids::enums::SetType::Sentinel),
        ("Sleep", mids::enums::SetType::Sleep),
        ("Slow Movement", mids::enums::SetType::Slow),
        ("Sniper Attacks", mids::enums::SetType::Snipe),
        ("Soldiers of Arachnos Archetype Sets", mids::enums::SetType::Arachnos),
        ("Stalker Archetype Sets", mids::enums::SetType::Stalker),
        ("Stuns", mids::enums::SetType::Stun),
        ("Tanker Archetype Sets", mids::enums::SetType::Tanker),
        ("Teleport", mids::enums::SetType::Teleport),
        ("Threat Duration", mids::enums::SetType::Threat),
        ("To Hit Buff", mids::enums::SetType::ToHit),
        ("To Hit Debuff", mids::enums::SetType::ToHitDeb),
        ("Universal Damage Sets", mids::enums::SetType::UniversalDamage),
        ("Universal Travel", mids::enums::SetType::Travel),
    ]);

    let mut changed = false;
    for p in &mut mdb.powers {
        // If there's an AttribMod, skip this.
        if p.effects.iter().any(|e| e.power_attribs != mids::enums::PowerAttribs::None) {
            continue;
        }
        let nl: String = p.full_name.to_ascii_lowercase();
        let mut cod_full_name = &p.full_name;
        if !known_bad_display_name.contains(nl.as_str()) {
            // Look up the display name instead of relying on full name.
            let (mut sn, _) = nl.rsplit_once('.').unwrap();
            if let Some(&repl) = powerset_map.get(sn) {
                sn = repl;
            }
            if let Some(sidx) = cidx.get(sn) {
                if let Some(n) = sidx.get(&p.display_name) {
                    cod_full_name = n;
                }
            }
        }
        let cod_p = match cdb.load_power(cod_full_name) {
            Ok(p) => p,
            Err(e) => {
                if let Some(ZipError::FileNotFound) = e.downcast_ref::<ZipError>() {
                    continue;
                }
                return Err(e);
            }
        };

        if p.group_name != "Boosts" && p.group_name != "Incarnate" {
            // Fix eligible enhancement sets.
            let mids_enhs = BTreeSet::from_iter(p.enhancements.iter().copied());
            let cod_enhs = BTreeSet::from_iter(cod_p.boosts_allowed.iter().filter_map(|b| boost_map.get(b.as_str()).copied()));

            if mids_enhs != cod_enhs {
                let missing: Vec<&str> = cod_enhs
                    .difference(&mids_enhs)
                    .map(|&e| eclasses[e as usize].as_ref().unwrap().as_str())
                    .collect();
                let extra: Vec<&str> = mids_enhs
                    .difference(&cod_enhs)
                    .map(|&e| eclasses[e as usize].as_ref().unwrap().as_str())
                    .collect();
                if !missing.is_empty() {
                    if !extra.is_empty() {
                        println!(
                            "{} ({}): boosts fixed adding {:?} removing {:?}",
                            p.full_name, p.display_name, missing, extra
                        );
                    } else {
                        println!("{} ({}): boosts fixed adding {:?}", p.full_name, p.display_name, missing);
                    }
                } else if !extra.is_empty() {
                    println!("{} ({}): boosts fixed removing {:?}", p.full_name, p.display_name, extra);
                }
                p.enhancements = Vec::from_iter(cod_enhs);
                p.enhancements.sort_unstable();
                p.boosts_allowed = p.enhancements.iter().map(|&e| eclasses[e as usize].clone().unwrap()).collect();
                changed = true;
            }

            let mids_sets = BTreeSet::from_iter(p.set_types.iter().copied());
            let mut cod_sets = BTreeSet::from_iter(cod_p.allowed_boostset_cats.iter().map(|b| *boostset_map.get(b.as_str()).unwrap()));
            // There is no Flight/Teleport & Sprints category, because Sprint doesn't fly or teleport, but Mids did a dumb.
            if cod_sets.contains(&mids::enums::SetType::Flight) {
                cod_sets.insert(mids::enums::SetType::FlightNoSprint);
            }
            if cod_sets.contains(&mids::enums::SetType::Teleport) {
                cod_sets.insert(mids::enums::SetType::TeleportNoSprint);
            }
            if mids_sets != cod_sets {
                let missing: Vec<mids::enums::SetType> = cod_sets.difference(&mids_sets).copied().collect();
                let extra: Vec<mids::enums::SetType> = mids_sets.difference(&cod_sets).copied().collect();
                if !missing.is_empty() {
                    if !extra.is_empty() {
                        println!(
                            "{} ({}): sets fixed adding {:?} removing {:?}",
                            p.full_name, p.display_name, missing, extra
                        );
                    } else {
                        println!("{} ({}): sets fixed adding {:?}", p.full_name, p.display_name, missing);
                    }
                } else if !extra.is_empty() {
                    println!("{} ({}): sets fixed removing {:?}", p.full_name, p.display_name, extra);
                }
                p.set_types = Vec::from_iter(cod_sets);
                p.set_types.sort_unstable();
                changed = true;
            }
        }

        // Fix level at which the power becomes available.
        let cod_level = if p.group_name == "Pool" && cod_p.available_level == 0 {
            4 // Pools aren't available before level 4.
        } else if cod_p.available_level == 0 && cod_p.power_lifetime != 0.0 {
            0 // Temporary powers shouldn't have levels. (In particular, Seismic Shockwaves.)
        } else {
            cod_p.available_level + 1
        };
        if p.level != cod_level {
            println!("{}: level {} fixed to {} [from {}]", p.full_name, p.level, cod_level, cod_p.full_name);
            p.level = cod_level;
            changed = true;
        }

        /*
        if p.group_name != "Boosts" && p.display_name != cod_p.display_name {
            println!(
                "{}: display_name {} should be {} [from {}]",
                p.full_name, p.display_name, cod_p.display_name, cod_p.full_name
            );
        }
        */

        // Powers that don't end in an "Always" don't always have sensible values. If Mids isn't
        // using redirects for this power, skip syncing certain problematic attributes (looking at
        // you, Time Bomb).
        let mids_has_redirect = p.effects.iter().any(|e| e.effect_type == mids::enums::EffectType::PowerRedirect);
        let cod_safe_redirect = !cod_p.redirect.last().is_some_and(|r| r.condition_expression != "Always");
        if mids_has_redirect || cod_safe_redirect {
            // Fix recharge time.
            if p.recharge_time != cod_p.recharge_time {
                println!(
                    "{} ({}): recharge_time {} fixed to {} [from {}]",
                    p.full_name, p.display_name, p.recharge_time, cod_p.recharge_time, cod_p.full_name
                );
                p.recharge_time = cod_p.recharge_time;
                p.base_recharge_time = p.recharge_time;
                changed = true;
            }
            // Fix cast time.
            if p.cast_time != cod_p.activation_time {
                println!(
                    "{} ({}): cast_time {} fixed to {} [from {}]",
                    p.full_name, p.display_name, p.cast_time, cod_p.activation_time, cod_p.full_name
                );
                p.cast_time = cod_p.activation_time;
                changed = true;
            }
        }
    }

    if changed {
        let (h, t) = mdb.version.rsplit_once('.').unwrap();
        let build: usize = t.parse().unwrap();
        mdb.version = format!("{}.{}", h, build + 1);
        println!("Updated DB version to {}", mdb.version);

        let w = prompt_save("Save location", "Mids Reborn Database (*.mhd)")?;
        mids::to_writer(w, &mdb)?;
    }
    Ok(())
}
