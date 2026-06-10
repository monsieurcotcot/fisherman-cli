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
    pub scrap_metal_kg: Option<f64>,
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

pub type MyError = Box<dyn std::error::Error + Send + Sync>;

fn parse_json_from_file_or_embedded<T: serde::de::DeserializeOwned>(
    paths: &[&str],
    embedded: &str,
) -> Result<T, MyError> {
    for path in paths {
        if let Ok(content) = std::fs::read_to_string(path) {
            match serde_json::from_str(&content) {
                Ok(val) => {
                    tracing::info!("Loaded dynamic file from {}", path);
                    return Ok(val);
                }
                Err(e) => {
                    tracing::error!("Failed to parse dynamic file {}: {}. Falling back...", path, e);
                }
            }
        }
    }
    // Fallback directly to the compiled-in JSON embedded content
    let val = serde_json::from_str(embedded)?;
    Ok(val)
}

fn load_game_data_fr() -> Result<GameData, MyError> {
    let fish_data = parse_json_from_file_or_embedded(
        &["/app/data/fish_data.json", "data/fish_data.json"],
        include_str!("../data/fish_data.json"),
    )?;

    let junk_data = parse_json_from_file_or_embedded(
        &["/app/data/junk_data.json", "data/junk_data.json"],
        include_str!("../data/junk_data.json"),
    )?;

    let fail_messages = parse_json_from_file_or_embedded(
        &["/app/data/fail_messages.json", "data/fail_messages.json"],
        include_str!("../data/fail_messages.json"),
    )?;

    let cf_disappear_messages = parse_json_from_file_or_embedded(
        &["/app/data/cf_disappear_messages.json", "data/cf_disappear_messages.json"],
        include_str!("../data/cf_disappear_messages.json"),
    )?;

    let cf_edge_messages = parse_json_from_file_or_embedded(
        &["/app/data/cf_edge_messages.json", "data/cf_edge_messages.json"],
        include_str!("../data/cf_edge_messages.json"),
    )?;

    Ok(GameData {
        fish_data,
        junk_data,
        fail_messages,
        cf_disappear_messages,
        cf_edge_messages,
    })
}

fn load_game_data_en() -> Result<GameData, MyError> {
    let fish_data = parse_json_from_file_or_embedded(
        &["/app/data/fish_data_en.json", "data/fish_data_en.json"],
        include_str!("../data/fish_data.json"), // Fallback embedding FR if EN file missing
    )?;

    let junk_data = parse_json_from_file_or_embedded(
        &["/app/data/junk_data_en.json", "data/junk_data_en.json"],
        include_str!("../data/junk_data.json"), // Fallback embedding FR if EN file missing
    )?;

    let fail_messages = parse_json_from_file_or_embedded(
        &["/app/data/fail_messages_en.json", "data/fail_messages_en.json"],
        include_str!("../data/fail_messages.json"), // Fallback embedding FR if EN file missing
    )?;

    let cf_disappear_messages = parse_json_from_file_or_embedded(
        &["/app/data/cf_disappear_messages_en.json", "data/cf_disappear_messages_en.json"],
        include_str!("../data/cf_disappear_messages_en.json"),
    )?;

    let cf_edge_messages = parse_json_from_file_or_embedded(
        &["/app/data/cf_edge_messages_en.json", "data/cf_edge_messages_en.json"],
        include_str!("../data/cf_edge_messages_en.json"),
    )?;

    Ok(GameData {
        fish_data,
        junk_data,
        fail_messages,
        cf_disappear_messages,
        cf_edge_messages,
    })
}

pub fn get_game_data_fr() -> Arc<GameData> {
    if let Ok(reader) = GAME_DATA_FR.read() {
        if let Some(ref data) = *reader {
            return Arc::clone(data);
        }
    }
    // Fallback if not initialized or poisoned
    let data = Arc::new(load_game_data_fr().unwrap_or_else(|_| GameData {
        fish_data: HashMap::new(),
        junk_data: HashMap::new(),
        fail_messages: Vec::new(),
        cf_disappear_messages: Vec::new(),
        cf_edge_messages: Vec::new(),
    }));
    if let Ok(mut writer) = GAME_DATA_FR.write() {
        *writer = Some(Arc::clone(&data));
    }
    data
}

pub fn get_game_data_en() -> Arc<GameData> {
    if let Ok(reader) = GAME_DATA_EN.read() {
        if let Some(ref data) = *reader {
            return Arc::clone(data);
        }
    }
    // Fallback if not initialized or poisoned
    let data = Arc::new(load_game_data_en().unwrap_or_else(|_| GameData {
        fish_data: HashMap::new(),
        junk_data: HashMap::new(),
        fail_messages: Vec::new(),
        cf_disappear_messages: Vec::new(),
        cf_edge_messages: Vec::new(),
    }));
    if let Ok(mut writer) = GAME_DATA_EN.write() {
        *writer = Some(Arc::clone(&data));
    }
    data
}

