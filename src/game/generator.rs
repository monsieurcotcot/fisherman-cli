use rand::seq::SliceRandom;
use rand::Rng;
use rand_distr::{Distribution, Normal};
use crate::config::{get_fish_data, get_junk_data, Rarity, FishData};
use crate::models::Fish;

pub fn generate_fish() -> Option<Fish> {
    generate_item(false)
}

pub fn generate_junk() -> Option<Fish> {
    generate_item(true)
}

fn generate_item(is_junk: bool) -> Option<Fish> {
    let mut rng = rand::thread_rng();
    
    // Choose rarity based on odds
    let odds = Rarity::odds();
    let total_weight: f64 = odds.iter().map(|(_, w)| w).sum();
    let mut choice = rng.gen_range(0.0..total_weight);
    
    let mut selected_rarity = Rarity::Common;
    for (rarity, weight) in odds {
        if choice < weight {
            selected_rarity = rarity;
            break;
        }
        choice -= weight;
    }
    
    let all_data = if is_junk { get_junk_data() } else { get_fish_data() };
    let item_list = all_data.get(&selected_rarity)?;
    let item_data: &FishData = item_list.choose(&mut rng)?;
    
    // Generate size and weight (only for fish)
    let (size, weight) = if is_junk {
        (0.0, 0.0)
    } else {
        let normal = Normal::new(item_data.size_mean, item_data.size_sigma).ok()?;
        let mut s = normal.sample(&mut rng);
        if s < item_data.size_min { s = item_data.size_min; }
        s = (s * 100.0).round() / 100.0;
        let mut w = 0.01 * s.powi(3);
        w = (w * 100.0).round() / 100.0;
        (s, w)
    };
    
    // Generate state
    let states = vec!["badly damaged", "damaged", "worn", "good", "pristine"];
    let weights = vec![10, 30, 40, 30, 10];
    
    let state = if let Some(s) = &item_data.force_state {
        s.clone()
    } else if item_data.force_pristine.unwrap_or(false) {
        "pristine".to_string()
    } else {
        // Weighted random choice for state
        let total_state_weight: i32 = weights.iter().sum();
        let mut state_choice = rng.gen_range(0..total_state_weight);
        let mut selected_state = "worn";
        for (i, weight) in weights.iter().enumerate() {
            if state_choice < *weight {
                selected_state = states[i];
                break;
            }
            state_choice -= weight;
        }
        selected_state.to_string()
    };
    
    // Get description for the state
    let descriptions = item_data.descriptions.get(&state)
        .or_else(|| item_data.descriptions.get("good")) // Fallback if no specific desc
        .cloned()
        .unwrap_or_else(|| vec!["No description available.".to_string()]);
        
    let description = descriptions.choose(&mut rng).cloned().unwrap_or_else(|| "No description available.".to_string());
    
    if is_junk {
        Some(Fish::new_junk(
            item_data.name.clone(),
            selected_rarity,
            size,
            weight,
            state,
            description,
        ))
    } else {
        Some(Fish::new(
            item_data.name.clone(),
            selected_rarity,
            size,
            weight,
            state,
            description,
        ))
    }
}
