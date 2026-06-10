use std::sync::Arc;
use chrono::Utc;
use rand::seq::SliceRandom;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::ClientConfig;
use twitch_irc::TwitchIRCClient;
use twitch_irc::message::ServerMessage;
use twitch_irc::SecureTCPTransport;

use crate::{AppState, PendingSale, PendingTrade, PendingGive};
use crate::game::{generate_fish, generate_junk};
use crate::config::{get_fail_attempt_reasons, FailMessageEntry, Rarity};

#[derive(Debug, PartialEq, Clone)]
pub enum SellArg {
    ConfirmYes,
    ConfirmNo,
    ById(i64),
    All {
        rarity: Option<Rarity>,
        state: Option<String>,
    },
    ByName {
        name: String,
        state: Option<String>,
        quantity: i64,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum GiveArg {
    ConfirmYes,
    ConfirmNo,
    Gold { amount: i64, recipient: String },
    ById { catch_id: i64, recipient: String },
    ByName { name: String, recipient: String },
}

/// Extrait l'état (éventuellement composé) et la quantité des tokens de commande.
/// Gère l'ordre flexible : "Bar pristine 2" ou "Bar 2 pristine".
fn extract_state_and_quantity(tokens: &mut Vec<&str>) -> (Option<String>, i64) {
    let mut state = None;
    let mut quantity = 1;

    // 1. D'abord, regarder les deux derniers tokens pour les états composés (ex: "badly damaged")
    let n = tokens.len();
    if n >= 2 {
        let last_two = format!("{} {}", tokens[n - 2], tokens[n - 1]).to_lowercase();
        if last_two == "badly damaged" || last_two == "très endommagé" || last_two == "tres endommage" {
            state = Some("badly damaged".to_string());
            tokens.pop();
            tokens.pop();
        }
    }

    // Si pas d'état composé trouvé, regarder le dernier token
    if state.is_none() && !tokens.is_empty() {
        if let Some(&last) = tokens.last() {
            let matched = match last.to_lowercase().as_str() {
                "pristine" | "parfait" => Some("pristine".to_string()),
                "good" | "bon" | "bonne" => Some("good".to_string()),
                "worn" | "usé" | "use" => Some("worn".to_string()),
                "damaged" | "endommagé" | "endommage" => Some("damaged".to_string()),
                "badly" | "tres" | "très" => Some("badly damaged".to_string()),
                _ => None,
            };
            if let Some(s) = matched {
                state = Some(s);
                tokens.pop();
            }
        }
    }

    // 2. Regarder si le nouveau dernier token est un nombre (quantité)
    if !tokens.is_empty() {
        if let Some(&last) = tokens.last() {
            if let Ok(qty) = last.parse::<i64>() {
                if qty > 0 {
                    quantity = qty;
                    tokens.pop();
                }
            }
        }
    }

    // 3. Si on a trouvé une quantité mais pas encore d'état (ordre: "Bar pristine 2"),
    // l'état se trouve alors juste avant la quantité !
    if state.is_none() && !tokens.is_empty() {
        let n = tokens.len();
        if n >= 2 {
            let last_two = format!("{} {}", tokens[n - 2], tokens[n - 1]).to_lowercase();
            if last_two == "badly damaged" || last_two == "très endommagé" || last_two == "tres endommage" {
                state = Some("badly damaged".to_string());
                tokens.pop();
                tokens.pop();
            }
        }
        
        if state.is_none() && !tokens.is_empty() {
            if let Some(&last) = tokens.last() {
                let matched = match last.to_lowercase().as_str() {
                    "pristine" | "parfait" => Some("pristine".to_string()),
                    "good" | "bon" | "bonne" => Some("good".to_string()),
                    "worn" | "usé" | "use" => Some("worn".to_string()),
                    "damaged" | "endommagé" | "endommage" => Some("damaged".to_string()),
                    "badly" | "tres" | "très" => Some("badly damaged".to_string()),
                    _ => None,
                };
                if let Some(s) = matched {
                    state = Some(s);
                    tokens.pop();
                }
            }
        }
    }

    (state, quantity)
}

pub fn normalize_bin_name(name: &str) -> Option<&'static str> {
    match name.to_lowercase().as_str() {
        "bleu" | "bleue" | "blue" => Some("bleu"),
        "jaune" | "yellow" => Some("jaune"),
        "vert" | "verte" | "green" => Some("vert"),
        "marron" | "brown" => Some("marron"),
        "gris" | "grise" | "gray" | "grey" => Some("gris"),
        "noir" | "noire" | "black" => Some("noir"),
        "decharge" | "décharge" | "landfill" | "scrapyard" | "dump" | "scrap" => Some("decharge"),
        _ => None,
    }
}

pub fn parse_recycle_args(args_str: &str) -> Option<(i64, String)> {
    let parts: Vec<&str> = args_str.split_whitespace().collect();
    if parts.len() < 2 {
        return None;
    }
    
    let id_str = parts[0].trim();
    let id = if id_str.starts_with('#') {
        id_str[1..].parse::<i64>().ok()?
    } else {
        id_str.parse::<i64>().ok()?
    };
    
    let bin_str = parts[1].trim().to_lowercase();
    Some((id, bin_str))
}

pub fn parse_sell_args(args_str: &str) -> Option<SellArg> {
    let args_str = args_str.trim();
    if args_str.is_empty() {
        return None;
    }

    let lower = args_str.to_lowercase();
    if lower == "oui" || lower == "yes" || lower == "y" {
        return Some(SellArg::ConfirmYes);
    }
    if lower == "non" || lower == "no" || lower == "n" {
        return Some(SellArg::ConfirmNo);
    }
    if lower == "all" || lower == "tout" || lower.starts_with("all ") || lower.starts_with("tout ") {
        let remainder = if lower.starts_with("all") {
            args_str[3..].trim()
        } else {
            args_str[4..].trim()
        };

        if remainder.is_empty() {
            return Some(SellArg::All { rarity: None, state: None });
        }

        let remainder_lower = remainder.to_lowercase();
        let tokens: Vec<&str> = remainder_lower.split_whitespace().collect();

        let mut parsed_rarity = None;

        let get_rarity = |tok: &str| -> Option<Rarity> {
            match tok {
                "common" | "commun" => Some(Rarity::Common),
                "uncommon" | "peu commun" | "peu-commun" => Some(Rarity::Uncommon),
                "rare" => Some(Rarity::Rare),
                "veryrare" | "very-rare" | "very rare" | "très rare" | "tres rare" | "très-rare" | "tres-rare" => Some(Rarity::VeryRare),
                "epic" | "épique" | "epique" => Some(Rarity::Epic),
                "legendary" | "légendaire" | "legendaire" => Some(Rarity::Legendary),
                "mythical" | "mythique" => Some(Rarity::Mythical),
                "divin" | "divine" => Some(Rarity::Divin),
                _ => None,
            }
        };

        let mut checked_state = None;
        let mut cleaned_tokens = tokens.clone();

        if remainder_lower.contains("badly damaged") || remainder_lower.contains("très endommagé") || remainder_lower.contains("tres endommage") {
            checked_state = Some("badly damaged".to_string());
            cleaned_tokens.retain(|&t| t != "badly" && t != "damaged" && t != "très" && t != "tres" && t != "endommagé" && t != "endommage");
        } else if remainder_lower.contains("bon état") || remainder_lower.contains("bon etat") {
            checked_state = Some("good".to_string());
            cleaned_tokens.retain(|&t| t != "bon" && t != "état" && t != "etat");
        }

        if checked_state.is_none() {
            for &tok in &tokens {
                match tok {
                    "damaged" | "endommagé" | "endommage" => {
                        checked_state = Some("damaged".to_string());
                    }
                    "worn" | "usé" | "use" => {
                        checked_state = Some("worn".to_string());
                    }
                    "good" | "bon" => {
                        checked_state = Some("good".to_string());
                    }
                    "pristine" | "parfait" => {
                        checked_state = Some("pristine".to_string());
                    }
                    _ => {}
                }
            }
            if checked_state.is_some() {
                cleaned_tokens.retain(|&t| t != "damaged" && t != "endommagé" && t != "endommage" && t != "worn" && t != "usé" && t != "use" && t != "good" && t != "bon" && t != "pristine" && t != "parfait");
            }
        }

        let cleaned_str = cleaned_tokens.join(" ");
        if cleaned_str.contains("peu commun") || cleaned_str.contains("peu-commun") {
            parsed_rarity = Some(Rarity::Uncommon);
        } else if cleaned_str.contains("very rare") || cleaned_str.contains("very-rare") || cleaned_str.contains("très rare") || cleaned_str.contains("tres rare") || cleaned_str.contains("très-rare") || cleaned_str.contains("tres-rare") {
            parsed_rarity = Some(Rarity::VeryRare);
        } else {
            for tok in cleaned_tokens {
                if let Some(r) = get_rarity(tok) {
                    parsed_rarity = Some(r);
                    break;
                }
            }
        }

        return Some(SellArg::All { rarity: parsed_rarity, state: checked_state });
    }

    if args_str.starts_with('#') {
        if let Ok(id) = args_str[1..].parse::<i64>() {
            return Some(SellArg::ById(id));
        } else {
            return None;
        }
    }

    let mut tokens: Vec<&str> = args_str.split_whitespace().collect();
    if tokens.is_empty() {
        return None;
    }

    let (state, quantity) = extract_state_and_quantity(&mut tokens);

    if tokens.is_empty() {
        return None;
    }

    let name = tokens.join(" ");
    if name.to_lowercase() == "pristine banana" && (quantity == 1 || quantity == 2) {
        let final_name = format!("Pristine Banana {}", quantity);
        return Some(SellArg::ByName {
            name: final_name,
            state,
            quantity: 1,
        });
    }

    Some(SellArg::ByName {
        name,
        state,
        quantity,
    })
}

#[derive(Debug, PartialEq, Clone)]
pub enum TradeArg {
    Accept,
    Cancel,
    Direct {
        catch_id: i64,
        price: i64,
        recipient: String,
    },
    Barter {
        catch_id: i64,
        recipient: String,
    },
}

pub fn parse_trade_args(args_str: &str) -> Option<TradeArg> {
    let args_str = args_str.trim();
    if args_str.is_empty() {
        return None;
    }

    let lower = args_str.to_lowercase();
    if lower == "accept" || lower == "oui" || lower == "yes" {
        return Some(TradeArg::Accept);
    }
    if lower == "cancel" || lower == "non" || lower == "no" {
        return Some(TradeArg::Cancel);
    }

    let tokens: Vec<&str> = args_str.split_whitespace().collect();
    if tokens.len() < 2 {
        return None;
    }

    if !tokens[0].starts_with('#') {
        return None;
    }
    let catch_id = tokens[0][1..].parse::<i64>().ok()?;

    if tokens.len() == 2 {
        let recipient = tokens[1].trim_start_matches('@').to_lowercase();
        return Some(TradeArg::Barter { catch_id, recipient });
    }

    if tokens.len() == 3 {
        let price = tokens[1].parse::<i64>().ok()?;
        let recipient = tokens[2].trim_start_matches('@').to_lowercase();
        return Some(TradeArg::Direct { catch_id, price, recipient });
    }

    // Gère le cas où l'utilisateur met par erreur des espaces superflus autour de l'ID ou de l'arobase
    if tokens.len() > 3 {
        // Recherche de la cible @destinataire à la fin
        if let Some(&last_token) = tokens.last() {
            if last_token.starts_with('@') || tokens[tokens.len() - 2] == "@" {
                let recipient = last_token.trim_start_matches('@').to_lowercase();
                // Essayer de voir si l'avant dernier token est le prix
                if let Ok(price) = tokens[tokens.len() - 2].parse::<i64>() {
                    return Some(TradeArg::Direct { catch_id, price, recipient });
                }
                return Some(TradeArg::Barter { catch_id, recipient });
            }
        }
    }

    None
}

pub fn parse_give_args(args_str: &str) -> Option<GiveArg> {
    let args_str = args_str.trim();
    if args_str.is_empty() {
        return None;
    }

    let lower = args_str.to_lowercase();
    if lower == "oui" || lower == "yes" || lower == "y" {
        return Some(GiveArg::ConfirmYes);
    }
    if lower == "non" || lower == "no" || lower == "n" {
        return Some(GiveArg::ConfirmNo);
    }

    let tokens: Vec<&str> = args_str.split_whitespace().collect();
    if tokens.len() < 2 {
        return None;
    }

    // Identify recipient and item
    if tokens[0].starts_with('@') {
        let recipient = tokens[0].trim_start_matches('@').to_lowercase();
        let item = tokens[1..].join(" ");
        return parse_item_give(recipient, &item);
    }

    if tokens.last().unwrap().starts_with('@') {
        let recipient = tokens.last().unwrap().trim_start_matches('@').to_lowercase();
        let item = tokens[..tokens.len() - 1].join(" ");
        return parse_item_give(recipient, &item);
    }

    let first = tokens[0];
    if first.starts_with('#') || first.parse::<i64>().is_ok() {
        let recipient = tokens[1..].join(" ").to_lowercase();
        let item = first;
        return parse_item_give(recipient, item);
    }

    let last = *tokens.last().unwrap();
    if last.starts_with('#') || last.parse::<i64>().is_ok() {
        let recipient = tokens[..tokens.len() - 1].join(" ").to_lowercase();
        let item = last;
        return parse_item_give(recipient, item);
    }

    if tokens.len() > 2 {
        let recipient_first = tokens[0].to_lowercase();
        let item_first = tokens[1..].join(" ");
        return Some(GiveArg::ByName { name: item_first, recipient: recipient_first });
    }

    let recipient = tokens[0].to_lowercase();
    let name = tokens[1].to_string();
    Some(GiveArg::ByName { name, recipient })
}

fn parse_item_give(recipient: String, item: &str) -> Option<GiveArg> {
    let item = item.trim();
    if item.is_empty() {
        return None;
    }
    if let Ok(amount) = item.parse::<i64>() {
        return Some(GiveArg::Gold { amount, recipient });
    }
    if item.starts_with('#') {
        if let Ok(catch_id) = item[1..].parse::<i64>() {
            return Some(GiveArg::ById { catch_id, recipient });
        }
    }
    Some(GiveArg::ByName { name: item.to_string(), recipient })
}

pub fn get_base_price(name: &str) -> i64 {
    if name == "Pristine Banana 1" || name == "Pristine Banana 2" {
        return 5000;
    } else if name == "Gemme VIP" || name == "Gemme VIP (TEST)" {
        return 10000;
    } else if name.to_lowercase().contains("carte postale") {
        return 50000;
    }

    // 1. Try French catalog
    let game_data_fr = crate::config::get_game_data(false);
    for (rarity, fishes) in &game_data_fr.fish_data {
        for fish in fishes {
            if fish.name.to_lowercase() == name.to_lowercase() {
                if let Some(p) = fish.price {
                    if p > 0 {
                        return p as i64;
                    }
                }
                let mut hash = 0u32;
                for c in fish.name.bytes() {
                    hash = hash.wrapping_add(c as u32).wrapping_mul(31);
                }
                let (min_p, max_p) = match rarity {
                    crate::config::Rarity::Common => (40, 95),
                    crate::config::Rarity::Uncommon => (100, 220),
                    crate::config::Rarity::Rare => (230, 480),
                    crate::config::Rarity::VeryRare => (500, 950),
                    crate::config::Rarity::Epic => (1000, 2400),
                    crate::config::Rarity::Legendary => (2500, 4800),
                    crate::config::Rarity::Mythical => (5000, 12000),
                    crate::config::Rarity::Divin => (15000, 45000),
                };
                let range = max_p - min_p + 1;
                let offset = (hash % range as u32) as i64;
                let mut time_multiplier = 1.0;
                if fish.preferred_time.is_some() || fish.time_restriction.is_some() {
                    time_multiplier = 1.3;
                }
                return ((min_p + offset) as f64 * time_multiplier).round() as i64;
            }
        }
    }
    for (_, junks) in &game_data_fr.junk_data {
        for junk in junks {
            if junk.name.to_lowercase() == name.to_lowercase() {
                return 10;
            }
        }
    }

    // 2. Try English catalog
    let game_data_en = crate::config::get_game_data(true);
    for (rarity, fishes) in &game_data_en.fish_data {
        for fish in fishes {
            if fish.name.to_lowercase() == name.to_lowercase() {
                if let Some(p) = fish.price {
                    if p > 0 {
                        return p as i64;
                    }
                }
                let mut hash = 0u32;
                for c in fish.name.bytes() {
                    hash = hash.wrapping_add(c as u32).wrapping_mul(31);
                }
                let (min_p, max_p) = match rarity {
                    crate::config::Rarity::Common => (40, 95),
                    crate::config::Rarity::Uncommon => (100, 220),
                    crate::config::Rarity::Rare => (230, 480),
                    crate::config::Rarity::VeryRare => (500, 950),
                    crate::config::Rarity::Epic => (1000, 2400),
                    crate::config::Rarity::Legendary => (2500, 4800),
                    crate::config::Rarity::Mythical => (5000, 12000),
                    crate::config::Rarity::Divin => (15000, 45000),
                };
                let range = max_p - min_p + 1;
                let offset = (hash % range as u32) as i64;
                let mut time_multiplier = 1.0;
                if fish.preferred_time.is_some() || fish.time_restriction.is_some() {
                    time_multiplier = 1.3;
                }
                return ((min_p + offset) as f64 * time_multiplier).round() as i64;
            }
        }
    }
    for (_, junks) in &game_data_en.junk_data {
        for junk in junks {
            if junk.name.to_lowercase() == name.to_lowercase() {
                return 10;
            }
        }
    }

    50
}

pub fn get_stored_state_multiplier(state: &str) -> f64 {
    match state.to_lowercase().as_str() {
        "pristine" => 3.0,
        "good" => 1.0,
        "worn" => 0.8,
        "damaged" => 0.5,
        "badly damaged" => 0.2,
        _ => 1.0,
    }
}

pub fn get_state_weight(state: &str) -> i32 {
    match state.to_lowercase().as_str() {
        "badly damaged" => 0,
        "damaged" => 1,
        "worn" => 2,
        "good" => 3,
        "pristine" => 4,
        _ => 5,
    }
}

pub fn get_size_multiplier(name: &str, size: f64) -> f64 {
    // Try French catalog first
    let game_data_fr = crate::config::get_game_data(false);
    for (_, fishes) in &game_data_fr.fish_data {
        for fish in fishes {
            if fish.name.to_lowercase() == name.to_lowercase() {
                let mean = fish.size_mean;
                if mean > 0.0 {
                    return (size / mean).clamp(0.5, 1.8);
                }
            }
        }
    }
    // Try English catalog second
    let game_data_en = crate::config::get_game_data(true);
    for (_, fishes) in &game_data_en.fish_data {
        for fish in fishes {
            if fish.name.to_lowercase() == name.to_lowercase() {
                let mean = fish.size_mean;
                if mean > 0.0 {
                    return (size / mean).clamp(0.5, 1.8);
                }
            }
        }
    }
    1.0
}

pub async fn start_bot(state: Arc<AppState>, access_token: String) {
    let mut abort_lock = state.bot_abort_handle.write().await;
    if let Some(handle) = abort_lock.take() { handle.abort(); }

    let credentials = StaticLoginCredentials::new("bot".to_string(), Some(access_token));
    let config = ClientConfig::new_simple(credentials);
    let (mut incoming_messages, client) = TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    let mut client_lock = state.twitch_client.write().await;
    *client_lock = Some(client.clone());
    drop(client_lock);

    let state_clone = Arc::clone(&state);
    let channel_name = state.channel.clone();

    let handle = tokio::spawn(async move {
        let _ = client.join(channel_name.to_lowercase());
        let channel_pulse = channel_name.to_lowercase();
        let client_hello = client.clone();
        let channel_hello = channel_name.to_lowercase();
        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            let _ = client_hello.say(channel_hello, "✅ Le bot est en ligne ! Bon jeu et bonne pêche ! 🎣".to_string()).await;
        });
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
                tracing::info!("[Pulse] #{}", channel_pulse);
            }
        });

        while let Some(message) = incoming_messages.recv().await {
            if let ServerMessage::Privmsg(msg) = message {
                let text = msg.message_text.trim().to_lowercase();
                let username = msg.sender.name.to_lowercase();
                tracing::info!("[Chat] {} : {}", username, text);

                let state_task_daily = Arc::clone(&state_clone);
                let client_msg_daily = client.clone();
                let channel_login_daily = msg.channel_login.clone();
                let username_daily = username.clone();

                tokio::spawn(async move {
                    let today = chrono::Utc::now().date_naive();
                    
                    // 1. Check RAM cache first
                    let already_claimed = state_task_daily.daily_reward_cache.get(&username_daily)
                        .as_deref() == Some(&today);

                    if !already_claimed {
                        // 2. Query/Create player in DB
                        if let Ok(player) = state_task_daily.repo.get_or_create_player(&username_daily).await {
                            let mut claim_needed = true;
                            
                            if let Some(last_time) = player.last_daily_reward_at {
                                if last_time.date_naive() == today {
                                    claim_needed = false;
                                }
                            }

                            if claim_needed {
                                let mut consecutive = player.consecutive_days;
                                let mut total = player.total_days;

                                if let Some(last_time) = player.last_daily_reward_at {
                                    let last_date = last_time.date_naive();
                                    match state_task_daily.repo.count_stream_days_between(last_date, today).await {
                                        Ok(count) => {
                                            if count > 0 {
                                                // Stream was online at least once since last reward, and viewer missed it -> Streak broken!
                                                consecutive = 1;
                                            } else {
                                                // No stream days were missed -> Streak continues!
                                                consecutive += 1;
                                            }
                                            total += 1;
                                        }
                                        Err(_) => {
                                            // Fallback
                                            if last_date == today - chrono::Duration::days(1) {
                                                consecutive += 1;
                                            } else {
                                                consecutive = 1;
                                            }
                                            total += 1;
                                        }
                                    }
                                } else {
                                    // First login reward
                                    consecutive = 1;
                                    total = 1;
                                }

                                // Capped multiplier at 10 days
                                let consecutive_capped = consecutive.min(10);
                                let mut reward_gold = 200 + 50 * consecutive_capped as i64 + 10 * total as i64;

                                let is_king = state_task_daily.repo.is_banana_king(player.id.unwrap()).await.unwrap_or(false);
                                if is_king {
                                    reward_gold += 5000;
                                }

                                if let Ok(_) = state_task_daily.repo.claim_daily_reward(player.id.unwrap(), consecutive, total, reward_gold).await {
                                    let use_english = match &player.language {
                                        Some(l) => l == "en",
                                        None => state_task_daily.use_english,
                                    };

                                    let msg = if use_english {
                                        if is_king {
                                            format!(
                                                "🎁 @{} just logged in! They receive their daily bonus of {} gold 🪙 (including 5000 gold Banana King salary 👑) (Streak: {} days 🔥, Cumulative: {} days total)!",
                                                username_daily, reward_gold, consecutive, total
                                            )
                                        } else {
                                            format!(
                                                "🎁 @{} just logged in! They receive their daily bonus of {} gold 🪙 (Streak: {} days 🔥, Cumulative: {} days total)!",
                                                username_daily, reward_gold, consecutive, total
                                            )
                                        }
                                    } else {
                                        if is_king {
                                            format!(
                                                "🎁 @{} vient de se connecter ! Il reçoit son bonus quotidien de {} po 🪙 (dont 5000 po de salaire de Roi Banane 👑) (Série : {} jours d'affilée 🔥, Cumul : {} jours total) !",
                                                username_daily, reward_gold, consecutive, total
                                            )
                                        } else {
                                            format!(
                                                "🎁 @{} vient de se connecter ! Il reçoit son bonus quotidien de {} po 🪙 (Série : {} jours d'affilée 🔥, Cumul : {} jours total) !",
                                                username_daily, reward_gold, consecutive, total
                                            )
                                        }
                                    };

                                    let _ = client_msg_daily.say(channel_login_daily, msg).await;
                                }
                            }

                            // 3. Populate RAM cache
                            state_task_daily.daily_reward_cache.insert(username_daily, today);
                        }
                    }
                });
                
                if text == "!fish help" || text == "!pêche help" || text == "!peche help" {
                    let mut help_msg = "📖 !fish | stats | top | list <nom> | info <nom> | sell <nom/ID> | recycle #id poubelle | trade #id | give <objet/montant> @destinataire | coinflip <montant> | lang [fr/en/reset] | Tape !fish help sell, recycle, trade, give, coinflip ou lang".to_string();
                    if username == "monsieurcotcot" {
                        help_msg.push_str(" | 🛠️ Admin: !admin backup | !admin restore | !fish reset <pseudo> | !fish simulate <pseudo> <n> | !fish purge");
                    }
                    let _ = client.say(msg.channel_login.clone(), help_msg).await;
                } else if text.starts_with("!fish help ") || text.starts_with("!peche help ") || text.starts_with("!pêche help ") {
                    let sub = text.split_whitespace().skip(2).collect::<Vec<&str>>().join(" ");
                    let reply = match sub.as_str() {
                        "sell" | "vendre" | "vends" | "vend" => {
                            "💰 Vente: !fish sell <nom/id> [état] [qté], ou '!fish sell all [rareté] [état]'. Ex: '!fish sell #42', '!fish sell all', '!fish sell all rare', '!fish sell all pristine', '!fish sell all common worn'. Confirme par '!fish sell oui' (durée 1m).".to_string()
                        },
                        "recycle" | "recycler" => {
                            "♻️ Recyclage : '!fish recycle #id_capture poubelle' (Ex: !fish recycle #42 jaune). Poubelles : bleu (papier/carton), jaune (plastiques/métaux), vert (verre), marron (organique), gris (e-waste/ampoules), noir(e) (tout-venant), decharge (autres).".to_string()
                        },
                        "trade" | "echange" | "échanger" | "echanger" => {
                            "🤝 Échanges : 1) Vente Directe : '!fish trade #id_catch prix @destinataire' (Ex: !fish trade #15 250 @pseudo). 2) Troc : Initié par '!fish trade #id_A @destinataire', puis le destinataire propose un contre-troc '!fish trade #id_B @pseudo', suivi des validations.".to_string()
                        },
                        "give" | "gift" | "don" | "donner" => {
                            "🎁 Dons : '!fish give [montant] @destinataire' (pour envoyer de l'or) ou '!fish give [nom/#id] @destinataire' (pour offrir un objet). Valide les dons d'objets avec '!fish give oui' sous 60s.".to_string()
                        },
                        "coinflip" | "cf" => {
                            "🪙 Coinflip : '!fish coinflip <montant>' ou '!fish coinflip all'. Tentez de doubler vos pièces d'or sur un coup de pile ou face ! Mise minimale : 10 po 🪙.".to_string()
                        },
                        "lang" | "language" | "langue" => {
                            "🌐 Langue : '!fish lang fr' pour passer en Français, '!fish lang en' pour l'Anglais, ou '!fish lang reset' pour la langue automatique par défaut (anglais sur !fish, français sur !peche).".to_string()
                        },
                        _ => format!("📖 Commande inconnue. Utilise '!fish help' ou '!fish help sell' ou '!fish help recycle' ou '!fish help trade' ou '!fish help give' ou '!fish help coinflip' pour plus de détails.")
                    };
                    let _ = client.say(msg.channel_login.clone(), reply).await;
                } else if text == "!fish lang fr" || text == "!peche lang fr" || text == "!pêche lang fr" {
                    let state_task = Arc::clone(&state_clone);
                    let client_msg = client.clone();
                    let channel_login = msg.channel_login.clone();
                    tokio::spawn(async move {
                        if let Ok(player) = state_task.repo.get_or_create_player(&username).await {
                            if let Ok(_) = state_task.repo.update_player_language(player.id.unwrap(), Some("fr".to_string())).await {
                                let _ = client_msg.say(channel_login, format!("🌐 @{}, ton jeu est désormais configuré en Français ! 🇫🇷", username)).await;
                            }
                        }
                    });
                } else if text == "!fish lang en" || text == "!peche lang en" || text == "!pêche lang en" {
                    let state_task = Arc::clone(&state_clone);
                    let client_msg = client.clone();
                    let channel_login = msg.channel_login.clone();
                    tokio::spawn(async move {
                        if let Ok(player) = state_task.repo.get_or_create_player(&username).await {
                            if let Ok(_) = state_task.repo.update_player_language(player.id.unwrap(), Some("en".to_string())).await {
                                let _ = client_msg.say(channel_login, format!("🌐 @{}, your game is now configured in English! 🇬🇧", username)).await;
                            }
                        }
                    });
                } else if text == "!fish lang reset" || text == "!peche lang reset" || text == "!pêche lang reset" || text == "!fish lang default" || text == "!peche lang default" || text == "!pêche lang default" {
                    let state_task = Arc::clone(&state_clone);
                    let client_msg = client.clone();
                    let channel_login = msg.channel_login.clone();
                    tokio::spawn(async move {
                        if let Ok(player) = state_task.repo.get_or_create_player(&username).await {
                            if let Ok(_) = state_task.repo.update_player_language(player.id.unwrap(), None).await {
                                let _ = client_msg.say(channel_login, format!("🌐 @{}, préférence de langue réinitialisée (défaut automatique) ! ⚙️", username)).await;
                            }
                        }
                    });
                } else if text == "buble" || text == "!fish buble" || text == "!peche buble" || text == "!pêche buble" {
                    let state_task = Arc::clone(&state_clone);
                    let client_msg = client.clone();
                    let channel_login = msg.channel_login.clone();
                    tokio::spawn(async move {
                        if let Ok(player) = state_task.repo.get_or_create_player(&username).await {
                            let use_english = match &player.language {
                                Some(l) => l == "en",
                                None => text.starts_with("!fish") || text == "buble",
                            };

                            // Reset attempts
                            {
                                let mut att = state_task.offline_attempts.write().await;
                                att.remove(&username);
                            }
                            // Grant access
                            {
                                let mut bypassed = state_task.offline_bypassed.write().await;
                                bypassed.insert(username.clone());
                            }

                            let msg_str = if use_english {
                                format!("🔑 @{}, access granted! You can now fish offline! 🌊", username)
                            } else {
                                format!("🔑 @{}, accès autorisé ! Tu peux désormais pêcher hors-ligne ! 🌊", username)
                            };
                            let _ = client_msg.say(channel_login, msg_str).await;
                        }
                    });
                } else if text == "!fish stats" || text == "!fish stat" || text == "!peche stats" || text == "!pêche stats" {
                    let state_task = Arc::clone(&state_clone);
                    let client_msg = client.clone();
                    let channel_login = msg.channel_login.clone();
                    let base_url = std::env::var("REDIRECT_URI").unwrap_or_default().replace("/auth/callback", "");
                    tokio::spawn(async move {
                        if let Ok(p) = state_task.repo.get_or_create_player(&username).await {
                            let fish_count = p.successful_attempts - p.junk_count - p.banana_count - p.postcard_count - p.gem_count;
                            
                            // Calcul du taux de complétion du Musée (Poissons et Déchets)
                            let mut fish_percent = 0;
                            let mut junk_percent = 0;
                            if let Ok(museum) = state_task.repo.get_player_museum(p.id.unwrap()).await {
                                let fish_names = crate::config::get_fish_names_lower(state_task.use_english);
                                let junk_names = crate::config::get_junk_names_lower(state_task.use_english);
                                
                                let total_fish = fish_names.len();
                                let total_junk = junk_names.len();
                                
                                let mut discovered_fish = 0;
                                let mut discovered_junk = 0;
                                for item in museum {
                                    let name_lower = item.fish_name.to_lowercase();
                                    if fish_names.contains(&name_lower) {
                                        discovered_fish += 1;
                                    } else if junk_names.contains(&name_lower) {
                                        discovered_junk += 1;
                                    }
                                }
                                
                                if total_fish > 0 {
                                    fish_percent = (discovered_fish as f64 / total_fish as f64 * 100.0).round() as i32;
                                }
                                if total_junk > 0 {
                                    junk_percent = (discovered_junk as f64 / total_junk as f64 * 100.0).round() as i32;
                                }
                            }

                            let net_gold = p.coinflip_gold_won_total - p.coinflip_gold_lost_total;
                            let sign = if net_gold >= 0 { "+" } else { "" };
                            let gambling_str = if (p.coinflip_wins + p.coinflip_losses) > 0 {
                                format!(" | 🎰 Coinflip : {}V/{}D (Bilan: {}{} po, Record: +{} po)", p.coinflip_wins, p.coinflip_losses, sign, net_gold, p.coinflip_biggest_win)
                            } else {
                                "".to_string()
                            };

                            let msg_str = format!(
                                "📊 @{} : Niv. {} (XP: {}/{}) | {} 🪙 | 🏛️ Musée: 🐟 {}% • 🗑️ {}% | {} 🐟 | {} 🗑️ | {} 🍌 | {} 💎 | {} 📜{} | Détails : {}/player/{}", 
                                username, p.level, p.xp, p.xp_for_next_level(), p.gold, fish_percent, junk_percent, fish_count, p.junk_count, p.banana_count, p.gem_count, p.postcard_count, gambling_str, base_url, username
                            );
                            let _ = client_msg.say(channel_login, msg_str).await;
                        }
                    });
                } else if text == "!fish top" || text == "!peche top" || text == "!pêche top" {
                    let state_task = Arc::clone(&state_clone);
                    let client_msg = client.clone();
                    let channel_login = msg.channel_login.clone();
                    tokio::spawn(async move {
                        if let Ok(players) = state_task.repo.get_leaderboard().await {
                            let list: Vec<String> = players.iter().take(5).enumerate().map(|(i, p)| {
                                let fish_count = p.successful_attempts - p.junk_count - p.banana_count - p.postcard_count - p.gem_count;
                                format!("#{}. {} (Niv. {} | {} 🪙 | {} 🐟 | {} 🗑️ | {} 🍌 | {} 💎 | {} 📜)", i + 1, p.username, p.level, p.gold, fish_count, p.junk_count, p.banana_count, p.gem_count, p.postcard_count)
                            }).collect();
                            let _ = client_msg.say(channel_login, format!("🏆 Top Pêcheurs : {}", list.join(" | "))).await;
                        }
                    });
                } else if text == "!fish top cf" || text == "!peche top cf" || text == "!pêche top cf" ||
                          text == "!fish top hasard" || text == "!peche top hasard" || text == "!pêche top hasard" {
                    let state_task = Arc::clone(&state_clone);
                    let client_msg = client.clone();
                    let channel_login = msg.channel_login.clone();
                    tokio::spawn(async move {
                        if let Ok(players) = state_task.repo.get_gambling_leaderboard().await {
                            let list: Vec<String> = players.iter().enumerate().map(|(i, p)| {
                                let net_gold = p.coinflip_gold_won_total - p.coinflip_gold_lost_total;
                                format!("#{}. {} ({:+} po 🎰 | {}V/{}D)", i + 1, p.username, net_gold, p.coinflip_wins, p.coinflip_losses)
                            }).collect();
                            
                            if list.is_empty() {
                                let _ = client_msg.say(channel_login, "🎰 Aucun joueur n'a encore pris de risque au Coinflip !".to_string()).await;
                            } else {
                                let _ = client_msg.say(channel_login, format!("🏆 Hall of Fame - Top Parieurs (Gains Nets) : {}", list.join(" | "))).await;
                            }
                        }
                    });
                } else if text.starts_with("!fish list") || text.starts_with("!peche list") || text.starts_with("!pêche list") || text.starts_with("!fish liste") || text.starts_with("!peche liste") || text.starts_with("!pêche liste") {
                    let state_task = Arc::clone(&state_clone);
                    let client_msg = client.clone();
                    let channel_login = msg.channel_login.clone();
                    let raw_msg = msg.message_text.clone();
                    
                    tokio::spawn(async move {
                        let text_trim = raw_msg.trim().to_lowercase();
                        let arg = if text_trim.starts_with("!fish list") {
                            raw_msg["!fish list".len()..].trim()
                        } else if text_trim.starts_with("!fish liste") {
                            raw_msg["!fish liste".len()..].trim()
                        } else if text_trim.starts_with("!peche list") {
                            raw_msg["!peche list".len()..].trim()
                        } else if text_trim.starts_with("!peche liste") {
                            raw_msg["!peche liste".len()..].trim()
                        } else if text_trim.starts_with("!pêche list") {
                            raw_msg["!pêche list".len()..].trim()
                        } else if text_trim.starts_with("!pêche liste") {
                            raw_msg["!pêche liste".len()..].trim()
                        } else {
                            ""
                        };

                        if arg.is_empty() {
                            let _ = client_msg.say(channel_login, format!("⚠️ @{}, spécifie le nom d'un poisson ou objet pour lister tes exemplaires, ex : !fish list Ayu", username)).await;
                            return;
                        }

                        if let Ok(player) = state_task.repo.get_or_create_player(&username).await {
                            if let Ok(catches) = state_task.repo.get_player_catches(player.id.unwrap()).await {
                                let matching_catches: Vec<_> = catches.into_iter()
                                    .filter(|c| c.name.to_lowercase() == arg.to_lowercase())
                                    .collect();

                                if matching_catches.is_empty() {
                                    let _ = client_msg.say(channel_login, format!("⚠️ @{}, tu ne possèdes aucun '{}' dans ton inventaire.", username, arg)).await;
                                    return;
                                }

                                // Group by state
                                let mut pristine_cnt = 0;
                                let mut good_cnt = 0;
                                let mut worn_cnt = 0;
                                let mut damaged_cnt = 0;
                                let mut badly_damaged_cnt = 0;

                                for c in &matching_catches {
                                    match c.state.to_lowercase().as_str() {
                                        "pristine" => pristine_cnt += 1,
                                        "good" => good_cnt += 1,
                                        "worn" => worn_cnt += 1,
                                        "damaged" => damaged_cnt += 1,
                                        "badly damaged" => badly_damaged_cnt += 1,
                                        _ => good_cnt += 1,
                                    }
                                }

                                let base_price = get_base_price(&matching_catches[0].name);

                                let mut parts = Vec::new();
                                if pristine_cnt > 0 { parts.push(format!("Pristine x{} ({} po)", pristine_cnt, (base_price as f64 * 3.0).round() as i64)); }
                                if good_cnt > 0 { parts.push(format!("Good x{} ({} po)", good_cnt, (base_price as f64 * 1.0).round() as i64)); }
                                if worn_cnt > 0 { parts.push(format!("Worn x{} ({} po)", worn_cnt, (base_price as f64 * 0.8).round() as i64)); }
                                if damaged_cnt > 0 { parts.push(format!("Damaged x{} ({} po)", damaged_cnt, (base_price as f64 * 0.5).round() as i64)); }
                                if badly_damaged_cnt > 0 { parts.push(format!("Badly Damaged x{} ({} po)", badly_damaged_cnt, (base_price as f64 * 0.2).round() as i64)); }

                                let _ = client_msg.say(channel_login, format!("📋 @{}, exemplaires de '{}' : {} | Total : {} exemplaire(s)", username, matching_catches[0].name, parts.join(" | "), matching_catches.len())).await;
                            }
                        }
                    });
                } else if text.starts_with("!fish info") || text.starts_with("!peche info") || text.starts_with("!pêche info") || text.starts_with("!fish infos") || text.starts_with("!peche infos") || text.starts_with("!pêche infos") {
                    let state_task = Arc::clone(&state_clone);
                    let client_msg = client.clone();
                    let channel_login = msg.channel_login.clone();
                    let raw_msg = msg.message_text.clone();
                    
                    tokio::spawn(async move {
                        let text_trim = raw_msg.trim().to_lowercase();
                        let arg = if text_trim.starts_with("!fish info") {
                            raw_msg["!fish info".len()..].trim()
                        } else if text_trim.starts_with("!fish infos") {
                            raw_msg["!fish infos".len()..].trim()
                        } else if text_trim.starts_with("!peche info") {
                            raw_msg["!peche info".len()..].trim()
                        } else if text_trim.starts_with("!peche infos") {
                            raw_msg["!peche infos".len()..].trim()
                        } else if text_trim.starts_with("!pêche info") {
                            raw_msg["!pêche info".len()..].trim()
                        } else if text_trim.starts_with("!pêche infos") {
                            raw_msg["!pêche infos".len()..].trim()
                        } else {
                            ""
                        };

                        if arg.is_empty() {
                            let _ = client_msg.say(channel_login, format!("⚠️ @{}, spécifie le nom exact du poisson, son identifiant de capture unique ou son index de musée, ex : !fish info Ayu, !fish info #12 ou !fish info 1", username)).await;
                            return;
                        }

                        // Check if searching by Museum ID
                        let mut museum_id = None;
                        let arg_lower = arg.to_lowercase();
                        if let Ok(id) = arg_lower.parse::<i32>() {
                            museum_id = Some(id);
                        } else if arg_lower.starts_with('m') {
                            if let Ok(id) = arg_lower[1..].trim().parse::<i32>() {
                                museum_id = Some(id);
                            }
                        } else if arg_lower.starts_with("museum") {
                            if let Ok(id) = arg_lower["museum".len()..].trim().parse::<i32>() {
                                museum_id = Some(id);
                            }
                        } else if arg_lower.starts_with("musée") {
                            if let Ok(id) = arg_lower["musée".len()..].trim().parse::<i32>() {
                                museum_id = Some(id);
                            }
                        } else if arg_lower.starts_with("musee") {
                            if let Ok(id) = arg_lower["musee".len()..].trim().parse::<i32>() {
                                museum_id = Some(id);
                            }
                        }

                        if let Some(m_id) = museum_id {
                            let game_data = crate::config::get_game_data(state_task.use_english);
                            let mut found_fish = None;

                            for (_, fishes) in &game_data.fish_data {
                                for fish in fishes {
                                    if fish.id == Some(m_id) {
                                        found_fish = Some(fish.clone());
                                        break;
                                    }
                                }
                            }

                            if found_fish.is_none() {
                                for (_, junks) in &game_data.junk_data {
                                    for junk in junks {
                                        if junk.id == Some(m_id) {
                                            found_fish = Some(junk.clone());
                                            break;
                                        }
                                    }
                                }
                            }

                            if let Some(f) = found_fish {
                                let loc = f.location.unwrap_or_else(|| "Inconnu".to_string());
                                let hours = match f.time_restriction.as_deref() {
                                    Some("before_22h") => "Avant 22h (Jour/Soirée)",
                                    Some("after_22h") => "Après 22h (Nuit/Prolongation)",
                                    _ => "Toutes heures",
                                };
                                let period = f.preferred_season.unwrap_or_else(|| "Toute l'année".to_string());
                                let base_price = get_base_price(&f.name);

                                // Count owned by current user
                                let count = if let Ok(player) = state_task.repo.get_or_create_player(&username).await {
                                    state_task.repo.count_fish_owned_by_player(player.id.unwrap_or(0), &f.name).await.unwrap_or(0)
                                } else {
                                    0
                                };

                                let count_msg = if count > 0 {
                                    format!("🎣 Tu possèdes {} exemplaire(s) dans ton inventaire.", count)
                                } else {
                                    "🎣 Tu n'en possèdes pas encore.".to_string()
                                };

                                let _ = client_msg.say(
                                    channel_login,
                                    format!(
                                        "🔍 [Musée #{}] {} | Lieu: {} | Horaires: {} | Période: {} | Prix de base: {} po 🪙 | {}",
                                        m_id, f.name, loc, hours, period, base_price, count_msg
                                    )
                                ).await;
                            } else {
                                let _ = client_msg.say(channel_login, format!("❌ @{}, aucun poisson ou objet avec l'index de musée #{} dans le catalogue.", username, m_id)).await;
                            }
                            return;
                        }

                        // Check if searching by unique capture ID
                        if arg.starts_with('#') {
                            if let Ok(catch_id) = arg[1..].parse::<i64>() {
                                match state_task.repo.get_catch_by_id(catch_id).await {
                                    Ok(Some((c, owner_name))) => {
                                        // Count owned by current user
                                        let count = if let Ok(player) = state_task.repo.get_or_create_player(&username).await {
                                            state_task.repo.count_fish_owned_by_player(player.id.unwrap_or(0), &c.name).await.unwrap_or(0)
                                        } else {
                                            0
                                        };

                                        // Try to find the species in catalog to get location/time/season
                                        let game_data = crate::config::get_game_data(state_task.use_english);
                                        let mut found_fish = None;
                                        for (_, fishes) in &game_data.fish_data {
                                            for fish in fishes {
                                                if fish.name.to_lowercase() == c.name.to_lowercase() {
                                                    found_fish = Some(fish.clone());
                                                    break;
                                                }
                                            }
                                        }
                                        if found_fish.is_none() {
                                            for (_, junks) in &game_data.junk_data {
                                                for junk in junks {
                                                    if junk.name.to_lowercase() == c.name.to_lowercase() {
                                                        found_fish = Some(junk.clone());
                                                        break;
                                                    }
                                                }
                                            }
                                        }

                                        let rarity_label = match c.rarity {
                                            crate::config::Rarity::Common => "Commun ⚪",
                                            crate::config::Rarity::Uncommon => "Inhabituel 🟢",
                                            crate::config::Rarity::Rare => "Rare 🔵",
                                            crate::config::Rarity::VeryRare => "Très Rare 🟡",
                                            crate::config::Rarity::Epic => "Épique 🟣",
                                            crate::config::Rarity::Legendary => "Légendaire 🟠",
                                            crate::config::Rarity::Mythical => "Mythique 🔴",
                                            crate::config::Rarity::Divin => "Divin 👑",
                                        };

                                        let base_price = get_base_price(&c.name);
                                        let mult = if c.is_junk { 1.0 } else { get_stored_state_multiplier(&c.state) };
                                        let estimated_value = if c.is_junk { 10 } else { ((base_price as f64 * mult).round() as i64).max(1) };

                                        let count_msg = if count > 0 {
                                            format!("🎣 Tu possèdes {} exemplaire(s) dans ton inventaire.", count)
                                        } else {
                                            "🎣 Tu n'en possèdes pas encore.".to_string()
                                        };

                                        let catcher_name = c.caught_by.as_deref().unwrap_or("Inconnu");
                                        let owner_msg = if owner_name.to_lowercase() == catcher_name.to_lowercase() {
                                            format!("Propriétaire: @{}", owner_name)
                                        } else {
                                            format!("Propriétaire actuel: @{} (Capturé par: @{})", owner_name, catcher_name)
                                        };

                                        let museum_id_label = if let Some(f) = &found_fish {
                                            if let Some(m_id) = f.id {
                                                format!(" (Musée #{})", m_id)
                                            } else {
                                                "".to_string()
                                            }
                                        } else {
                                            "".to_string()
                                        };

                                        if let Some(f) = found_fish {
                                            let loc = f.location.unwrap_or_else(|| "Inconnu".to_string());
                                            let hours = match f.time_restriction.as_deref() {
                                                Some("before_22h") => "Avant 22h (Jour/Soirée)",
                                                Some("after_22h") => "Après 22h (Nuit/Prolongation)",
                                                _ => "Toutes heures",
                                            };
                                            let period = f.preferred_season.unwrap_or_else(|| "Toute l'année".to_string());

                                            let _ = client_msg.say(
                                                channel_login,
                                                format!(
                                                    "🔍 [Capture #{}] {}{} ({}, {}cm, {}kg, État: {}) | Valeur: {} po 🪙 (Base: {}) | Lieu: {} | Période: {} | Horaires: {} | {} | {}",
                                                    catch_id, c.name, museum_id_label, rarity_label, c.size, c.weight, c.state, estimated_value, base_price, loc, period, hours, owner_msg, count_msg
                                                )
                                            ).await;
                                        } else {
                                            let _ = client_msg.say(
                                                channel_login,
                                                format!(
                                                    "🔍 [Capture #{}] {} (État: {}) | Valeur: {} po 🪙 | {} | {}",
                                                    catch_id, c.name, c.state, estimated_value, owner_msg, count_msg
                                                )
                                            ).await;
                                        }
                                    }
                                    Ok(None) => {
                                        let _ = client_msg.say(channel_login, format!("❌ @{}, la capture #{} est introuvable.", username, catch_id)).await;
                                    }
                                    Err(e) => {
                                        tracing::error!("Failed to fetch catch: {}", e);
                                        let _ = client_msg.say(channel_login, format!("❌ @{}, erreur lors de la recherche de la capture.", username)).await;
                                    }
                                }
                                return;
                            }
                        }

                        // Search in config game_data (by name)
                        let game_data = crate::config::get_game_data(state_task.use_english);
                        let mut found_fish = None;

                        for (_, fishes) in &game_data.fish_data {
                            for fish in fishes {
                                if fish.name.to_lowercase() == arg.to_lowercase() {
                                    found_fish = Some(fish.clone());
                                    break;
                                }
                            }
                        }

                        if found_fish.is_none() {
                            for (_, junks) in &game_data.junk_data {
                                for junk in junks {
                                    if junk.name.to_lowercase() == arg.to_lowercase() {
                                        found_fish = Some(junk.clone());
                                        break;
                                    }
                                }
                            }
                        }

                        if let Some(f) = found_fish {
                            let loc = f.location.unwrap_or_else(|| "Inconnu".to_string());
                            let hours = match f.time_restriction.as_deref() {
                                Some("before_22h") => "Avant 22h (Jour/Soirée)",
                                Some("after_22h") => "Après 22h (Nuit/Prolongation)",
                                _ => "Toutes heures",
                            };
                            let period = f.preferred_season.unwrap_or_else(|| "Toute l'année".to_string());
                            let base_price = get_base_price(&f.name);

                            // Count owned by current user
                            let count = if let Ok(player) = state_task.repo.get_or_create_player(&username).await {
                                state_task.repo.count_fish_owned_by_player(player.id.unwrap_or(0), &f.name).await.unwrap_or(0)
                            } else {
                                0
                            };

                            let count_msg = if count > 0 {
                                format!("🎣 Tu possèdes {} exemplaire(s) dans ton inventaire.", count)
                            } else {
                                "🎣 Tu n'en possèdes pas encore.".to_string()
                            };

                            let _ = client_msg.say(
                                channel_login,
                                format!(
                                    "🔍 [{}] Lieu : {} | Horaires : {} | Période : {} | Prix de base : {} po 🪙 | {}",
                                    f.name, loc, hours, period, base_price, count_msg
                                )
                            ).await;
                        } else {
                            let _ = client_msg.say(channel_login, format!("❌ @{}, aucun objet ou poisson sous le nom '{}' dans le catalogue.", username, arg)).await;
                        }
                    });
                } else if text.starts_with("!fish coinflip") || text.starts_with("!fish cf") ||
                          text.starts_with("!peche coinflip") || text.starts_with("!peche cf") ||
                          text.starts_with("!pêche coinflip") || text.starts_with("!pêche cf") {
                    let state_task = Arc::clone(&state_clone);
                    let client_msg = client.clone();
                    let channel_login = msg.channel_login.clone();
                    let raw_msg = msg.message_text.clone();

                    tokio::spawn(async move {
                        let text_trim = raw_msg.trim().to_lowercase();
                        let arg = if text_trim.starts_with("!fish coinflip") {
                            raw_msg["!fish coinflip".len()..].trim()
                        } else if text_trim.starts_with("!fish cf") {
                            raw_msg["!fish cf".len()..].trim()
                        } else if text_trim.starts_with("!peche coinflip") {
                            raw_msg["!peche coinflip".len()..].trim()
                        } else if text_trim.starts_with("!peche cf") {
                            raw_msg["!peche cf".len()..].trim()
                        } else if text_trim.starts_with("!pêche coinflip") {
                            raw_msg["!pêche coinflip".len()..].trim()
                        } else if text_trim.starts_with("!pêche cf") {
                            raw_msg["!pêche cf".len()..].trim()
                        } else {
                            ""
                        }.to_string();

                        if arg.is_empty() {
                            let _ = client_msg.say(
                                channel_login,
                                format!("⚠️ @{}, usage : !fish coinflip [montant] ou !fish coinflip all (mise min: 10 po 🪙).", username)
                            ).await;
                            return;
                        }

                        // Récupérer le joueur
                        let player = match state_task.repo.get_or_create_player(&username).await {
                            Ok(p) => p,
                            Err(e) => {
                                tracing::error!("Failed to get/create player for coinflip: {:?}", e);
                                return;
                            }
                        };

                        let player_gold = player.gold;

                        // Analyser le montant misé
                        let wager_amount = if arg.eq_ignore_ascii_case("all") || arg.eq_ignore_ascii_case("tout") {
                            player_gold
                        } else {
                            match arg.parse::<i64>() {
                                Ok(val) if val > 0 => val,
                                _ => {
                                    let _ = client_msg.say(
                                        channel_login,
                                        format!("⚠️ @{}, montant invalide ! Utilise un nombre entier supérieur ou égal à 10, ou \"all\".", username)
                                    ).await;
                                    return;
                                }
                            }
                        };

                        // Validation de la mise minimale
                        if wager_amount < 10 {
                            let _ = client_msg.say(
                                channel_login,
                                format!("⚠️ @{}, la mise minimale est de 10 pièces d'or 🪙.", username)
                            ).await;
                            return;
                        }

                        // Validation du solde
                        if player_gold < wager_amount {
                            let _ = client_msg.say(
                                channel_login,
                                format!("⚠️ @{}, tu n'as pas assez de pièces d'or ! Tu possèdes actuellement {} po 🪙.", username, player_gold)
                            ).await;
                            return;
                        }

                        // Tirage aléatoire :
                        // 47% de chances de gagner normalement (win)
                        // 47% de chances de perdre normalement (lose)
                        // 5% de chances de perte insolite (disappear)
                        // 1% de chances de tomber sur la tranche (edge, gain x3)
                        use rand::Rng;
                        let roll = rand::thread_rng().gen_range(0.0..100.0);

                        enum CoinflipOutcome {
                            Win,
                            Lose,
                            Disappear,
                            Edge,
                        }

                        let outcome = if roll <= 47.0 {
                            CoinflipOutcome::Win
                        } else if roll <= 94.0 {
                            CoinflipOutcome::Lose
                        } else if roll <= 99.0 {
                            CoinflipOutcome::Disappear
                        } else {
                            CoinflipOutcome::Edge
                        };

                        let use_english = match player.language.as_deref() {
                            Some("en") => true,
                            Some("fr") => false,
                            _ => raw_msg.trim().to_lowercase().starts_with("!fish"),
                        };

                        match outcome {
                            CoinflipOutcome::Win => {
                                match state_task.repo.record_coinflip_result(player.id.unwrap(), wager_amount, true).await {
                                    Ok(updated_player) => {
                                        let new_gold = updated_player.gold;
                                        let mut msg_text = if use_english {
                                            format!(
                                                "🪙 @{} flips a coin... WON! 🔴 (+{} gold)! You now have {} gold 🪙 !",
                                                username, wager_amount, new_gold
                                            )
                                        } else {
                                            format!(
                                                "🪙 @{} lance une pièce... GAGNÉ ! 🔴 (+{} po) ! Tu as maintenant {} pièces d'or 🪙 !",
                                                username, wager_amount, new_gold
                                            )
                                        };
                                        if updated_player.coinflip_current_win_streak >= 3 {
                                            if use_english {
                                                msg_text.push_str(&format!(" 🔥 STREAK OF {} CONSECUTIVE WINS! 🟥", updated_player.coinflip_current_win_streak));
                                            } else {
                                                msg_text.push_str(&format!(" 🔥 SÉRIE DE {} VICTOIRES D'AFFILÉE ! 🟥", updated_player.coinflip_current_win_streak));
                                            }
                                        }
                                        let _ = client_msg.say(channel_login, msg_text).await;
                                    }
                                    Err(e) => {
                                        tracing::error!("Failed to update player gold in coinflip win: {:?}", e);
                                        let _ = client_msg.say(channel_login, if use_english {
                                            format!("⚠️ @{}, a technical error occurred while updating your gold.", username)
                                        } else {
                                            format!("⚠️ @{}, une erreur technique est survenue lors de la mise à jour de tes pièces d'or.", username)
                                        }).await;
                                    }
                                }
                            }
                            CoinflipOutcome::Lose => {
                                match state_task.repo.record_coinflip_result(player.id.unwrap(), wager_amount, false).await {
                                    Ok(updated_player) => {
                                        let new_gold = updated_player.gold;
                                        let mut msg_text = if use_english {
                                            format!(
                                                "🪙 @{} flips a coin... LOST! ⚪ (-{} gold)! You now have {} gold 🪙 !",
                                                username, wager_amount, new_gold
                                            )
                                        } else {
                                            format!(
                                                "🪙 @{} lance une pièce... PERDU ! ⚪ (-{} po) ! Tu as maintenant {} pièces d'or 🪙 !",
                                                username, wager_amount, new_gold
                                            )
                                        };
                                        if updated_player.coinflip_current_loss_streak >= 3 {
                                            if use_english {
                                                msg_text.push_str(&format!(" 💀 STREAK OF {} CONSECUTIVE LOSSES... ⬜", updated_player.coinflip_current_loss_streak));
                                            } else {
                                                msg_text.push_str(&format!(" 💀 SÉRIE DE {} DÉFAITES D'AFFILÉE... ⬜", updated_player.coinflip_current_loss_streak));
                                            }
                                        }
                                        let _ = client_msg.say(channel_login, msg_text).await;
                                    }
                                    Err(e) => {
                                        tracing::error!("Failed to update player gold in coinflip lose: {:?}", e);
                                        let _ = client_msg.say(channel_login, if use_english {
                                            format!("⚠️ @{}, a technical error occurred while updating your gold.", username)
                                        } else {
                                            format!("⚠️ @{}, une erreur technique est survenue lors de la mise à jour de tes pièces d'or.", username)
                                        }).await;
                                    }
                                }
                            }
                            CoinflipOutcome::Disappear => {
                                let game_data = if use_english {
                                    crate::config::get_game_data_en()
                                } else {
                                    crate::config::get_game_data_fr()
                                };
                                let templates = &game_data.cf_disappear_messages;
                                let chosen_template = if !templates.is_empty() {
                                    let idx = rand::thread_rng().gen_range(0..templates.len());
                                    &templates[idx]
                                } else {
                                    if use_english {
                                        "🪙 @{username} flips the coin... but it disappeared!"
                                    } else {
                                        "🪙 @{username} lance la pièce... mais elle a disparu !"
                                    }
                                };

                                match state_task.repo.record_coinflip_result(player.id.unwrap(), wager_amount, false).await {
                                    Ok(updated_player) => {
                                        let new_gold = updated_player.gold;
                                        let msg_text = chosen_template
                                            .replace("{username}", &username)
                                            .replace("{wager}", &wager_amount.to_string())
                                            .replace("{gold}", &new_gold.to_string());
                                        let _ = client_msg.say(channel_login, msg_text).await;
                                    }
                                    Err(e) => {
                                        tracing::error!("Failed to update player gold in coinflip disappear: {:?}", e);
                                        let _ = client_msg.say(channel_login, if use_english {
                                            format!("⚠️ @{}, a technical error occurred while updating your gold.", username)
                                        } else {
                                            format!("⚠️ @{}, une erreur technique est survenue lors de la mise à jour de tes pièces d'or.", username)
                                        }).await;
                                    }
                                }
                            }
                            CoinflipOutcome::Edge => {
                                let game_data = if use_english {
                                    crate::config::get_game_data_en()
                                } else {
                                    crate::config::get_game_data_fr()
                                };
                                let templates = &game_data.cf_edge_messages;
                                let chosen_template = if !templates.is_empty() {
                                    let idx = rand::thread_rng().gen_range(0..templates.len());
                                    &templates[idx]
                                } else {
                                    if use_english {
                                        "🪙 @{username} flips the coin... and it lands on its edge! Triple! (+{gold_earned} gold)! You have {gold} gold."
                                    } else {
                                        "🪙 @{username} lance la pièce... et elle s'arrête sur la tranche ! Triplé ! (+{gold_earned} po)! Tu as {gold} po."
                                    }
                                };

                                match state_task.repo.record_coinflip_edge_result(player.id.unwrap(), wager_amount).await {
                                    Ok(updated_player) => {
                                        let new_gold = updated_player.gold;
                                        let gold_earned = wager_amount * 2;
                                        let msg_text = chosen_template
                                            .replace("{username}", &username)
                                            .replace("{wager}", &wager_amount.to_string())
                                            .replace("{gold_earned}", &gold_earned.to_string())
                                            .replace("{gold}", &new_gold.to_string());
                                        let _ = client_msg.say(channel_login, msg_text).await;
                                    }
                                    Err(e) => {
                                        tracing::error!("Failed to update player gold in coinflip edge: {:?}", e);
                                        let _ = client_msg.say(channel_login, if use_english {
                                            format!("⚠️ @{}, a technical error occurred while updating your gold.", username)
                                        } else {
                                            format!("⚠️ @{}, une erreur technique est survenue lors de la mise à jour de tes pièces d'or.", username)
                                        }).await;
                                    }
                                }
                            }
                        }
                    });
                } else if text.starts_with("!fish sell") || text.starts_with("!fish vend") || text.starts_with("!fish vendre") || text.starts_with("!fish vends") || text.starts_with("!peche sell") || text.starts_with("!peche vend") || text.starts_with("!peche vendre") || text.starts_with("!peche vends") || text.starts_with("!pêche sell") || text.starts_with("!pêche vend") || text.starts_with("!pêche vendre") || text.starts_with("!pêche vends") {
                    let state_task = Arc::clone(&state_clone);
                    let client_msg = client.clone();
                    let channel_login = msg.channel_login.clone();
                    let raw_msg = msg.message_text.clone();
                    
                    tokio::spawn(async move {
                        let text_trim = raw_msg.trim().to_lowercase();
                        let arg = if text_trim.starts_with("!fish sell") {
                            raw_msg["!fish sell".len()..].trim()
                        } else if text_trim.starts_with("!fish vend") {
                            raw_msg["!fish vend".len()..].trim()
                        } else if text_trim.starts_with("!fish vendre") {
                            raw_msg["!fish vendre".len()..].trim()
                        } else if text_trim.starts_with("!fish vends") {
                            raw_msg["!fish vends".len()..].trim()
                        } else if text_trim.starts_with("!peche sell") {
                            raw_msg["!peche sell".len()..].trim()
                        } else if text_trim.starts_with("!peche vend") {
                            raw_msg["!peche vend".len()..].trim()
                        } else if text_trim.starts_with("!peche vendre") {
                            raw_msg["!peche vendre".len()..].trim()
                        } else if text_trim.starts_with("!peche vends") {
                            raw_msg["!peche vends".len()..].trim()
                        } else if text_trim.starts_with("!pêche sell") {
                            raw_msg["!pêche sell".len()..].trim()
                        } else if text_trim.starts_with("!pêche vend") {
                            raw_msg["!pêche vend".len()..].trim()
                        } else if text_trim.starts_with("!pêche vendre") {
                            raw_msg["!pêche vendre".len()..].trim()
                        } else if text_trim.starts_with("!pêche vends") {
                            raw_msg["!pêche vends".len()..].trim()
                        } else {
                            ""
                        };

                        let parsed = parse_sell_args(arg);
                        if parsed.is_none() {
                            let _ = client_msg.say(channel_login, format!("⚠️ @{}, usage : !fish sell [poisson] [état] [qté], !fish sell #[id_capture], ou !fish sell all [rareté] [état]", username)).await;
                            return;
                        }

                        match parsed.unwrap() {
                            SellArg::ConfirmYes => {
                                let mut sales = state_task.pending_sales.write().await;
                                if let Some(pending) = sales.get(&username) {
                                    if Utc::now().signed_duration_since(pending.created_at).num_seconds() <= 60 {
                                        let pending_clone = pending.clone();
                                        drop(sales); // drop write lock before DB
                                        if let Ok(_) = state_task.repo.execute_gold_sale(pending_clone.player_id, &pending_clone.catch_ids, pending_clone.gold_earned).await {
                                            let _ = client_msg.say(channel_login, format!("💸 @{}, vente réussie ! Tu as vendu {} exemplaire(s) pour {} pièces d'or 🪙.", username, pending_clone.catch_ids.len(), pending_clone.gold_earned)).await;
                                        } else {
                                            let _ = client_msg.say(channel_login, format!("❌ @{}, une erreur est survenue lors de la vente.", username)).await;
                                        }
                                        state_task.pending_sales.write().await.remove(&username);
                                    } else {
                                        sales.remove(&username);
                                        let _ = client_msg.say(channel_login, format!("⚠️ @{}, proposition de vente expirée (1 min).", username)).await;
                                    }
                                } else {
                                    let _ = client_msg.say(channel_login, format!("⚠️ @{}, aucune proposition de vente en attente.", username)).await;
                                }
                            }
                            SellArg::ConfirmNo => {
                                let mut sales = state_task.pending_sales.write().await;
                                if sales.remove(&username).is_some() {
                                    let _ = client_msg.say(channel_login, format!("❌ @{}, proposition de vente annulée.", username)).await;
                                } else {
                                    let _ = client_msg.say(channel_login, format!("⚠️ @{}, aucune proposition de vente en attente.", username)).await;
                                }
                            }
                            SellArg::ById(id) => {
                                if let Ok(player) = state_task.repo.get_or_create_player(&username).await {
                                    if let Ok(catches) = state_task.repo.get_player_catches(player.id.unwrap()).await {
                                        let target = catches.into_iter().find(|c| c.id == Some(id));
                                        if let Some(c) = target {
                                            let base = get_base_price(&c.name);
                                            let mult = if c.is_junk { 1.0 } else { get_stored_state_multiplier(&c.state) };
                                            let price = if c.is_junk { 10 } else { ((base as f64 * mult).round() as i64).max(1) };

                                            let pending = PendingSale {
                                                player_id: player.id.unwrap(),
                                                catch_ids: vec![id],
                                                catch_names: vec![c.name.clone()],
                                                gold_earned: price,
                                                created_at: Utc::now(),
                                            };
                                            state_task.pending_sales.write().await.insert(username.clone(), pending);
                                            let _ = client_msg.say(channel_login, format!("💰 @{}, tu es sur le point de vendre '{}' (#{}, {}) pour {} pièces d'or 🪙. Tape !fish sell oui pour valider (1 min max) !", username, c.name, id, c.state, price)).await;
                                        } else {
                                            let _ = client_msg.say(channel_login, format!("❌ @{}, capture #{} introuvable dans ton inventaire.", username, id)).await;
                                        }
                                    }
                                }
                            }
                            SellArg::All { rarity, state } => {
                                if let Ok(player) = state_task.repo.get_or_create_player(&username).await {
                                    if let Ok(catches) = state_task.repo.get_player_catches(player.id.unwrap()).await {
                                        let sellable: Vec<_> = catches.into_iter()
                                            .filter(|c| {
                                                let name_lower = c.name.to_lowercase();
                                                let is_non_fish = name_lower.contains("banana")
                                                    || name_lower.contains("carte postale")
                                                    || name_lower.contains("gemme")
                                                    || c.is_junk;

                                                if is_non_fish {
                                                    return false;
                                                }

                                                if let Some(r) = rarity {
                                                    if c.rarity != r {
                                                        return false;
                                                    }
                                                }

                                                if let Some(ref s) = state {
                                                    if c.state.to_lowercase() != s.to_lowercase() {
                                                        return false;
                                                    }
                                                }

                                                true
                                            })
                                            .collect();

                                        if sellable.is_empty() {
                                            let filter_err = match (rarity, &state) {
                                                (Some(r), Some(s)) => format!(" avec la rareté {:?} et l'état {}", r, s),
                                                (Some(r), None) => format!(" avec la rareté {:?}", r),
                                                (None, Some(s)) => format!(" avec l'état {}", s),
                                                (None, None) => "".to_string(),
                                            };
                                            let _ = client_msg.say(channel_login, format!("⚠️ @{}, tu n'as aucun poisson vendable{} dans ton inventaire.", username, filter_err)).await;
                                            return;
                                        }

                                        let mut total_price = 0;
                                        let mut selected_ids = Vec::new();
                                        let mut selected_names = Vec::new();

                                        for c in &sellable {
                                            let base = get_base_price(&c.name);
                                            let mult = get_stored_state_multiplier(&c.state);
                                            let price = ((base as f64 * mult).round() as i64).max(1);
                                            total_price += price;
                                            selected_ids.push(c.id.unwrap());
                                            selected_names.push(c.name.clone());
                                        }

                                        let pending = PendingSale {
                                            player_id: player.id.unwrap(),
                                            catch_ids: selected_ids,
                                            catch_names: selected_names,
                                            gold_earned: total_price,
                                            created_at: Utc::now(),
                                        };
                                        state_task.pending_sales.write().await.insert(username.clone(), pending);

                                        let rarity_str = rarity.map(|r| match r {
                                            Rarity::Common => "commun",
                                            Rarity::Uncommon => "peu commun",
                                            Rarity::Rare => "rare",
                                            Rarity::VeryRare => "très rare",
                                            Rarity::Epic => "épique",
                                            Rarity::Legendary => "légendaire",
                                            Rarity::Mythical => "mythique",
                                            Rarity::Divin => "divin",
                                        });

                                        let filter_desc = match (rarity_str, &state) {
                                            (Some(r), Some(s)) => format!("de rareté {} et en état {}", r, s),
                                            (Some(r), None) => format!("de rareté {}", r),
                                            (None, Some(s)) => format!("en état {}", s),
                                            (None, None) => "".to_string(),
                                        };
                                        let filter_space = if filter_desc.is_empty() { "".to_string() } else { format!(" {} ", filter_desc) };

                                        let _ = client_msg.say(channel_login, format!("💰 @{}, tu es sur le point de vendre TOUS tes poissons{} ({} objets, hors déchets, bananes, cartes postales et gemmes) pour un total de {} pièces d'or 🪙. Tape !fish sell oui pour valider (1 min max) !", username, filter_space, sellable.len(), total_price)).await;
                                    }
                                }
                            }
                            SellArg::ByName { name, state, quantity } => {
                                if let Ok(player) = state_task.repo.get_or_create_player(&username).await {
                                    if let Ok(catches) = state_task.repo.get_player_catches(player.id.unwrap()).await {
                                        let mut matching: Vec<_> = catches.into_iter()
                                            .filter(|c| {
                                                let name_match = c.name.to_lowercase() == name.to_lowercase();
                                                let state_match = match &state {
                                                    Some(s) => c.state.to_lowercase() == s.to_lowercase(),
                                                    None => true,
                                                };
                                                name_match && state_match
                                            })
                                            .collect();

                                        if matching.is_empty() {
                                            let state_str = state.map(|s| format!(" ({})", s)).unwrap_or_default();
                                            let _ = client_msg.say(channel_login, format!("⚠️ @{}, tu ne possèdes aucun '{}'{} dans ton inventaire.", username, name, state_str)).await;
                                            return;
                                        }

                                        if (matching.len() as i64) < quantity {
                                            let _ = client_msg.say(channel_login, format!("⚠️ @{}, tu ne possèdes que {} exemplaire(s) de '{}' (requis: {}).", username, matching.len(), name, quantity)).await;
                                            return;
                                        }

                                        // If state is not specified, sort most-damaged-first
                                        if state.is_none() {
                                            matching.sort_by_key(|c| get_state_weight(&c.state));
                                        }

                                        // Select the first `quantity` elements
                                        let selected = &matching[0..(quantity as usize)];
                                        let mut total_price = 0;
                                        let mut selected_ids = Vec::new();
                                        let mut selected_names = Vec::new();

                                        for c in selected {
                                            let base = get_base_price(&c.name);
                                            let mult = if c.is_junk { 1.0 } else { get_stored_state_multiplier(&c.state) };
                                            let price = if c.is_junk { 10 } else { ((base as f64 * mult).round() as i64).max(1) };
                                            total_price += price;
                                            selected_ids.push(c.id.unwrap());
                                            selected_names.push(c.name.clone());
                                        }

                                        let pending = PendingSale {
                                            player_id: player.id.unwrap(),
                                            catch_ids: selected_ids,
                                            catch_names: selected_names,
                                            gold_earned: total_price,
                                            created_at: Utc::now(),
                                        };
                                        state_task.pending_sales.write().await.insert(username.clone(), pending);
                                        let _ = client_msg.say(channel_login, format!("💰 @{}, tu es sur le point de vendre {}x '{}' pour {} pièces d'or 🪙. Tape !fish sell oui pour valider (1 min max) !", username, quantity, name, total_price)).await;
                                    }
                                }
                            }
                        }
                    });
                } else if text.starts_with("!fish recycle") || text.starts_with("!fish recycler") ||
                           text.starts_with("!peche recycle") || text.starts_with("!peche recycler") ||
                           text.starts_with("!pêche recycle") || text.starts_with("!pêche recycler") {
                    let state_task = Arc::clone(&state_clone);
                    let client_msg = client.clone();
                    let channel_login = msg.channel_login.clone();
                    let raw_msg = msg.message_text.clone();
                    
                    tokio::spawn(async move {
                        let text_trim = raw_msg.trim().to_lowercase();
                        let arg = if text_trim.starts_with("!fish recycler") {
                            raw_msg["!fish recycler".len()..].trim()
                        } else if text_trim.starts_with("!fish recycle") {
                            raw_msg["!fish recycle".len()..].trim()
                        } else if text_trim.starts_with("!peche recycler") {
                            raw_msg["!peche recycler".len()..].trim()
                        } else if text_trim.starts_with("!peche recycle") {
                            raw_msg["!peche recycle".len()..].trim()
                        } else if text_trim.starts_with("!pêche recycler") {
                            raw_msg["!pêche recycler".len()..].trim()
                        } else if text_trim.starts_with("!pêche recycle") {
                            raw_msg["!pêche recycle".len()..].trim()
                        } else {
                            ""
                        };

                        let parsed = parse_recycle_args(arg);
                        if parsed.is_none() {
                            let _ = client_msg.say(channel_login, format!("⚠️ @{}, usage : !fish recycle #[id_capture] [poubelle] (ex : !fish recycle #42 jaune). Poubelles : bleu, jaune, vert, marron, gris, noir(e), decharge.", username)).await;
                            return;
                        }

                        let (catch_id, bin_name) = parsed.unwrap();
                        let normalized_bin = normalize_bin_name(&bin_name);
                        if normalized_bin.is_none() {
                            let _ = client_msg.say(channel_login, format!("⚠️ @{}, poubelle '{}' inconnue. Poubelles valides : bleu, jaune, vert, marron, gris, noir(e), decharge.", username, bin_name)).await;
                            return;
                        }
                        let normalized_bin = normalized_bin.unwrap();

                        if let Ok(player) = state_task.repo.get_or_create_player(&username).await {
                            let use_english = player.language.as_deref() == Some("en") || (player.language.is_none() && text_trim.starts_with("!fish"));
                            
                            if let Ok(catches) = state_task.repo.get_player_catches(player.id.unwrap()).await {
                                let target = catches.into_iter().find(|c| c.id == Some(catch_id));
                                if let Some(c) = target {
                                    if !c.is_junk {
                                        let msg = if use_english {
                                            format!("❌ @{}, you cannot recycle '{}' (#{}) because it is not a junk item! Only junk items can be recycled.", username, c.name, catch_id)
                                        } else {
                                            format!("❌ @{}, tu ne peux pas recycler '{}' (#{}) car ce n'est pas un déchet ! Seuls les déchets peuvent être recyclés.", username, c.name, catch_id)
                                        };
                                        let _ = client_msg.say(channel_login, msg).await;
                                        return;
                                    }

                                    let game_data = crate::config::get_game_data(use_english);
                                    let junk_ref = &game_data.junk_data;
                                    let mut correct_bin = "decharge".to_string();
                                    let mut bonus = 10;
                                    let mut malus = 20;

                                    let mut found_config = false;
                                    for list in junk_ref.values() {
                                        if let Some(config_item) = list.iter().find(|item| item.name.to_lowercase() == c.name.to_lowercase()) {
                                            correct_bin = config_item.bin.clone().unwrap_or_else(|| "decharge".to_string());
                                            bonus = config_item.recycling_notoriety_bonus.unwrap_or(10);
                                            malus = config_item.recycling_notoriety_malus.unwrap_or(20);
                                            found_config = true;
                                            break;
                                        }
                                    }

                                    if !found_config {
                                        let game_data_fr = crate::config::get_game_data(false);
                                        let junk_ref_fr = &game_data_fr.junk_data;
                                        for list in junk_ref_fr.values() {
                                            if let Some(config_item) = list.iter().find(|item| item.name.to_lowercase() == c.name.to_lowercase()) {
                                                correct_bin = config_item.bin.clone().unwrap_or_else(|| "decharge".to_string());
                                                bonus = config_item.recycling_notoriety_bonus.unwrap_or(10);
                                                malus = config_item.recycling_notoriety_malus.unwrap_or(20);
                                                break;
                                            }
                                        }
                                    }

                                    let is_correct = normalized_bin == correct_bin;
                                    let change = if is_correct { bonus } else { -malus };
                                    let total_notoriety = (player.eco_notoriety + change).max(0);

                                    if let Ok(_) = state_task.repo.execute_recycling(player.id.unwrap(), catch_id, change).await {
                                        let msg = if is_correct {
                                            if use_english {
                                                format!("♻️ @{} successfully recycled '{}' (#{}) in the {} bin! Eco notoriety: +{} (Total: {}).", username, c.name, catch_id, bin_name, bonus, total_notoriety)
                                            } else {
                                                format!("♻️ @{} a recyclé avec succès '{}' (#{}) dans la poubelle {} ! Notoriété écolo : +{} (Total : {}).", username, c.name, catch_id, bin_name, bonus, total_notoriety)
                                            }
                                        } else {
                                            if use_english {
                                                format!("🚯 @{} put '{}' (#{}) in the wrong bin (used {}, belonged in {})! Eco notoriety: -{} (Total: {}).", username, c.name, catch_id, bin_name, correct_bin, malus, total_notoriety)
                                            } else {
                                                format!("🚯 @{} a mis '{}' (#{}) dans la mauvaise poubelle (utilisé {}, requis {}) ! Notoriété écolo : -{} (Total : {}).", username, c.name, catch_id, bin_name, correct_bin, malus, total_notoriety)
                                            }
                                        };
                                        let _ = client_msg.say(channel_login, msg).await;
                                    } else {
                                        let msg = if use_english {
                                            format!("❌ @{}, an error occurred while recycling catch #{}.", username, catch_id)
                                        } else {
                                            format!("❌ @{}, une erreur est survenue lors du recyclage de la capture #{}.", username, catch_id)
                                        };
                                        let _ = client_msg.say(channel_login, msg).await;
                                    }
                                } else {
                                    let msg = if use_english {
                                        format!("❌ @{}, catch #{} not found in your inventory.", username, catch_id)
                                    } else {
                                        format!("❌ @{}, capture #{} introuvable dans ton inventaire.", username, catch_id)
                                    };
                                    let _ = client_msg.say(channel_login, msg).await;
                                }
                            }
                        }
                    });
                } else if text.starts_with("!fish trade") || text.starts_with("!peche trade") || text.starts_with("!pêche trade") || text.starts_with("!fish echange") || text.starts_with("!fish échange") || text.starts_with("!peche echange") || text.starts_with("!pêche échange") {
                    let state_task = Arc::clone(&state_clone);
                    let client_msg = client.clone();
                    let channel_login = msg.channel_login.clone();
                    let raw_msg = msg.message_text.clone();
                    
                    tokio::spawn(async move {
                        let text_trim = raw_msg.trim().to_lowercase();
                        let arg = if text_trim.starts_with("!fish trade") {
                            raw_msg["!fish trade".len()..].trim()
                        } else if text_trim.starts_with("!peche trade") {
                            raw_msg["!peche trade".len()..].trim()
                        } else if text_trim.starts_with("!pêche trade") {
                            raw_msg["!pêche trade".len()..].trim()
                        } else if text_trim.starts_with("!fish echange") {
                            raw_msg["!fish echange".len()..].trim()
                        } else if text_trim.starts_with("!fish échange") {
                            raw_msg["!fish échange".len()..].trim()
                        } else if text_trim.starts_with("!peche echange") {
                            raw_msg["!peche echange".len()..].trim()
                        } else if text_trim.starts_with("!pêche échange") {
                            raw_msg["!pêche échange".len()..].trim()
                        } else {
                            ""
                        };

                        let parsed = parse_trade_args(arg);
                        if parsed.is_none() {
                            let _ = client_msg.say(channel_login, format!("⚠️ @{}, usage : Direct : !fish trade #[id] [prix] @destinataire | Troc : !fish trade #[id] @destinataire | accept | cancel", username)).await;
                            return;
                        }

                        // Clean up expired trades first
                        {
                            let mut trades = state_task.pending_trades.write().await;
                            trades.retain(|t| {
                                match t {
                                    PendingTrade::Direct { created_at, .. } => {
                                        Utc::now().signed_duration_since(*created_at).num_seconds() <= 60
                                    }
                                    PendingTrade::Barter { last_activity, .. } => {
                                        Utc::now().signed_duration_since(*last_activity).num_seconds() <= 60
                                    }
                                }
                            });
                        }

                        match parsed.unwrap() {
                            TradeArg::Accept => {
                                let mut found_idx = None;
                                {
                                    let trades = state_task.pending_trades.read().await;
                                    for (i, t) in trades.iter().enumerate() {
                                        match t {
                                            PendingTrade::Direct { buyer_username, .. } => {
                                                if buyer_username == &username {
                                                    found_idx = Some(i);
                                                    break;
                                                }
                                            }
                                            PendingTrade::Barter { player_a_username, player_b_username, step, .. } => {
                                                if *step == 2 && (player_a_username == &username || player_b_username == &username) {
                                                    found_idx = Some(i);
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                }

                                if let Some(idx) = found_idx {
                                    let mut trades = state_task.pending_trades.write().await;
                                    let trade = &mut trades[idx];
                                    match trade.clone() {
                                        PendingTrade::Direct { seller_id, seller_username, buyer_username, catch_id, catch_name, price, .. } => {
                                            trades.remove(idx);
                                            drop(trades);

                                            if let Ok(buyer) = state_task.repo.get_or_create_player(&buyer_username).await {
                                                if buyer.gold < price {
                                                    let _ = client_msg.say(channel_login, format!("❌ @{}, tu n'as pas assez de pièces d'or (requis: {} po, tu as {} po).", buyer_username, price, buyer.gold)).await;
                                                    return;
                                                }

                                                if let Ok(_) = state_task.repo.execute_direct_trade(catch_id, seller_id, buyer.id.unwrap(), price).await {
                                                    let _ = client_msg.say(channel_login, format!("🤝 Échange réussi ! @{} a acheté '{}' (#{}) de @{} pour {} pièces d'or 🪙 !", buyer_username, catch_name, catch_id, seller_username, price)).await;
                                                } else {
                                                    let _ = client_msg.say(channel_login, format!("❌ @{}, une erreur est survenue lors de l'échange.", buyer_username)).await;
                                                }
                                            }
                                        }
                                        PendingTrade::Barter { player_a_id, player_a_username, catch_a_id, catch_a_name, player_b_username, catch_b_id, catch_b_name, mut player_a_accepted, mut player_b_accepted, .. } => {
                                            if player_a_username == username {
                                                player_a_accepted = true;
                                            } else if player_b_username == username {
                                                player_b_accepted = true;
                                            }

                                            if player_a_accepted && player_b_accepted {
                                                trades.remove(idx);
                                                drop(trades);

                                                if let Ok(player_b) = state_task.repo.get_or_create_player(&player_b_username).await {
                                                    if let Ok(_) = state_task.repo.execute_barter_trade(catch_a_id, player_a_id, catch_b_id.unwrap(), player_b.id.unwrap()).await {
                                                        let _ = client_msg.say(channel_login, format!("🔄 Troc réussi ! @{} a échangé '{}' (#{}) contre '{}' (#{}) de @{} !", player_a_username, catch_a_name, catch_a_id, catch_b_name.unwrap(), catch_b_id.unwrap(), player_b_username)).await;
                                                    } else {
                                                        let _ = client_msg.say(channel_login, format!("❌ Échange échoué. Assurez-vous que les poissons sont toujours dans les inventaires respectifs.")).await;
                                                    }
                                                }
                                            } else {
                                                if let PendingTrade::Barter { player_a_accepted: a_acc, player_b_accepted: b_acc, last_activity, .. } = &mut trades[idx] {
                                                    *a_acc = player_a_accepted;
                                                    *b_acc = player_b_accepted;
                                                    *last_activity = Utc::now();
                                                }
                                                let other = if player_a_username == username { &player_b_username } else { &player_a_username };
                                                let _ = client_msg.say(channel_login, format!("✅ @{} a accepté l'échange. @{}, tape à ton tour !fish trade accept pour finaliser l'échange (1 min max) !", username, other)).await;
                                            }
                                        }
                                    }
                                } else {
                                    let mut is_step1_target = false;
                                    {
                                        let trades = state_task.pending_trades.read().await;
                                        for t in trades.iter() {
                                            if let PendingTrade::Barter { player_b_username, step, .. } = t {
                                                if player_b_username == &username && *step == 1 {
                                                    is_step1_target = true;
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                    if is_step1_target {
                                        let _ = client_msg.say(channel_login, format!("⚠️ @{}, tu dois d'abord proposer ton poisson d'échange : !fish trade #[ton_id] @[partenaire] !", username)).await;
                                    } else {
                                        let _ = client_msg.say(channel_login, format!("⚠️ @{}, aucun échange en attente de ta confirmation.", username)).await;
                                    }
                                }
                            }
                            TradeArg::Cancel => {
                                let mut found_idx = None;
                                {
                                    let trades = state_task.pending_trades.read().await;
                                    for (i, t) in trades.iter().enumerate() {
                                        match t {
                                            PendingTrade::Direct { seller_username, buyer_username, .. } => {
                                                if seller_username == &username || buyer_username == &username {
                                                    found_idx = Some(i);
                                                    break;
                                                }
                                            }
                                            PendingTrade::Barter { player_a_username, player_b_username, .. } => {
                                                if player_a_username == &username || player_b_username == &username {
                                                    found_idx = Some(i);
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                }

                                if let Some(idx) = found_idx {
                                    state_task.pending_trades.write().await.remove(idx);
                                    let _ = client_msg.say(channel_login, format!("❌ @{}, échange annulé.", username)).await;
                                } else {
                                    let _ = client_msg.say(channel_login, format!("⚠️ @{}, aucun échange en cours te concernant.", username)).await;
                                }
                            }
                            TradeArg::Direct { catch_id, price, recipient } => {
                                if recipient == username {
                                    let _ = client_msg.say(channel_login, format!("❌ @{}, tu ne peux pas faire d'échange avec toi-même.", username)).await;
                                    return;
                                }

                                if price < 0 {
                                    let _ = client_msg.say(channel_login, format!("❌ @{}, le prix doit être positif.", username)).await;
                                    return;
                                }

                                if let Ok(player) = state_task.repo.get_or_create_player(&username).await {
                                    if let Ok(catches) = state_task.repo.get_player_catches(player.id.unwrap()).await {
                                        let catch = catches.into_iter().find(|c| c.id == Some(catch_id));
                                        if let Some(c) = catch {
                                            if let Ok(buyer) = state_task.repo.get_or_create_player(&recipient).await {
                                                if buyer.gold < price {
                                                    let _ = client_msg.say(channel_login, format!("❌ @{}, @{} n'a pas assez d'or (requis: {} po, il a {} po).", username, recipient, price, buyer.gold)).await;
                                                    return;
                                                }

                                                let pending = PendingTrade::Direct {
                                                    seller_id: player.id.unwrap(),
                                                    seller_username: username.clone(),
                                                    buyer_username: recipient.clone(),
                                                    catch_id,
                                                    catch_name: c.name.clone(),
                                                    price,
                                                    created_at: Utc::now(),
                                                };

                                                {
                                                    let mut trades = state_task.pending_trades.write().await;
                                                    trades.retain(|t| {
                                                        match t {
                                                            PendingTrade::Direct { seller_username, buyer_username, .. } => {
                                                                !(seller_username == &username && buyer_username == &recipient)
                                                            }
                                                            _ => true
                                                        }
                                                    });
                                                    trades.push(pending);
                                                }

                                                let _ = client_msg.say(channel_login, format!("🤝 @{}, @{} te propose d'acheter son poisson '{}' (#{}) pour {} pièces d'or 🪙. Tape !fish trade accept pour accepter (1 min max) !", recipient, username, c.name, catch_id, price)).await;
                                            }
                                        } else {
                                            let _ = client_msg.say(channel_login, format!("❌ @{}, capture #{} introuvable dans ton inventaire.", username, catch_id)).await;
                                        }
                                    }
                                }
                            }
                            TradeArg::Barter { catch_id, recipient } => {
                                if recipient == username {
                                    let _ = client_msg.say(channel_login, format!("❌ @{}, tu ne peux pas faire d'échange avec toi-même.", username)).await;
                                    return;
                                }

                                if let Ok(player) = state_task.repo.get_or_create_player(&username).await {
                                    if let Ok(catches) = state_task.repo.get_player_catches(player.id.unwrap()).await {
                                        let catch = catches.into_iter().find(|c| c.id == Some(catch_id));
                                        if let Some(c) = catch {
                                            let mut found_step1_idx = None;
                                            {
                                                let trades = state_task.pending_trades.read().await;
                                                for (i, t) in trades.iter().enumerate() {
                                                    if let PendingTrade::Barter { player_a_username, player_b_username, step, .. } = t {
                                                        if player_a_username == &recipient && player_b_username == &username && *step == 1 {
                                                            found_step1_idx = Some(i);
                                                            break;
                                                        }
                                                    }
                                                }
                                            }

                                            if let Some(idx) = found_step1_idx {
                                                let mut trades = state_task.pending_trades.write().await;
                                                if let PendingTrade::Barter { catch_b_id, catch_b_name, step, last_activity, .. } = &mut trades[idx] {
                                                    *catch_b_id = Some(catch_id);
                                                    *catch_b_name = Some(c.name.clone());
                                                    *step = 2;
                                                    *last_activity = Utc::now();
                                                }
                                                let trade_clone = trades[idx].clone();
                                                drop(trades);

                                                if let PendingTrade::Barter { catch_a_id, catch_a_name, .. } = trade_clone {
                                                    let _ = client_msg.say(channel_login, format!("🔄 Contre-proposition ! @{} propose '{}' (#{}) contre '{}' (#{}) de @{}. Pour confirmer cet échange, vous devez TOUS LES DEUX tapez !fish trade accept (1 min max) !", username, c.name, catch_id, catch_a_name, catch_a_id, recipient)).await;
                                                }
                                            } else {
                                                let pending = PendingTrade::Barter {
                                                    player_a_id: player.id.unwrap(),
                                                    player_a_username: username.clone(),
                                                    catch_a_id: catch_id,
                                                    catch_a_name: c.name.clone(),
                                                    player_b_username: recipient.clone(),
                                                    catch_b_id: None,
                                                    catch_b_name: None,
                                                    step: 1,
                                                    player_a_accepted: false,
                                                    player_b_accepted: false,
                                                    last_activity: Utc::now(),
                                                };

                                                {
                                                    let mut trades = state_task.pending_trades.write().await;
                                                    trades.retain(|t| {
                                                        match t {
                                                            PendingTrade::Barter { player_a_username, player_b_username, .. } => {
                                                                !((player_a_username == &username && player_b_username == &recipient) || (player_a_username == &recipient && player_b_username == &username))
                                                            }
                                                            _ => true
                                                        }
                                                    });
                                                    trades.push(pending);
                                                }

                                                let _ = client_msg.say(channel_login, format!("🤝 @{}, @{} te propose d'échanger son poisson '{}' (#{}). Fais une contre-proposition en tapant !fish trade #[ton_id] @{} dans la minute !", recipient, username, c.name, catch_id, username)).await;
                                            }
                                        } else {
                                            let _ = client_msg.say(channel_login, format!("❌ @{}, capture #{} introuvable dans ton inventaire.", username, catch_id)).await;
                                        }
                                    }
                                }
                            }
                        }
                    });
                } else if text.starts_with("!fish give") || text.starts_with("!peche give") || text.starts_with("!pêche give") ||
                           text.starts_with("!fish gift") || text.starts_with("!peche gift") || text.starts_with("!pêche gift") ||
                           text.starts_with("!fish don")  || text.starts_with("!peche don")  || text.starts_with("!pêche don")  ||
                           text.starts_with("!fish donner") || text.starts_with("!peche donner") || text.starts_with("!pêche donner") {
                    let state_task = Arc::clone(&state_clone);
                    let client_msg = client.clone();
                    let channel_login = msg.channel_login.clone();
                    let raw_msg = msg.message_text.clone();
                    
                    tokio::spawn(async move {
                        let text_trim = raw_msg.trim().to_lowercase();
                        let mut arg = "";
                        let prefixes = [
                            "!fish give", "!peche give", "!pêche give",
                            "!fish gift", "!peche gift", "!pêche gift",
                            "!fish donner", "!peche donner", "!pêche donner",
                            "!fish don", "!peche don", "!pêche don",
                        ];
                        for p in &prefixes {
                            if text_trim.starts_with(p) {
                                arg = raw_msg[p.len()..].trim();
                                break;
                            }
                        }

                        let parsed = parse_give_args(arg);
                        if parsed.is_none() {
                            let _ = client_msg.say(channel_login, format!("⚠️ @{}, usage : !fish give [montant/objet] @[destinataire]", username)).await;
                            return;
                        }

                        match parsed.unwrap() {
                            GiveArg::ConfirmYes => {
                                let mut gives = state_task.pending_gives.write().await;
                                if let Some(pending) = gives.get(&username) {
                                    if Utc::now().signed_duration_since(pending.created_at).num_seconds() <= 60 {
                                        let pending_clone = pending.clone();
                                        drop(gives); // drop lock before db call
                                        
                                        match state_task.repo.execute_catch_transfer(
                                            pending_clone.catch_id,
                                            pending_clone.giver_id,
                                            pending_clone.receiver_id,
                                        ).await {
                                            Ok(_) => {
                                                let _ = client_msg.say(
                                                    channel_login,
                                                    format!(
                                                        "🎁 @{}, tu as donné '{}' (#{}) à @{} ! 🤝",
                                                        username,
                                                        pending_clone.catch_name,
                                                        pending_clone.catch_id,
                                                        pending_clone.receiver_username
                                                    )
                                                ).await;
                                            }
                                            Err(e) => {
                                                tracing::error!("Failed to execute catch transfer: {:?}", e);
                                                let _ = client_msg.say(
                                                    channel_login,
                                                    format!("❌ @{}, impossible de donner cet objet. Une erreur est survenue.", username)
                                                ).await;
                                            }
                                        }
                                        state_task.pending_gives.write().await.remove(&username);
                                    } else {
                                        gives.remove(&username);
                                        let _ = client_msg.say(
                                            channel_login,
                                            format!("⚠️ @{}, proposition de don expirée (1 min).", username)
                                        ).await;
                                    }
                                } else {
                                    let _ = client_msg.say(
                                        channel_login,
                                        format!("⚠️ @{}, aucune proposition de don en attente.", username)
                                    ).await;
                                }
                            }
                            GiveArg::ConfirmNo => {
                                let mut gives = state_task.pending_gives.write().await;
                                if gives.remove(&username).is_some() {
                                    let _ = client_msg.say(
                                        channel_login,
                                        format!("🎁 @{}, proposition de don annulée.", username)
                                    ).await;
                                } else {
                                    let _ = client_msg.say(
                                        channel_login,
                                        format!("⚠️ @{}, aucune proposition de don en attente.", username)
                                    ).await;
                                }
                            }
                            GiveArg::Gold { amount, recipient } => {
                                if recipient == username {
                                    let _ = client_msg.say(channel_login, format!("❌ @{}, tu ne peux pas te donner de l'or à toi-même.", username)).await;
                                    return;
                                }

                                if amount <= 0 {
                                    let _ = client_msg.say(channel_login, format!("❌ @{}, le montant doit être supérieur à 0.", username)).await;
                                    return;
                                }

                                // Retrieve giver player info
                                let giver = match state_task.repo.get_or_create_player(&username).await {
                                    Ok(g) => g,
                                    Err(e) => {
                                        tracing::error!("Failed to retrieve giver player: {:?}", e);
                                        return;
                                    }
                                };

                                if giver.gold < amount {
                                    let _ = client_msg.say(
                                        channel_login,
                                        format!(
                                            "❌ @{}, tu n'as pas assez de pièces d'or (requis: {} po, tu as {} po).",
                                            username, amount, giver.gold
                                        )
                                    ).await;
                                    return;
                                }

                                // Retrieve recipient player info
                                let receiver = match state_task.repo.get_player(&recipient).await {
                                    Ok(Some(r)) => r,
                                    Ok(None) => {
                                        let _ = client_msg.say(
                                            channel_login,
                                            format!("❌ @{}, le joueur @{} n'a pas encore de compte de pêche.", username, recipient)
                                        ).await;
                                        return;
                                    }
                                    Err(e) => {
                                        tracing::error!("Failed to retrieve recipient player: {:?}", e);
                                        return;
                                    }
                                };

                                let giver_id = giver.id.unwrap_or(0);
                                let receiver_id = receiver.id.unwrap_or(0);

                                match state_task.repo.execute_gold_transfer(giver_id, receiver_id, amount).await {
                                    Ok(_) => {
                                        let _ = client_msg.say(
                                            channel_login,
                                            format!("🪙 @{} a donné {} pièces d'or à @{} ! 🤝", username, amount, recipient)
                                        ).await;
                                    }
                                    Err(e) => {
                                        tracing::error!("Failed to execute gold transfer: {:?}", e);
                                        let _ = client_msg.say(
                                            channel_login,
                                            format!("❌ @{}, une erreur est survenue lors du transfert de pièces d'or.", username)
                                        ).await;
                                    }
                                }
                            }
                            GiveArg::ById { catch_id, recipient } => {
                                if recipient == username {
                                    let _ = client_msg.say(channel_login, format!("❌ @{}, tu ne peux pas donner un objet à toi-même.", username)).await;
                                    return;
                                }

                                // Retrieve giver player info
                                let giver = match state_task.repo.get_or_create_player(&username).await {
                                    Ok(g) => g,
                                    Err(e) => {
                                        tracing::error!("Failed to retrieve giver player: {:?}", e);
                                        return;
                                    }
                                };

                                // Retrieve recipient player info
                                let receiver = match state_task.repo.get_player(&recipient).await {
                                    Ok(Some(r)) => r,
                                    Ok(None) => {
                                        let _ = client_msg.say(
                                            channel_login,
                                            format!("❌ @{}, le joueur @{} n'a pas encore de compte de pêche.", username, recipient)
                                        ).await;
                                        return;
                                    }
                                    Err(e) => {
                                        tracing::error!("Failed to retrieve recipient player: {:?}", e);
                                        return;
                                    }
                                };

                                let giver_id = giver.id.unwrap_or(0);
                                let receiver_id = receiver.id.unwrap_or(0);

                                // Check catch ownership
                                match state_task.repo.get_catch_by_id(catch_id).await {
                                    Ok(Some((fish, owner_name))) => {
                                        if owner_name.to_lowercase() != username {
                                            let _ = client_msg.say(
                                                channel_login,
                                                format!("❌ @{}, tu ne possèdes pas la capture #{}.", username, catch_id)
                                            ).await;
                                            return;
                                        }

                                        // Register pending give
                                        let pending = PendingGive {
                                            giver_id,
                                            giver_username: username.clone(),
                                            receiver_id,
                                            receiver_username: recipient.clone(),
                                            catch_id,
                                            catch_name: fish.name.clone(),
                                            created_at: Utc::now(),
                                        };
                                        state_task.pending_gives.write().await.insert(username.clone(), pending);

                                        let _ = client_msg.say(
                                            channel_login,
                                            format!(
                                                "🎁 @{}, tu es sur le point de donner '{}' (#{}) à @{}. Tape !fish give oui pour valider (1 min max) !",
                                                username, fish.name, catch_id, recipient
                                            )
                                        ).await;
                                    }
                                    Ok(None) => {
                                        let _ = client_msg.say(
                                            channel_login,
                                            format!("❌ @{}, la capture #{} est introuvable.", username, catch_id)
                                        ).await;
                                    }
                                    Err(e) => {
                                        tracing::error!("Failed to check catch ownership: {:?}", e);
                                        let _ = client_msg.say(
                                            channel_login,
                                            format!("❌ @{}, impossible de vérifier la capture #{}.", username, catch_id)
                                        ).await;
                                    }
                                }
                            }
                            GiveArg::ByName { name, recipient } => {
                                // Retrieve giver player info
                                let giver = match state_task.repo.get_or_create_player(&username).await {
                                    Ok(g) => g,
                                    Err(e) => {
                                        tracing::error!("Failed to retrieve giver player: {:?}", e);
                                        return;
                                    }
                                };
                                let giver_id = giver.id.unwrap_or(0);

                                // Get giver's inventory to help resolve name/recipient ambiguity
                                let catches = match state_task.repo.get_player_catches(giver_id).await {
                                    Ok(c) => c,
                                    Err(e) => {
                                        tracing::error!("Failed to retrieve player catches: {:?}", e);
                                        return;
                                    }
                                };

                                // Resolve recipient and name using combinations
                                let (chosen_recipient, chosen_item_name) = if raw_msg.contains('@') {
                                    (recipient, name)
                                } else {
                                    // Ambiguity resolution helper
                                    let recip_a = recipient.clone();
                                    let item_a = name.clone();
                                    
                                    // Try B combo
                                    let full_str = format!("{} {}", recipient, name);
                                    let tokens: Vec<&str> = full_str.split_whitespace().collect();
                                    let (recip_b, item_b) = if tokens.len() == 2 {
                                        (tokens[1].to_lowercase(), tokens[0].to_string())
                                    } else {
                                        (tokens.last().unwrap().to_lowercase(), tokens[..tokens.len()-1].join(" "))
                                    };

                                    let player_a = state_task.repo.get_player(&recip_a).await.ok().flatten();
                                    let player_b = state_task.repo.get_player(&recip_b).await.ok().flatten();

                                    let has_item_a = catches.iter().any(|c| c.name.to_lowercase() == item_a.to_lowercase());
                                    let has_item_b = catches.iter().any(|c| c.name.to_lowercase() == item_b.to_lowercase());

                                    if player_b.is_some() && has_item_b && !(player_a.is_some() && has_item_a) {
                                        (recip_b, item_b)
                                    } else if player_a.is_some() && has_item_a && !(player_b.is_some() && has_item_b) {
                                        (recip_a, item_a)
                                    } else if player_b.is_some() && player_a.is_none() {
                                        (recip_b, item_b)
                                    } else if player_a.is_some() && player_b.is_none() {
                                        (recip_a, item_a)
                                    } else {
                                        (recip_a, item_a)
                                    }
                                };

                                if chosen_recipient == username {
                                    let _ = client_msg.say(channel_login, format!("❌ @{}, tu ne peux pas donner un objet à toi-même.", username)).await;
                                    return;
                                }

                                // Retrieve recipient player info
                                let receiver = match state_task.repo.get_player(&chosen_recipient).await {
                                    Ok(Some(r)) => r,
                                    Ok(None) => {
                                        let _ = client_msg.say(
                                            channel_login,
                                            format!("❌ @{}, le joueur @{} n'a pas encore de compte de pêche.", username, chosen_recipient)
                                        ).await;
                                        return;
                                    }
                                    Err(e) => {
                                        tracing::error!("Failed to retrieve recipient player: {:?}", e);
                                        return;
                                    }
                                };
                                let receiver_id = receiver.id.unwrap_or(0);

                                // Find the catch in giver's inventory
                                if let Some(c) = catches.iter().find(|c| c.name.to_lowercase() == chosen_item_name.to_lowercase()) {
                                    let catch_id = c.id.unwrap();
                                    
                                    // Register pending give
                                    let pending = PendingGive {
                                        giver_id,
                                        giver_username: username.clone(),
                                        receiver_id,
                                        receiver_username: chosen_recipient.clone(),
                                        catch_id,
                                        catch_name: c.name.clone(),
                                        created_at: Utc::now(),
                                    };
                                    state_task.pending_gives.write().await.insert(username.clone(), pending);

                                    let _ = client_msg.say(
                                        channel_login,
                                        format!(
                                            "🎁 @{}, tu es sur le point de donner '{}' (#{}) à @{}. Tape !fish give oui pour valider (1 min max) !",
                                            username, c.name, catch_id, chosen_recipient
                                        )
                                    ).await;
                                } else {
                                    let _ = client_msg.say(
                                        channel_login,
                                        format!("❌ @{}, tu ne possèdes aucun '{}' dans ton inventaire.", username, chosen_item_name)
                                    ).await;
                                }
                            }
                        }
                    });
                } else if text.starts_with("!fish reset") || text.starts_with("!peche reset") || text.starts_with("!pêche reset") {
                    let state_task = Arc::clone(&state_clone);
                    let client_msg = client.clone();
                    let channel_login = msg.channel_login.clone();
                    let args: Vec<String> = text.split_whitespace().map(|s| s.to_string()).collect();

                    tokio::spawn(async move {
                        if args.len() >= 3 && args[2] == "all" {
                            state_task.pending_resets_all.write().await.insert(username.clone(), Utc::now());
                            let _ = client_msg.say(channel_login, format!("⚠️ @{}, tape !fish yes all pour confirmer ton reset COMPLET (toutes vos statistiques et inventaire seront supprimés définitivement !).", username)).await;
                        } else if args.len() >= 3 && username == "monsieurcotcot" {
                            let target = args[2].to_lowercase();
                            if let Ok(_) = state_task.repo.reset_player(&target).await {
                                // Clear RAM cache for the target to allow claiming daily reward / resetting rate limit
                                state_task.daily_reward_cache.remove(&target);
                                state_task.rate_limiter.remove(&target);
                                state_task.pending_sales.write().await.remove(&target);
                                state_task.pending_trades.write().await.retain(|t| match t {
                                    PendingTrade::Direct { seller_username, buyer_username, .. } => {
                                        seller_username != &target && buyer_username != &target
                                    }
                                    PendingTrade::Barter { player_a_username, player_b_username, .. } => {
                                        player_a_username != &target && player_b_username != &target
                                    }
                                });
                                let _ = client_msg.say(channel_login, format!("♻️ @{}, l'inventaire de @{} a été réinitialisé par l'administrateur.", username, target)).await;
                            }
                        } else {
                            state_task.pending_resets.write().await.insert(username.clone(), Utc::now());
                            let _ = client_msg.say(channel_login, format!("⚠️ @{}, tape !fish yes pour confirmer ton propre reset de statistiques.", username)).await;
                        }
                    });
                } else if text == "!fish yes all" || text == "!peche yes all" || text == "!pêche yes all" {
                    let state_task = Arc::clone(&state_clone);
                    let client_msg = client.clone();
                    let channel_login = msg.channel_login.clone();
                    tokio::spawn(async move {
                        let mut pending = state_task.pending_resets_all.write().await;
                        if let Some(time) = pending.get(&username) {
                            if Utc::now().signed_duration_since(*time).num_minutes() < 2 {
                                if let Ok(_) = state_task.repo.reset_player_all(&username).await {
                                    // Clear RAM cache for the user to allow claiming daily reward again
                                    state_task.daily_reward_cache.remove(&username);
                                    state_task.rate_limiter.remove(&username);
                                    state_task.pending_sales.write().await.remove(&username);
                                    state_task.pending_trades.write().await.retain(|t| match t {
                                        PendingTrade::Direct { seller_username, buyer_username, .. } => {
                                            seller_username != &username && buyer_username != &username
                                        }
                                        PendingTrade::Barter { player_a_username, player_b_username, .. } => {
                                            player_a_username != &username && player_b_username != &username
                                        }
                                    });
                                    let _ = client_msg.say(channel_login, format!("💥 @{}, reset total réussi ! Toutes vos statistiques et inventaire ont été supprimés.", username)).await;
                                }
                                pending.remove(&username);
                            }
                        }
                    });
                } else if text == "!fish yes" || text == "!peche yes" || text == "!pêche yes" {
                    let state_task = Arc::clone(&state_clone);
                    let client_msg = client.clone();
                    let channel_login = msg.channel_login.clone();
                    tokio::spawn(async move {
                        let mut pending = state_task.pending_resets.write().await;
                        if let Some(time) = pending.get(&username) {
                            if Utc::now().signed_duration_since(*time).num_minutes() < 2 {
                                if let Ok(_) = state_task.repo.reset_player(&username).await {
                                    // Clear RAM cache for the user to allow claiming daily reward again
                                    state_task.daily_reward_cache.remove(&username);
                                    state_task.rate_limiter.remove(&username);
                                    state_task.pending_sales.write().await.remove(&username);
                                    state_task.pending_trades.write().await.retain(|t| match t {
                                        PendingTrade::Direct { seller_username, buyer_username, .. } => {
                                            seller_username != &username && buyer_username != &username
                                        }
                                        PendingTrade::Barter { player_a_username, player_b_username, .. } => {
                                            player_a_username != &username && player_b_username != &username
                                        }
                                    });
                                    let _ = client_msg.say(channel_login, format!("♻️ @{}, reset réussi ! Vos statistiques actives ont été réinitialisées.", username)).await;
                                }
                                pending.remove(&username);
                            }
                        }
                    });
                } else if text.starts_with("!fish simulate ") && username == "monsieurcotcot" {
                    let state_task = Arc::clone(&state_clone);
                    let client_msg = client.clone();
                    let channel_login = msg.channel_login.clone();
                    let args: Vec<String> = text.split_whitespace().map(|s| s.to_string()).collect();

                    if args.len() >= 3 {
                        let target_user = args[2].to_lowercase();
                        let count = if args.len() >= 4 { args[3].parse::<u32>().unwrap_or(10) } else { 10 };

                        tokio::spawn(async move {
                            tracing::info!("[Admin] Simulation de {} lancers pour {}", count, target_user);
                            if let Ok(player) = state_task.repo.get_or_create_player(&target_user).await {
                                if let Some(player_id) = player.id {
                                    match state_task.repo.execute_simulation(player_id, &target_user, count, state_task.use_english).await {
                                        Ok((success_count, junk_count, fail_count, final_level)) => {
                                            let _ = client_msg.say(channel_login, format!("✅ Simulation terminée pour @{} : {} poissons, {} déchets, {} échecs. Niv. {}", target_user, success_count, junk_count, fail_count, final_level)).await;
                                        }
                                        Err(e) => {
                                            tracing::error!("❌ Erreur de simulation pour @{} : {}", target_user, e);
                                            let _ = client_msg.say(channel_login, format!("❌ Erreur de simulation pour @{} : {}", target_user, e)).await;
                                        }
                                    }
                                }
                            }
                        });
                    }
                } else if text == "!admin backup" && username == "monsieurcotcot" {
                    let state_task = Arc::clone(&state_clone);
                    let client_msg = client.clone();
                    let channel_login = msg.channel_login.clone();
                    tokio::spawn(async move {
                        if let Ok(players) = state_task.repo.get_all_players().await {
                            let backups: Vec<crate::db::PlayerBackup> = players.into_iter().map(|p| crate::db::PlayerBackup {
                                username: p.username,
                                total_attempts: p.total_attempts,
                                successful_attempts: p.successful_attempts,
                                failed_attempts: p.failed_attempts,
                                level: p.level,
                                xp: p.xp,
                                vip_until: p.vip_until,
                                gold: Some(p.gold),
                                eco_notoriety: Some(p.eco_notoriety),
                                millionaire_at: p.millionaire_at,
                            }).collect();
                            
                            if let Ok(json) = serde_json::to_string_pretty(&backups) {
                                if let Ok(_) = tokio::fs::write("data/players_backup.json", json).await {
                                    let _ = client_msg.say(channel_login, format!("💾 [Admin] Backup forcé : {} joueurs sauvegardés.", backups.len())).await;
                                }
                            }
                        }
                    });
                } else if text == "!admin restore" && username == "monsieurcotcot" {
                    let state_task = Arc::clone(&state_clone);
                    let client_msg = client.clone();
                    let channel_login = msg.channel_login.clone();
                    tokio::spawn(async move {
                        if let Ok(data) = tokio::fs::read_to_string("data/players_backup.json").await {
                            if let Ok(backups) = serde_json::from_str::<Vec<crate::db::PlayerBackup>>(&data) {
                                let _ = client_msg.say(channel_login.clone(), format!("⚠️ [Admin] Restauration forcée de {} joueurs en cours...", backups.len())).await;
                                for backup in backups {
                                    if let Ok(player_id) = state_task.repo.restore_player(&backup).await {
                                        let mut success_count = 0;
                                        let mut fail_count = 0;
                                        for _ in 0..backup.total_attempts {
                                            let success_chance = 0.35 + (backup.level as f64 - 1.0) * (0.20 / 199.0);
                                            let junk_chance = 0.05;
                                            let r: f64 = rand::random::<f64>();
                                            if r < success_chance {
                                                if let Some(fish) = generate_fish(state_task.use_english) {
                                                    let _ = state_task.repo.save_catch_only(player_id, fish, Some(&backup.username)).await;
                                                    success_count += 1;
                                                }
                                            } else if r < (success_chance + junk_chance) {
                                                if let Some(junk) = generate_junk(state_task.use_english) {
                                                    let _ = state_task.repo.save_catch_only(player_id, junk, Some(&backup.username)).await;
                                                    success_count += 1;
                                                }
                                            } else {
                                                fail_count += 1;
                                            }
                                        }
                                        let _ = state_task.repo.update_player_stats_after_restore(player_id, success_count, fail_count).await;
                                    }
                                }
                                // Clear RAM caches after full restore to avoid stale caches
                                state_task.daily_reward_cache.clear();
                                state_task.rate_limiter.clear();
                                state_task.pending_sales.write().await.clear();
                                state_task.pending_trades.write().await.clear();
                                state_task.pending_resets.write().await.clear();
                                state_task.pending_resets_all.write().await.clear();
                                let _ = client_msg.say(channel_login, "✅ [Admin] Restauration terminée.".to_string()).await;
                            }
                        }
                    });
                } else if text == "!fish purge" && username == "monsieurcotcot" {
                    let state_task = Arc::clone(&state_clone);
                    let client_msg = client.clone();
                    let channel_login = msg.channel_login.clone();
                    tokio::spawn(async move {
                        state_task.pending_purges.write().await.insert(username.clone(), Utc::now());
                        let _ = client_msg.say(channel_login, "⚠️ @monsieurcotcot, es-tu SÛR de vouloir supprimer TOUTES les données de la base de données ? Tape !fish purge yes dans les 2 prochaines minutes pour confirmer.".to_string()).await;
                    });
                } else if text == "!fish purge yes" && username == "monsieurcotcot" {
                    let state_task = Arc::clone(&state_clone);
                    let client_msg = client.clone();
                    let channel_login = msg.channel_login.clone();
                    tokio::spawn(async move {
                        let mut pending = state_task.pending_purges.write().await;
                        if let Some(time) = pending.get(&username) {
                            if Utc::now().signed_duration_since(*time).num_minutes() < 2 {
                                if let Ok(_) = state_task.repo.purge_all_data().await {
                                    // Clear all RAM caches to allow first-time daily rewards and remove stale states
                                    state_task.daily_reward_cache.clear();
                                    state_task.rate_limiter.clear();
                                    state_task.pending_sales.write().await.clear();
                                    state_task.pending_trades.write().await.clear();
                                    state_task.pending_resets.write().await.clear();
                                    state_task.pending_resets_all.write().await.clear();
                                    let _ = client_msg.say(channel_login, "🧹 [Admin] Base de données purgée ! Toutes les données ont été supprimées.".to_string()).await;
                                } else {
                                    let _ = client_msg.say(channel_login, "❌ [Admin] Une erreur est survenue lors de la purge de la base de données.".to_string()).await;
                                }
                                pending.remove(&username);
                            }
                        } else {
                            let _ = client_msg.say(channel_login, "⚠️ @monsieurcotcot, aucune purge en attente. Tape !fish purge d'abord.".to_string()).await;
                        }
                    });
                } else if text == "!fish" || text == "!peche" || text == "!pêche" || (text == "!fish testvip" && (username == "monsieurcotcot" || username == "ze_fisherman" || username == "ze_tester")) {
                    let state_task = Arc::clone(&state_clone);
                    let client_msg = client.clone();
                    let channel_login = msg.channel_login.clone();
                    let is_test = text == "!fish testvip";

                    tokio::spawn(async move {
                        if let Ok(mut player) = state_task.repo.get_or_create_player(&username).await {
                            let is_admin = username == "monsieurcotcot";
                            let is_bypass_user = username == "monsieurcotcot" || username == "ze_fisherman" || username == "ze_tester";
                            
                            let top_eco_username = match state_task.repo.get_top_eco_player().await {
                                Ok(Some(p)) => Some(p.username.to_lowercase()),
                                _ => None,
                            };
                            let is_top_eco = top_eco_username.as_ref() == Some(&player.username.to_lowercase());
                            let base_cooldown = player.get_current_cooldown();
                            let current_cooldown = if is_top_eco {
                                (base_cooldown - 10).max(0)
                            } else {
                                base_cooldown
                            };

                            // Détermination de la langue de retour :
                            // 1. Préférence utilisateur en base si définie.
                            // 2. Sinon, anglais par défaut pour les commandes !fish, français pour !peche/!pêche.
                            let use_english = match &player.language {
                                Some(lang) => lang == "en",
                                None => text.starts_with("!fish"),
                            };

                            // Vérification du statut live du stream (sauf pour l'admin ou en mode test)
                            let is_live = is_stream_online(&state_task).await;
                            if is_live {
                                let mut att = state_task.offline_attempts.write().await;
                                att.remove(&username);
                            }
                            let is_bypassed = {
                                let bypassed = state_task.offline_bypassed.read().await;
                                bypassed.contains(&username)
                            };

                            if !is_live && !is_bypassed && !is_bypass_user && !is_test {
                                let attempts;
                                {
                                    let mut att = state_task.offline_attempts.write().await;
                                    let entry = att.entry(username.clone()).or_insert(0);
                                    *entry += 1;
                                    attempts = *entry;
                                }

                                if attempts >= 3 {
                                    let msg_str = if use_english {
                                        format!("🤐 @{}, you really insist... Give me the secret password to unlock offline fishing!", username)
                                    } else {
                                        format!("🤐 @{}, tu insistes vraiment... Donne-moi le mot de passe secret pour déverrouiller la pêche hors-ligne !", username)
                                    };
                                    let _ = client_msg.say(channel_login, msg_str).await;
                                } else {
                                    let phrases = get_offline_phrases(use_english);
                                    let phrase = phrases.choose(&mut rand::thread_rng()).unwrap_or(&"Offline!");
                                    let _ = client_msg.say(channel_login, format!("@{}, {}", username, phrase)).await;
                                }
                                return;
                            }

                            // Vérification du coût en or (10 po requis)
                            if player.gold < 10 && !is_admin && !is_test {
                                let msg_str = if use_english {
                                    format!(
                                        "⚠️ @{}, you don't have enough gold coins to fish (requires: 10 gold, you have {} gold). Chat in the stream tomorrow to claim your daily bonus or sell fish via !fish sell!",
                                        username, player.gold
                                    )
                                } else {
                                    format!(
                                        "⚠️ @{}, tu n'as pas assez de pièces d'or pour pêcher (requis: 10 po, tu as {} po). Écris un message sur le live demain pour obtenir ton bonus quotidien ou vends des poissons via !fish sell !",
                                        username, player.gold
                                    )
                                };
                                let _ = client_msg.say(channel_login, msg_str).await;
                                return;
                            }

                            let can_fish = match player.last_fishing_time {
                                Some(last_time) => {
                                    let now = Utc::now();
                                    let diff = now.signed_duration_since(last_time).num_seconds();
                                    diff >= current_cooldown
                                }
                                None => true,
                            };

                            if can_fish || is_test || is_admin {
                                let today = Utc::now().date_naive();
                                let (junk_caught, junk_target) = state_task.repo
                                    .get_or_update_daily_junk_event(today)
                                    .await
                                    .unwrap_or((0, 0));

                                let repo_attempts = state_task.repo.clone();
                                tokio::spawn(async move {
                                    let _ = repo_attempts.increment_daily_attempts(today).await;
                                });

                                let is_event_active = junk_caught < junk_target;
                                let level_factor = (player.level as f64 - 1.0) / 199.0;
                                let success_rate = 0.35 + (level_factor * 0.20);
                                let junk_rate = if is_event_active { 0.20 } else { 0.05 };
                                let roll = rand::random::<f64>();

                                if is_test || roll < success_rate {
                                    let fish = if is_test { crate::models::Fish::new("Gemme VIP (TEST)".to_string(), crate::config::Rarity::Legendary, 1.0, 100.0, "pristine".to_string(), "Gemme de test.".to_string()) } 
                                                   else { match generate_fish(use_english) { Some(f) => f, None => return } };

                                    let leveled_up = player.add_xp(25);
                                    if fish.name == "Gemme VIP" || is_test {
                                        let mins = if is_test { 1 } else { match fish.state.as_str() { "badly damaged" => 20, "damaged" => 40, "worn" => 60, "good" => 80, "pristine" => 240, _ => 20 } };
                                        player.vip_until = Some(Utc::now() + chrono::Duration::minutes(mins));
                                        let auth_vip = Arc::clone(&state_task.auth);
                                        let ch_vip = channel_login.clone();
                                        let u_vip = username.clone();
                                        tokio::spawn(async move {
                                            if let Some(mut t) = auth_vip.load_streamer_tokens() {
                                                if t.expires_at < Utc::now() {
                                                    if let Ok(new_t) = auth_vip.refresh_tokens(&t.refresh_token).await {
                                                        let _ = auth_vip.save_streamer_tokens(&new_t);
                                                        t = new_t;
                                                    }
                                                }
                                                if let (Ok(b), Ok(u)) = (auth_vip.get_user_id(&t.access_token, &ch_vip).await, auth_vip.get_user_id(&t.access_token, &u_vip).await) {
                                                    let _ = auth_vip.add_vip(&b, &u, &t.access_token).await;
                                                }
                                            }
                                        });
                                    }
                                     
                                     let catch_id_opt = match state_task.repo.save_attempt(&player, true, Some(fish.clone())).await {
                                         Ok(id) => id,
                                         Err(e) => {
                                             tracing::error!("Failed to save catch: {}", e);
                                             None
                                         }
                                     };

                                     let mut resp = if use_english {
                                         format!("🐟 @{} caught a {} ({} cm)! {}", username, fish.name, fish.size, fish.description)
                                     } else {
                                         format!("🐟 @{} a pêché un(e) {} ({} cm) ! {}", username, fish.name, fish.size, fish.description)
                                     };

                                     if use_english {
                                         resp.push_str(&format!(" (Rarity: {:?})", fish.rarity));
                                     } else {
                                         resp.push_str(&format!(" (Rareté: {:?})", fish.rarity));
                                     }

                                     if let Some(id) = catch_id_opt {
                                         resp.push_str(&format!(" (ID: #{})", id));
                                     }

                                     if fish.name == "Gemme VIP" || is_test { 
                                         let d = if is_test { "1 MIN" } else { match fish.state.as_str() { "pristine" => "4H", "good" => "80 MIN", "worn" => "60 MIN", "damaged" => "40 MIN", _ => "20 MIN" } };
                                         if use_english {
                                             resp.push_str(&format!(" 🌟 YOU ARE VIP FOR {}! 🌟", d));
                                         } else {
                                             resp.push_str(&format!(" 🌟 TU ES VIP PENDANT {} ! 🌟", d));
                                         }
                                     }
                                     if fish.name == "Pristine Banana 1" || fish.name == "Pristine Banana 2" {
                                         if let Some(player_id) = player.id {
                                             if let Ok(stolen_from) = state_task.repo.check_and_execute_banana_theft(player_id, &fish.name).await {
                                                 if let Some(old_user) = stolen_from {
                                                     if use_english {
                                                         resp.push_str(&format!(" 🍌 @{} STOLE the {} from @{}! 🤫", username, fish.name, old_user));
                                                     } else {
                                                         resp.push_str(&format!(" 🍌 @{} a VOLÉ la {} à @{} ! 🤫", username, fish.name, old_user));
                                                     }
                                                 }
                                             }
                                             let other_banana = if fish.name == "Pristine Banana 1" { "Pristine Banana 2" } else { "Pristine Banana 1" };
                                             if let Ok(has_other) = state_task.repo.has_caught_banana(player_id, other_banana).await {
                                                 if has_other {
                                                     if let Ok(already_king) = state_task.repo.is_active_king(player_id).await {
                                                         if !already_king {
                                                             if use_english {
                                                                 resp.push_str(&format!(" 👑 @{} becomes the new BANANA KING! 👑", username));
                                                             } else {
                                                                 resp.push_str(&format!(" 👑 @{} devient le nouveau ROI DES BANANES ! 👑", username));
                                                             }
                                                         }
                                                     }
                                                 }
                                             }
                                         }
                                     }
                                     if leveled_up {
                                         if use_english {
                                             resp.push_str(&format!(" ✨ LEVEL UP! Lvl. {}!", player.level));
                                         } else {
                                             resp.push_str(&format!(" ✨ LEVEL UP ! Niv. {} !", player.level));
                                         }
                                     }
                                     resp.push_str(&format!(" ⏳{}s", current_cooldown));
                                     let _ = client_msg.say(channel_login.clone(), resp).await;

                                     if let Some(catch_id) = catch_id_opt {
                                         let state_bg = state_task.clone();
                                         let ch_bg = channel_login.clone();
                                         tokio::spawn(async move {
                                             if let Some(t) = state_bg.auth.load_tokens() {
                                                 if let Some(title) = state_bg.auth.get_stream_info(&ch_bg, &t.access_token).await {
                                                     let _ = state_bg.repo.update_catch_stream_title(catch_id, &title).await;
                                                 }
                                             }
                                         });
                                     }
                                } else if roll < success_rate + junk_rate {
                                    if let Some(junk) = generate_junk(use_english) {
                                        let catch_id_opt = match state_task.repo.save_attempt(&player, true, Some(junk.clone())).await {
                                            Ok(id) => id,
                                            Err(e) => {
                                                tracing::error!("Failed to save junk catch: {}", e);
                                                None
                                            }
                                        };

                                        let leveled_up = player.add_xp(5);
                                        let (new_caught, target) = state_task.repo
                                            .increment_daily_junk_caught(today)
                                            .await
                                            .unwrap_or((junk_caught + 1, junk_target));

                                        let mut resp = if use_english {
                                            format!("🗑️ @{} reeled in some trash: {}! {}", username, junk.name, junk.description)
                                        } else {
                                            format!("🗑️ @{} a remonté un déchet : {} ! {}", username, junk.name, junk.description)
                                        };

                                        if use_english {
                                            resp.push_str(&format!(" (Rarity: {:?})", junk.rarity));
                                        } else {
                                            resp.push_str(&format!(" (Rareté: {:?})", junk.rarity));
                                        }

                                        if let Some(id) = catch_id_opt {
                                            resp.push_str(&format!(" (ID: #{})", id));
                                        }

                                        if is_event_active {
                                            if use_english {
                                                resp.push_str(&format!(" (Junk Event: {}/{})", new_caught, target));
                                            } else {
                                                resp.push_str(&format!(" (Événement Déchets : {}/{})", new_caught, target));
                                            }
                                        }

                                        if leveled_up {
                                            if use_english {
                                                resp.push_str(&format!(" ✨ LEVEL UP! Lvl. {}!", player.level));
                                            } else {
                                                resp.push_str(&format!(" ✨ LEVEL UP ! Niv. {} !", player.level));
                                            }
                                        }
                                        resp.push_str(&format!(" ⏳{}s", current_cooldown));
                                        let _ = client_msg.say(channel_login.clone(), resp).await;

                                        if let Some(catch_id) = catch_id_opt {
                                            let state_bg = state_task.clone();
                                            let ch_bg = channel_login.clone();
                                            tokio::spawn(async move {
                                                if let Some(t) = state_bg.auth.load_tokens() {
                                                    if let Some(title) = state_bg.auth.get_stream_info(&ch_bg, &t.access_token).await {
                                                        let _ = state_bg.repo.update_catch_stream_title(catch_id, &title).await;
                                                    }
                                                }
                                            });
                                        }
                                    }
                                } else {
                                    let reasons = get_fail_attempt_reasons(use_english);
                                    let default_entry = FailMessageEntry::Simple("Pas de chance !".to_string());
                                    let entry = reasons.choose(&mut rand::thread_rng()).cloned().unwrap_or(default_entry);

                                    let is_king = state_task.repo.is_banana_king(player.id.unwrap()).await.unwrap_or(false);

                                    let (text, gold_pen, mut cooldown_pen) = match &entry {
                                        FailMessageEntry::Simple(t) => (t.clone(), 0, 0),
                                        FailMessageEntry::Detailed(det) => {
                                            (det.text.clone(), det.gold_penalty.unwrap_or(0), det.cooldown_penalty.unwrap_or(0))
                                        }
                                    };

                                    let original_cooldown_pen = cooldown_pen;
                                    if is_king {
                                        cooldown_pen = 0;
                                    }

                                    let formatted_text = text.replace("#viewer_name#", &username);
                                    let leveled_up = player.add_xp(5);
                                    let mut resp = formatted_text;

                                    if is_king && original_cooldown_pen > 0 {
                                        if use_english {
                                            resp.push_str(" 👑 (Banana King Immunity: Cooldown penalty bypassed!)");
                                        } else {
                                            resp.push_str(" 👑 (Immunité du Roi Banane : Malus de cooldown annulé !)");
                                        }
                                    }

                                    if leveled_up {
                                        if use_english {
                                            resp.push_str(&format!(" ✨ LEVEL UP! Lvl. {}!", player.level));
                                        } else {
                                            resp.push_str(&format!(" ✨ LEVEL UP ! Niv. {} !", player.level));
                                        }
                                    }
                                    resp.push_str(&format!(" ⏳{}s", current_cooldown + cooldown_pen));
                                    let _ = client_msg.say(channel_login, resp).await;
                                    let _ = state_task.repo.save_attempt(&player, false, None).await;

                                    if gold_pen > 0 || cooldown_pen > 0 {
                                        if let Some(pid) = player.id {
                                            let _ = state_task.repo.apply_extra_fail_penalty(pid, gold_pen, cooldown_pen).await;
                                        }
                                    }
                                }
                            } else {
                                let rem = match player.last_fishing_time {
                                    Some(last_time) => {
                                        let now = Utc::now();
                                        let diff = now.signed_duration_since(last_time).num_seconds();
                                        if diff >= current_cooldown { 0 } else { current_cooldown - diff }
                                    }
                                    None => 0,
                                };
                                if let Some(id) = player.id { let _ = state_task.repo.add_cooldown_penalty(id).await; }
                                let msg_str = if use_english {
                                    format!("⏳ @{}, wait another {}s! (+20s and -20 gold penalty) ⚠️", username, rem + 20)
                                } else {
                                    format!("⏳ @{}, attends encore {}s ! (+20s et pénalité de 20 gold) ⚠️", username, rem + 20)
                                };
                                let _ = client_msg.say(channel_login, msg_str).await;
                            }
                        }
                    });
                }
            }
        }
    });
    *abort_lock = Some(handle);
}

pub async fn is_stream_online(state: &AppState) -> bool {
    let now = Utc::now();
    
    // Check cache
    {
        let cache = state.stream_live_cache.read().await;
        if let Some((is_live, last_checked)) = *cache {
            if now.signed_duration_since(last_checked).num_seconds() < 30 {
                return is_live;
            }
        }
    }
    
    // Cache expired or empty, fetch from Twitch API
    let tokens_opt = state.auth.load_tokens();
    let is_live = if let Some(t) = tokens_opt {
        if state.auth.get_stream_info(&state.channel, &t.access_token).await.is_some() {
            true
        } else {
            // It could be that the stream is offline, OR the token is invalid.
            // Let's validate the token.
            if !state.auth.validate_token(&t.access_token).await {
                tracing::warn!("⚠️ [Auth] Bot token is invalid during stream status check. Attempting refresh...");
                match state.auth.refresh_tokens(&t.refresh_token).await {
                    Ok(new_t) => {
                        let _ = state.auth.save_tokens(&new_t);
                        // Retry stream check with new token
                        state.auth.get_stream_info(&state.channel, &new_t.access_token).await.is_some()
                    }
                    Err(e) => {
                        tracing::error!("❌ [Auth] Failed to refresh bot tokens: {}", e);
                        false
                    }
                }
            } else {
                // Token is valid, so the stream is actually offline!
                false
            }
        }
    } else {
        false
    };
    
    // Write cache
    {
        let mut cache = state.stream_live_cache.write().await;
        *cache = Some((is_live, now));
    }
    
    if is_live {
        let repo = Arc::clone(&state.repo);
        let today = now.date_naive();
        tokio::spawn(async move {
            let _ = repo.record_stream_live_date(today).await;
        });
    }
    
    is_live
}

fn get_offline_phrases(use_english: bool) -> Vec<&'static str> {
    if use_english {
        vec![
            "Hey you! monsieurcotcot's stream is offline, what on earth are you doing here? 🧐",
            "Shh... The fish are sleeping, and so is the streamer. Come back when we are live! 🤫",
            "You are fishing in complete darkness, friend. The stream is turned off! 🌑",
            "No stream, no fish. That's the law of nature (and Twitch). 🚫",
            "The lake is closed for nightly maintenance. Move along! 🚧",
            "Uh, you are talking to a bot. The boss has gone to sleep. 🤖",
            "Offline fishing? That's like trying to catch the wind with a sieve. 💨",
            "Intrusion alert! A clandestine fisherman spotted on the offline channel! 🚨",
            "Even the seasonal fish refuses to bite if monsieurcotcot isn't here to cast. 🎤",
            "Sorry, the fishing customs office is closed. Go home! 🛂",
            "Hey! Trying to sneak a catch while nobody is watching? 😉",
            "The pufferfish is staring at you with judging eyes. 🐡",
            "There is no audience to applaud your divine catch. What's the point? 🤷",
            "The gamekeeper kikettebot is patrolling... Watch out or you might get banned! 👮",
            "The hooks have been locked in the garage. Come back later! 🔑",
            "Offline fishing? You've got big plans, but it's a no. ❌",
            "The fish took advantage of the streamer's absence to host a pool party. 🏖️",
            "Say, shouldn't you be doing homework instead of spamming an offline channel? 📚",
            "The water is freezing and the chat is empty. Nothing will bite here. ❄️",
            "Hey there! Poaching is severely punished by the poney law. 🐴",
            "But... are you all alone in the dark on the lake shore? That's a bit creepy. 👻",
            "The fish have migrated to another server while waiting for the stream. 💻",
            "Halt! Fishing without an audience is a poetic offense. 🎭",
            "The stream's water level is at 0%. Refilling next stream! 🚰",
            "Patience is a virtue, but fishing offline is just a waste of bait. 🪱",
            "A wild error appeared: Stream is 404. Fish not found! 🔌",
            "Are you trying to level up in secret? We see you! 👀",
            "The bait has gone on strike until monsieurcotcot starts the live. ✊",
            "Go grab a coffee, watch a replay, and wait for the live! ☕",
            "No live, no glory! Reconnect when the green light is on. 🟢",
        ]
    } else {
        vec![
            "Hey oh toi là ! Le stream de monsieurcotcot n'est pas en ligne, qu'est-ce que tu fous là ? 🧐",
            "Chut... Les poissons dorment et le streameur aussi. Reviens quand ce sera en ligne ! 🤫",
            "Tu pêches dans le noir complet là, l'ami. Le live est éteint ! 🌑",
            "Pas de live, pas de poissons. C'est la loi de la nature (et de Twitch). 🚫",
            "Le lac est fermé pour maintenance nocturne. Circulez ! 🚧",
            "Euh, tu parles à un bot là. Le patron est parti dormir. 🤖",
            "Pêcher hors-live ? C'est comme essayer de chasser le vent avec une passoire. 💨",
            "Alerte intrusion ! Un pêcheur clandestin repéré sur le canal hors-ligne ! 🚨",
            "Même le poisson de la saison refuse de mordre si monsieurcotcot n'est pas là pour commenter. 🎤",
            "Désolé, la douane de la pêche est fermée. Rentrez chez vous ! 🛂",
            "Hé ! Tu essaies de resquiller pendant que personne ne regarde ? 😉",
            "Le poisson-globe t'observe avec un air de jugement désapprobateur. 🐡",
            "Il n'y a pas de spectateurs pour applaudir ta prise divine. À quoi bon ? 🤷",
            "Le garde-pêche kikettebot rôde... Fais gaffe à ne pas te faire ban ! 👮",
            "Les hameçons ont été rangés au garage. Revenez plus tard ! 🔑",
            "Pêcher hors-ligne ? Tu as de grands projets, toi. Mais c'est non. ❌",
            "Les poissons ont profité de l'absence du streameur pour organiser une pool party. 🏖️",
            "Dis donc, tu n'aurais pas des devoirs à faire au lieu de spammer un canal éteint ? 📚",
            "L'eau est gelée et le chat est désert. Rien ne mordra ici. ❄️",
            "Hé ho ! La pêche clandestine est sévèrement punie par la loi du poney. 🐴",
            "Mais... tu es tout seul dans le noir sur la rive du lac ? C'est un peu flippant. 👻",
            "Les poissons ont migré vers un autre serveur en attendant le début du live. 💻",
            "Halte-là ! La pêche sans public est un délit poétique. 🎭",
            "Le niveau d'eau du live est à 0%. Remplissage au prochain stream ! 🚰",
            "La patience est une vertu, mais pêcher hors-live est juste un gaspillage d'appât. 🪱",
            "Une erreur sauvage est apparue : Stream est 404. Poissons introuvables ! 🔌",
            "Tu essaies de monter de niveau en cachette ? On te voit ! 👀",
            "Les appâts se sont mis en grève en attendant que monsieurcotcot lance le live. ✊",
            "Va prendre un café, regarde un replay, et attends le direct ! ☕",
            "Pas de live, pas de gloire ! Reconnecte-toi quand le voyant est vert. 🟢",
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sell_args_confirmations() {
        assert_eq!(parse_sell_args("oui"), Some(SellArg::ConfirmYes));
        assert_eq!(parse_sell_args("YES"), Some(SellArg::ConfirmYes));
        assert_eq!(parse_sell_args("y"), Some(SellArg::ConfirmYes));
        assert_eq!(parse_sell_args("non"), Some(SellArg::ConfirmNo));
        assert_eq!(parse_sell_args("NO"), Some(SellArg::ConfirmNo));
        assert_eq!(parse_sell_args("n"), Some(SellArg::ConfirmNo));
        assert_eq!(parse_sell_args("all"), Some(SellArg::All { rarity: None, state: None }));
        assert_eq!(parse_sell_args("TOUT"), Some(SellArg::All { rarity: None, state: None }));
        assert_eq!(parse_sell_args("all common"), Some(SellArg::All { rarity: Some(Rarity::Common), state: None }));
        assert_eq!(parse_sell_args("all badly damaged"), Some(SellArg::All { rarity: None, state: Some("badly damaged".to_string()) }));
        assert_eq!(parse_sell_args("all rare pristine"), Some(SellArg::All { rarity: Some(Rarity::Rare), state: Some("pristine".to_string()) }));
    }

    #[test]
    fn test_parse_sell_args_by_id() {
        assert_eq!(parse_sell_args("#123"), Some(SellArg::ById(123)));
        assert_eq!(parse_sell_args("#999"), Some(SellArg::ById(999)));
        assert_eq!(parse_sell_args("#abc"), None); // invalide
    }

    #[test]
    fn test_parse_sell_args_by_name_states_and_quantities() {
        // Cas classique : Nom simple
        assert_eq!(
            parse_sell_args("Bar"),
            Some(SellArg::ByName { name: "Bar".to_string(), state: None, quantity: 1 })
        );

        // Nom avec espace
        assert_eq!(
            parse_sell_args("Grand requin blanc"),
            Some(SellArg::ByName { name: "Grand requin blanc".to_string(), state: None, quantity: 1 })
        );

        // Nom + quantité
        assert_eq!(
            parse_sell_args("Bar 3"),
            Some(SellArg::ByName { name: "Bar".to_string(), state: None, quantity: 3 })
        );

        // Nom + état
        assert_eq!(
            parse_sell_args("Bar pristine"),
            Some(SellArg::ByName { name: "Bar".to_string(), state: Some("pristine".to_string()), quantity: 1 })
        );

        // Nom + état composé
        assert_eq!(
            parse_sell_args("Bar badly damaged"),
            Some(SellArg::ByName { name: "Bar".to_string(), state: Some("badly damaged".to_string()), quantity: 1 })
        );
        assert_eq!(
            parse_sell_args("Bar très endommagé"),
            Some(SellArg::ByName { name: "Bar".to_string(), state: Some("badly damaged".to_string()), quantity: 1 })
        );

        // Ordre flexible : Nom + état + quantité
        assert_eq!(
            parse_sell_args("Bar pristine 5"),
            Some(SellArg::ByName { name: "Bar".to_string(), state: Some("pristine".to_string()), quantity: 5 })
        );

        // Ordre flexible : Nom + quantité + état
        assert_eq!(
            parse_sell_args("Bar 5 pristine"),
            Some(SellArg::ByName { name: "Bar".to_string(), state: Some("pristine".to_string()), quantity: 5 })
        );

        // Ordre flexible avec état composé et quantité
        assert_eq!(
            parse_sell_args("Grand requin blanc 3 badly damaged"),
            Some(SellArg::ByName { name: "Grand requin blanc".to_string(), state: Some("badly damaged".to_string()), quantity: 3 })
        );
        assert_eq!(
            parse_sell_args("Grand requin blanc badly damaged 3"),
            Some(SellArg::ByName { name: "Grand requin blanc".to_string(), state: Some("badly damaged".to_string()), quantity: 3 })
        );
    }

    #[test]
    fn test_parse_sell_args_special_banana() {
        assert_eq!(
            parse_sell_args("pristine banana 1"),
            Some(SellArg::ByName { name: "Pristine Banana 1".to_string(), state: None, quantity: 1 })
        );
        assert_eq!(
            parse_sell_args("pristine banana 2"),
            Some(SellArg::ByName { name: "Pristine Banana 2".to_string(), state: None, quantity: 1 })
        );
        // Cas où la banane a aussi un état supplémentaire
        assert_eq!(
            parse_sell_args("pristine banana 1 pristine"),
            Some(SellArg::ByName { name: "Pristine Banana 1".to_string(), state: Some("pristine".to_string()), quantity: 1 })
        );
    }

    #[test]
    fn test_parse_trade_args() {
        // Confirmations
        assert_eq!(parse_trade_args("oui"), Some(TradeArg::Accept));
        assert_eq!(parse_trade_args("accept"), Some(TradeArg::Accept));
        assert_eq!(parse_trade_args("non"), Some(TradeArg::Cancel));
        assert_eq!(parse_trade_args("cancel"), Some(TradeArg::Cancel));

        // Troc (Barter)
        assert_eq!(
            parse_trade_args("#123 @monsieurcotcot"),
            Some(TradeArg::Barter { catch_id: 123, recipient: "monsieurcotcot".to_string() })
        );
        // Sans l'arobase
        assert_eq!(
            parse_trade_args("#123 monsieurcotcot"),
            Some(TradeArg::Barter { catch_id: 123, recipient: "monsieurcotcot".to_string() })
        );

        // Vente directe (Direct)
        assert_eq!(
            parse_trade_args("#123 500 @monsieurcotcot"),
            Some(TradeArg::Direct { catch_id: 123, price: 500, recipient: "monsieurcotcot".to_string() })
        );
        // Avec espaces superflus
        assert_eq!(
            parse_trade_args("#123  500  @MonsieurCotCot"),
            Some(TradeArg::Direct { catch_id: 123, price: 500, recipient: "monsieurcotcot".to_string() })
        );
    }

    #[test]
    fn test_parse_give_args() {
        assert_eq!(parse_give_args("100 @monsieurcotcot"), Some(GiveArg::Gold { amount: 100, recipient: "monsieurcotcot".to_string() }));
        assert_eq!(parse_give_args("@MonsieurCotCot 250"), Some(GiveArg::Gold { amount: 250, recipient: "monsieurcotcot".to_string() }));
        assert_eq!(parse_give_args("50 monsieurcotcot"), Some(GiveArg::Gold { amount: 50, recipient: "monsieurcotcot".to_string() }));
        assert_eq!(parse_give_args("monsieurcotcot 500"), Some(GiveArg::Gold { amount: 500, recipient: "monsieurcotcot".to_string() }));
        assert_eq!(parse_give_args("100"), None);
        assert_eq!(parse_give_args("abc @monsieurcotcot"), Some(GiveArg::ByName { name: "abc".to_string(), recipient: "monsieurcotcot".to_string() }));
        assert_eq!(parse_give_args(""), None);

        assert_eq!(parse_give_args("#42 @monsieurcotcot"), Some(GiveArg::ById { catch_id: 42, recipient: "monsieurcotcot".to_string() }));
        assert_eq!(parse_give_args("@monsieurcotcot #42"), Some(GiveArg::ById { catch_id: 42, recipient: "monsieurcotcot".to_string() }));
        assert_eq!(parse_give_args("Sardine @monsieurcotcot"), Some(GiveArg::ByName { name: "Sardine".to_string(), recipient: "monsieurcotcot".to_string() }));
        assert_eq!(parse_give_args("oui"), Some(GiveArg::ConfirmYes));
        assert_eq!(parse_give_args("non"), Some(GiveArg::ConfirmNo));
    }

    #[test]
    fn test_offline_phrases_count() {
        let fr_phrases = get_offline_phrases(false);
        let en_phrases = get_offline_phrases(true);
        assert_eq!(fr_phrases.len(), 30, "French phrases count must be exactly 30");
        assert_eq!(en_phrases.len(), 30, "English phrases count must be exactly 30");

        // Verify that some phrases contain monsieurcotcot
        assert!(fr_phrases[0].contains("monsieurcotcot"));
        assert!(en_phrases[0].contains("monsieurcotcot"));
    }

    #[test]
    fn test_bypass_users() {
        let bypass_users = vec!["monsieurcotcot", "ze_fisherman", "ze_tester"];
        for u in bypass_users {
            let is_bypass = u == "monsieurcotcot" || u == "ze_fisherman" || u == "ze_tester";
            assert!(is_bypass);
        }
        let normal_user = "someone_else";
        let is_bypass = normal_user == "monsieurcotcot" || normal_user == "ze_fisherman" || normal_user == "ze_tester";
        assert!(!is_bypass);
    }

    #[test]
    fn test_recycle_helpers() {
        assert_eq!(normalize_bin_name("bleu"), Some("bleu"));
        assert_eq!(normalize_bin_name("bleue"), Some("bleu"));
        assert_eq!(normalize_bin_name("BLUE"), Some("bleu"));
        assert_eq!(normalize_bin_name("jaune"), Some("jaune"));
        assert_eq!(normalize_bin_name("yellow"), Some("jaune"));
        assert_eq!(normalize_bin_name("vert"), Some("vert"));
        assert_eq!(normalize_bin_name("verte"), Some("vert"));
        assert_eq!(normalize_bin_name("green"), Some("vert"));
        assert_eq!(normalize_bin_name("marron"), Some("marron"));
        assert_eq!(normalize_bin_name("brown"), Some("marron"));
        assert_eq!(normalize_bin_name("gris"), Some("gris"));
        assert_eq!(normalize_bin_name("grise"), Some("gris"));
        assert_eq!(normalize_bin_name("grey"), Some("gris"));
        assert_eq!(normalize_bin_name("gray"), Some("gris"));
        assert_eq!(normalize_bin_name("noir"), Some("noir"));
        assert_eq!(normalize_bin_name("noire"), Some("noir"));
        assert_eq!(normalize_bin_name("black"), Some("noir"));
        assert_eq!(normalize_bin_name("decharge"), Some("decharge"));
        assert_eq!(normalize_bin_name("décharge"), Some("decharge"));
        assert_eq!(normalize_bin_name("dump"), Some("decharge"));
        assert_eq!(normalize_bin_name("landfill"), Some("decharge"));
        assert_eq!(normalize_bin_name("invalid"), None);

        assert_eq!(parse_recycle_args("#42 jaune"), Some((42, "jaune".to_string())));
        assert_eq!(parse_recycle_args("100 green"), Some((100, "green".to_string())));
        assert_eq!(parse_recycle_args("#42"), None);
        assert_eq!(parse_recycle_args("jaune"), None);
        assert_eq!(parse_recycle_args(""), None);
    }
}
