use std::collections::HashMap;
use std::sync::Arc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    VeryRare,
    Epic,
    Legendary,
    Mythical,
    Divin,
}

impl Rarity {
    pub fn odds() -> Vec<(Rarity, f64)> {
        vec![
            (Rarity::Common, 60.0),
            (Rarity::Uncommon, 20.0),
            (Rarity::Rare, 10.0),
            (Rarity::VeryRare, 5.78),
            (Rarity::Epic, 3.0),
            (Rarity::Legendary, 1.0),
            (Rarity::Mythical, 0.2),
            (Rarity::Divin, 0.02),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FishData {
    pub name: String,
    pub size_min: f64,
    pub size_mean: f64,
    pub size_sigma: f64,
    pub force_pristine: Option<bool>,
    pub force_state: Option<String>,
    pub descriptions: HashMap<String, Vec<String>>,
    pub id: Option<i32>,
    pub price: Option<i32>,
    pub location: Option<String>,
    pub preferred_time: Option<String>,
    pub preferred_season: Option<String>,
    pub months: Option<Vec<i32>>,
    pub fun_fact: Option<String>,
    pub time_restriction: Option<String>,
    pub bin: Option<String>,
    pub recycling_notoriety_bonus: Option<i64>,
    pub recycling_notoriety_malus: Option<i64>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum FailMessageEntry {
    Simple(String),
    Detailed(DetailedFailMessage),
}

#[derive(Deserialize, Debug, Clone)]
pub struct DetailedFailMessage {
    pub text: String,
    pub gold_penalty: Option<i64>,
    pub cooldown_penalty: Option<i64>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GameData {
    pub fish_data: HashMap<Rarity, Vec<FishData>>,
    pub junk_data: HashMap<Rarity, Vec<FishData>>,
    pub fail_messages: Vec<FailMessageEntry>,
    pub cf_disappear_messages: Vec<String>,
    pub cf_edge_messages: Vec<String>,
}

static GAME_DATA_FR: std::sync::RwLock<Option<Arc<GameData>>> = std::sync::RwLock::new(None);
static GAME_DATA_EN: std::sync::RwLock<Option<Arc<GameData>>> = std::sync::RwLock::new(None);

static FISH_NAMES_LOWER_FR: std::sync::RwLock<Option<Arc<std::collections::HashSet<String>>>> = std::sync::RwLock::new(None);
static JUNK_NAMES_LOWER_FR: std::sync::RwLock<Option<Arc<std::collections::HashSet<String>>>> = std::sync::RwLock::new(None);
static FISH_NAMES_LOWER_EN: std::sync::RwLock<Option<Arc<std::collections::HashSet<String>>>> = std::sync::RwLock::new(None);
static JUNK_NAMES_LOWER_EN: std::sync::RwLock<Option<Arc<std::collections::HashSet<String>>>> = std::sync::RwLock::new(None);

fn read_file_or_fallback(paths: &[&str], fallback: &str) -> String {
    for path in paths {
        if let Ok(content) = std::fs::read_to_string(path) {
            tracing::info!("Loaded dynamic file from {}", path);
            return content;
        }
    }
    tracing::info!("Falling back to embedded content");
    fallback.to_string()
}

fn load_game_data_fr() -> GameData {
    let fish_content = read_file_or_fallback(
        &["/app/data/fish_data.json", "data/fish_data.json"],
        include_str!("../data/fish_data.json"),
    );
    let fish_data: HashMap<Rarity, Vec<FishData>> = serde_json::from_str(&fish_content)
        .expect("Failed to parse fish_data.json");

    let junk_content = read_file_or_fallback(
        &["/app/data/junk_data.json", "data/junk_data.json"],
        include_str!("../data/junk_data.json"),
    );
    let junk_data: HashMap<Rarity, Vec<FishData>> = serde_json::from_str(&junk_content)
        .expect("Failed to parse junk_data.json");

    let fail_content = read_file_or_fallback(
        &["/app/data/fail_messages.json", "data/fail_messages.json"],
        include_str!("../data/fail_messages.json"),
    );
    let fail_messages: Vec<FailMessageEntry> = serde_json::from_str(&fail_content)
        .expect("Failed to parse fail_messages.json");

    let disappear_content = read_file_or_fallback(
        &["/app/data/cf_disappear_messages.json", "data/cf_disappear_messages.json"],
        include_str!("../data/cf_disappear_messages.json"),
    );
    let cf_disappear_messages: Vec<String> = serde_json::from_str(&disappear_content)
        .expect("Failed to parse cf_disappear_messages.json");

    let edge_content = read_file_or_fallback(
        &["/app/data/cf_edge_messages.json", "data/cf_edge_messages.json"],
        include_str!("../data/cf_edge_messages.json"),
    );
    let cf_edge_messages: Vec<String> = serde_json::from_str(&edge_content)
        .expect("Failed to parse cf_edge_messages.json");

    GameData {
        fish_data,
        junk_data,
        fail_messages,
        cf_disappear_messages,
        cf_edge_messages,
    }
}

fn load_game_data_en() -> GameData {
    let fish_content = read_file_or_fallback(
        &["/app/data/fish_data_en.json", "data/fish_data_en.json"],
        include_str!("../data/fish_data.json"), // Fallback embedding FR if EN file missing
    );
    let fish_data: HashMap<Rarity, Vec<FishData>> = serde_json::from_str(&fish_content)
        .expect("Failed to parse fish_data_en.json");

    let junk_content = read_file_or_fallback(
        &["/app/data/junk_data_en.json", "data/junk_data_en.json"],
        include_str!("../data/junk_data.json"), // Fallback embedding FR if EN file missing
    );
    let junk_data: HashMap<Rarity, Vec<FishData>> = serde_json::from_str(&junk_content)
        .expect("Failed to parse junk_data_en.json");

    let fail_content = read_file_or_fallback(
        &["/app/data/fail_messages_en.json", "data/fail_messages_en.json"],
        include_str!("../data/fail_messages.json"), // Fallback embedding FR if EN file missing
    );
    let fail_messages: Vec<FailMessageEntry> = serde_json::from_str(&fail_content)
        .expect("Failed to parse fail_messages_en.json");

    let disappear_content = read_file_or_fallback(
        &["/app/data/cf_disappear_messages_en.json", "data/cf_disappear_messages_en.json"],
        include_str!("../data/cf_disappear_messages_en.json"),
    );
    let cf_disappear_messages: Vec<String> = serde_json::from_str(&disappear_content)
        .expect("Failed to parse cf_disappear_messages_en.json");

    let edge_content = read_file_or_fallback(
        &["/app/data/cf_edge_messages_en.json", "data/cf_edge_messages_en.json"],
        include_str!("../data/cf_edge_messages_en.json"),
    );
    let cf_edge_messages: Vec<String> = serde_json::from_str(&edge_content)
        .expect("Failed to parse cf_edge_messages_en.json");

    GameData {
        fish_data,
        junk_data,
        fail_messages,
        cf_disappear_messages,
        cf_edge_messages,
    }
}

pub fn get_game_data_fr() -> Arc<GameData> {
    {
        if let Some(ref data) = *GAME_DATA_FR.read().unwrap() {
            return Arc::clone(data);
        }
    }
    let data = Arc::new(load_game_data_fr());
    let mut writer = GAME_DATA_FR.write().unwrap();
    *writer = Some(Arc::clone(&data));
    data
}

pub fn get_game_data_en() -> Arc<GameData> {
    {
        if let Some(ref data) = *GAME_DATA_EN.read().unwrap() {
            return Arc::clone(data);
        }
    }
    let data = Arc::new(load_game_data_en());
    let mut writer = GAME_DATA_EN.write().unwrap();
    *writer = Some(Arc::clone(&data));
    data
}

pub fn reload_all_game_data() -> Result<(), String> {
    // 1. Safe parsing of both FR and EN configs from disk
    let new_fr = std::panic::catch_unwind(|| load_game_data_fr());
    if new_fr.is_err() {
        return Err("Failed to reload French data - check JSON syntax".to_string());
    }

    let new_en = std::panic::catch_unwind(|| load_game_data_en());
    if new_en.is_err() {
        return Err("Failed to reload English data - check JSON syntax".to_string());
    }

    // 2. Wrap in Arc
    let fr_arc = Arc::new(new_fr.unwrap());
    let en_arc = Arc::new(new_en.unwrap());

    // 3. Write locks swap
    *GAME_DATA_FR.write().unwrap() = Some(fr_arc);
    *GAME_DATA_EN.write().unwrap() = Some(en_arc);

    // 4. Invalidate lowercase cache sets
    *FISH_NAMES_LOWER_FR.write().unwrap() = None;
    *JUNK_NAMES_LOWER_FR.write().unwrap() = None;
    *FISH_NAMES_LOWER_EN.write().unwrap() = None;
    *JUNK_NAMES_LOWER_EN.write().unwrap() = None;

    tracing::info!("✅ Game data successfully reloaded dynamically!");
    Ok(())
}

// Get game data by language
pub fn get_game_data(use_english: bool) -> Arc<GameData> {
    if use_english {
        get_game_data_en()
    } else {
        get_game_data_fr()
    }
}

// Fallback old methods cloning French data (backward compatibility)
pub fn get_fish_data() -> HashMap<Rarity, Vec<FishData>> {
    get_game_data_fr().fish_data.clone()
}

pub fn get_junk_data() -> HashMap<Rarity, Vec<FishData>> {
    get_game_data_fr().junk_data.clone()
}

pub fn get_fail_attempt_reasons_old() -> Vec<FailMessageEntry> {
    get_game_data_fr().fail_messages.clone()
}

pub fn get_fail_attempt_reasons(use_english: bool) -> Vec<FailMessageEntry> {
    let data = if use_english { get_game_data_en() } else { get_game_data_fr() };
    data.fail_messages.clone()
}

// Caching and thread-safe helpers for lowercase sets
pub fn get_fish_names_lower(use_english: bool) -> Arc<std::collections::HashSet<String>> {
    if use_english {
        {
            if let Some(ref set) = *FISH_NAMES_LOWER_EN.read().unwrap() {
                return Arc::clone(set);
            }
        }
        let game_data = get_game_data_en();
        let set: std::collections::HashSet<String> = game_data.fish_data
            .values()
            .flat_map(|v| v.iter().map(|f| f.name.to_lowercase()))
            .collect();
        let leaked = Arc::new(set);
        let mut writer = FISH_NAMES_LOWER_EN.write().unwrap();
        *writer = Some(Arc::clone(&leaked));
        leaked
    } else {
        {
            if let Some(ref set) = *FISH_NAMES_LOWER_FR.read().unwrap() {
                return Arc::clone(set);
            }
        }
        let game_data = get_game_data_fr();
        let set: std::collections::HashSet<String> = game_data.fish_data
            .values()
            .flat_map(|v| v.iter().map(|f| f.name.to_lowercase()))
            .collect();
        let leaked = Arc::new(set);
        let mut writer = FISH_NAMES_LOWER_FR.write().unwrap();
        *writer = Some(Arc::clone(&leaked));
        leaked
    }
}

pub fn get_junk_names_lower(use_english: bool) -> Arc<std::collections::HashSet<String>> {
    if use_english {
        {
            if let Some(ref set) = *JUNK_NAMES_LOWER_EN.read().unwrap() {
                return Arc::clone(set);
            }
        }
        let game_data = get_game_data_en();
        let set: std::collections::HashSet<String> = game_data.junk_data
            .values()
            .flat_map(|v| v.iter().map(|j| j.name.to_lowercase()))
            .collect();
        let leaked = Arc::new(set);
        let mut writer = JUNK_NAMES_LOWER_EN.write().unwrap();
        *writer = Some(Arc::clone(&leaked));
        leaked
    } else {
        {
            if let Some(ref set) = *JUNK_NAMES_LOWER_FR.read().unwrap() {
                return Arc::clone(set);
            }
        }
        let game_data = get_game_data_fr();
        let set: std::collections::HashSet<String> = game_data.junk_data
            .values()
            .flat_map(|v| v.iter().map(|j| j.name.to_lowercase()))
            .collect();
        let leaked = Arc::new(set);
        let mut writer = JUNK_NAMES_LOWER_FR.write().unwrap();
        *writer = Some(Arc::clone(&leaked));
        leaked
    }
}
