use core::fmt;
use serde::Deserialize;
use std::sync::LazyLock;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Mission {
    pub name: String,
    description: String,
    #[serde(default)]
    details: Vec<String>,
    #[serde(default, rename = "200cc_adjustment")]
    two_hundred_cc_adjustment: String,
    #[serde(default)] // Makes this optional
    needs_random_item: bool,
    #[serde(default)] // Makes this optional
    needs_random_loadout: bool,
    #[serde(default)] // Makes this optional
    needs_random_number: usize,
    #[serde(default, rename = "all_items")]
    needs_item_checklist: bool,
    #[serde(default)]
    needs_gacha_item_checklist: bool,
    #[serde(default)]
    needs_coop_singles: bool,
}

impl fmt::Display for Mission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Deserialize)]
struct Missions {
    #[serde(rename = "missions")]
    solo_missions: Vec<Mission>,
    #[serde(rename = "coop_granprix")]
    coop_missions: Vec<Mission>,
    #[serde(rename = "coop_single")]
    coop_single_missions: Vec<Mission>,
}

pub static SOLO_MISSIONS: LazyLock<Vec<Mission>> = std::sync::LazyLock::new(|| {
    toml::from_str::<Missions>(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/missions.toml"
    )))
    .unwrap()
    .solo_missions
});
pub static COOP_MISSIONS: LazyLock<Vec<Mission>> = std::sync::LazyLock::new(|| {
    toml::from_str::<Missions>(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/missions.toml"
    )))
    .unwrap()
    .coop_missions
});
pub static COOP_SINGLE_MISSIONS: LazyLock<Vec<Mission>> = std::sync::LazyLock::new(|| {
    toml::from_str::<Missions>(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/missions.toml"
    )))
    .unwrap()
    .coop_single_missions
});
