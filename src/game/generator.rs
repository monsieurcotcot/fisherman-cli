use rand::seq::SliceRandom;
use rand::Rng;
use rand_distr::{Distribution, Normal};
use crate::config::{get_fish_ref, get_junk_ref, Rarity, FishData};
use crate::models::Fish;

pub fn generate_fish(use_english: bool) -> Option<Fish> {
    generate_item(false, use_english)
}

pub fn generate_junk(use_english: bool) -> Option<Fish> {
    generate_item(true, use_english)
}

fn generate_item(is_junk: bool, use_english: bool) -> Option<Fish> {
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
    
    let all_data = if is_junk { get_junk_ref(use_english) } else { get_fish_ref(use_english) };
    let item_list = all_data.get(&selected_rarity)?;
    
    let item_data: &FishData = if is_junk {
        item_list.choose(&mut rng)?
    } else {
        use chrono::Datelike;
        use chrono::Timelike;
        
        let now = chrono::Local::now();
        let current_month = now.month() as i32;
        let current_hour = now.hour();
        
        // Pondération horaire (Stream 20h - 00h, après 22h la logique s'inverse)
        // Considère après 22h entre 22h et 4h du matin (prolongation live)
        let is_after_22h = current_hour >= 22 || current_hour < 4;
        
        let mut weighted_pool = Vec::new();
        for fish in item_list {
            // A. Pondération saisonnière : 5 si en saison (ou sans restriction), 1 si hors-saison (20% de probabilité)
            let s_weight = match &fish.months {
                Some(months) if !months.is_empty() => {
                    if months.contains(&current_month) { 5 } else { 1 }
                }
                _ => 5,
            };
            
            // B. Pondération horaire
            let t_weight = match &fish.time_restriction {
                Some(restriction) => {
                    if restriction == "before_22h" {
                        if is_after_22h { 1 } else { 5 }
                    } else if restriction == "after_22h" {
                        if is_after_22h { 5 } else { 1 }
                    } else {
                        5
                    }
                }
                None => 5,
            };
            
            // Le poids combiné est le produit des deux pondérations
            weighted_pool.push((fish, s_weight * t_weight));
        }
        
        let total_pool_weight: i32 = weighted_pool.iter().map(|(_, w)| w).sum();
        if total_pool_weight == 0 {
            item_list.choose(&mut rng)?
        } else {
            let mut choice = rng.gen_range(0..total_pool_weight);
            let mut selected_fish = item_list.choose(&mut rng)?;
            for (fish, weight) in &weighted_pool {
                if choice < *weight {
                    selected_fish = *fish;
                    break;
                }
                choice -= weight;
            }
            selected_fish
        }
    };
    
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
        if w < 0.01 { w = 0.01; }
        (s, w)
    };
    
    // Generate state
    let (states, weights) = if is_junk {
        (vec!["badly damaged", "damaged", "worn"], vec![30, 40, 30])
    } else {
        (vec!["badly damaged", "damaged", "worn", "good", "pristine"], vec![10, 30, 40, 30, 10])
    };
    
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_fish() {
        // Since get_fish_ref reads from game_data.json, this test also validates the JSON structure.
        let fish = generate_fish(false);
        assert!(fish.is_some(), "Should generate at least one fish");
        let f = fish.unwrap();
        assert!(!f.name.is_empty(), "Fish name should not be empty");
        assert!(f.size > 0.0, "Fish size should be positive");
        assert!(f.weight > 0.0, "Fish weight should be positive");
    }

    #[test]
    fn test_generate_junk() {
        let junk = generate_junk(false);
        assert!(junk.is_some(), "Should generate at least one junk");
        let j = junk.unwrap();
        assert!(!j.name.is_empty(), "Junk name should not be empty");
        // Check if state is damaged or badly damaged since junk is always damaged
        assert!(j.state == "damaged" || j.state == "badly damaged" || j.state == "worn", "Junk should have a damaged state");
    }
}
