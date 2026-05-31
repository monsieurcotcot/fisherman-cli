use std::collections::HashMap;
use std::sync::OnceLock;
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
}

#[derive(Deserialize, Debug, Clone)]
pub struct GameData {
    pub fish_data: HashMap<Rarity, Vec<FishData>>,
    pub junk_data: HashMap<Rarity, Vec<FishData>>,
    pub fail_messages: Vec<String>,
}

static GAME_DATA_FR: std::sync::RwLock<Option<&'static GameData>> = std::sync::RwLock::new(None);
static GAME_DATA_EN: std::sync::RwLock<Option<&'static GameData>> = std::sync::RwLock::new(None);

static FISH_NAMES_LOWER_FR: std::sync::RwLock<Option<&'static std::collections::HashSet<String>>> = std::sync::RwLock::new(None);
static JUNK_NAMES_LOWER_FR: std::sync::RwLock<Option<&'static std::collections::HashSet<String>>> = std::sync::RwLock::new(None);
static FISH_NAMES_LOWER_EN: std::sync::RwLock<Option<&'static std::collections::HashSet<String>>> = std::sync::RwLock::new(None);
static JUNK_NAMES_LOWER_EN: std::sync::RwLock<Option<&'static std::collections::HashSet<String>>> = std::sync::RwLock::new(None);

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
    let fail_messages: Vec<String> = serde_json::from_str(&fail_content)
        .expect("Failed to parse fail_messages.json");

    GameData {
        fish_data,
        junk_data,
        fail_messages,
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
    let fail_messages: Vec<String> = serde_json::from_str(&fail_content)
        .expect("Failed to parse fail_messages_en.json");

    GameData {
        fish_data,
        junk_data,
        fail_messages,
    }
}

pub fn get_game_data_fr() -> &'static GameData {
    {
        if let Some(data) = *GAME_DATA_FR.read().unwrap() {
            return data;
        }
    }
    let data = load_game_data_fr();
    let leaked = Box::leak(Box::new(data));
    let mut writer = GAME_DATA_FR.write().unwrap();
    *writer = Some(leaked);
    leaked
}

pub fn get_game_data_en() -> &'static GameData {
    {
        if let Some(data) = *GAME_DATA_EN.read().unwrap() {
            return data;
        }
    }
    let data = load_game_data_en();
    let leaked = Box::leak(Box::new(data));
    let mut writer = GAME_DATA_EN.write().unwrap();
    *writer = Some(leaked);
    leaked
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

    // 2. Leak static references
    let fr_leaked = Box::leak(Box::new(new_fr.unwrap()));
    let en_leaked = Box::leak(Box::new(new_en.unwrap()));

    // 3. Write locks swap
    *GAME_DATA_FR.write().unwrap() = Some(fr_leaked);
    *GAME_DATA_EN.write().unwrap() = Some(en_leaked);

    // 4. Invalidate lowercase cache sets
    *FISH_NAMES_LOWER_FR.write().unwrap() = None;
    *JUNK_NAMES_LOWER_FR.write().unwrap() = None;
    *FISH_NAMES_LOWER_EN.write().unwrap() = None;
    *JUNK_NAMES_LOWER_EN.write().unwrap() = None;

    tracing::info!("✅ Game data successfully reloaded dynamically!");
    Ok(())
}

// Get game data by language
pub fn get_game_data(use_english: bool) -> &'static GameData {
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

pub fn get_fail_attempt_reasons_old() -> Vec<&'static str> {
    get_game_data_fr().fail_messages.iter().map(|s| s.as_str()).collect()
}

// Direct static reference methods to avoid cloning completely
pub fn get_fish_ref(use_english: bool) -> &'static HashMap<Rarity, Vec<FishData>> {
    if use_english {
        &get_game_data_en().fish_data
    } else {
        &get_game_data_fr().fish_data
    }
}

pub fn get_junk_ref(use_english: bool) -> &'static HashMap<Rarity, Vec<FishData>> {
    if use_english {
        &get_game_data_en().junk_data
    } else {
        &get_game_data_fr().junk_data
    }
}

pub fn get_fail_attempt_reasons(use_english: bool) -> Vec<&'static str> {
    let data = if use_english { get_game_data_en() } else { get_game_data_fr() };
    data.fail_messages.iter().map(|s| s.as_str()).collect()
}

// Caching and thread-safe OnceLock helper getters for lowercase sets
pub fn get_fish_names_lower(use_english: bool) -> &'static std::collections::HashSet<String> {
    if use_english {
        {
            if let Some(set) = *FISH_NAMES_LOWER_EN.read().unwrap() {
                return set;
            }
        }
        let set: std::collections::HashSet<String> = get_fish_ref(true)
            .values()
            .flat_map(|v| v.iter().map(|f| f.name.to_lowercase()))
            .collect();
        let leaked = Box::leak(Box::new(set));
        let mut writer = FISH_NAMES_LOWER_EN.write().unwrap();
        *writer = Some(leaked);
        leaked
    } else {
        {
            if let Some(set) = *FISH_NAMES_LOWER_FR.read().unwrap() {
                return set;
            }
        }
        let set: std::collections::HashSet<String> = get_fish_ref(false)
            .values()
            .flat_map(|v| v.iter().map(|f| f.name.to_lowercase()))
            .collect();
        let leaked = Box::leak(Box::new(set));
        let mut writer = FISH_NAMES_LOWER_FR.write().unwrap();
        *writer = Some(leaked);
        leaked
    }
}

pub fn get_junk_names_lower(use_english: bool) -> &'static std::collections::HashSet<String> {
    if use_english {
        {
            if let Some(set) = *JUNK_NAMES_LOWER_EN.read().unwrap() {
                return set;
            }
        }
        let set: std::collections::HashSet<String> = get_junk_ref(true)
            .values()
            .flat_map(|v| v.iter().map(|j| j.name.to_lowercase()))
            .collect();
        let leaked = Box::leak(Box::new(set));
        let mut writer = JUNK_NAMES_LOWER_EN.write().unwrap();
        *writer = Some(leaked);
        leaked
    } else {
        {
            if let Some(set) = *JUNK_NAMES_LOWER_FR.read().unwrap() {
                return set;
            }
        }
        let set: std::collections::HashSet<String> = get_junk_ref(false)
            .values()
            .flat_map(|v| v.iter().map(|j| j.name.to_lowercase()))
            .collect();
        let leaked = Box::leak(Box::new(set));
        let mut writer = JUNK_NAMES_LOWER_FR.write().unwrap();
        *writer = Some(leaked);
        leaked
    }
}
