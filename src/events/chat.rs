use std::collections::HashSet;

use azalea::{chat::ChatPacket, prelude::*};

use crate::{app_config::config, app_state::State};

fn parse_list(message: &String) -> HashSet<String> {
    message
        .splitn(2, ':')
        .nth(1)
        .unwrap_or("")
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect()
}

async fn parse_list_cleaned(message: &String, state: &State) -> HashSet<String> {
    let mut parsed = HashSet::new();

    let part = message.splitn(2, ':').nth(1).unwrap_or("");

    for s in part.split(',').map(str::trim).filter(|s| !s.is_empty()) {
        if let Some(cleaned) = clean_resident_name(s, &state).await {
            parsed.insert(cleaned);
        }
    }
    parsed
}

async fn clean_resident_name(name: &str, state: &State) -> Option<String> {
    let cleaned: String = name
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '*' | ' '))
        .collect::<String>()
        .trim()
        .to_string();

    if !cleaned.contains(' ') {
        return Some(cleaned);
    }

    let residents = state.town_residents.lock().await.clone();

    cleaned
        .split_whitespace()
        .find(|word| residents.contains(*word))
        .map(|w| w.to_string())
}

pub async fn handle_chat(bot: &Client, state: &State, chat: ChatPacket) -> anyhow::Result<()> {
    let message_ansi = chat.message().to_ansi();
    let message = chat.message().to_string();
    println!("[CHAT] {}", message_ansi);

    let sudo_player = &config().sudo_player;
    if let Some(player) = sudo_player {
        if message.starts_with("✉ [MSG]") && message.contains(format!("{} →", player).as_str()) {
            let after_arrow = message.split_once("→ ").unwrap().1;
            let msg = after_arrow.split_once(' ').unwrap().1;
    
            if msg.contains("{}") {
                for (_uuid, info) in bot.tab_list() {
                    let msg = msg.replace("{}", info.profile.name.as_str());
                    bot.chat(msg);
                }
            } else {
                bot.chat(msg);
            }
        }
    }

    if message.contains("[+] Lestallum") {
        {
            *state.on_towny.lock().await = true;
        }
        println!("[LOG] Bot joined Towny!");
        bot.chat("/pvp on");
        bot.chat("/t");
        bot.chat("/t reslist");
        bot.chat("/t ranklist");
        bot.chat("/msg Lestallum INIT_END");
    }

    if message.starts_with("Residents") {
        let parsed = parse_list(&message);
        let mut residents = state.town_residents.lock().await;
        residents.extend(parsed);
    }
    if message.starts_with("Mayor") {
        let name = message.splitn(2, ':').nth(1).map(str::trim).unwrap();
        let name = clean_resident_name(name, &state).await.unwrap();
        *state.town_mayor.lock().await = name;
    }
    if message.starts_with("Co-mayor") {
        let parsed = parse_list_cleaned(&message, &state).await;
        state.town_comayors.lock().await.extend(parsed);
    }
    if message.starts_with("Assistant") {
        let parsed = parse_list_cleaned(&message, &state).await;
        state.town_assistants.lock().await.extend(parsed);
    }
    if message.starts_with("Helper") {
        let parsed = parse_list_cleaned(&message, &state).await;
        state.town_helpers.lock().await.extend(parsed);
    }
    if message.starts_with("Recruiter") {
        let parsed = parse_list_cleaned(&message, &state).await;
        state.town_recruiters.lock().await.extend(parsed);
    }
    if message.starts_with("Builder") {
        let parsed = parse_list_cleaned(&message, &state).await;
        state.town_builders.lock().await.extend(parsed);
    }
    if message.starts_with("Vip") {
        let parsed = parse_list_cleaned(&message, &state).await;
        state.town_vips.lock().await.extend(parsed);
    }
    if message.starts_with("Sheriff") {
        let parsed = parse_list_cleaned(&message, &state).await;
        state.town_sheriffs.lock().await.extend(parsed);
    }
    if message.starts_with("Trusted") {
        let parsed = parse_list(&message);
        state.town_trusteds.lock().await.extend(parsed);
    }
    Ok(())
}
