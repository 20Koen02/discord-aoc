use reqwest::header::COOKIE;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Member {
    pub name: Option<String>,
    pub local_score: u32,
    pub stars: u32,
}

#[derive(Deserialize, Debug)]
struct ParseLeaderboard {
    members: HashMap<String, Member>,
}

pub struct Leaderboard {
    pub members: Vec<Member>,
}

pub enum FetchError {
    Reqwest(reqwest::Error),
    JsonParse(serde_json::Error),
}

pub async fn fetch_leaderboard(
    lb_id: u32,
    year: u16,
    session: String,
) -> Result<Leaderboard, FetchError> {
    let url = format!(
        "https://adventofcode.com/{}/leaderboard/private/view/{}.json",
        year, lb_id
    );
    let cookie = format!("session={}", session);

    let client = reqwest::Client::new();
    let result = client.get(&url).header(COOKIE, cookie).send().await;

    let response = match result {
        Ok(res) => res,
        Err(err) => {
            println!("{:?}", err);
            return Err(FetchError::Reqwest(err));
        }
    };

    // Get json from response
    let json = match response.text().await {
        Ok(json) => json,
        Err(err) => {
            println!("{:?}", err);
            return Err(FetchError::Reqwest(err));
        }
    };

    // Parse json
    let leaderboard: ParseLeaderboard = match serde_json::from_str(&json) {
        Ok(lb) => lb,
        Err(err) => {
            println!("{:?}", err);
            return Err(FetchError::JsonParse(err));
        }
    };

    let mut members: Vec<Member> = leaderboard.members.into_values().collect();

    members.sort_by_key(|x| x.local_score);
    members.reverse();

    let lb: Leaderboard = Leaderboard {
        members,
    };

    Ok(lb)
}
