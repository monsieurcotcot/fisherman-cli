use std::sync::Arc;
use chrono::Utc;
use rand::seq::SliceRandom;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::ClientConfig;
use twitch_irc::TwitchIRCClient;
use twitch_irc::message::ServerMessage;
use twitch_irc::SecureTCPTransport;

use crate::AppState;
use crate::game::{generate_fish, generate_junk};
use crate::config::get_fail_attempt_reasons;

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
                
                if text == "!fish help" || text == "!pêche help" || text == "!peche help" {
                    let mut help_msg = "📖 !fish | !pêche | !fish stats | !fish top | !fish reset | !fish reset all".to_string();
                    if username == "monsieurcotcot" {
                        help_msg.push_str(" | 🛠️ Admin: !admin backup | !admin restore | !admin season_reset <nom_saison> | !fish reset <pseudo> | !fish simulate <pseudo> <n> | !fish purge");
                    }
                    let _ = client.say(msg.channel_login.clone(), help_msg).await;
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

                            let msg_str = format!(
                                "{}📊 @{} : Niv. {} (XP: {}/{}) | {} 🐟 | {} 🗑️ | {} 🍌 | {} 💎 | {} 📜 | Détails : {}/player/{}", 
                                badge_prefix, username, p.level, p.xp, p.xp_for_next_level(), fish_count, p.junk_count, p.banana_count, p.gem_count, p.postcard_count, base_url, username
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
                                format!("#{}. {} (Niv. {} | {} 🐟 | {} 🗑️ | {} 🍌 | {} 💎 | {} 📜)", i + 1, p.username, p.level, fish_count, p.junk_count, p.banana_count, p.gem_count, p.postcard_count)
                            }).collect();
                            let _ = client_msg.say(channel_login, format!("🏆 Top Pêcheurs : {}", list.join(" | "))).await;
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
                            if let Ok(mut player) = state_task.repo.get_or_create_player(&target_user).await {
                                let mut success_count = 0;
                                let mut junk_count = 0;
                                let mut fail_count = 0;

                                for _ in 0..count {
                                    let level_factor = (player.level as f64 - 1.0) / 199.0;
                                    let success_rate = 0.35 + (level_factor * 0.20);
                                    let junk_rate = 0.05;
                                    let roll = rand::random::<f64>();

                                    if roll < success_rate {
                                        if let Some(fish) = generate_fish() {
                                            success_count += 1;
                                            player.add_xp(25);
                                            let _ = state_task.repo.save_attempt(&player, true, Some(fish)).await;
                                        }
                                    } else if roll < success_rate + junk_rate {
                                        if let Some(junk) = generate_junk() {
                                            junk_count += 1;
                                            player.add_xp(5);
                                            let _ = state_task.repo.save_attempt(&player, true, Some(junk)).await;
                                        }
                                    } else {
                                        fail_count += 1;
                                        player.add_xp(5);
                                        let _ = state_task.repo.save_attempt(&player, false, None).await;
                                    }
                                }
                                let _ = client_msg.say(channel_login, format!("✅ Simulation terminée pour @{} : {} poissons, {} déchets, {} échecs. Niv. {}", target_user, success_count, junk_count, fail_count, player.level)).await;
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
                                                if let Some(fish) = generate_fish() {
                                                    let _ = state_task.repo.save_catch_only(player_id, fish).await;
                                                    success_count += 1;
                                                }
                                            } else if r < (success_chance + junk_chance) {
                                                if let Some(junk) = generate_junk() {
                                                    let _ = state_task.repo.save_catch_only(player_id, junk).await;
                                                    success_count += 1;
                                                }
                                            } else {
                                                fail_count += 1;
                                            }
                                        }
                                        let _ = state_task.repo.update_player_stats_after_restore(player_id, success_count, fail_count).await;
                                    }
                                }
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
                            if player.can_fish() || is_test || is_admin {
                                let level_factor = (player.level as f64 - 1.0) / 199.0;
                                let success_rate = 0.35 + (level_factor * 0.20);
                                let junk_rate = 0.05;
                                let roll = rand::random::<f64>();

                                if is_test || roll < success_rate {
                                    let mut fish = if is_test { crate::models::Fish::new("Gemme VIP (TEST)".to_string(), crate::config::Rarity::Legendary, 1.0, 100.0, "pristine".to_string(), "Gemme de test.".to_string()) } 
                                                   else { match generate_fish() { Some(f) => f, None => return } };

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
                                                     resp.push_str(&format!(" 👑 @{} devient le nouveau ROI DES BANANES ! 👑", username));
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
                                    if let Some(mut junk) = generate_junk() {
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
                                    let reasons = get_fail_attempt_reasons();
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
