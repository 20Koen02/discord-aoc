use super::aoc_get;
use super::format;
use chrono::Datelike;
use serenity::model::interactions::application_command::ApplicationCommandInteractionDataOptionValue;
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use std::env;

pub async fn aoc_command(command: &ApplicationCommandInteraction) -> String {
    let current_date = chrono::Utc::now();
    let month = current_date.month();
    let default_year: u16 = match month {
        12 => current_date.year() as u16,
        _ => current_date.year() as u16 - 1,
    };

    let year: u16 = match command
        .data
        .options
        .iter()
        .find(|option| option.name == "year")
    {
        Some(option) => match option.resolved.as_ref().expect("Expected year") {
            ApplicationCommandInteractionDataOptionValue::Integer(year) => *year as u16,
            _ => default_year,
        },
        None => default_year,
    };

    let session: String = env::var("AOC_SESSION")
        .expect("Expected a session cookie in the environment")
        .parse()
        .expect("Session is not valid");

    if let Ok(lb) = aoc_get::fetch_leaderboard(635254, year, session).await {
        let mut longest = lb
            .members
            .iter()
            .map(|x| {
                if let Some(name) = &x.name {
                    name.len()
                } else {
                    0
                }
            })
            .max()
            .unwrap_or(0);

        if longest < 9 {
            longest = 9
        }
        let leaderboard = lb
            .members
            .iter()
            .enumerate()
            .map(|(i, member)| {
                let mut medal = "";
                if i == 0 {
                    medal = "🥇";
                } else if i == 1 {
                    medal = "🥈";
                } else if i == 2 {
                    medal = "🥉";
                }

                let name: String = member
                    .name
                    .as_ref()
                    .unwrap_or(&"Anonymous".to_string())
                    .to_string();

                format!(
                    "{}) {} - {} {} {} ⭐ {}",
                    format::add_padding((i + 1).to_string(), 2),
                    format::add_padding(member.local_score.to_string(), 4),
                    name,
                    format::get_dashes(longest, name.len()),
                    format::add_padding(member.stars.to_string(), 2),
                    medal
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        format!(
            "**=== Leaderboard {} ===**```{}```*Created with \\❤️ by Koen02#2933*",
            year, leaderboard
        )
    } else {
        "Not found".to_string()
    }
}
