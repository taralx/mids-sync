use std::{
    collections::HashMap,
    io::{Read, Seek},
};

use serde::{de::DeserializeOwned, Deserialize};
use zip::ZipArchive;

#[derive(Debug, Deserialize)]
pub struct ArchetypesIndex {
    pub player_archetypes: Vec<String>,
    pub npc_archetypes: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Archetype {
    pub name: String,
    pub primary_category: String,
    pub secondary_category: String,
}

#[derive(Debug, Deserialize)]
pub struct PowersIndex {
    pub power_categories: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct PowerCategory {
    pub powerset_names: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Powerset {
    pub power_names: Vec<String>,
    pub power_display_names: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Power {
    pub full_name: String,
    pub display_name: String,
    pub accuracy: f32,
    pub activation_time: f32,
    pub recharge_time: f32,
    pub boosts_allowed: Vec<String>,
    pub allowed_boostset_cats: Vec<String>,
    pub available_level: i32,
    pub redirect: Vec<Redirect>,
}

#[derive(Debug, Deserialize)]
pub struct Redirect {
    pub name: String,
    pub condition_expression: String,
    pub show_in_info: bool,
}

pub struct Database<R: Read + Seek> {
    pub zip: ZipArchive<R>,
}

impl<R: Read + Seek> Database<R> {
    pub fn load_json<T: DeserializeOwned>(&mut self, path: impl AsRef<str>) -> anyhow::Result<T> {
        Ok(serde_json::from_reader(self.zip.by_name(path.as_ref())?)?)
    }

    pub fn revision(&mut self) -> anyhow::Result<String> {
        #[derive(Deserialize)]
        struct Index {
            revision: String,
        }
        Ok(self.load_json::<Index>("index.json")?.revision)
    }

    pub fn load_power(&mut self, name: impl AsRef<str>) -> anyhow::Result<Power> {
        let mut path = format!("powers/{}.json", name.as_ref().replace(".", "/"));
        path.make_ascii_lowercase();
        self.load_json(path)
    }

    pub fn index(&mut self) -> anyhow::Result<HashMap<String, HashMap<String, String>>> {
        let mut map = HashMap::new();
        let pi: PowersIndex = self.load_json("powers/index.json")?;
        for mut cat in pi.power_categories {
            cat.make_ascii_lowercase();
            let pc: PowerCategory = self.load_json(format!("powers/{}/index.json", cat))?;
            for set in pc.powerset_names {
                // set is {category}.{set} but already lowercase.
                let Some((_, set_dir)) = set.split_once('.') else {
                    anyhow::bail!("data error: {:?} (from {}) missing dot", set, cat)
                };
                let ps: Powerset = self.load_json(format!("powers/{}/{}/index.json", cat, set_dir))?;
                let mut submap = HashMap::new();
                for (display_name, power_name) in ps.power_display_names.into_iter().zip(ps.power_names) {
                    submap.entry(display_name).and_modify(String::clear).or_insert(power_name);
                }
                submap.retain(|_k, v| !v.is_empty());
                map.insert(set, submap);
            }
        }
        Ok(map)
    }
}
