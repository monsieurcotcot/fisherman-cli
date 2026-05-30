use std::sync::Arc;
use chrono::Utc;
use rand::seq::SliceRandom;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::ClientConfig;
use twitch_irc::TwitchIRCClient;
use twitch_irc::message::ServerMessage;
use twitch_irc::SecureTCPTransport;

use crate::{AppState, PendingSale, PendingTrade};
use crate::game::{generate_fish, generate_junk};
use crate::config::get_fail_attempt_reasons;

#[derive(Debug, PartialEq, Clone)]
pub enum SellArg {
    ConfirmYes,
    ConfirmNo,
    ById(i64),
    ByName {
        name: String,
        state: Option<String>,
        quantity: i64,
    },
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
                    let already_claimed = {
                        let cache = state_task_daily.daily_reward_cache.read().await;
                        cache.get(&username_daily) == Some(&today)
                    };

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
                                    if last_time.date_naive() == today - chrono::Duration::days(1) {
                                        // Streak continues
                                        consecutive += 1;
                                        total += 1;
                                    } else {
                                        // Streak broken
                                        consecutive = 1;
                                        total += 1;
                                    }
                                } else {
                                    // First login reward
                                    consecutive = 1;
                                    total = 1;
                                }

                                // Capped multiplier at 10 days
                                let consecutive_capped = consecutive.min(10);
                                let reward_gold = 200 + 50 * consecutive_capped as i64 + 10 * total as i64;

                                if let Ok(_) = state_task_daily.repo.claim_daily_reward(player.id.unwrap(), consecutive, total, reward_gold).await {
                                    let _ = client_msg_daily.say(
                                        channel_login_daily,
                                        format!(
                                            "🎁 @{} vient de se connecter ! Il reçoit son bonus quotidien de {} po 🪙 (Série : {} jours d'affilée 🔥, Cumul : {} jours total) !",
                                            username_daily, reward_gold, consecutive, total
                                        )
                                    ).await;
                                }
                            }

                            // 3. Populate RAM cache
                            let mut cache = state_task_daily.daily_reward_cache.write().await;
                            cache.insert(username_daily, today);
                        }
                    }
                });
                
                if text == "!fish help" || text == "!pêche help" || text == "!peche help" {
                    let mut help_msg = "📖 !fish | stats | top | list <nom> | info <nom> | sell <nom/ID> | trade #id | coinflip <montant> | Tape !fish help sell ou !fish help trade ou !fish help coinflip pour les détails".to_string();
                    if username == "monsieurcotcot" {
                        help_msg.push_str(" | 🛠️ Admin: !admin backup | !admin restore | !admin season_reset <nom_saison> | !fish reset <pseudo> | !fish simulate <pseudo> <n> | !fish purge");
                    }
                    let _ = client.say(msg.channel_login.clone(), help_msg).await;
                } else if text.starts_with("!fish help ") || text.starts_with("!peche help ") || text.starts_with("!pêche help ") {
                    let sub = text.split_whitespace().skip(2).collect::<Vec<&str>>().join(" ");
                    let reply = match sub.as_str() {
                        "sell" | "vendre" | "vends" | "vend" => {
                            "💰 Vente : !fish sell <nom/id_poisson> [état] [qté]. Ex : '!fish sell bar pristine 1' ou '!fish sell #42'. Si l'état est omis, vend les plus abîmés en premier. Confirme par '!fish sell oui' (durée 1m).".to_string()
                        },
                        "trade" | "echange" | "échanger" | "echanger" => {
                            "🤝 Échanges : 1) Vente Directe : '!fish trade #id_catch prix @destinataire' (Ex: !fish trade #15 250 @pseudo). 2) Troc : Initié par '!fish trade #id_A @destinataire', puis le destinataire propose un contre-troc '!fish trade #id_B @pseudo', suivi des validations.".to_string()
                        },
                        "coinflip" | "cf" => {
                            "🪙 Coinflip : '!fish coinflip <montant>' ou '!fish coinflip all'. Tentez de doubler vos pièces d'or sur un coup de pile ou face ! Mise minimale : 10 po 🪙.".to_string()
                        },
                        _ => format!("📖 Commande inconnue. Utilise '!fish help' ou '!fish help sell' ou '!fish help trade' ou '!fish help coinflip' pour plus de détails.")
                    };
                    let _ = client.say(msg.channel_login.clone(), reply).await;
                } else if text == "!fish stats" || text == "!fish stat" || text == "!peche stats" || text == "!pêche stats" {
                    let state_task = Arc::clone(&state_clone);
                    let client_msg = client.clone();
                    let channel_login = msg.channel_login.clone();
                    let base_url = std::env::var("REDIRECT_URI").unwrap_or_default().replace("/auth/callback", "");
                    tokio::spawn(async move {
                        if let Ok(p) = state_task.repo.get_or_create_player(&username).await {
                            let fish_count = p.successful_attempts - p.junk_count - p.banana_count - p.postcard_count - p.gem_count;
                            
                            // Récupérer et formater les badges de trophées éternels
                            let mut badges = Vec::new();
                            if let Ok(trophies) = state_task.repo.get_player_trophies(p.id.unwrap()).await {
                                for t in trophies {
                                    let emoji = match t.trophy_tier.as_str() {
                                        "Obsidienne" => "🌌",
                                        "Diamant" => "❄️",
                                        "Platinium" => "💎",
                                        "Or" => "🥇",
                                        "Argent" => "🥈",
                                        "Bronze" => "🥉",
                                        "Night" => "🌙",
                                        "Voleur" => "🍌",
                                        "Eboueur" => "🧹",
                                        "Divin" => "👑",
                                        _ => "🏆",
                                    };
                                    let season_clean = if let Some(idx) = t.season.find(" (") {
                                        &t.season[..idx]
                                    } else {
                                        &t.season
                                    };
                                    let season_display = season_clean.replace("Saison ", "");
                                    badges.push(format!("[{} {}]", emoji, season_display));
                                }
                            }
                            let badge_prefix = if !badges.is_empty() {
                                format!("{} ", badges.join(" "))
                            } else {
                                "".to_string()
                            };

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
                                "{}📊 @{} : Niv. {} (XP: {}/{}) | {} 🪙 | 🏛️ Musée: 🐟 {}% • 🗑️ {}% | {} 🐟 | {} 🗑️ | {} 🍌 | {} 💎 | {} 📜{} | Détails : {}/player/{}", 
                                badge_prefix, username, p.level, p.xp, p.xp_for_next_level(), p.gold, fish_percent, junk_percent, fish_count, p.junk_count, p.banana_count, p.gem_count, p.postcard_count, gambling_str, base_url, username
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
                                let _ = client_msg.say(channel_login, "🎰 Aucun joueur n'a encore pris de risque au Coinflip cette saison !".to_string()).await;
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

                        // Tirage aléatoire (49% de chances de gagner)
                        use rand::Rng;
                        let win = rand::thread_rng().gen_range(0.0..100.0) <= 49.0;

                        // Mettre à jour l'or en DB de manière atomique
                        match state_task.repo.record_coinflip_result(player.id.unwrap(), wager_amount, win).await {
                            Ok(updated_player) => {
                                let new_gold = updated_player.gold;
                                if win {
                                    let mut msg_text = format!(
                                        "🪙 @{} lance une pièce... GAGNÉ ! 🔴 (+{} po) ! Tu as maintenant {} pièces d'or 🪙 !",
                                        username, wager_amount, new_gold
                                    );
                                    if updated_player.coinflip_current_win_streak >= 3 {
                                        msg_text.push_str(&format!(" 🔥 SÉRIE DE {} VICTOIRES D'AFFILÉE ! 🟥", updated_player.coinflip_current_win_streak));
                                    }
                                    let _ = client_msg.say(channel_login, msg_text).await;
                                } else {
                                    let mut msg_text = format!(
                                        "🪙 @{} lance une pièce... PERDU ! ⚪ (-{} po) ! Tu as maintenant {} pièces d'or 🪙 !",
                                        username, wager_amount, new_gold
                                    );
                                    if updated_player.coinflip_current_loss_streak >= 3 {
                                        msg_text.push_str(&format!(" 💀 SÉRIE DE {} DÉFAITES D'AFFILÉE... ⬜", updated_player.coinflip_current_loss_streak));
                                    }
                                    let _ = client_msg.say(channel_login, msg_text).await;
                                }
                            }
                            Err(e) => {
                                tracing::error!("Failed to update player gold in coinflip: {:?}", e);
                                let _ = client_msg.say(
                                    channel_login,
                                    format!("⚠️ @{}, une erreur technique est survenue lors de la mise à jour de tes pièces d'or.", username)
                                ).await;
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
                            let _ = client_msg.say(channel_login, format!("⚠️ @{}, usage : !fish sell [poisson] [état] [qté], ou !fish sell #[id_capture]", username)).await;
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
                } else if text.starts_with("!fish reset") {
                    let state_task = Arc::clone(&state_clone);
                    let client_msg = client.clone();
                    let channel_login = msg.channel_login.clone();
                    let args: Vec<String> = text.split_whitespace().map(|s| s.to_string()).collect();

                    tokio::spawn(async move {
                        if args.len() >= 3 && args[2] == "all" {
                            state_task.pending_resets_all.write().await.insert(username.clone(), Utc::now());
                            let _ = client_msg.say(channel_login, format!("⚠️ @{}, tape !fish yes all pour confirmer ton reset COMPLET (stats actives ET trophées éternels seront supprimés définitivement !).", username)).await;
                        } else if args.len() >= 3 && username == "monsieurcotcot" {
                            let target = args[2].to_lowercase();
                            if let Ok(_) = state_task.repo.reset_player(&target).await {
                                // Clear RAM cache for the target to allow claiming daily reward / resetting rate limit
                                state_task.daily_reward_cache.write().await.remove(&target);
                                state_task.rate_limiter.write().await.remove(&target);
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
                            let _ = client_msg.say(channel_login, format!("⚠️ @{}, tape !fish yes pour confirmer ton propre reset (les trophées éternels seront conservés).", username)).await;
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
                                    state_task.daily_reward_cache.write().await.remove(&username);
                                    state_task.rate_limiter.write().await.remove(&username);
                                    state_task.pending_sales.write().await.remove(&username);
                                    state_task.pending_trades.write().await.retain(|t| match t {
                                        PendingTrade::Direct { seller_username, buyer_username, .. } => {
                                            seller_username != &username && buyer_username != &username
                                        }
                                        PendingTrade::Barter { player_a_username, player_b_username, .. } => {
                                            player_a_username != &username && player_b_username != &username
                                        }
                                    });
                                    let _ = client_msg.say(channel_login, format!("💥 @{}, reset total réussi ! Tous vos trophées éternels et statistiques actives ont été supprimés.", username)).await;
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
                                    state_task.daily_reward_cache.write().await.remove(&username);
                                    state_task.rate_limiter.write().await.remove(&username);
                                    state_task.pending_sales.write().await.remove(&username);
                                    state_task.pending_trades.write().await.retain(|t| match t {
                                        PendingTrade::Direct { seller_username, buyer_username, .. } => {
                                            seller_username != &username && buyer_username != &username
                                        }
                                        PendingTrade::Barter { player_a_username, player_b_username, .. } => {
                                            player_a_username != &username && player_b_username != &username
                                        }
                                    });
                                    let _ = client_msg.say(channel_login, format!("♻️ @{}, reset réussi ! Vos statistiques actives de saison ont été réinitialisées (trophées conservés).", username)).await;
                                }
                                pending.remove(&username);
                            }
                        }
                    });
                } else if text.starts_with("!admin season_reset ") && username == "monsieurcotcot" {
                    let state_task = Arc::clone(&state_clone);
                    let client_msg = client.clone();
                    let channel_login = msg.channel_login.clone();
                    let args: Vec<String> = msg.message_text.split_whitespace().map(|s| s.to_string()).collect();
                    tokio::spawn(async move {
                        if args.len() >= 3 {
                            let season_name = args[2..].join(" ");
                            if let Ok(_) = state_task.repo.archive_and_reset_season(&season_name).await {
                                // Clear RAM caches
                                state_task.daily_reward_cache.write().await.clear();
                                state_task.rate_limiter.write().await.clear();
                                state_task.pending_sales.write().await.clear();
                                state_task.pending_trades.write().await.clear();
                                state_task.pending_resets.write().await.clear();
                                state_task.pending_resets_all.write().await.clear();
                                let _ = client_msg.say(channel_login, format!("🏆 La Saison '{}' est close ! Les exploits ont été gravés éternellement, place à la nouvelle saison ! 🎣", season_name)).await;
                            } else {
                                let _ = client_msg.say(channel_login, "❌ [Admin] Erreur lors de la réinitialisation de la saison.".to_string()).await;
                            }
                        } else {
                            let _ = client_msg.say(channel_login, "⚠️ [Admin] Syntaxe : !admin season_reset <nom_saison>".to_string()).await;
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
                                state_task.daily_reward_cache.write().await.clear();
                                state_task.rate_limiter.write().await.clear();
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
                                    state_task.daily_reward_cache.write().await.clear();
                                    state_task.rate_limiter.write().await.clear();
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
                            
                            // Vérification du coût en or (10 po requis)
                            if player.gold < 10 && !is_admin && !is_test {
                                let _ = client_msg.say(
                                    channel_login,
                                    format!(
                                        "⚠️ @{}, tu n'as pas assez de pièces d'or pour pêcher (requis: 10 po, tu as {} po). Écris un message sur le live demain pour obtenir ton bonus quotidien ou vends des poissons via !fish sell !",
                                        username, player.gold
                                    )
                                ).await;
                                return;
                            }

                            if player.can_fish() || is_test || is_admin {
                                let level_factor = (player.level as f64 - 1.0) / 199.0;
                                let success_rate = 0.35 + (level_factor * 0.20);
                                let junk_rate = 0.05;
                                let roll = rand::random::<f64>();

                                if is_test || roll < success_rate {
                                    let mut fish = if is_test { crate::models::Fish::new("Gemme VIP (TEST)".to_string(), crate::config::Rarity::Legendary, 1.0, 100.0, "pristine".to_string(), "Gemme de test.".to_string()) } 
                                                   else { match generate_fish(state_task.use_english) { Some(f) => f, None => return } };

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
                                     let mut resp = format!("🐟 @{} a pêché un(e) {} ({} cm) ! {}", username, fish.name, fish.size, fish.description);
                                     if fish.name == "Gemme VIP" || is_test { 
                                         let d = if is_test { "1 MIN" } else { match fish.state.as_str() { "pristine" => "4H", "good" => "80 MIN", "worn" => "60 MIN", "damaged" => "40 MIN", _ => "20 MIN" } };
                                         resp.push_str(&format!(" 🌟 TU ES VIP PENDANT {} ! 🌟", d)); 
                                     }
                                     if fish.name == "Pristine Banana 1" || fish.name == "Pristine Banana 2" {
                                         if let Some(player_id) = player.id {
                                             if let Ok(stolen_from) = state_task.repo.check_and_execute_banana_theft(player_id, &fish.name).await {
                                                 if let Some(old_user) = stolen_from {
                                                     resp.push_str(&format!(" 🍌 @{} a VOLÉ la {} à @{} ! 🤫", username, fish.name, old_user));
                                                 }
                                             }
                                             let other_banana = if fish.name == "Pristine Banana 1" { "Pristine Banana 2" } else { "Pristine Banana 1" };
                                             if let Ok(has_other) = state_task.repo.has_caught_banana(player_id, other_banana).await {
                                                 if has_other {
                                                     if let Ok(already_king) = state_task.repo.is_active_king(player_id).await {
                                                         if !already_king {
                                                             resp.push_str(&format!(" 👑 @{} devient le nouveau ROI DES BANANES ! 👑", username));
                                                         }
                                                     }
                                                 }
                                             }
                                         }
                                     }
                                     if leveled_up { resp.push_str(&format!(" ✨ LEVEL UP ! Niv. {} !", player.level)); }
                                     let _ = client_msg.say(channel_login.clone(), resp).await;

                                    let state_bg = state_task.clone();
                                    let ch_bg = channel_login.clone();
                                    tokio::spawn(async move {
                                        if let Some(t) = state_bg.auth.load_tokens() { fish.stream_title = state_bg.auth.get_stream_info(&ch_bg, &t.access_token).await; }
                                        let _ = state_bg.repo.save_attempt(&player, true, Some(fish)).await;
                                    });
                                } else if roll < success_rate + junk_rate {
                                    if let Some(mut junk) = generate_junk(state_task.use_english) {
                                        let leveled_up = player.add_xp(5);
                                        let mut resp = format!("🗑️ @{} a remonté un déchet : {} ! {}", username, junk.name, junk.description);
                                        if junk.rarity != crate::config::Rarity::Common { resp.push_str(&format!(" (Rareté: {:?})", junk.rarity)); }

                                        if leveled_up { resp.push_str(&format!(" ✨ LEVEL UP ! Niv. {} !", player.level)); }
                                        let _ = client_msg.say(channel_login.clone(), resp).await;

                                        let state_bg = state_task.clone();
                                        let ch_bg = channel_login.clone();
                                        tokio::spawn(async move {
                                            if let Some(t) = state_bg.auth.load_tokens() { junk.stream_title = state_bg.auth.get_stream_info(&ch_bg, &t.access_token).await; }
                                            let _ = state_bg.repo.save_attempt(&player, true, Some(junk)).await;
                                        });
                                    }
                                } else {
                                    let reasons = get_fail_attempt_reasons(state_task.use_english);
                                    let reason = reasons.choose(&mut rand::thread_rng()).unwrap_or(&"Pas de chance !").replace("#viewer_name#", &username);
                                    let leveled_up = player.add_xp(5);
                                    let mut resp = reason;
                                    if leveled_up { resp.push_str(&format!(" ✨ LEVEL UP ! Niv. {} !", player.level)); }
                                    let _ = client_msg.say(channel_login, resp).await;
                                    let _ = state_task.repo.save_attempt(&player, false, None).await;
                                }
                            } else {
                                let rem = player.get_remaining_cooldown();
                                if let Some(id) = player.id { let _ = state_task.repo.add_cooldown_penalty(id).await; }
                                let _ = client_msg.say(channel_login, format!("⏳ @{}, attends encore {}s ! (+5s penalty) ⚠️", username, rem + 5)).await;
                            }
                        }
                    });
                }
            }
        }
    });
    *abort_lock = Some(handle);
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
}
