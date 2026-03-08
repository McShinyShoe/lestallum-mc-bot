use std::collections::HashSet;

use azalea::{chat::ChatPacket, prelude::*};

use crate::{app_config::config, bot::app_state::State};

fn parse_list(message: &str) -> anyhow::Result<HashSet<String>> {
    Ok(message
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect())
}

fn parse_towny_list(message: &str) -> anyhow::Result<(String, usize, HashSet<String>)> {
    let (raw_key, list_message) = message
        .split_once(':')
        .ok_or_else(|| anyhow::anyhow!("Invalid towny list format"))?;

    let raw_key = raw_key.trim();

    let (key, count) = if let Some((name, rest)) = raw_key.split_once('[') {
        let number = rest
            .strip_suffix(']')
            .ok_or_else(|| anyhow::anyhow!("Invalid bracket format"))?
            .parse::<usize>()?;

        (name.trim(), number)
    } else {
        (raw_key, 1)
    };

    let list = parse_list(list_message)?;

    Ok((key.to_string(), count, list))
}

async fn parse_towny_list_cleaned(
    message: &str,
    state: &State,
) -> anyhow::Result<(String, usize, HashSet<String>)> {
    let (key, count, list) = parse_towny_list(message)?;

    let mut cleaned = HashSet::with_capacity(list.len());

    for name in list {
        let cleaned_name = clean_resident_name(&name, state).await?;
        cleaned.insert(cleaned_name);
    }

    Ok((key, count, cleaned))
}

async fn clean_resident_name(name: &str, state: &State) -> anyhow::Result<String> {
    let cleaned: String = name
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '*' | ' '))
        .collect::<String>()
        .trim()
        .to_string();

    if !cleaned.contains(' ') {
        return Ok(cleaned);
    }

    let residents = state.town_residents.lock().await.clone();

    cleaned
        .split_whitespace()
        .find(|word| residents.contains(*word))
        .map(|w| w.to_string())
        .ok_or_else(|| anyhow::anyhow!("Failed to clean resident name"))
}

pub async fn handle_chat(bot: &Client, state: &State, chat: ChatPacket) -> anyhow::Result<()> {
    let message_ansi = chat.message().to_ansi();
    let message = chat.message().to_string();
    tracing::info!("[CHAT] {}", message_ansi);

    if message.starts_with("✉ [MSG]") {
        let content = message
            .strip_prefix("✉ [MSG] ")
            .ok_or_else(|| anyhow::anyhow!("Invalid DM format: missing prefix"))?;

        let (from, rest) = content
            .split_once(" → ")
            .ok_or_else(|| anyhow::anyhow!("Invalid DM format: missing arrow"))?;
        let (to, msg) = rest
            .split_once(' ')
            .ok_or_else(|| anyhow::anyhow!("Invalid DM format: missing message"))?;

        on_message_dm(&bot, &state, from, to, msg)?;
    }

    if message.contains("[+] Lestallum") {
        {
            *state.on_towny.lock().await = true;
        }
        tracing::info!("Bot joinde towny!");
        bot.chat("/pvp on");
        bot.chat("/t");
        bot.chat("/t reslist");
        bot.chat("/t ranklist");
        bot.chat("/msg Lestallum INIT_END");
    }

    if message.starts_with("Residents") {
        let (_, _, parsed) = parse_towny_list(&message)?;
        let mut residents = state.town_residents.lock().await;
        residents.extend(parsed);
    }
    if message.starts_with("Mayor") {
        let (_, _, parsed) = parse_towny_list_cleaned(&message, &state).await?;
        *state.town_mayor.lock().await = parsed
            .iter()
            .next()
            .ok_or_else(|| anyhow::anyhow!("Failed to parse towny list, its empty"))?
            .to_owned();
    }
    if message.starts_with("Co-mayor") {
        let (_, _, parsed) = parse_towny_list_cleaned(&message, &state).await?;
        state.town_comayors.lock().await.extend(parsed);
    }
    if message.starts_with("Assistant") {
        let (_, _, parsed) = parse_towny_list_cleaned(&message, &state).await?;
        state.town_assistants.lock().await.extend(parsed);
    }
    if message.starts_with("Helper") {
        let (_, _, parsed) = parse_towny_list_cleaned(&message, &state).await?;
        state.town_helpers.lock().await.extend(parsed);
    }
    if message.starts_with("Recruiter") {
        let (_, _, parsed) = parse_towny_list_cleaned(&message, &state).await?;
        state.town_recruiters.lock().await.extend(parsed);
    }
    if message.starts_with("Builder") {
        let (_, _, parsed) = parse_towny_list_cleaned(&message, &state).await?;
        state.town_builders.lock().await.extend(parsed);
    }
    if message.starts_with("Vip") {
        let (_, _, parsed) = parse_towny_list_cleaned(&message, &state).await?;
        state.town_vips.lock().await.extend(parsed);
    }
    if message.starts_with("Sheriff") {
        let (_, _, parsed) = parse_towny_list_cleaned(&message, &state).await?;
        state.town_sheriffs.lock().await.extend(parsed);
    }
    if message.starts_with("Trusted") {
        let (_, _, parsed) = parse_towny_list(&message)?;
        state.town_trusteds.lock().await.extend(parsed);
    }
    Ok(())
}

fn on_message_dm(
    bot: &Client,
    state: &State,
    from: &str,
    to: &str,
    msg: &str,
) -> anyhow::Result<()> {
    tracing::info!("Got message from {} to {}: {}", from, to, msg);
    let sudo_player = &config().sudo_player;
    if let Some(player) = sudo_player {
        if from == player {
            if msg.contains("{}") {
                for (_uuid, info) in bot.tab_list() {
                    let msg = msg.replace("{}", info.profile.name.as_str());
                    bot.chat(msg);
                }
            } else {
                bot.chat(msg);
            }
        }
        return Ok(());
    }

    bot.chat(format!(
        "/msg {} Hi! this is a bot, and your message goes to nowhere ...",
        from
    ));
    Ok(())
}