pub fn reload_all_game_data() -> Result<(), MyError> {
    // 1. Safe parsing of both FR and EN configs from disk
    let new_fr = load_game_data_fr()?;
    let new_en = load_game_data_en()?;

    // 2. Wrap in Arc
    let fr_arc = Arc::new(new_fr);
    let en_arc = Arc::new(new_en);

    // 3. Write locks swap
    if let Ok(mut writer) = GAME_DATA_FR.write() {
        *writer = Some(fr_arc);
    } else {
        return Err("GAME_DATA_FR RwLock is poisoned".into());
    }

    if let Ok(mut writer) = GAME_DATA_EN.write() {
        *writer = Some(en_arc);
    } else {
        return Err("GAME_DATA_EN RwLock is poisoned".into());
    }

    // 4. Invalidate lowercase cache sets
    if let Ok(mut writer) = FISH_NAMES_LOWER_FR.write() {
        *writer = None;
    }
    if let Ok(mut writer) = JUNK_NAMES_LOWER_FR.write() {
        *writer = None;
    }
    if let Ok(mut writer) = FISH_NAMES_LOWER_EN.write() {
        *writer = None;
    }
    if let Ok(mut writer) = JUNK_NAMES_LOWER_EN.write() {
        *writer = None;
    }

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
#[allow(dead_code)]
pub fn get_fish_data() -> HashMap<Rarity, Vec<FishData>> {
    get_game_data_fr().fish_data.clone()
}

#[allow(dead_code)]
pub fn get_junk_data() -> HashMap<Rarity, Vec<FishData>> {
    get_game_data_fr().junk_data.clone()
}

#[allow(dead_code)]
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
        if let Ok(reader) = FISH_NAMES_LOWER_EN.read() {
            if let Some(ref set) = *reader {
                return Arc::clone(set);
            }
        }
        let game_data = get_game_data_en();
        let set: std::collections::HashSet<String> = game_data.fish_data
            .values()
            .flat_map(|v| v.iter().map(|f| f.name.to_lowercase()))
            .collect();
        let leaked = Arc::new(set);
        if let Ok(mut writer) = FISH_NAMES_LOWER_EN.write() {
            *writer = Some(Arc::clone(&leaked));
        }
        leaked
    } else {
        if let Ok(reader) = FISH_NAMES_LOWER_FR.read() {
            if let Some(ref set) = *reader {
                return Arc::clone(set);
            }
        }
        let game_data = get_game_data_fr();
        let set: std::collections::HashSet<String> = game_data.fish_data
            .values()
            .flat_map(|v| v.iter().map(|f| f.name.to_lowercase()))
            .collect();
        let leaked = Arc::new(set);
        if let Ok(mut writer) = FISH_NAMES_LOWER_FR.write() {
            *writer = Some(Arc::clone(&leaked));
        }
        leaked
    }
}

pub fn get_junk_names_lower(use_english: bool) -> Arc<std::collections::HashSet<String>> {
    if use_english {
        if let Ok(reader) = JUNK_NAMES_LOWER_EN.read() {
            if let Some(ref set) = *reader {
                return Arc::clone(set);
            }
        }
        let game_data = get_game_data_en();
        let set: std::collections::HashSet<String> = game_data.junk_data
            .values()
            .flat_map(|v| v.iter().map(|j| j.name.to_lowercase()))
            .collect();
        let leaked = Arc::new(set);
        if let Ok(mut writer) = JUNK_NAMES_LOWER_EN.write() {
            *writer = Some(Arc::clone(&leaked));
        }
        leaked
    } else {
        if let Ok(reader) = JUNK_NAMES_LOWER_FR.read() {
            if let Some(ref set) = *reader {
                return Arc::clone(set);
            }
        }
        let game_data = get_game_data_fr();
        let set: std::collections::HashSet<String> = game_data.junk_data
            .values()
            .flat_map(|v| v.iter().map(|j| j.name.to_lowercase()))
            .collect();
        let leaked = Arc::new(set);
        if let Ok(mut writer) = JUNK_NAMES_LOWER_FR.write() {
            *writer = Some(Arc::clone(&leaked));
        }
        leaked
    }
}

pub fn is_permanent_vip(username: &str) -> bool {
    let name_lower = username.to_lowercase();
    let paths = &["/app/data/permanent_vips.json", "data/permanent_vips.json"];
    for path in paths {
        if let Ok(content) = std::fs::read_to_string(path) {
            if let Ok(list) = serde_json::from_str::<Vec<String>>(&content) {
                return list.iter().any(|v| v.to_lowercase() == name_lower);
            }
        }
    }
    false
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_embedded_game_data() {
        // Test that the default statically-embedded content parses successfully without error
        let fr_data = load_game_data_fr();
        assert!(fr_data.is_ok(), "Embedded French game data must parse successfully");
        
        let en_data = load_game_data_en();
        assert!(en_data.is_ok(), "Embedded English game data must parse successfully");
    }

    #[test]
    fn test_parse_json_from_file_or_embedded_invalid_json() {
        // Test that invalid JSON content correctly returns an error instead of panicking
        let invalid_json = "{ invalid json }";
        let result: Result<HashMap<Rarity, Vec<FishData>>, MyError> = 
            parse_json_from_file_or_embedded(&["non_existent_file.json"], invalid_json);
        
        assert!(result.is_err(), "Invalid JSON must return an error and not panic");
    }

    #[test]
    fn test_is_permanent_vip() {
        // nigntube is in permanent_vips.json and should be resolved as permanent VIP case-insensitively
        assert!(is_permanent_vip("nigntube"));
        assert!(is_permanent_vip("NIGNTUBE"));
        assert!(!is_permanent_vip("non_existent_random_user"));
    }
}
