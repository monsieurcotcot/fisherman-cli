use rand::seq::SliceRandom;
use rand::Rng;
use rand_distr::{Distribution, Normal};
use crate::config::{get_fish_data, Rarity, FishData};
use crate::models::Fish;

pub fn generate_fish() -> Option<Fish> {
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
    
    let all_fish_data = get_fish_data();
    let fish_list = all_fish_data.get(&selected_rarity)?;
    let fish_data: &FishData = fish_list.choose(&mut rng)?;
    
    // Generate size based on normal distribution
    let normal = Normal::new(fish_data.size_mean, fish_data.size_sigma).ok()?;
    let mut size = normal.sample(&mut rng);
    if size < fish_data.size_min {
        size = fish_data.size_min;
    }
    size = (size * 100.0).round() / 100.0; // Round to 2 decimals

    // Generate weight based on size (cubic relationship: mass ~ volume)
    // Formula: weight (g) = factor * size(cm)^3
    // We use a base factor of 0.01 (average for many fish)
    let mut weight = 0.01 * size.powi(3);
    weight = (weight * 100.0).round() / 100.0; // Round to 2 decimals
    
    // Generate state
    let states = vec!["badly damaged", "damaged", "worn", "good", "pristine"];
    let weights = vec![10, 30, 40, 30, 10];
    
    let state = if fish_data.force_pristine.unwrap_or(false) {
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
    let descriptions = fish_data.descriptions.get(&state)
        .or_else(|| fish_data.descriptions.get("good")) // Fallback if no specific desc
        .cloned()
        .unwrap_or_else(|| vec!["No description available.".to_string()]);
        
    let description = descriptions.choose(&mut rng).cloned().unwrap_or_else(|| "No description available.".to_string());
    
    Some(Fish::new(
        fish_data.name.clone(),
        selected_rarity,
        size,
        weight,
        state,
        description,
    ))
}
