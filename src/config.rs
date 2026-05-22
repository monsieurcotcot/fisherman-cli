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

static GAME_DATA: OnceLock<GameData> = OnceLock::new();

pub fn get_game_data() -> &'static GameData {
    GAME_DATA.get_or_init(|| {
        let paths = vec![
            "/app/data/game_data.json",
            "data/game_data.json",
            "src/data/game_data.json",
        ];
        let mut json_str = None;
        for path in paths {
            if let Ok(content) = std::fs::read_to_string(path) {
                tracing::info!("Loaded game_data.json dynamically from {}", path);
                json_str = Some(content);
                break;
            }
        }
        
        let content = json_str.unwrap_or_else(|| {
            tracing::info!("Falling back to embedded game_data.json");
            include_str!("data/game_data.json").to_string()
        });
        
        serde_json::from_str(&content).expect("Failed to parse game_data.json")
    })
}

pub fn get_fish_data() -> HashMap<Rarity, Vec<FishData>> {
    get_game_data().fish_data.clone()
}

pub fn get_junk_data() -> HashMap<Rarity, Vec<FishData>> {
    get_game_data().junk_data.clone()
}

pub fn get_fail_attempt_reasons() -> Vec<&'static str> {
    get_game_data().fail_messages.iter().map(|s| s.as_str()).collect()
}
